# Typify

Typify compiles JSON Schema documents into Rust types. It can be used in one of
three ways:

- via the macro `import_types!("types.json")` to generate Rust types directly
in your program

- via a builder interface to generate Rust types in `build.rs`

- or via the builder functions to generate persistent files e.g. when building
API bindings.

## JSON Schema → Rust types

Typify translates JSON Schema types in a few different ways depending on some basic properties of the schema:

### Built-in types

Integers, floating-point numbers, strings, etc. Those all have straightforward
representations in Rust. The only significant nuance is how to select the
appropriate built-in type based on type attributes. For example, a JSON Schema
might specify a maximum and/or minimum that indicates the appropriate integral
type to use.

String schemas that include a `format` are represented with the appropriate
Rust type. For example `{ "type": "string", "format": "uuid" }` is represented
as a `uuid::Uuid` (which requires the `uuid` crate be included as a dependency).

### Arrays

JSON Schema arrays can turn into one of three Rust types `Vec<T>`,
`HashSet<T>`, and tuples depending on the schema properties. An array may have
a fixed length that matches a fixed list of item types; this is well
represented by a Rust tuples. The distintion between `Vec<T>` and `HashSet<T>`
is only if the schema's `uniqueItems` field is `false` or `true` respectively.

### Objects

In general, objects turn in to Rust structs. If, however, the schema defines no
properties, Typify emits a `HashMap<String, T>` if the `additionalProperties`
schema specifies `T` or a `HashMap<String, serde_json::Value>` otherwise.

Properties that are not in the `required` set ar typically represented as an
`Option<T>` with the `#[serde(default)]` attribute applied. Non-required
properties with types that already have a default value (such as a `Vec<T>`)
simply get the `#[serde(default)]` attribute (so you won't see e.g.
`Option<Vec<T>>`).

### OneOf

The `OneOf` construct maps to a Rust enum. Typify maps this to the various [serde enum types](https://serde.rs/enum-representations.html).

### AnyOf / AllOf

The `anyOf` and `allOf` constructs are a little trickier to handle, but (in
general) Typify models these as structs where each member is decorated with the
`#[serde(flatten)]` attribute (with `Option` wrappers in the case of `anyOf`).

## WIP

Typify is a work in progress. Changes that affect output will be indicated with
a breaking change to the crate version number.

In general, if you have a JSON Schema that causes Typify to fail or if the
generated type isn't what you expect, please file an issue.

There are some known areas where we'd like to improve:

### User-defined type-handling

A useful addition would be to have Typify (builder or macro) accept a map of
JSON Schemas to named Rust types. This would allow consumers to substitute a
preferred type for some known schema.

### Configurable dependencies

A string schema with `format` set to `uuid` will result in the `uuid::Uuid`
type; similarly, a `format` of `date` translates to
`chrono::Date<chrono::offset::Utc>`. For users that don't want dependencies on
`uuid` or `chrono` it would be useful for Typify to optionally represent those
as `String` (or as some other, consumer-specified type).

### Cyclic types

Typify has special-case handling for self-referential types. For example:

```rust
struct A {
    a: Box<A>,
}
```
.. but it does not support more complex cycles such as A -> B -> A.