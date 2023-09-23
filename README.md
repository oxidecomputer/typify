# Typify

Typify compiles JSON Schema documents into Rust types. It can be used in one of
several ways:

- using the [`cargo typify`](./cargo-typify/README.md) command

- via the macro `import_types!("types.json")` to generate Rust types directly in
  your program

- via a builder interface to generate Rust types in `build.rs` or `xtask`

- via the builder functions to generate persistent files e.g. when building
  API bindings

**If generation fails, doesn't compile or is generally lousy**: Please file an
issue and include the JSON Schema and Rust output (if there is any). Use `cargo
typify` command to generate code from the command-line. It's even more helpful
if you can articulate the output you'd ideally like to see.

## JSON Schema → Rust types

Typify translates JSON Schema types in a few different ways depending on some
basic properties of the schema:

### Built-in types

Integers, floating-point numbers, strings, etc. Those all have straightforward
representations in Rust. The only significant nuance is how to select the
appropriate built-in type based on type attributes. For example, a JSON Schema
might specify a maximum and/or minimum that indicates the appropriate integral
type to use.

String schemas that include a `format` are represented with the appropriate Rust
type. For example `{ "type": "string", "format": "uuid" }` is represented as a
`uuid::Uuid` (which requires the `uuid` crate be included as a dependency).

### Arrays

JSON Schema arrays can turn into one of three Rust types `Vec<T>`, `HashSet<T>`,
and tuples depending on the schema properties. An array may have a fixed length
that matches a fixed list of item types; this is well represented by a Rust
tuples. The distinction between `Vec<T>` and `HashSet<T>` is only if the
schema's `uniqueItems` field is `false` or `true` respectively.

### Objects

In general, objects turn in to Rust structs. If, however, the schema defines no
properties, Typify emits a `HashMap<String, T>` if the `additionalProperties`
schema specifies `T` or a `HashMap<String, serde_json::Value>` otherwise.

Properties that are not in the `required` set are typically represented as an
`Option<T>` with the `#[serde(default)]` attribute applied. Non-required
properties with types that already have a default value (such as a `Vec<T>`)
simply get the `#[serde(default)]` attribute (so you won't see e.g.
`Option<Vec<T>>`).

### OneOf

The `oneOf` construct maps to a Rust enum. Typify maps this to the various
[serde enum types](https://serde.rs/enum-representations.html).

### AllOf

The 'allOf' construct is handled by merging schemas. While most of the time,
typify tries to preserve and share type names, it can't always do this when
merging schemas. You may end up with fields replicated across type; optimizing
this generation is an area of active work.

### AnyOf

The `anyOf` construct is much trickier. If can be close to an `enum` (`oneOf`),
but where no particular variant might be canonical or unique for particular
data. While today we (imprecisely) model these as structs with optional,
flattened members, this is one of the weaker areas of code generation.

Issues describing example schemas and desired output are welcome and helpful.

## Formatting

You can format generated code using crates such as
[rustfmt-wrapper](https://docs.rs/rustfmt-wrapper) and
[prettyplease](https://docs.rs/prettyplease). This can be particularly useful
when checking in code or emitting code from a `build.rs`.

The examples below show different ways to convert a `TypeSpace` to a string
(`typespace` is a `typify::TypeSpace`).


### `rustfmt`

Best for generation of code that might be checked in alongside hand-written
code such as in the case of an `xtask` or stand-alone code generator (list
`cargo-typify`).

```rust
rustfmt_wrapper::rustfmt(typespace.to_stream().to_string())?
```

### `prettyplease`

Best for `build.rs` scripts where transitive dependencies might not have
`rustfmt` installed so should be self-contained.

```rust
prettyplease::unparse(&syn::parse2::<syn::File>(typespace.to_stream())?)
```

### No formatting

If no human will ever see the code (and this is almost never the case).

```rust
typespace.to_stream().to_string()
```

## WIP

Typify is a work in progress. Changes that affect output will be indicated with
a breaking change to the crate version number.

In general, if you have a JSON Schema that causes Typify to fail or if the
generated type isn't what you expect, please file an issue.

There are some known areas where we'd like to improve:

### Complex JSON Schema types

JSON schema can express a wide variety of types. Some of them are easy to model
in Rust; others aren't. There's a lot of work to be done to handle esoteric
types. Examples from users are very helpful in this regard.

### Bounded numbers

Bounded numbers aren't very well handled. Consider, for example, the schema:

```json
{
  "type": "integer",
  "minimum": 1,
  "maximum": 6
}
```

The resulting types won't enforce those value constraints.

### Configurable dependencies

A string schema with `format` set to `uuid` will result in the `uuid::Uuid`
type; similarly, a `format` of `date` translates to
`chrono::Date<chrono::offset::Utc>`. For users that don't want dependencies on
`uuid` or `chrono` it would be useful for Typify to optionally represent those
as `String` (or as some other, consumer-specified type).