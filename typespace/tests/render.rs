use quote::{format_ident, quote};
use typespace::{
    no_cycles, JsonValue, StructProperty, StructPropertySerde, StructPropertyState, Type,
    TypeStruct, TypeTupleStruct, TypeUnitStruct, TypespaceBuilder, TypespaceSettings,
    TypespaceSettingsOptionalNullable, TypespaceSettingsStd,
};
use typespace_test_macro::check_and_include;

// Stub for the user-provided type referenced by TypespaceSettingsOptionalNullable::CustomType.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OptionField<T> {
    #[default]
    #[serde(skip)]
    Absent,
    Null,
    Present(T),
}
impl<T> OptionField<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, OptionField::Absent)
    }
}

#[test]
fn test_struct_field_serde() {
    let configs = [
        (
            "ConflatedAsAbsent",
            TypespaceSettings {
                std: TypespaceSettingsStd::Unqualified,
                optional_nullable: TypespaceSettingsOptionalNullable::ConflateAsAbsent,
            },
        ),
        (
            "ConflatedAsNull",
            TypespaceSettings {
                std: TypespaceSettingsStd::Unqualified,
                optional_nullable: TypespaceSettingsOptionalNullable::ConflateAsNull,
            },
        ),
        (
            "DoubleOption",
            TypespaceSettings {
                std: TypespaceSettingsStd::Unqualified,
                optional_nullable: TypespaceSettingsOptionalNullable::DoubleOption,
            },
        ),
        (
            "CustomType",
            TypespaceSettings {
                std: TypespaceSettingsStd::Unqualified,
                optional_nullable: TypespaceSettingsOptionalNullable::CustomType(
                    "OptionField".to_string(),
                ),
            },
        ),
    ];

    // For each configuration we create a type with the following fields:
    // - optional_string: A string that may be absent
    // - required_option: Either a string or null, but must be present
    // - optional_option: A string, null, or absent
    // - default_string: A string with the intrinsic default (i.e. "")
    // - default_option: A string or null with the intrinsic default (i.e. null)
    // - peanut_string: A string with a custom default of "peanuts"
    // - peanut_option: A string or null with a custom default of "peanuts"
    let outputs = configs.into_iter().map(|(name, settings)| {
        let mut builder = TypespaceBuilder::default();

        let string_id = "string".to_string();
        builder.insert(string_id.clone(), Type::String);

        let option_id = "option_string".to_string();
        builder.insert(option_id.clone(), Type::Option(string_id.clone()));

        let properties = vec![
            StructProperty::new(
                format_ident!("optional_string"),
                StructPropertySerde::None,
                StructPropertyState::Optional,
                None,
                string_id.clone(),
            ),
            StructProperty::new(
                format_ident!("required_option"),
                StructPropertySerde::None,
                StructPropertyState::Required,
                None,
                option_id.clone(),
            ),
            StructProperty::new(
                format_ident!("optional_option"),
                StructPropertySerde::None,
                StructPropertyState::Optional,
                None,
                option_id.clone(),
            ),
            StructProperty::new(
                format_ident!("default_string"),
                StructPropertySerde::None,
                StructPropertyState::Default,
                None,
                string_id.clone(),
            ),
            StructProperty::new(
                format_ident!("default_option"),
                StructPropertySerde::None,
                StructPropertyState::Default,
                None,
                option_id.clone(),
            ),
            StructProperty::new(
                format_ident!("peanut_string"),
                StructPropertySerde::None,
                StructPropertyState::DefaultValue(JsonValue::new(serde_json::json!("peanuts"))),
                None,
                string_id.clone(),
            ),
            StructProperty::new(
                format_ident!("peanut_option"),
                StructPropertySerde::None,
                StructPropertyState::DefaultValue(JsonValue::new(serde_json::json!("peanuts"))),
                None,
                option_id.clone(),
            ),
        ];

        builder.insert(
            "X".to_string(),
            Type::Struct(TypeStruct::new(name, None, properties, false)),
        );

        let ts = builder.finalize(no_cycles).unwrap();
        let out = ts.render(settings);

        out
    });

    let output = quote! {
        #( #outputs )*
    };

    #[check_and_include("tests/output/test_struct_field_serde.rs", output)]
    fn inner() {
        use serde::Deserialize;

        // optional_string absent → None
        let v: import::ConflatedAsAbsent =
            serde_json::from_str(r#"{"required_option": null}"#).unwrap();
        assert!(v.optional_string.is_none());
        assert!(v.required_option.is_none());
        assert_eq!(v.peanut_string, "peanuts");
        assert_eq!(v.peanut_option, Some("peanuts".to_string()));

        // optional_string present → Some
        let v: import::ConflatedAsAbsent =
            serde_json::from_str(r#"{"required_option": null, "optional_string": "hi"}"#).unwrap();
        assert_eq!(v.optional_string, Some("hi".to_string()));

        // DoubleOption: optional_option absent → None, present-null → Some(None)
        let v: import::DoubleOption = serde_json::from_str(r#"{"required_option": null}"#).unwrap();
        assert!(v.optional_option.is_none());

        // CustomType: use OptionField stub from outer scope
        let value = serde_json::json!({
            "required_option": null,
        });
        let value = import::CustomType::deserialize(value).unwrap();
        assert!(matches!(value.optional_option, OptionField::Absent));

        let value = serde_json::json!({
            "required_option": null,
            "optional_option": null,
        });
        let value = import::CustomType::deserialize(value).unwrap();
        assert!(
            matches!(value.optional_option, OptionField::Null),
            "expected Null, got {:?}",
            value.optional_option,
        );

        let value = serde_json::json!({
            "required_option": null,
            "optional_option": "howdy",
        });
        let value = import::CustomType::deserialize(value).unwrap();
        assert!(matches!(
            value.optional_option,
            OptionField::Present(s) if s == "howdy"
        ));
    }
}

#[test]
fn test_unit_struct() {
    let mut builder = TypespaceBuilder::default();

    builder.insert(
        "MyUnitStruct".to_string(),
        Type::UnitStruct(TypeUnitStruct::new(
            "MyUnitStruct",
            None,
            serde_json::json!("<<+>>"),
        )),
    );

    let ts = builder.finalize(no_cycles).expect("finalize typespace");

    #[check_and_include("tests/output/test_unit_struct.rs", ts.render(TypespaceSettings::default()))]
    fn inner() {
        let value = import::MyUnitStruct;
        assert_eq!(serde_json::to_string(&value).unwrap(), "\"<<+>>\"");

        assert!(serde_json::from_str::<import::MyUnitStruct>("\"<<+>>\"").is_ok());
        assert!(serde_json::from_str::<import::MyUnitStruct>("null").is_err());
    }
}

#[test]
fn test_tuple_struct() {
    let mut builder = TypespaceBuilder::default();

    let int_id = "integer".to_string();
    builder.insert(int_id.clone(), Type::Integer("u32".to_string()));

    let string_id = "string".to_string();
    builder.insert(string_id.clone(), Type::String);

    let string_vec_id = "string_vec".to_string();
    builder.insert(string_vec_id.clone(), Type::Vec(string_id.clone()));

    builder.insert(
        "MyTupleStruct".to_string(),
        Type::TupleStruct(TypeTupleStruct::new(
            "MyTupleStruct",
            None,
            vec![string_id, int_id],
            Some(string_vec_id),
        )),
    );

    let ts = builder.finalize(no_cycles).expect("finalize typespace");

    #[check_and_include("tests/output/test_tuple_struct.rs", ts.render(TypespaceSettings::default()))]
    fn inner() {
        // Serialization: rest Vec<String> is a nested array (simple derive behavior).
        let value = import::MyTupleStruct("hello".to_string(), 42, vec![]);
        assert_eq!(serde_json::to_string(&value).unwrap(), r#"["hello",42,[]]"#);

        let value = import::MyTupleStruct(
            "hello".to_string(),
            42,
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"["hello",42,["a","b","c"]]"#
        );

        // Deserialization.
        let value = serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42,[]]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert!(value.2.is_empty());

        let value =
            serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42,["a","b"]]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert_eq!(value.2, vec!["a", "b"]);

        assert!(serde_json::from_str::<import::MyTupleStruct>(r#"[]"#).is_err());
        assert!(serde_json::from_str::<import::MyTupleStruct>(r#"["hello"]"#).is_err());
    }
}

// A map whose key is a struct containing a float cannot satisfy the Ord and Eq
// constraints required of BTreeMap keys; finalize should return Err.
#[test]
fn test_map_key_struct_with_float() {
    let mut builder = TypespaceBuilder::default();

    let float_id = "float".to_string();
    builder.insert(float_id.clone(), Type::Float("f64".to_string()));

    let key_id = "key".to_string();
    builder.insert(
        key_id.clone(),
        Type::Struct(TypeStruct::new(
            "Key",
            None,
            vec![StructProperty::new(
                format_ident!("value"),
                StructPropertySerde::None,
                StructPropertyState::Required,
                None,
                float_id,
            )],
            false,
        )),
    );

    let value_id = "value".to_string();
    builder.insert(value_id.clone(), Type::String);

    builder.insert("map".to_string(), Type::Map(key_id, value_id));

    assert!(builder.finalize(no_cycles).is_err());
}

// Test a some simple cyclic types.
#[test]
fn test_cycles() {
    let mut builder = TypespaceBuilder::default();

    let mut id = 0;

    let mut next = || {
        id += 1;
        id
    };

    let struct_a_id = next();
    builder.insert(
        struct_a_id,
        Type::Struct(TypeStruct::new(
            "A",
            None,
            vec![StructProperty::new(
                format_ident!("a"),
                StructPropertySerde::None,
                StructPropertyState::Optional,
                None,
                struct_a_id,
            )],
            false,
        )),
    );

    let b_id = next();
    let c_id = next();
    builder.insert(
        b_id,
        Type::Struct(TypeStruct::new(
            "B",
            None,
            vec![StructProperty::new(
                format_ident!("c"),
                StructPropertySerde::None,
                StructPropertyState::Optional,
                None,
                c_id,
            )],
            false,
        )),
    );
    builder.insert(
        c_id,
        Type::Struct(TypeStruct::new(
            "C",
            None,
            vec![StructProperty::new(
                format_ident!("b"),
                StructPropertySerde::None,
                StructPropertyState::Optional,
                None,
                b_id,
            )],
            false,
        )),
    );

    let ts = builder
        .finalize(|_: &i32| next())
        .expect("finalize typespace");

    #[check_and_include("tests/output/test_cycles.rs", ts.render(TypespaceSettings::default()))]
    fn inner() {
        let value = serde_json::json!({
            "a": {
                "a": {}
            }
        });
        let value = serde_json::from_value::<import::A>(value).unwrap();
        assert!(value.a.is_some());
        assert!(value.a.as_ref().unwrap().a.is_some());
        assert!(value.a.unwrap().a.unwrap().a.is_none());

        // A with a nested A that has no 'a' field (absent = None).
        let a: import::A = serde_json::from_str(r#"{"a": {}}"#).unwrap();
        assert!(a.a.is_some());
        assert!(a.a.unwrap().a.is_none());

        // B and C cycle: B with absent 'c', C with nested B with absent 'c'.
        let _b: import::B = serde_json::from_str(r#"{}"#).unwrap();
        let _c: import::C = serde_json::from_str(r#"{"b": {}}"#).unwrap();
    }
}
