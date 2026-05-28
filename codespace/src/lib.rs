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
//! [`Codespace`] holds a root [`Mod`]; each [`Mod`] holds named items (raw
//! [`TokenStream`] fragments) and named sub-[`Mod`]s in separate maps.
//! When you're done building, [`Codespace::into_stream`] flattens everything
//! into a single [`TokenStream`] suitable for writing to a file or handing to
//! a proc-macro output.
//!
//! ## Paths
//!
//! [`Codespace::add_item`] and [`Mod::add_item`] accept a `"::"` delimited
//! path. All but the last segment name submodules (and must be valid Rust
//! identifiers); the final segment is an arbitrary sort key that is never
//! emitted as a token.
//!
//! ```rust
//! use codespace::Codespace;
//! use quote::quote;
//!
//! let mut cs = Codespace::default();
//! cs.add_item("Status", quote! { pub enum Status { Active, Inactive } });
//! cs.add_item("defaults::status_default", quote! {
//!     pub fn status_default() -> Status { Status::Active }
//! });
//! // Renders to:
//! //   pub enum Status { ... }
//! //   pub mod defaults { pub fn status_default() ... }
//! let _tokens = cs.into_stream();
//! ```
//!
//! ## Ordering
//!
//! Within each [`Mod`], all items are emitted first (in sort-key order),
//! followed by all submodules (in alphabetical order by name).

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// A structured collection of generated Rust items, organized into a tree of
/// modules.
///
/// `Codespace` is the entry point. It owns a root [`Mod`] whose contents are
/// emitted flat (with no surrounding `mod` block) by [`into_stream`].
///
/// - [`add_item`](Self::add_item) — add a [`TokenStream`] fragment at a
///   `"::"` delimited path; intermediate segments become submodules.
/// - [`get_root_mod`](Self::get_root_mod) — borrow the root [`Mod`] directly
///   for finer-grained manipulation.
#[derive(Debug, Default)]
pub struct Codespace {
    root: Mod,
}

impl Codespace {
    /// Add (or extend) an item at the given path.
    ///
    /// `path` is a `"::"` delimited string. All segments except the last must
    /// be valid Rust identifiers (they become `pub mod` names). The final
    /// segment is an arbitrary sort key that is never emitted as a token.
    ///
    /// See [`Mod::add_item`] for the single-level form.
    ///
    /// # Panics
    ///
    /// Panics if any intermediate path segment is not a valid Rust identifier.
    pub fn add_item(&mut self, path: impl Into<String>, tokens: TokenStream) {
        let path = path.into();
        let mut segs = path.split("::").peekable();
        let mut m = &mut self.root;

        loop {
            let seg = segs.next().expect("path must not be empty");
            if segs.peek().is_none() {
                // Last segment is the sort key, not a mod name.
                m.add_item(seg, tokens);
                return;
            }
            m = m.get_mod(seg);
        }
    }

    /// Return a mutable reference to the root [`Mod`].
    pub fn get_root_mod(&mut self) -> &mut Mod {
        &mut self.root
    }

    /// Consume the codespace and render it into a [`TokenStream`].
    ///
    /// The root module's contents are emitted flat. Each submodule is wrapped
    /// in a `pub mod name { ... }` block. Within each level, all items are
    /// emitted first (in sort-key order), then all submodules (alphabetically).
    pub fn into_stream(self) -> TokenStream {
        self.root.into_stream()
    }
}

/// A node in the [`Codespace`] tree.
///
/// A `Mod` holds two independent ordered maps:
///
/// - **items**: [`TokenStream`] fragments keyed by an arbitrary sort string
///   (the key is never emitted as a token), emitted in key order.
/// - **submodules**: named nested [`Mod`]s keyed by valid Rust identifiers,
///   rendered as `pub mod name { ... }` blocks in alphabetical order after
///   all items.
#[derive(Debug, Default)]
pub struct Mod {
    items: BTreeMap<String, TokenStream>,
    mods: BTreeMap<String, Mod>,
}

impl Mod {
    /// Add (or extend) an item in this module under the given sort `key`.
    ///
    /// `key` is an arbitrary string used only for ordering — it is never
    /// emitted as a token and does not need to be a valid Rust identifier.
    ///
    /// If an item already exists under `key`, `tokens` is appended to it,
    /// allowing multiple fragments to accumulate under one key:
    ///
    /// ```rust
    /// use codespace::Codespace;
    /// use quote::quote;
    ///
    /// let mut cs = Codespace::default();
    /// cs.get_root_mod().add_item("Foo", quote! { pub struct Foo(u32); });
    /// cs.get_root_mod().add_item("Foo", quote! { impl Foo { pub fn value(&self) -> u32 { self.0 } } });
    /// // Both fragments appear in the output under the same key.
    /// ```
    pub fn add_item(&mut self, key: impl Into<String>, tokens: TokenStream) {
        self.items
            .entry(key.into())
            .and_modify(|existing| existing.extend(tokens.clone()))
            .or_insert(tokens);
    }

    /// Get (or create) a named submodule.
    ///
    /// If a submodule named `name` already exists it is returned; otherwise a
    /// new empty [`Mod`] is created and returned. Safe to call multiple times
    /// with the same name.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not a valid Rust identifier.
    pub fn get_mod(&mut self, name: impl Into<String>) -> &mut Mod {
        let name = name.into();
        // Validate now so the error points here rather than at render time.
        proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
        self.mods.entry(name).or_default()
    }

