# codespace

A structured container for generated Rust code.

Code generators that emit raw `TokenStream`s face a practical problem: a single
type definition often requires multiple top-level items — the struct or enum
itself, helper functions, `impl` blocks, and so on. Putting all of those into
one `TokenStream` makes it hard to keep related items together or to route
ancillary items (e.g. serde default helpers) into a dedicated module.

`codespace` solves this by providing a tree-shaped accumulator. A `Codespace`
holds a root `Mod`; each `Mod` holds named items (raw `TokenStream` fragments)
and named sub-`Mod`s in separate maps. When you're done building,
`Codespace::into_stream` flattens everything into a single `TokenStream`
suitable for writing to a file or handing to a proc-macro output.

## Usage

```rust
use codespace::Codespace;
use quote::quote;

let mut cs = Codespace::default();

// Root-level item — key is just a sort hint, never emitted.
cs.add_item("Status", quote! {
    pub enum Status { Active, Inactive }
});

// "::" in the path creates a submodule; final segment is the sort key.
cs.add_item("defaults::status_default", quote! {
    pub fn status_default() -> Status { Status::Active }
});

let tokens = cs.into_stream();
// Renders to:
//   pub enum Status { Active, Inactive }
//   pub mod defaults {
//       pub fn status_default() -> Status { Status::Active }
//   }
```

## Paths

`Codespace::add_item` accepts a `"::"` delimited path. All but the last
segment name submodules (and must be valid Rust identifiers). The final
segment is an arbitrary sort key that is never emitted as a token.

`Mod::add_item` is the single-level form: the key is just a sort string, no
path splitting occurs. Use `Codespace::get_root_mod` (or `Mod::get_mod`) to
navigate the tree manually when you need finer-grained control.

## Ordering

Within each `Mod`, all items are emitted first in sort-key order, followed by
all submodules in alphabetical order by name. Sort keys are compared as plain
strings, so a `"#"` prefix (ASCII 35) will sort before any lowercase letter —
useful for preamble items like `use` imports.

## Extending items

Adding a second fragment under an existing key appends to it rather than
replacing it. This lets you build up a type and its `impl` blocks separately:

```rust
cs.get_root_mod().add_item("Foo", quote! { pub struct Foo(u32); });
cs.get_root_mod().add_item("Foo", quote! { impl Foo { pub fn value(&self) -> u32 { self.0 } } });
// Both fragments appear in the output.
```
