// Copyright 2022 Oxide Computer Company

use quote::quote;
use schemars::{gen::SchemaGenerator, schema::RootSchema, schema::Schema, JsonSchema};
use serde::Serialize;
use typify_impl::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};

#[allow(dead_code)]
#[derive(JsonSchema)]
struct CompoundType {
    value1: String,
    value2: u64,
}

#[allow(dead_code)]
#[derive(JsonSchema, Serialize)]
enum StringEnum {
    One,
    Two,
    BuckleMyShoe,
}

#[allow(dead_code)]
#[derive(JsonSchema, Serialize)]
#[serde(default = "default_pair")]
struct Pair {
    a: StringEnum,
    b: StringEnum,
}

fn default_pair() -> Pair {
    Pair {
        a: StringEnum::One,
        b: StringEnum::Two,
    }
}

#[derive(JsonSchema)]
#[allow(dead_code)]
struct AllTheTraits {
    ok: String,
}

fn add_type<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = T::json_schema(generator).into_object();
    schema.metadata().title = Some(T::schema_name());
    schema.into()
}

#[test]
fn test_generation() {
    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_derive("JsonSchema".to_string())
            .with_type_mod("types")
            .with_struct_builder(true)
            .with_patch(
                "AllTheTraits",
                TypeSpacePatch::default()
                    .with_derive("Hash")
                    .with_derive("Ord")
                    .with_derive("PartialOrd")
                    .with_derive("Eq")
                    .with_derive("PartialEq"),
            ),
    );

    let mut generator = SchemaGenerator::default();
    let body_schema = add_type::<CompoundType>(&mut generator);
    let string_schema = add_type::<String>(&mut generator);
    let opt_int_schema = add_type::<Option<u32>>(&mut generator);
    let strenum_schema = add_type::<StringEnum>(&mut generator);
    let pair_schema = add_type::<Pair>(&mut generator);
    let all_the_traits = add_type::<AllTheTraits>(&mut generator);

    type_space
        .add_ref_types(generator.take_definitions())
        .unwrap();

    let tid = type_space.add_type(&body_schema).unwrap();
    let t = type_space.get_type(&tid).unwrap();
    let ret = t.ident();
    let body = t.parameter_ident();

    let string_id = type_space.add_type(&string_schema).unwrap();
    let string = type_space.get_type(&string_id).unwrap().parameter_ident();
    let opt_int_id = type_space.add_type(&opt_int_schema).unwrap();
    let opt_int = type_space.get_type(&opt_int_id).unwrap().parameter_ident();
    let strenum_id = type_space.add_type(&strenum_schema).unwrap();
    let strenum = type_space.get_type(&strenum_id).unwrap().parameter_ident();
    let _ = type_space.add_type(&pair_schema).unwrap();
    let _ = type_space.add_type(&all_the_traits).unwrap();

    let types = type_space.to_stream();

    let file = quote! {
        mod types {
            #types
        }

        pub fn do_stuff(
            body: #body,
            string: #string,
            opt_int: #opt_int,
            strenum: #strenum,
        ) -> #ret {
            todo!()
        }
    };

    let fmt = rustfmt_wrapper::rustfmt(file.to_string()).unwrap();

    expectorate::assert_contents("tests/generator.out", fmt.as_str());
}

#[test]
fn test_required_nullable_object_with_title_uses_distinct_inner_name() {
    let schema: RootSchema = serde_json::from_value(serde_json::json!({
        "definitions": {
            "aaa-wrapper": {
                "type": "object",
                "properties": {
                    "author_association": {
                        "title": "author_association",
                        "type": "string",
                        "enum": ["OWNER", "MEMBER"]
                    }
                }
            },
            "author-association": {
                "title": "author_association",
                "type": "string",
                "enum": ["OWNER", "MEMBER"]
            },
            "simple-user": {
                "title": "Simple User",
                "type": "object",
                "properties": {
                    "login": {
                        "type": "string"
                    }
                },
                "required": ["login"]
            },
            "auto-merge": {
                "title": "Auto merge",
                "type": ["object", "null"],
                "properties": {
                    "enabled_by": {
                        "$ref": "#/definitions/simple-user"
                    },
                    "merge_method": {
                        "type": "string",
                        "enum": ["merge", "squash", "rebase"]
                    },
                    "commit_title": {
                        "type": "string"
                    },
                    "commit_message": {
                        "type": "string"
                    }
                },
                "required": [
                    "enabled_by",
                    "merge_method",
                    "commit_title",
                    "commit_message"
                ]
            }
        }
    }))
    .unwrap();

    let mut type_space = TypeSpace::default();
    type_space.add_root_schema(schema).unwrap();

    let fmt = rustfmt_wrapper::rustfmt(type_space.to_stream().to_string()).unwrap();

    assert!(fmt.contains("pub struct AutoMerge(pub ::std::option::Option<AutoMergeInner>);"));
    assert!(fmt.contains("pub struct AutoMergeInner {"));
    assert!(!fmt.contains("pub struct AutoMerge(pub ::std::option::Option<AutoMerge>);"));
    assert_eq!(fmt.matches("pub enum AuthorAssociation {").count(), 1);
}
