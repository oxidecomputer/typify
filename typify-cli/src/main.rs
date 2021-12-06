use schemars::schema::Schema;
use std::env;
use typify::TypeSpace;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("{} need 1 argument", env!("CARGO_PKG_NAME"))
    }

    let content = std::fs::read_to_string(&args[1]).unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::default();
    type_space.add_ref_types(schema.definitions).unwrap();
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space.add_type(&Schema::Object(schema.schema)).unwrap();
    }

    let content = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    println!("{}", content)
}
