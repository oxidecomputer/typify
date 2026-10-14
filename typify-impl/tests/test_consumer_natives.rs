// Copyright 2026 Oxide Computer Company

//! A type the consumer names stands in for whatever it implements.

use schemars::schema::RootSchema;
use typespace::TypespaceTrait;
use typify_impl::{TypeSpace, TypeSpaceSettings};

fn schema() -> RootSchema {
    serde_json::from_value(serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Holder",
        "type": "object",
        "properties": {
            "ids": {
                "type": "array",
                "items": { "$ref": "#/definitions/Id" },
                "uniqueItems": true
            }
        },
        "required": ["ids"],
        "definitions": {
            "Id": { "type": "string" }
        }
    }))
    .unwrap()
}

/// A set of a replaced type generates. typify cannot see what the
/// replacement implements, so what a set asks of it is left to the
/// compiler, which checks the real type; typify's own refusal was a
/// claim it had no basis for.
#[test]
fn a_set_of_a_replaced_type_generates() {
    let mut settings = TypeSpaceSettings::default();
    settings.with_replacement("Id", "my_crate::Id", [].into_iter());
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema()).unwrap();
    let rendered = type_space.to_stream().unwrap().to_string();
    assert!(rendered.contains("my_crate :: Id"), "{rendered}");
}

/// The same holds under a crate-wide required trait.
#[test]
fn a_replaced_type_satisfies_a_required_trait() {
    let mut settings = TypeSpaceSettings::default();
    settings
        .with_replacement("Id", "my_crate::Id", [].into_iter())
        .map_typespace_settings(|s| s.with_required_trait(TypespaceTrait::PartialEq));
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema()).unwrap();
    type_space.to_stream().unwrap();
}
