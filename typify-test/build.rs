use std::collections::HashSet;
use std::{env, fs, path::Path};

use schemars::schema::Schema;
use schemars::JsonSchema;
use serde::Serialize;
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
    #[schemars(default = "things")]
    e: Things,
    #[schemars(default = "yes_yes")]
    f: Option<bool>,
}

fn nope() -> bool {
    true
}

fn answer() -> i32 {
    42
}

fn yes_yes() -> Option<bool> {
    Some(true)
}

#[allow(dead_code)]
#[derive(JsonSchema, Serialize)]
struct Things {
    aa: u32,
    bb: String,
}

fn things() -> Things {
    Things {
        aa: 42,
        bb: "forty-two".to_string(),
    }
}

#[allow(dead_code)]
#[derive(JsonSchema)]
struct WithSet {
    set: HashSet<TestStruct>,
}

struct LoginName(String);
impl JsonSchema for LoginName {
    fn schema_name() -> String {
        "LoginName".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> Schema {
        schemars::schema::SchemaObject {
            string: Some(Box::new(schemars::schema::StringValidation {
                max_length: Some(8),
                min_length: Some(1),
                pattern: Some("^[a-z]*$".to_string()),
            })),
            ..Default::default()
        }
        .into()
    }
}

struct Pancakes(String);
impl JsonSchema for Pancakes {
    fn schema_name() -> String {
        "Pancakes".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> Schema {
        schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::String.into()),
            format: Some("pancakes".to_string()),
            ..Default::default()
        }
        .into()
    }

    fn is_referenceable() -> bool {
        false
    }
}

#[derive(JsonSchema)]
struct UnknownFormat {
    #[allow(dead_code)]
    pancakes: Pancakes,
}

fn main() {
    let mut type_space = TypeSpace::default();

    WithSet::add(&mut type_space);
    LoginName::add(&mut type_space);
    UnknownFormat::add(&mut type_space);
    ipnetwork::IpNetwork::add(&mut type_space);

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, contents).unwrap();
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
