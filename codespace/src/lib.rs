//! A structured container for generated Rust code.
//!
//! Code generators that emit raw [`TokenStream`]s face a practical problem:
//! a single type definition often requires multiple top-level items — the
//! struct or enum itself, helper functions, `impl` blocks, and so on. Putting
//! all of those into one `TokenStream` makes it hard to keep related items
//! together or to route ancillary items (e.g. serde default helpers) into a
//! dedicated module.
//!
//! `codespace` solves this by providing a tree-shaped accumulator. A
//! [`Codespace`] holds a root [`Mod`]; each [`Mod`] can contain any mix of
//! named items (raw [`TokenStream`] fragments) and named sub-[`Mod`]s.
//! When you're done building, [`Codespace::into_stream`] flattens everything
//! into a single [`TokenStream`] suitable for writing to a file or handing to
//! a proc-macro output.
//!
//! ## Ordering
//!
//! Entries within a [`Mod`] are kept in a [`BTreeMap`], so output is
//! deterministic and sorted alphabetically by name. Because Rust identifiers
//! are case-sensitive and ASCII uppercase sorts before lowercase, PascalCase
//! type names (e.g. `"Foo"`) appear before snake_case module names (e.g.
//! `"defaults"`) — a natural convention for generated code.
//!
//! ## Example
//!
//! ```rust
//! use codespace::Codespace;
//! use quote::quote;
//!
//! let mut cs = Codespace::default();
//!
//! // Add a struct to the root.
//! cs.add_item("Status", quote! {
//!     #[derive(serde::Serialize, serde::Deserialize)]
//!     pub enum Status { Active, Inactive }
//! });
//!
//! // Add a serde default helper to a "defaults" submodule.
//! cs.get_mod("defaults").add_item("status_default", quote! {
//!     pub fn status_default() -> Status { Status::Active }
//! });
//!
//! // Renders to:
//! //   pub enum Status { Active, Inactive }
//! //   pub mod defaults {
//! //       pub fn status_default() -> Status { Status::Active }
//! //   }
//! let _tokens = cs.into_stream();
//! ```

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// A structured collection of generated Rust items, organized into a tree of
/// modules.
///
/// `Codespace` is the entry point into the tree. It owns a root [`Mod`] whose
/// contents are emitted flat (with no surrounding `mod` block) when you call
/// [`into_stream`](Self::into_stream). Items and submodules added to the root
/// are ordered alphabetically by name.
///
/// For most use cases you interact with `Codespace` via its convenience
/// methods, which delegate directly to the root [`Mod`]:
///
/// - [`add_item`](Self::add_item) — add a named [`TokenStream`] fragment
/// - [`get_mod`](Self::get_mod) — get or create a named submodule
#[derive(Debug, Default)]
pub struct Codespace {
    root: Mod,
}

impl Codespace {
    /// Add (or extend) a named item in the root module.
    ///
    /// See [`Mod::add_item`] for full semantics.
    pub fn add_item(&mut self, name: impl Into<String>, tokens: TokenStream) {
        self.root.add_item(name, tokens);
    }

    /// Get (or create) a named submodule of the root module.
    ///
    /// See [`Mod::get_mod`] for full semantics.
    pub fn get_mod(&mut self, name: impl Into<String>) -> &mut Mod {
        self.root.get_mod(name)
    }

    /// Consume the codespace and render it into a [`TokenStream`].
    ///
    /// The root module's contents are emitted flat. Each submodule is wrapped
    /// in a `pub mod name { ... }` block. Items and submodules are ordered
    /// alphabetically by name within each level of the tree.
    pub fn into_stream(self) -> TokenStream {
        self.root.into_stream()
    }
}

/// A node in the [`Codespace`] tree.
///
/// A `Mod` holds an ordered collection of named entries. Each entry is either:
///
/// - an **item**: a raw [`TokenStream`] fragment (a type definition, an `impl`
///   block, a function, etc.), or
/// - a **submodule**: a nested `Mod` that will be wrapped in
///   `pub mod name { ... }` when rendered.
///
/// A name may not be used for both an item and a submodule within the same
/// `Mod`; attempting to do so panics (see [`add_item`](Self::add_item) and
/// [`get_mod`](Self::get_mod)).
///
/// Entries are stored in a [`BTreeMap`] and therefore rendered in alphabetical
/// order. Because ASCII uppercase (A–Z, 65–90) sorts before lowercase (a–z,
/// 97–122), PascalCase type names appear before snake_case submodule names in
/// the output.
#[derive(Debug, Default)]
pub struct Mod {
    entries: BTreeMap<String, ModEntry>,
}

#[derive(Debug)]
enum ModEntry {
    Item(TokenStream),
    Submod(Mod),
}

impl Mod {
    /// Add a named [`TokenStream`] item to this module.
    ///
    /// If an item with `name` already exists, `tokens` is appended to it.
    /// This is intentional: it lets callers accumulate multiple fragments
    /// under one logical key — for example, a struct definition followed by
    /// one or more `impl` blocks:
    ///
    /// ```rust
    /// use codespace::Codespace;
    /// use quote::quote;
    ///
    /// let mut cs = Codespace::default();
    /// cs.add_item("Foo", quote! { pub struct Foo(u32); });
    /// cs.add_item("Foo", quote! { impl Foo { pub fn value(&self) -> u32 { self.0 } } });
    /// // Both fragments appear in the output under the same "Foo" key.
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `name` is already occupied by a submodule.
    pub fn add_item(&mut self, name: impl Into<String>, tokens: TokenStream) {
        let name = name.into();
        match self.entries.entry(name.clone()) {
            Entry::Occupied(mut e) => match e.get_mut() {
                ModEntry::Item(existing) => existing.extend(tokens),
                ModEntry::Submod(_) => panic!("'{name}' is already a submodule"),
            },
            Entry::Vacant(e) => {
                e.insert(ModEntry::Item(tokens));
            }
        }
    }

