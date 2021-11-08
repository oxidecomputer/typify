# Typify

Compile JSON Schema documents into Rust types. This can be used ...

- via the macro `import_types!("types.json")` to generate Rust types directly
in your program

- via a builder interface to generate Rust types in `build.rs`

- or via the builder functions to generate persistent files e.g. when building
API bindings.

## JSON Schema → Rust types

We can divide types in JSON Schema into a few classes:

### Simple types

Integers, floating-point numbers, strings, etc. Those all have straightforward
translations. The only significant nuance is how to select the appropriate
built-in type.

### Arrays

JSON Schema arrays can turn into three Rust types `Vec<T>`, `HashSet<T>`, and
tuples. Arrays may have a fixed length that matches a fixed list of item types;
this matches well with Rust tuples. The distintion between `Vec<T>` and `HashSet<T>` is only if the `uniqueItems` field is `true`.

### Objects

In general Objects turn in to Rust structs, but if there are no properties
defined Typify models this as a `HashMap<String, T>` if the
`additionalProperties` schema specifies `T` or a `HashMap<String,
serde_json::Value>` otherwise. Properties that are not in the `required` set and represented as an `Option<T>`.

### OneOf

The `OneOf` construct maps to a Rust enum. Typify maps this to the various [serde enum types](https://serde.rs/enum-representations.html).

### AnyOf / AllOf

The `anyOf` and `allOf` constructs are a little trickier to handle, but (in
general) Typify models these as structs where each member is decorated with the
`#[serde(flatten)]` attribute.

## WIP

Typify is a work in progress. Changes that affect output will likely be
breaking changes. We will continue to update the crate version number with each
change; be cognizant when updating to a new version.

Bounded numbers aren't very well handled. Consider, for example, the schema:

```json
{
    "type": "integer",
    "minimum": 1,
    "maximum": 6
}
```

The resulting types won't enforce those value constraints.

Similarly, patterns and lengths for strings are not enforced.

In general, if you have a JSON Schema that causes Typify to fail or if the
generated type isn't what you expect, please file an issue.