use std::collections::HashSet;
use std::{env, fs, path::Path};

use schemars::schema::Schema;
use schemars::JsonSchema;
use typify::TypeSpace;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct TestStruct {
    a: u32,
    b: u32,
    #[schemars(default = "nope")]
    c: bool,
    #[schemars(default = "answer")]
    d: i32,
}

fn nope() -> bool {
    true
}

fn answer() -> i32 {
    42
}

#[allow(dead_code)]
#[derive(JsonSchema)]
struct WithSet {
    set: HashSet<TestStruct>,
}

fn main() {
    let mut type_space = TypeSpace::default();

    WithSet::add(&mut type_space);

    let content = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, &content).unwrap();
}

trait AddType {
    fn add(type_space: &mut TypeSpace);
}

impl<T> AddType for T
where
    T: JsonSchema,
{
    fn add(type_space: &mut TypeSpace) {
        let schema = schemars::schema_for!(T);
        type_space.add_ref_types(schema.definitions).unwrap();

        let base_type = &schema.schema;
        // Only convert the top-level type if it has a name
        if base_type
            .metadata
            .as_ref()
            .and_then(|m| m.title.as_ref())
            .is_some()
        {
            let _ = type_space.add_type(&Schema::Object(schema.schema)).unwrap();
        }
    }
}
