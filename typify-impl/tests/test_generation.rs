// Copyright 2021 Oxide Computer Company

use quote::quote;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::Serialize;
use typify_impl::TypeSpace;

#[allow(dead_code)]
#[derive(JsonSchema)]
struct CompoundType {
    value1: String,
    value2: u64,
}

#[allow(dead_code)]
#[derive(JsonSchema)]
enum StringEnum {
    One,
    Two,
    BuckleMyShoe,
}

#[allow(dead_code)]
#[derive(JsonSchema, Serialize)]
#[serde(default = "default_pair")]
struct Pair {
    a: String,
    b: String,
}

fn default_pair() -> Pair {
    Pair {
        a: "A".to_string(),
        b: "B".to_string(),
    }
}

fn add_type<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = T::json_schema(generator).into_object();
    schema.metadata().title = Some(T::schema_name());
    schema.into()
}

#[test]
fn test_generation() {
    let mut type_space = TypeSpace::default();

    type_space.add_derive(quote! { JsonSchema });
    type_space.set_type_mod("types");

    let mut generator = SchemaGenerator::default();
    let body_schema = add_type::<CompoundType>(&mut generator);
    let string_schema = add_type::<String>(&mut generator);
    let opt_int_schema = add_type::<Option<u32>>(&mut generator);
    let strenum_schema = add_type::<StringEnum>(&mut generator);
    let pair_schema = add_type::<Pair>(&mut generator);

    println!("{:#?}", pair_schema);

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
