// Copyright 2024 Oxide Computer Company

use std::{error::Error, fs::File, io::BufReader};

use expectorate::assert_contents;
use glob::glob;
use quote::quote;
use schemars::schema::RootSchema;
use serde_json::json;
use typify::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};
use typify_impl::TypeSpaceImpl;

#[test]
fn test_schemas() {
    env_logger::init();
    // Make sure output is up to date.
    for entry in glob("tests/schemas/*.json").expect("Failed to read glob pattern") {
        validate_schema(entry.unwrap()).unwrap();
    }

    // Make sure it all compiles.
    trybuild::TestCases::new().pass("tests/schemas/*.rs");
}

fn validate_schema(path: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut out_path = path.clone();
    out_path.set_extension("rs");

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
            )
            // Our test use of the x-rust-type extension only refers to things
            // in std.
            .with_crate(
                "std",
                typify::CrateVers::Version("1.0.0".parse().unwrap()),
                None,
            ),
    );
    type_space.add_root_schema(root_schema)?;

    // Make a file with the generated code.
    let code = quote! {
        // Some types impl their own Deserialize and fully qualify the name.
        #[allow(unused_imports)]
        use serde::{Deserialize, Serialize};

        #type_space

        fn main() {}
    };
    let text = rustfmt_wrapper::rustfmt(code)?;
    assert_contents(out_path, &text);

    Ok(())
}
