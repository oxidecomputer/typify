# Typify

A Rust type generator for JSON Schema (and OpenAPI).

## Principles

Typify tries to generate types that are maximally faithful to the input
schemas. Only valid data should deserialize into the generated types and
instances of generated types should always serialize without error. In other
words, whether created by hand or through deserialization, type instances
should reflect data that conforms to the schema validation.

### Exceptions

For certain enum types, typify is *slightly* less than perfectly precise.

Externally tagged enums in rust require an object with a single property; we
use this construction even if other properties should be permitted (though they
would not be represented in the Rust type).

Similarly, adjacently tagged enums have a tag field and a content field (if
there is associated content), so an object with exactly 2 properties (or
exactly 1 property if there is no associated content). The
`deny_unknown_properties` field determines whether more properties are
permitted (if absent) or denied (if present). There is no mechanism to deny
only *some* properties (while others are permitted but ignored). Here again, we
*acquiesce to this small imprecision.

## Influencing Code Generation

Typify allows consumers to influence how code is generated:

* Specify type, field, and variant names
* Inject externally defined types in lieu of generated types
* Specify types to use for map, set, and array constructions
* Choose from a set of known traits to implement, or specify additional traits
* Add attributes on generated types

This section details the various ways in which consumers may adjust code
generation to meet their needs. Some mechanisms are present in the input files
as extensions--these may also be added via JSON patch imperatives. Others are
programmatic inputs; most of these may be used for both macro and builder
consumers, but in some cases macro consumers may not have access to the full
range of configurability.


### Variant Names and Descriptions for Simple `enum` Schemas

When a schema uses `enum`, typify must derive a Rust identifier for each
variant. Typify can handle many cases automatically, but some enum values
cannot be mechanically converted to valid Rust identifiers. The `enum` array
may include any JSON value such as `[[],[]]`, `{"a": "b"}`, or `"<<1>>"`--none
of which have a straightforward translation into a valid Rust variant name.
This is particularly tricky to address programmatically when values differ only
by punctuation (e.g. `"enum": ["25GBASE-T", "2.5GBASE-T"] }`).

Typify will produce unhelpful variants names (`Variant0`, `Variant1`, ...) if
it's unable to determine a set of unique names.

`x-enum-varnames` is parallel array of strings that must be valid identifiers:

```json
{
  "type": "string",
  "enum": ["2.5GBASE-T", "25GBASE-T"],
  "x-enum-varnames": ["Speed2P5GBaseT", "Speed25GBaseT"]
}
```

`x-enum-descriptions` is a parallel array of documentation strings. These are
emitted as doc comments on the generated Rust variants.

```json
{
  "title": "operator",
  "enum": ["<", "=", ">"],
  "x-enum-varnames": ["LessThan", "Equal", "GreaterThan"],
  "x-enum-descriptions": [
    "Less than comparison",
    "Equality comparison",
    "Greater than comparison"
  ]
}
```

The value for each extension must be an array whose length matches that of the
`enum` array. Both extensions are cribbed from
[openapi-generator](https://openapi-generator.tech/docs/templating/#enum)--in
an effort to bring some nominal standardization into a not-very-standard
domain.

The generated code includes custom implementations for `serde::Serialize`,
`serde::Deserialize`, and `schemars::JsonSchema` that perform the appropriate
translations between variants and values.

<details>
<summary>Generated Rust (may not exactly match generated code)</summary>

```rust
#[derive(Clone, Debug)]
pub enum Operator {
    /// Less than comparison
    LessThan,
    /// Equality comparison
    Equal,
    /// Greater than comparison
    GreaterThan,
}

impl ::serde::Serialize for Operator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Self::LessThan => serializer.serialize_str("<"),
            Self::Equal => serializer.serialize_str("="),
            Self::GreaterThan => serializer.serialize_str(">"),
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        match <&str>::deserialize(deserializer)? {
            "<" => Ok(Self::LessThan),
            "=" => Ok(Self::Equal),
            ">" => Ok(Self::GreaterThan),
            s => Err(::serde::de::Error::unknown_variant(s, &["<", "=", ">"])),
        }
    }
}
```

</details>


## Generating Schemas With Typify Hints

XXX
