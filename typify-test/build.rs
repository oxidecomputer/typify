// Copyright 2025 Oxide Computer Company

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

fn generate_from_json_schema(json: &str, output_name: &str) {
    let root_schema: schemars::schema::RootSchema = serde_json::from_str(json).unwrap();
    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(root_schema).unwrap();
    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push(output_name);
    fs::write(out_file, contents).unwrap();
}

fn main() {
    // Generate types for runtime serde integration tests.
    // Using inline JSON schemas to avoid dependency issues with external crates.

    // PR #991: Integer before Number in untagged enums
    generate_from_json_schema(
        r#"{
            "definitions": {
                "IntOrStr": {
                    "type": ["integer", "string"]
                }
            }
        }"#,
        "codegen_int_or_str.rs",
    );

    // PR #918: Required fields with defaults
    generate_from_json_schema(
        r#"{
            "definitions": {
                "RequiredWithDefaults": {
                    "type": "object",
                    "required": ["name", "count"],
                    "properties": {
                        "name": { "type": "string", "default": "unnamed" },
                        "count": { "type": "integer", "default": 0 },
                        "label": { "type": "string" }
                    }
                }
            }
        }"#,
        "codegen_required_defaults.rs",
    );

    // PR #986: Bounded integer newtypes
    generate_from_json_schema(
        r#"{
            "definitions": {
                "Dscp": {
                    "type": "integer",
                    "format": "uint8",
                    "minimum": 0,
                    "maximum": 63
                }
            }
        }"#,
        "codegen_dscp.rs",
    );

    // PR #975: Integer width selection [1..32] should use NonZeroU8, not NonZeroU64
    generate_from_json_schema(
        r#"{
            "definitions": {
                "SmallRange": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 32
                }
            }
        }"#,
        "codegen_small_range.rs",
    );

    // PR #948: Special char variant names
    generate_from_json_schema(
        r#"{
            "definitions": {
                "Comparator": {
                    "anyOf": [
                        { "type": "string", "const": "=" },
                        { "type": "string", "const": ">" },
                        { "type": "string", "const": "<" },
                        { "type": "string", "const": "\u2265" },
                        { "type": "string", "const": ">=" },
                        { "type": "string", "const": "\u2264" },
                        { "type": "string", "const": "<=" },
                        { "type": "string", "const": "\u2260" },
                        { "type": "string", "const": "!=" }
                    ]
                }
            }
        }"#,
        "codegen_comparator.rs",
    );

    // PR #414: anyOf with mixed types (would have panicked before)
    generate_from_json_schema(
        r#"{
            "definitions": {
                "AnyOfMixed": {
                    "anyOf": [
                        { "type": "object", "properties": { "value": { "type": "string" } }, "required": ["value"] },
                        { "type": "string" },
                        { "type": "integer" }
                    ]
                }
            }
        }"#,
        "codegen_any_of_mixed.rs",
    );

    // PR #954: not schema (would have panicked before)
    generate_from_json_schema(
        r#"{
            "definitions": {
                "NotObject": { "not": { "type": "object" } },
                "ArrayNonObjects": {
                    "type": "array",
                    "items": { "not": { "type": "object" } }
                }
            }
        }"#,
        "codegen_not_types.rs",
    );

    // JSON Schema 2020-12: uses $defs, prefixItems, items:false
    {
        let schema_json = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "Address": {
                    "type": "object",
                    "properties": {
                        "street": { "type": "string" },
                        "city": { "type": "string" }
                    },
                    "required": ["street", "city"]
                },
                "Tag": {
                    "type": "string",
                    "minLength": 1
                }
            },
            "type": "object",
            "title": "Location",
            "properties": {
                "address": { "$ref": "#/$defs/Address" },
                "tag": { "$ref": "#/$defs/Tag" }
            },
            "required": ["address"],
            "dependentRequired": {
                "tag": ["address"]
            }
        }"##;
        let value: serde_json::Value = serde_json::from_str(schema_json).unwrap();
        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space.add_schema_from_value(value).unwrap();
        let contents =
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
        let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
        out_file.push("codegen_2020_12.rs");
        fs::write(out_file, contents).unwrap();
    }

    // External $ref: main schema references types.json
    {
        let main_json = r#"{
            "type": "object",
            "title": "Order",
            "definitions": {},
            "properties": {
                "item": { "$ref": "types.json#/definitions/Item" },
                "quantity": { "type": "integer" }
            },
            "required": ["item", "quantity"]
        }"#;
        let types_json = r#"{
            "definitions": {
                "Item": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "price": { "type": "number" }
                    },
                    "required": ["name", "price"]
                }
            }
        }"#;
        let main_value: serde_json::Value = serde_json::from_str(main_json).unwrap();
        let types_value: serde_json::Value = serde_json::from_str(types_json).unwrap();
        let mut externals = std::collections::BTreeMap::new();
        externals.insert("types.json".to_string(), types_value);
        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
        type_space
            .add_schema_with_externals(main_value, externals)
            .unwrap();
        let contents =
            prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());
        let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
        out_file.push("codegen_external_ref.rs");
        fs::write(out_file, contents).unwrap();
    }

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
    settings.with_map_type("CustomMap");
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