    /// Consume this module and render its contents into a [`TokenStream`].
    ///
    /// Items are emitted first in sort-key order, followed by submodules in
    /// alphabetical order. Each submodule is wrapped in `pub mod name { ... }`.
    /// The output has no surrounding `mod` block.
    pub fn into_stream(self) -> TokenStream {
        let mut out = TokenStream::new();
        for (_, tokens) in self.items {
            out.extend(tokens);
        }
        for (name, m) in self.mods {
            let ident = format_ident!("{}", name);
            let contents = m.into_stream();
            out.extend(quote! {
                pub mod #ident {
                    #contents
                }
            });
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
        assert!(out.contains("struct Bar"));
        assert!(out.contains("struct Foo"));
        let bar_pos = out.find("struct Bar").unwrap();
        let foo_pos = out.find("struct Foo").unwrap();
        assert!(bar_pos < foo_pos, "expected alphabetical order");
    }

    #[test]
    fn extend_same_key() {
        let mut cs = Codespace::default();
        cs.add_item("Foo", quote! { pub struct Foo; });
        cs.add_item("Foo", quote! { impl Foo {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("struct Foo"));
        assert!(out.contains("impl Foo"));
    }

    #[test]
    fn path_creates_submod() {
        let mut cs = Codespace::default();
        cs.add_item("Foo", quote! { pub struct Foo; });
        cs.add_item("defaults::foo_value", quote! { pub fn foo_value() {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("pub mod defaults"));
        assert!(out.contains("fn foo_value"));
        let item_pos = out.find("struct Foo").unwrap();
        let mod_pos = out.find("pub mod defaults").unwrap();
        assert!(item_pos < mod_pos, "items precede submodules");
    }

    #[test]
    fn path_nested() {
        let mut cs = Codespace::default();
        cs.add_item("outer::inner::deep", quote! { pub fn deep() {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("pub mod outer"));
        assert!(out.contains("pub mod inner"));
        assert!(out.contains("fn deep"));
    }

    #[test]
    fn items_before_mods() {
        let mut cs = Codespace::default();
        cs.add_item("zzz", quote! { pub struct Zzz; });
        cs.add_item("aaa::f", quote! { pub fn f() {} });
        let out = cs.into_stream().to_string();
        let item_pos = out.find("struct Zzz").unwrap();
        let mod_pos = out.find("pub mod aaa").unwrap();
        assert!(item_pos < mod_pos, "items always precede submodules");
    }

    #[test]
    fn get_root_mod_direct_access() {
        let mut cs = Codespace::default();
        cs.get_root_mod()
            .get_mod("defaults")
            .add_item("f", quote! { pub fn f() {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("pub mod defaults"));
        assert!(out.contains("fn f"));
    }

    #[test]
    fn same_path_twice_extends() {
        let mut cs = Codespace::default();
        cs.add_item("defaults::a", quote! { pub fn a() {} });
        cs.add_item("defaults::b", quote! { pub fn b() {} });
        let out = cs.into_stream().to_string();
        let mod_start = out.find("pub mod defaults").unwrap();
        assert!(out[mod_start..].contains("fn a"));
        assert!(out[mod_start..].contains("fn b"));
    }

    #[test]
    fn item_and_mod_may_share_name() {
        let mut cs = Codespace::default();
        cs.add_item("foo", quote! { pub struct Foo; });
        cs.add_item("foo::bar", quote! { pub fn bar() {} });
        let out = cs.into_stream().to_string();
        assert!(out.contains("struct Foo"));
        assert!(out.contains("pub mod foo"));
        assert!(out.contains("fn bar"));
    }

    #[test]
    #[should_panic]
    fn invalid_mod_name_panics() {
        let mut cs = Codespace::default();
        cs.add_item("not-valid-ident::key", quote! {});
    }

    #[test]
    fn mod_add_item_colons_are_literal_sort_key() {
        // Mod::add_item does NOT do path splitting — "::" is just a sort key char.
        let mut cs = Codespace::default();
        cs.get_root_mod().add_item("a::b", quote! { pub struct X; });
        let out = cs.into_stream().to_string();
        // No submodule should exist; the item appears flat at root.
        assert!(!out.contains("pub mod"), "no submodule expected");
        assert!(out.contains("struct X"));
    }

    #[test]
    fn special_sort_key_orders_before_alpha() {
        // '#' (ASCII 35) sorts before all lowercase letters, so "#preamble" items
        // appear before e.g. "Foo".
        let mut cs = Codespace::default();
        cs.get_root_mod()
            .add_item("Foo", quote! { pub struct Foo; });
        cs.get_root_mod()
            .add_item("#preamble", quote! { use std::collections::BTreeMap; });
        let out = cs.into_stream().to_string();
        let use_pos = out.find("BTreeMap").unwrap();
        let foo_pos = out.find("struct Foo").unwrap();
        assert!(use_pos < foo_pos, "#preamble should sort before Foo");
    }

    #[test]
    fn multiple_submods_alphabetical() {
        let mut cs = Codespace::default();
        cs.add_item("zzz::a", quote! { pub fn a() {} });
        cs.add_item("aaa::b", quote! { pub fn b() {} });
        cs.add_item("mmm::c", quote! { pub fn c() {} });
        let out = cs.into_stream().to_string();
        let aaa = out.find("mod aaa").unwrap();
        let mmm = out.find("mod mmm").unwrap();
        let zzz = out.find("mod zzz").unwrap();
        assert!(
            aaa < mmm && mmm < zzz,
            "submods should appear alphabetically"
        );
    }

    #[test]
    #[should_panic]
    fn get_mod_invalid_ident_panics() {
        let mut cs = Codespace::default();
        cs.get_root_mod().get_mod("not-valid");
    }
}
