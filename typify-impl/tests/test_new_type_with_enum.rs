// Copyright 2022 Oxide Computer Company

use std::{fs::File, io::BufReader, path::Path};

use schemars::schema::{RootSchema, Schema};
use typify_impl::TypeSpace;

#[test]
fn test_new_type_with_enum() {
    let mut type_space = TypeSpace::default();
    let path = Path::new("tests/new_type_with_enum.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut schema: RootSchema = serde_json::from_reader(reader).unwrap();
    schema.schema.metadata().title = Some("Everything".to_string());

    type_space.add_ref_types(schema.definitions).unwrap();
    type_space.add_type(&Schema::Object(schema.schema)).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/new_type_with_enum.out", fmt.as_str());
}
