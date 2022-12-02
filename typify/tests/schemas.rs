// Copyright 2022 Oxide Computer Company

use std::{error::Error, fs::File, io::BufReader};

use expectorate::assert_contents;
use glob::glob;
use schemars::schema::{RootSchema, Schema};
use typify::TypeSpace;

#[test]
fn test_schemas() {
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

    let mut type_space = TypeSpace::default();
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

    let code = format!(
        "{}\n{}\nfn main() {{}}\n",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    assert_contents(out_path, &code);

    Ok(())
}
