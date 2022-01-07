use std::{env, fs, path::Path};

use schemars::schema::Schema;
use typify::TypeSpace;

fn main() {
    let content = std::fs::read_to_string("../example.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::default();
    type_space.add_ref_types(schema.definitions).unwrap();
    let base_type = &schema.schema;
    // Only convert the top-level type if it has a name
    if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
        let _ = type_space.add_type(&Schema::Object(schema.schema)).unwrap();
    }

    let content = format!(
        "{}\n{}\n{}",
        "use schemars::JsonSchema;",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, &content).unwrap();
}
