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

String schemas that include a known `format` are represented with the
appropriate Rust type. For example `{ "type": "string", "format": "uuid" }` is
represented as a `uuid::Uuid` (which requires the `uuid` crate be included as a
dependency).

### Arrays

JSON Schema arrays can turn into one of three Rust types `Vec<T>`, `HashSet<T>`,
and tuples depending on the schema properties. An array may have a fixed length
that matches a fixed list of item types; this is well represented by a Rust
tuple. The distinction between `Vec<T>` and `HashSet<T>` is only if the
schema's `uniqueItems` field is `false` or `true` respectively.

### Objects

In general, objects turn into Rust structs. If, however, the schema defines no
properties, Typify emits a `HashMap<String, T>` if the `additionalProperties`
schema specifies `T` or a `HashMap<String, serde_json::Value>` otherwise.

Properties of generated `struct` that are not in the `required` set are
typically represented as an `Option<T>` with the `#[serde(default)]` attribute
applied. Non-required properties with types that already have a default value
(such as a `Vec<T>`) simply get the `#[serde(default)]` attribute (so you won't
see e.g. `Option<Vec<T>>`).

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

## Rust -> Schema -> Rust

Schemas derived from Rust types may include an extension that provides
information about the original type:

```json
{
  "type": "object",
  "properties": { .. },
  "x-rust-type": {
    "crate": "crate-o-types",
    "version": "1.0.0",
    "path": "crate_o_types::some_mod::SomeType"
  }
}
```

The extension includes the name of the crate, a Cargo-style version
requirements spec, and the full path (that must start with ident-converted name
of the crate).

Each of the modes of using typify allow for a list of crates and versions to be
specified. In this case, if the user specifies "crate-o-types@1.0.1" for
example, then typify would use its `SomeType` type rather than generating one
according to the schema.

### Using types from other crates

Each mode of using typify has a method for controlling the use of types with
`x-rust-type` annotations. The default is to ignore them. The recommended
method is to specify each crate and version you intend to use. You can
additionally supply the `*` version for crates (which may result in
incompatibilities) or you can define a policy to allow the use of all "unknown"
crates (which may require that addition of dependencies for those crates).

For the CLI:
```console
$ cargo typify --unknown-crates allow --crate oxnet@1.0.0 ...
```

For the builder:
```rust
let mut settings = typify::TypeSpaceSettings::default();
settings.with_unknown_crates(typify::UnknownPolicy::Allow)
    .with_crate("oxnet", typify::CrateVers::Version("1.0.0".parse().unwrap()));
```

For the macro:
```rust
typify::import_types!(
  schema = "schema.json",
  unknown_types = Allow,
  crates {
    "oxnet" = "1.0.0"
  }
)
```

### Version requirements

The `version` field within the `x-rust-type` extension follows the Cargo
version requirements specification. If the extension specifies `0.1.0` of a
crate and the user states that they're using `0.1.1`, then the type is used;
conversely, if the extension specifies `0.2.2` and the user is only using
`0.2.0` the type is not used.

Crate authors may choose to adhere to greater stability than otherwise provided
by semver. If the extension version is `>=0.1.0, <1.0.0` then the crate author
is committing to the schema compatibility of the given type on all releases
until `1.0.0`. It is important that crate authors populate the `version` field
in a way that upholds type availability. For example, while `*` is a valid
value, it is only conceivably valid if the type in question were available in
the first ever version of a crate published and never changed incompatibly in
any subsequent version.

### Type parameters

The `x-rust-type` extension may also specify type parameters:

```json
{
  "$defs": {
    "Sprocket": {
      "type": "object",
      "properties": { .. },
      "x-rust-type": {
        "crate": "util",
        "version": "0.1.0",
        "path": "util::Sprocket",
        "parameters": [
          {
            "$ref": "#/$defs/Gizmo"
          }
        ]
      }
    },
    "Gizmo": {
      "type": "object",
      "properties": { .. },
      "x-rust-type": {
        "crate": "util",
        "version": "0.1.0",
        "path": "util::Gizmo"
      }
    }
  }
}
```

With the `util@0.1.0` crate specified during type generation, schemas
referencing `#/$defs/Sprocket` would use the (non-generated) type
`util::Sprocket<util::Gizmo>`.

The `parameters` field is an array of schemas. They may be inline schemas or
referenced schemas.

### Including `x-rust-type` in your library

The schema for the expected value is as follows:

```json
{
  "description": "schema for the x-rust-type extension",
  "type": "object",
  "properties": {
    "crate": {
      "type": "string",
      "pattern": "^[a-zA-Z0-9_-]+$"
    },
    "version": {
      "description": "semver requirements per a Cargo.toml dependencies entry",
      "type": "string"
    },
    "path": {
      "type": "string",
      "pattern": "^[a-zA-Z0-9_]+(::[a-zA-Z0-9+]+)*$"
    },
    "parameters": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Schema"
      }
    }
  },
  "required": [
    "crate",
    "path",
    "version"
  ]
}
```

The `version` field expresses the stability of your type. For example, if
`0.1.0` indicates that `0.1.1` users would be fine whereas `0.2.0` users would
not use the type (instead generating it). You can communicate a future
commitment beyond what semver implies by using the [Cargo version requirement
syntax](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#version-requirement-syntax).
For example `>=0.1.0, <1.0.0` says that the type will remain structurally
compatible from version `0.1.0` until `1.0.0`.

## Formatting

You can format generated code using crates such as
[rustfmt-wrapper](https://docs.rs/rustfmt-wrapper) and
[prettyplease](https://docs.rs/prettyplease). This can be particularly useful
when checking in code or emitting code from a `build.rs`.

The examples below show different ways to convert a `TypeSpace` to a string
(`typespace` is a `typify::TypeSpace`).


### `rustfmt`

Best for generation of code that might be checked in alongside hand-written
code such as in the case of an `xtask` or stand-alone code generator (such as
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
`chrono::naive::NaiveDate`. For users that don't want dependencies on
`uuid` or `chrono` it would be useful for Typify to optionally represent those
as `String` (or as some other, consumer-specified type).
