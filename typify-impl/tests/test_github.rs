use std::{fs::File, io::BufReader, path::Path};

use schemars::schema::RootSchema;
use typify_impl::TypeSpace;

#[test]
fn test_github() {
    let mut type_space = TypeSpace::default();

    let path = Path::new("tests/github.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let schema: RootSchema = serde_json::from_reader(reader).unwrap();

    println!("{:?}", schema);

    type_space.add_ref_types(schema.definitions).unwrap();

    println!("done");

    for type_entry in type_space.iter_types() {
        println!("entry {:#?}", type_entry);
        let output = type_entry.output(&type_space);
        println!("{:?}", output);
        let fmt = rustfmt_wrapper::rustfmt(output.to_string()).unwrap();
        println!("{}", fmt);
    }

    todo!();
}
