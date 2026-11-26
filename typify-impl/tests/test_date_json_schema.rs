// Copyright 2026 Oxide Computer Company

//! A jiff date omits `JsonSchema`, so a type that requires it and holds a jiff
//! date is refused when rendered.

use schemars::schema::RootSchema;
use typespace::TypespaceTrait;
use typify_impl::{DateLibrary, Error, TypeSpace, TypeSpaceSettings};

fn schema() -> RootSchema {
    serde_json::from_value(serde_json::json!({
        "title": "Event",
        "type": "object",
        "properties": {
            "when": { "type": "string", "format": "date-time" }
        },
        "required": ["when"]
    }))
    .unwrap()
}

fn jiff_settings() -> TypeSpaceSettings {
    let mut settings = TypeSpaceSettings::default();
    settings.with_date_library(DateLibrary::Jiff);
    settings
}

/// Requiring `JsonSchema` of a type holding a jiff `Timestamp` is refused,
/// naming the jiff type as the one that lacks the impl.
#[test]
fn json_schema_requirement_refuses_a_jiff_date() {
    let mut settings = jiff_settings();
    settings.map_typespace_settings(|s| s.with_required_trait(TypespaceTrait::JsonSchema));
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema()).unwrap();

    let err = type_space.to_stream().unwrap_err();
    assert!(
        matches!(err, Error::Typespace(_)) && err.to_string().contains("jiff::Timestamp"),
        "{err}"
    );
}

/// Without the requirement, the same schema generates a jiff date.
#[test]
fn a_jiff_date_generates_without_json_schema() {
    let mut type_space = TypeSpace::new(&jiff_settings());
    type_space.add_root_schema(schema()).unwrap();
    let rendered = type_space.to_stream().unwrap().to_string();
    assert!(rendered.contains(":: jiff :: Timestamp"), "{rendered}");
}

/// Chrono's date types do implement `JsonSchema`, so the same requirement
/// generates.
#[test]
fn a_chrono_date_satisfies_json_schema() {
    let mut settings = TypeSpaceSettings::default();
    settings
        .with_date_library(DateLibrary::Chrono)
        .map_typespace_settings(|s| s.with_required_trait(TypespaceTrait::JsonSchema));
    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema()).unwrap();
    type_space.to_stream().unwrap();
}
