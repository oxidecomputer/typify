// Copyright 2026 Oxide Computer Company

//! A type that typespace refuses to build is an error, not a panic.

use schemars::schema::RootSchema;
use typify_impl::{Error, TypeSpace, TypeSpaceSettings};

/// Two properties that sanitize to the same Rust field name.
#[test]
fn test_duplicate_field_names() {
    let schema: RootSchema = serde_json::from_value(serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Thing",
        "type": "object",
        "properties": {
            "fooBar": { "type": "string" },
            "foo_bar": { "type": "integer" }
        }
    }))
    .unwrap();

    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    let result = type_space.add_type_with_name(
        &schemars::schema::Schema::Object(schema.schema),
        Some("Thing".to_string()),
    );

    assert!(matches!(result, Err(Error::Typespace(_))), "{:?}", result);
}
