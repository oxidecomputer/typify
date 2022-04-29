// Copyright 2021 Oxide Computer Company

use std::{fs::File, io::BufReader, path::Path};

use schemars::schema::{RootSchema, Schema};
use typify_impl::TypeSpace;

#[test]
fn test_github() {
    let mut type_space = TypeSpace::default();

    let path = Path::new("tests/github.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let mut schema: RootSchema = serde_json::from_reader(reader).unwrap();
    schema.schema.metadata().title = Some("Everything".to_string());

    type_space.add_ref_types(schema.definitions).unwrap();
    type_space.add_type(&Schema::Object(schema.schema)).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/github.out", fmt.as_str());
}
