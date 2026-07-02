// Copyright 2026 Oxide Computer Company

//! Tests for the `with_double_option` setting, which renders optional +
//! nullable + no-default struct properties as `Option<Option<T>>` so that an
//! absent field, an explicit `null`, and a value remain distinguishable on the
//! wire (RFC 7396 merge-patch semantics).

use quote::quote;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use typify::{TypeSpace, TypeSpaceSettings};

/// A struct schema exercising the optional × nullable × has-default matrix.
fn matrix_schema() -> RootSchema {
    serde_json::from_value(json!({
        "type": "object",
        "title": "MergePatch",
        "required": ["req_nullable"],
        "properties": {
            // The only shape that should change: optional, nullable, no default.
            "opt_nullable_nodefault": { "type": ["string", "null"] },
            // Optional but not nullable: stays a single Option.
            "opt_nonnull_nodefault": { "type": "string" },
            // Optional and nullable but has a default: excluded (the default
            // already gives the absent case a meaning).
            "opt_nullable_default": { "type": ["string", "null"], "default": null },
            // Nullable but required: excluded (this is the Nullable<T> case).
            "req_nullable": { "type": ["string", "null"] }
        }
    }))
    .unwrap()
}

fn matrix_type_space(double_option: bool) -> TypeSpace {
    let mut settings = TypeSpaceSettings::default();
    settings.with_double_option(double_option);
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(matrix_schema()).unwrap();
    type_space
}

/// Generate the full output for the matrix struct, with `double_option` on/off.
fn generate(double_option: bool) -> String {
    matrix_type_space(double_option).to_stream().to_string()
}

/// Whitespace-insensitive occurrence count, since `TokenStream::to_string`
/// spaces tokens out unpredictably.
fn count(haystack: &str, needle: &str) -> usize {
    let squished: String = haystack.chars().filter(|c| !c.is_whitespace()).collect();
    squished.matches(needle).count()
}

/// Generate a struct from the given `required`/`properties` with
/// `double_option` on, and return the whitespace-stripped rendered type of the
/// named property (up to the field terminator).
fn field_type(required: serde_json::Value, properties: serde_json::Value, field: &str) -> String {
    let root: RootSchema = serde_json::from_value(json!({
        "type": "object",
        "title": "Thing",
        "required": required,
        "properties": properties,
    }))
    .unwrap();
    let mut settings = TypeSpaceSettings::default();
    settings.with_double_option(true);
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(root).unwrap();
    let out: String = type_space
        .to_stream()
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let key = format!("pub{field}:");
    let start = out
        .find(&key)
        .unwrap_or_else(|| panic!("no field {field} in:\n{out}"))
        + key.len();
    let end = out[start..].find(',').unwrap() + start;
    out[start..end].to_string()
}

const DOUBLE: &str = "::std::option::Option<::std::option::Option";

#[test]
fn test_double_option_off_is_unchanged() {
    let output = generate(false);
    // No helper module, no deserialize_with, and no nested option anywhere.
    assert_eq!(count(&output, "moddouble_option"), 0);
    assert_eq!(count(&output, "deserialize_with"), 0);
    assert_eq!(
        count(
            &output,
            "::std::option::Option<::std::option::Option<::std::string::String>>"
        ),
        0
    );
}

#[test]
fn test_double_option_on_wraps_only_eligible_property() {
    let output = generate(true);

    // Exactly one property qualifies: opt_nullable_nodefault. It gets the
    // nested option and the deserialize_with attribute; nothing else does.
    assert_eq!(
        count(
            &output,
            "::std::option::Option<::std::option::Option<::std::string::String>>"
        ),
        1,
        "exactly one field should become Option<Option<String>>:\n{output}"
    );
    assert_eq!(
        count(&output, "deserialize_with=\"double_option::deserialize\""),
        1,
        "exactly one field should carry the deserialize_with attr:\n{output}"
    );

    // The helper module is emitted exactly once, regardless of field count.
    assert_eq!(
        count(&output, "pubmoddouble_option"),
        1,
        "the double_option helper module should be emitted once:\n{output}"
    );
}

/// The gate keys off the converted type being an `Option`, so it fires for
/// every schema shape typify renders as `Option<T>`, not just `type: [T,
/// null]`. These all rendered as a single `Option` under the earlier
/// schema-shape-matching gate.
#[test]
fn test_double_option_covers_all_nullable_spellings() {
    let cases = [
        // anyOf with a null branch.
        json!({ "anyOf": [{ "type": "string" }, { "type": "null" }] }),
        // Singleton allOf wrapping a nullable schema (common OpenAPI shape for
        // attaching a description).
        json!({ "description": "d", "allOf": [{ "type": ["string", "null"] }] }),
        // Untyped enum containing null.
        json!({ "enum": ["a", "b", null] }),
    ];
    for schema in cases {
        let ty = field_type(json!([]), json!({ "f": schema.clone() }), "f");
        assert!(
            ty.starts_with(DOUBLE),
            "expected Option<Option<..>> for {schema}, got {ty}"
        );
    }
}

