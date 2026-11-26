// Copyright 2026 Oxide Computer Company

//! The crates a generated type space reports.

use schemars::schema::Schema;
use typify_impl::{CrateVers, DateLibrary, TypeSpace, TypeSpaceSettings};

fn names(type_space: &TypeSpace) -> Vec<String> {
    type_space
        .to_codespace()
        .unwrap()
        .dependencies()
        .map(|dep| dep.name.clone())
        .collect()
}

fn date_time_and_uuid() -> Schema {
    serde_json::from_value(serde_json::json!({
        "type": "object",
        "properties": {
            "when": { "type": "string", "format": "date-time" },
            "id": { "type": "string", "format": "uuid" }
        },
        "required": ["when", "id"]
    }))
    .unwrap()
}

fn generate_names(library: DateLibrary) -> Vec<String> {
    let mut settings = TypeSpaceSettings::default();
    settings.with_date_library(library);
    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_type_with_name(&date_time_and_uuid(), Some("Thing".to_string()))
        .unwrap();
    names(&type_space)
}

/// A string format mapped to a native type reports that type's crate,
/// alongside what typespace reports for the rendered code.
#[test]
fn chrono_reports_chrono() {
    assert_eq!(
        generate_names(DateLibrary::Chrono),
        ["chrono", "serde", "uuid"]
    );
}

#[test]
fn jiff_reports_jiff() {
    assert_eq!(generate_names(DateLibrary::Jiff), ["jiff", "serde", "uuid"]);
}

/// Plain strings link no date crate.
#[test]
fn string_reports_no_date_crate() {
    assert_eq!(generate_names(DateLibrary::String), ["serde", "uuid"]);
}

/// Each generation carries its own library, so generating with one does not
/// change what another generation reports.
#[test]
fn libraries_do_not_leak_between_generations() {
    let chrono = generate_names(DateLibrary::Chrono);
    let jiff = generate_names(DateLibrary::Jiff);
    assert_eq!(chrono, ["chrono", "serde", "uuid"]);
    assert_eq!(jiff, ["jiff", "serde", "uuid"]);
    assert_eq!(generate_names(DateLibrary::Chrono), chrono);
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
