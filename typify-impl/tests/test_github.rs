use std::{fs::File, io::BufReader, path::Path};

use quote::quote;
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

    type_space.add_ref_types(schema.definitions).unwrap();

    let types = type_space
        .iter_types()
        .map(|type_entry| type_entry.output(&type_space));

    let file = quote! {
        #(#types)*
    };

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/github.out", fmt.as_str());
}