    /// Get (or create) a named submodule.
    ///
    /// If a submodule named `name` already exists it is returned; otherwise a
    /// new empty [`Mod`] is inserted and returned. This makes it safe to call
    /// `get_mod` multiple times with the same name — you always get the same
    /// submodule back:
    ///
    /// ```rust
    /// use codespace::Codespace;
    /// use quote::quote;
    ///
    /// let mut cs = Codespace::default();
    /// cs.get_mod("defaults").add_item("foo_x", quote! { pub fn foo_x() -> u32 { 0 } });
    /// cs.get_mod("defaults").add_item("bar_y", quote! { pub fn bar_y() -> u32 { 1 } });
    /// // Both functions end up in the same "defaults" submodule.
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `name` is already occupied by an item.
    pub fn get_mod(&mut self, name: impl Into<String>) -> &mut Mod {
        let name = name.into();
        let entry = self
            .entries
            .entry(name.clone())
            .or_insert_with(|| ModEntry::Submod(Mod::default()));
        match entry {
            ModEntry::Submod(m) => m,
            ModEntry::Item(_) => panic!("'{name}' is already an item"),
        }
    }

    /// Consume this module and render its contents into a [`TokenStream`].
    ///
    /// The output contains the module's contents directly — there is no
    /// surrounding `mod` block (that is the caller's responsibility for
    /// non-root modules). Items are emitted as-is; submodules are wrapped in
    /// `pub mod name { ... }`.
    ///
    /// Entries appear in alphabetical order by name.
    pub fn into_stream(self) -> TokenStream {
        let mut out = TokenStream::new();
        for (name, entry) in self.entries {
            match entry {
                ModEntry::Item(tokens) => out.extend(tokens),
                ModEntry::Submod(m) => {
                    let ident = format_ident!("{}", name);
                    let contents = m.into_stream();
                    out.extend(quote! {
                        pub mod #ident {
                            #contents
                        }
                    });
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn empty_codespace() {
        let cs = Codespace::default();
        assert!(cs.into_stream().is_empty());
    }

    #[test]
    fn root_items_are_flat() {
        let mut cs = Codespace::default();
        cs.add_item("Foo", quote! { pub struct Foo; });
        cs.add_item("Bar", quote! { pub struct Bar; });
        let out = cs.into_stream().to_string();
        // BTreeMap order: Bar before Foo
        assert!(out.contains("struct Bar"));
        assert!(out.contains("struct Foo"));
        let bar_pos = out.find("struct Bar").unwrap();
        let foo_pos = out.find("struct Foo").unwrap();
        assert!(bar_pos < foo_pos, "expected alphabetical order");
    }

    #[test]
    fn extend_same_name() {
        let mut cs = Codespace::default();
        cs.add_item("Foo", quote! { pub struct Foo; });
        cs.add_item("Foo", quote! { impl Foo {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("struct Foo"));
        assert!(out.contains("impl Foo"));
    }

    #[test]
    fn submod_is_wrapped() {
        let mut cs = Codespace::default();
        cs.add_item("Foo", quote! { pub struct Foo; });
        cs.get_mod("defaults")
            .add_item("foo_x", quote! { pub fn foo_x() -> u32 { 0 } });
        let out = cs.into_stream().to_string();
        assert!(out.contains("pub mod defaults"));
        assert!(out.contains("fn foo_x"));
        // "Foo" (uppercase F=70) sorts before "defaults" (lowercase d=100)
        let foo_pos = out.find("struct Foo").unwrap();
        let mod_pos = out.find("pub mod defaults").unwrap();
        assert!(foo_pos < mod_pos, "root items should precede submodules");
    }

    #[test]
    fn nested_submods() {
        let mut cs = Codespace::default();
        cs.get_mod("outer").get_mod("inner").add_item(
            "deep",
            quote! { pub fn deep() {} },
        );
        let out = cs.into_stream().to_string();
        assert!(out.contains("pub mod outer"));
        assert!(out.contains("pub mod inner"));
        assert!(out.contains("fn deep"));
    }

    #[test]
    fn get_mod_same_name_twice() {
        let mut cs = Codespace::default();
        cs.get_mod("defaults")
            .add_item("a", quote! { pub fn a() {} });
        cs.get_mod("defaults")
            .add_item("b", quote! { pub fn b() {} });
        let out = cs.into_stream().to_string();
        // Both functions should be in the single "defaults" mod.
        let mod_start = out.find("pub mod defaults").unwrap();
        assert!(out[mod_start..].contains("fn a"));
        assert!(out[mod_start..].contains("fn b"));
    }

    #[test]
    #[should_panic(expected = "already a submodule")]
    fn item_name_conflicts_with_submod() {
        let mut cs = Codespace::default();
        cs.get_mod("foo");
        cs.add_item("foo", quote! { pub struct Foo; });
    }

    #[test]
    #[should_panic(expected = "already an item")]
    fn submod_name_conflicts_with_item() {
        let mut cs = Codespace::default();
        cs.add_item("foo", quote! { pub struct Foo; });
        cs.get_mod("foo");
    }
}
