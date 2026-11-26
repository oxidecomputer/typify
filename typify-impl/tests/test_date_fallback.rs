// Copyright 2026 Oxide Computer Company

//! With the `String` date library, `date` and `date-time` are plain strings.

use schemars::schema::RootSchema;
use typify_impl::{DateLibrary, TypeSpace, TypeSpaceSettings};

fn generate(schema: serde_json::Value) -> String {
    let schema: RootSchema = serde_json::from_value(schema).unwrap();
    let mut settings = TypeSpaceSettings::default();
    settings.with_date_library(DateLibrary::String);
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema).unwrap();
    type_space.to_stream().unwrap().to_string()
}

#[test]
fn dates_are_strings() {
    let rendered = generate(serde_json::json!({
        "title": "Event",
        "type": "object",
        "properties": {
            "when": { "type": "string", "format": "date-time" },
            "day": { "type": "string", "format": "date" }
        },
        "required": ["when", "day"]
    }));
    assert!(
        rendered.contains(":: std :: string :: String"),
        "{rendered}"
    );
    assert!(!rendered.contains("chrono"), "{rendered}");
    assert!(!rendered.contains("jiff"), "{rendered}");
}

/// A date format carries no length or pattern of its own, so the string
/// constraints still apply just as they do for any other string.
#[test]
fn a_date_keeps_its_string_constraints() {
    let rendered = generate(serde_json::json!({
        "title": "Event",
        "type": "object",
        "properties": {
            "day": { "type": "string", "format": "date", "pattern": "^2" }
        },
        "required": ["day"]
    }));
    assert!(!rendered.contains("chrono"), "{rendered}");
    assert!(!rendered.contains("jiff"), "{rendered}");
    assert!(rendered.contains("regress"), "{rendered}");
}
