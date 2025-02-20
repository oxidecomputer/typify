use std::collections::{HashMap, HashSet};
use std::{env, fs, path::Path};

use schemars::schema::Schema;
use schemars::JsonSchema;
use serde::Serialize;
use typify::{TypeSpace, TypeSpaceSettings};

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

#[allow(dead_code)]
#[derive(JsonSchema)]
struct WithMap {
    map: HashMap<String, String>,
}

struct LoginName;
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

struct NonAsciiChars;
impl JsonSchema for NonAsciiChars {
    fn schema_name() -> String {
        "NonAsciiChars".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> Schema {
        schemars::schema::SchemaObject {
            string: Some(Box::new(schemars::schema::StringValidation {
                max_length: Some(8),
                min_length: Some(2),
                pattern: None,
            })),
            ..Default::default()
        }
        .into()
    }
}

struct Pancakes;
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
    NonAsciiChars::add(&mut type_space);
    UnknownFormat::add(&mut type_space);
    ipnetwork::IpNetwork::add(&mut type_space);

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, contents).unwrap();

    // Generate with HashMap
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());

    WithMap::add(&mut type_space);

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen_hashmap.rs");
    fs::write(out_file, contents).unwrap();

    // Generate with a custom map type to validate requirements.
    let mut settings = TypeSpaceSettings::default();
    settings.with_map_type("CustomMap".to_string());
    let mut type_space = TypeSpace::new(&settings);

    WithMap::add(&mut type_space);

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen_custommap.rs");
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
        let _ = type_space.add_root_schema(schema).unwrap();
    }
}
