// Copyright 2026 Oxide Computer Company

//! With no date library enabled, `date` and `date-time` are plain strings.

#![cfg(not(any(feature = "chrono", feature = "jiff")))]

use schemars::schema::RootSchema;
use typify_impl::{TypeSpace, TypeSpaceSettings};

#[test]
fn dates_without_a_library_are_strings() {
    let schema: RootSchema = serde_json::from_value(serde_json::json!({
        "title": "Event",
        "type": "object",
        "properties": {
            "when": { "type": "string", "format": "date-time" },
            "day": { "type": "string", "format": "date" }
        },
        "required": ["when", "day"]
    }))
    .unwrap();
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space.add_root_schema(schema).unwrap();
    let rendered = type_space.to_stream().unwrap().to_string();
    assert!(
        rendered.contains(":: std :: string :: String"),
        "{rendered}"
    );
    assert!(!rendered.contains("chrono"), "{rendered}");
    assert!(!rendered.contains("jiff"), "{rendered}");
}
