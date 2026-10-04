// Copyright 2026 Oxide Computer Company

//! The crates a generated type space reports.

use schemars::schema::Schema;
use typify_impl::{CrateVers, TypeSpace, TypeSpaceSettings};

fn names(type_space: &TypeSpace) -> Vec<String> {
    type_space
        .to_codespace()
        .unwrap()
        .dependencies()
        .map(|dep| dep.name.clone())
        .collect()
}

/// A string format mapped to a native type reports that type's crate,
/// alongside what typespace reports for the rendered code.
#[test]
fn formats_report_their_crates() {
    let schema: Schema = serde_json::from_value(serde_json::json!({
        "type": "object",
        "properties": {
            "when": { "type": "string", "format": "date-time" },
            "id": { "type": "string", "format": "uuid" }
        },
        "required": ["when", "id"]
    }))
    .unwrap();
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space
        .add_type_with_name(&schema, Some("Thing".to_string()))
        .unwrap();
    assert_eq!(names(&type_space), ["chrono", "serde", "uuid"]);
}

/// A type from an `x-rust-type` extension reports its crate at the
/// version the settings declared for it, renamed if the settings
/// renamed it.
#[test]
fn rust_extension_crates_carry_their_version() {
    let schema: Schema = serde_json::from_value(serde_json::json!({
        "type": "object",
        "properties": {
            "value": {
                "type": "string",
                "x-rust-type": {
                    "crate": "my-crate",
                    "version": "1.2.3",
                    "path": "my_crate::Value"
                }
            }
        },
        "required": ["value"]
    }))
    .unwrap();
    let mut settings = TypeSpaceSettings::default();
    settings.with_crate(
        "my-crate",
        CrateVers::Version("1.2.3".parse().unwrap()),
        Some(&"their-crate".to_string()),
    );
    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_type_with_name(&schema, Some("Thing".to_string()))
        .unwrap();

    let codespace = type_space.to_codespace().unwrap();
    let my_crate = codespace
        .dependencies()
        .find(|dep| dep.name == "my-crate")
        .unwrap();
    assert_eq!(my_crate.version.to_string(), "^1.2.3");
    assert_eq!(my_crate.rename.as_deref(), Some("their_crate"));
}
