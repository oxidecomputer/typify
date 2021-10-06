use quote::quote;
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use typify_impl::{TypeEntryIdentifier, TypeSpace};

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

fn add_type<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = T::json_schema(generator).into_object();
    schema.metadata().title = Some(T::schema_name());
    schema.into()
}

#[test]
fn test_generation() {
    let mut type_space = TypeSpace::default();

    let mut generator = SchemaGenerator::default();
    let body_schema = add_type::<CompoundType>(&mut generator);
    let string_schema = add_type::<String>(&mut generator);
    let opt_int_schema = add_type::<Option<u32>>(&mut generator);
    let strenum_schema = add_type::<StringEnum>(&mut generator);

    type_space
        .add_ref_types(generator.take_definitions())
        .unwrap();

    let TypeEntryIdentifier {
        ident: ret,
        parameter: body,
    } = type_space.add_type_details(&body_schema).unwrap();
    let string = type_space
        .add_type_details(&string_schema)
        .unwrap()
        .parameter;
    let opt_int = type_space
        .add_type_details(&opt_int_schema)
        .unwrap()
        .parameter;
    let strenum = type_space
        .add_type_details(&strenum_schema)
        .unwrap()
        .parameter;

    let file = quote! {
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
