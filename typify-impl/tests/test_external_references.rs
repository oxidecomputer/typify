use schemars::schema::RootSchema;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use typify_impl::TypeSpace;

#[test]
fn test_external_references() {
    let mut type_space = TypeSpace::default();

    let path = Path::new("tests/external_references.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let schema: RootSchema = serde_json::from_reader(reader).unwrap();
    type_space.with_path("tests/external_references.json");
    type_space.add_root_schema(schema).unwrap();

    let file = type_space.to_stream();

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/external_references.out", fmt.as_str());
}
