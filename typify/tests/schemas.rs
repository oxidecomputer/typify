// Copyright 2025 Oxide Computer Company

use std::{error::Error, fs::File, io::BufReader};

use expectorate::assert_contents;
use glob::glob;
use quote::quote;
use schemars::schema::RootSchema;
use serde_json::json;
use typify::typespace::TypespaceTrait;
use typify::typespace::settings::ContainerType;
use typify::{DateLibrary, TypeSpace, TypeSpacePatch, TypeSpaceSettings};
use typify_impl::TypeSpaceImpl;

/// Fixtures with checked-in jiff output alongside their chrono output.
const JIFF_GOLDENS: &[&str] = &["maps", "types-with-defaults"];

/// Each fixture's checked-in output is compared with the chrono default. The
/// fixtures that also have jiff output are compared with that output too.
#[test]
fn test_schemas() {
    env_logger::init();
    for entry in glob("tests/schemas/*.json").expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        validate_schema(
            entry.clone(),
            entry.clone().with_extension("rs"),
            TypeSpaceSettings::default(),
        )
        .unwrap_or_else(|e| panic!("failed for {}: {e}", entry.to_string_lossy()));
    }
    for stem in JIFF_GOLDENS {
        let mut settings = TypeSpaceSettings::default();
        settings.with_date_library(DateLibrary::Jiff);
        validate_schema(
            format!("tests/schemas/{stem}.json").into(),
            format!("tests/schemas/jiff/{stem}.rs").into(),
            settings,
        )
        .unwrap_or_else(|e| panic!("failed for jiff {stem}: {e}"));
    }

    let cases = trybuild::TestCases::new();
    for entry in glob("tests/schemas/*.rs").expect("Failed to read glob pattern") {
        cases.pass(entry.unwrap());
    }
    for stem in JIFF_GOLDENS {
        cases.pass(format!("tests/schemas/jiff/{stem}.rs"));
    }
}

/// Ensure that setting the global config to use a custom map type works.
#[test]
fn test_custom_map() {
    let mut settings = TypeSpaceSettings::default();
    settings.map_typespace_settings(|s| {
        s.with_map_type(ContainerType::btree_map().with_path("std::collections::BTreeMap"))
    });
    validate_schema(
        "tests/schemas/maps.json".into(),
        "tests/schemas/maps_custom.rs".into(),
        settings,
    )
    .unwrap();

    trybuild::TestCases::new().pass("tests/schemas/maps_custom.rs");
}

/// Ensure that enumerated types include the enumeration in their JsonSchema
/// implementation.
#[test]
fn test_various_enums_json_schema() {
    let mut settings = TypeSpaceSettings::default();
    settings.map_typespace_settings(|s| s.with_required_trait(TypespaceTrait::JsonSchema));
    validate_schema(
        "tests/schemas/various-enums.json".into(),
        "tests/schemas/various-enums-json-schema.rs".into(),
        settings,
    )
    .unwrap();

    trybuild::TestCases::new().pass("tests/schemas/various-enums-json-schema.rs");
}

fn validate_schema(
    path: std::path::PathBuf,
    out_path: std::path::PathBuf,
    mut settings: TypeSpaceSettings,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let root_schema: RootSchema = serde_json::from_reader(reader)?;

    let schema_raw = json!(
        {
            "enum": [ 1, "one" ]
        }
    );
    let schema = serde_json::from_value(schema_raw).unwrap();

    let mut type_space = TypeSpace::new(
        settings
            .with_replacement(
                "HandGeneratedType",
                "String",
                [TypeSpaceImpl::Display].into_iter(),
            )
            .with_patch(
                "TypeThatNeedsMoreDerives",
                TypeSpacePatch::default()
                    .with_rename("TypeThatHasMoreDerives")
                    .with_derive("Eq")
                    .with_derive("PartialEq"),
            )
            .with_conversion(
                schema,
                "serde_json::Value",
                [TypeSpaceImpl::Display].into_iter(),
            )
            // Our test use of the x-rust-type extension only refers to things
            // in std.
            .with_crate(
                "std",
                typify::CrateVers::Version("1.0.0".parse().unwrap()),
                None,
            )
            .map_typespace_settings(|s| s.with_struct_builder(true)),
    );
    type_space.add_root_schema(root_schema)?;

    let types = type_space.to_stream()?;

    // Make a file with the generated code.
    let code = quote! {
        #![deny(warnings)]

        #types

        fn main() {}
    };
    let text = rustfmt_wrapper::rustfmt(code)?;
    assert_contents(out_path, &text);

    Ok(())
}
