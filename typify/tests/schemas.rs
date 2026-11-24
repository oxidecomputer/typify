// Copyright 2025 Oxide Computer Company

use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

use expectorate::assert_contents;
use glob::glob;
use quote::quote;
use schemars::schema::RootSchema;
use serde_json::json;
use typify::typespace::TypespaceTrait;
#[cfg(all(feature = "chrono", not(feature = "jiff")))]
use typify::typespace::settings::ContainerType;
use typify::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};
use typify_impl::TypeSpaceImpl;

/// Checked-in outputs that name chrono types. `maps` and `types-with-defaults`
/// are regenerated under jiff; `maps_custom` is chrono-only and is excluded
/// from the jiff compile pass.
const CHRONO_GOLDENS: &[&str] = &["maps", "types-with-defaults", "maps_custom"];

/// Whether `stem` has checked-in output that depends on the date library.
fn is_chrono_golden(stem: &str) -> bool {
    CHRONO_GOLDENS.contains(&stem)
}

#[test]
fn test_schemas() {
    env_logger::init();
    // Make sure output is up to date. Golden files that name chrono types
    // only match the chrono backend; under jiff they are regenerated into
    // `tests/schemas/jiff/` instead.
    let chrono_active = cfg!(feature = "chrono") && !cfg!(feature = "jiff");
    let jiff_active = cfg!(feature = "jiff");
    for entry in glob("tests/schemas/*.json").expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        let stem = entry.file_stem().unwrap().to_str().unwrap().to_string();
        if is_chrono_golden(&stem) && !chrono_active {
            if jiff_active {
                let out_path = PathBuf::from("tests/schemas/jiff").join(format!("{stem}.rs"));
                validate_schema(entry.clone(), out_path, TypeSpaceSettings::default())
                    .unwrap_or_else(|e| panic!("failed for {}: {e}", entry.to_string_lossy()))
            }
            continue;
        }
        let out_path = entry.clone().with_extension("rs");
        validate_schema(entry.clone(), out_path, TypeSpaceSettings::default())
            .unwrap_or_else(|e| panic!("failed for {}: {e}", entry.to_string_lossy()))
    }

    // Make sure it all compiles. Under jiff, the chrono-named goldens are
    // compiled from `jiff/` instead of the top-level directory.
    let cases = trybuild::TestCases::new();
    if jiff_active {
        cases.pass("tests/schemas/jiff/*.rs");
    }
    for entry in glob("tests/schemas/*.rs").expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        let stem = entry.file_stem().unwrap().to_str().unwrap();
        if chrono_active || !is_chrono_golden(stem) {
            cases.pass(&entry);
        }
    }
}

/// Ensure that setting the global config to use a custom map type works.
/// Compares against chrono-named golden output, so it runs only under chrono.
#[cfg(all(feature = "chrono", not(feature = "jiff")))]
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
