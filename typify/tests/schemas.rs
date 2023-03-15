// Copyright 2023 Oxide Computer Company

use std::{error::Error, fs::File, io::BufReader};

use expectorate::assert_contents;
use glob::glob;
use quote::quote;
use schemars::schema::{RootSchema, Schema};
use serde_json::json;
use typify::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};
use typify_impl::TypeSpaceImpl;

#[test]
fn test_schemas() {
    // Make sure output is up to date.
    for entry in glob("tests/schemas/*.json").expect("Failed to read glob pattern") {
        let entry = entry.unwrap();
        if cfg!(feature = "strict") && entry.to_str().unwrap().ends_with("various-enums.json") {
            continue;
        }
        eprintln!("validating {} ...", entry.display());
        validate_schema(entry).unwrap();
    }

    // Make sure it all compiles.
    trybuild::TestCases::new().pass("tests/schemas/*.rs");
}

fn validate_schema(path: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut out_path = path.clone();
    out_path.set_extension("rs");

    let file = File::open(path.clone())?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let root_schema: RootSchema = serde_json::from_reader(reader)?;

    let schema_raw = json! {
        {
            "enum": [ 1, "one" ]
        }
    };
    let schema = serde_json::from_value(schema_raw).unwrap();

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
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
            ),
    );
    type_space.add_ref_types(root_schema.definitions)?;

    let base_type = &root_schema.schema;

    // Only convert the top-level type if it has a name
    if base_type
        .metadata
        .as_ref()
        .and_then(|m| m.title.as_ref())
        .is_some()
    {
        let _ = type_space
            .add_type(&Schema::Object(root_schema.schema))
            .unwrap();
    }

    // Make a file with the generated code.
    let code = quote! {
        // Some types impl their own Deserialize and fully qualify the name.
        #[allow(unused_imports)]
        use serde::{Deserialize, Serialize};

        #type_space

        fn main() {}
    };
    let text = rustfmt_wrapper::rustfmt(code).unwrap();
    assert_contents(out_path, &text);

    Ok(())
}
