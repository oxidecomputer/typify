// Copyright 2023 Oxide Computer Company

use std::{fs::File, io::BufReader, path::Path};

use schemars::schema::RootSchema;
use typify_impl::{TypeSpace, TypeSpaceImpl, TypeSpaceSettings};

#[test]
fn test_github() {
    let mut type_space = TypeSpace::default();

    let path = Path::new("tests/github.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut schema: RootSchema = serde_json::from_reader(reader).unwrap();
    schema.schema.metadata().title = Some("Everything".to_string());

    type_space.add_root_schema(schema).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/github.out", fmt.as_str());
}

#[test]
fn test_vega() {
    env_logger::init();
    let mut settings = TypeSpaceSettings::default();
    let raw_schema = serde_json::json!(
        {
            "enum": [
              null,
              "normal",
              "bold",
              "lighter",
              "bolder",
              "100",
              "200",
              "300",
              "400",
              "500",
              "600",
              "700",
              "800",
              "900",
              100,
              200,
              300,
              400,
              500,
              600,
              700,
              800,
              900
            ]
          }
    );
    let schema = serde_json::from_value(raw_schema).unwrap();
    settings
        .with_conversion(schema, "MyEnum", [TypeSpaceImpl::FromStr].into_iter())
        // TODO ColorValue has resulted in an extremely expensive type; to
        // resolve this we need to do a better job of canonicalizing the schema
        // once rather than repeatedly doing quadratic expansions over it.
        // Alternatively we can memoize conversions rather than repeating them.
        .with_replacement("ColorValue", "ColorValue", [].into_iter());

    let mut type_space = TypeSpace::new(&settings);

    let path = Path::new("tests/vega.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut schema: RootSchema = serde_json::from_reader(reader).unwrap();
    schema.schema.metadata().title = Some("Everything".to_string());

    type_space.add_root_schema(schema).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/vega.out", fmt.as_str());
}