/// Regression guard: the setting must not disturb properties it does not apply
/// to. Nested `Option`s that arise incidentally are collapsed at construction,
/// so a required or already-nested field is unaffected — only the deliberate
/// wrap survives, and it is a single extra layer (not a triple option).
#[test]
fn test_double_option_leaves_organic_nesting_alone() {
    // A schema that would incidentally nest options: oneOf of a nullable type
    // and null.
    let nested = json!({ "oneOf": [{ "type": ["string", "null"] }, { "type": "null" }] });

    // Required: no wrap at all, and no incidental nesting leaks through.
    let req = field_type(json!(["f"]), json!({ "f": nested.clone() }), "f");
    assert_eq!(
        req, "::std::option::Option<::std::string::String>",
        "required field must stay a single Option"
    );

    // Optional: exactly one deliberate extra layer — Option<Option<String>>,
    // never Option<Option<Option<String>>>.
    let opt = field_type(json!([]), json!({ "f": nested }), "f");
    assert_eq!(
        opt, "::std::option::Option<::std::option::Option<::std::string::String>>",
        "optional field must be a double option, not a triple"
    );
}

/// Snapshot the actual generated code and confirm it compiles (via trybuild).
/// This is what verifies typify's real emitted output — the emitted field
/// attributes and the `double_option` helper module — not just a hand-written
/// mirror of it.
#[test]
fn test_double_option_generated_code_compiles() {
    let type_space = matrix_type_space(true);
    let code = quote! {
        #![deny(warnings)]

        #type_space

        fn main() {}
    };
    let text = rustfmt_wrapper::rustfmt(code).unwrap();
    // In a subdirectory so cargo doesn't pick it up as a stray integration-test
    // target; it is compiled only by the trybuild pass below.
    expectorate::assert_contents("tests/double_option/generated.rs", &text);
    trybuild::TestCases::new().pass("tests/double_option/generated.rs");
}

// The generated output is a `deserialize_with = "double_option::deserialize"`
// attribute plus the helper module below. These tests mirror that emitted shape
// exactly (cross-checked by the generation tests above) and exercise the serde
// round trip that the codegen can't run directly.
mod double_option {
    pub fn deserialize<'de, T, D>(
        deserializer: D,
    ) -> ::std::result::Result<::std::option::Option<::std::option::Option<T>>, D::Error>
    where
        T: ::serde::Deserialize<'de>,
        D: ::serde::Deserializer<'de>,
    {
        ::serde::Deserialize::deserialize(deserializer).map(::std::option::Option::Some)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Patch {
    #[serde(
        default,
        skip_serializing_if = "::std::option::Option::is_none",
        deserialize_with = "double_option::deserialize"
    )]
    name: Option<Option<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Outer {
    #[serde(flatten)]
    inner: Patch,
}

#[test]
fn test_double_option_serialize_three_states() {
    assert_eq!(
        serde_json::to_value(Patch { name: None }).unwrap(),
        json!({}),
        "absent field is omitted"
    );
    assert_eq!(
        serde_json::to_value(Patch { name: Some(None) }).unwrap(),
        json!({ "name": null }),
        "Some(None) serializes as null"
    );
    assert_eq!(
        serde_json::to_value(Patch {
            name: Some(Some("x".to_string()))
        })
        .unwrap(),
        json!({ "name": "x" }),
        "Some(Some(v)) serializes as the value"
    );
}

#[test]
fn test_double_option_deserialize_three_states() {
    assert_eq!(
        serde_json::from_value::<Patch>(json!({})).unwrap(),
        Patch { name: None },
        "absent field deserializes to None"
    );
    assert_eq!(
        serde_json::from_value::<Patch>(json!({ "name": null })).unwrap(),
        Patch { name: Some(None) },
        "explicit null deserializes to Some(None), not None"
    );
    assert_eq!(
        serde_json::from_value::<Patch>(json!({ "name": "x" })).unwrap(),
        Patch {
            name: Some(Some("x".to_string()))
        },
        "a value deserializes to Some(Some(v))"
    );
}

#[test]
fn test_double_option_survives_flatten() {
    // serde routes flattened structs through its Content buffer; confirm the
    // absent / null / value distinction survives that path too.
    let absent = serde_json::from_value::<Outer>(json!({})).unwrap();
    assert_eq!(absent.inner.name, None);

    let null = serde_json::from_value::<Outer>(json!({ "name": null })).unwrap();
    assert_eq!(null.inner.name, Some(None));

    let value = serde_json::from_value::<Outer>(json!({ "name": "x" })).unwrap();
    assert_eq!(value.inner.name, Some(Some("x".to_string())));
}
