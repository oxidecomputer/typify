// Copyright 2026 Oxide Computer Company

use std::{fs::File, io::BufReader, path::Path};

use schemars::schema::RootSchema;
use typify_impl::TypeSpace;

/// Verify that `allOf` schemas with three distinct string patterns are merged
/// with a sequence of lookaheads and that the generated `FromStr` impl checks
/// all three constraints.
#[test]
fn test_allof_three_patterns() {
    let mut type_space = TypeSpace::default();

    let path = Path::new("tests/all_of.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let schema: RootSchema = serde_json::from_reader(reader).unwrap();

    type_space.add_root_schema(schema).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/all_of.out", fmt.as_str());
}
