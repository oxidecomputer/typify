use quote::{format_ident, quote};
use typespace::{
    no_cycles, EnumTagType, EnumVariant, JsonValue, StructProperty, StructPropertySerde,
    StructPropertyState, Type, TypeEnum, TypeNative, TypeNewtypeConstraints, TypeNewtypeStruct,
    TypeStruct, TypeTupleStruct, TypeTypeAlias, TypeUnitStruct, TypespaceBuilder,
    TypespaceSettings, TypespaceSettingsOptionalNullable, TypespaceSettingsStd, VariantDetails,
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
            TypespaceSettings::default()
                .with_std(TypespaceSettingsStd::Unqualified)
                .with_optional_nullable(TypespaceSettingsOptionalNullable::ConflateAsAbsent),
        ),
        (
            "ConflatedAsNull",
            TypespaceSettings::default()
                .with_std(TypespaceSettingsStd::Unqualified)
                .with_optional_nullable(TypespaceSettingsOptionalNullable::ConflateAsNull),
        ),
        (
            "DoubleOption",
            TypespaceSettings::default()
                .with_std(TypespaceSettingsStd::Unqualified)
                .with_optional_nullable(TypespaceSettingsOptionalNullable::DoubleOption),
        ),
        (
            "CustomType",
            TypespaceSettings::default()
                .with_std(TypespaceSettingsStd::Unqualified)
                .with_optional_nullable(TypespaceSettingsOptionalNullable::CustomType(
                    "OptionField".to_string(),
                )),
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

        let ts = builder.finalize(settings, no_cycles).unwrap();
        let out = ts.render();

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

    let ts = builder
        .finalize(TypespaceSettings::default(), no_cycles)
        .expect("finalize typespace");

    #[check_and_include("tests/output/test_unit_struct.rs", ts.render())]
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

    let ts = builder
        .finalize(TypespaceSettings::default(), no_cycles)
        .expect("finalize typespace");

    #[check_and_include("tests/output/test_tuple_struct.rs", ts.render())]
    fn inner() {
        // Serialization: rest Vec<String> is flattened into the outer sequence.
        let value = import::MyTupleStruct("hello".to_string(), 42, vec![]);
        assert_eq!(serde_json::to_string(&value).unwrap(), r#"["hello",42]"#);

        let value = import::MyTupleStruct(
            "hello".to_string(),
            42,
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"["hello",42,"a","b","c"]"#
        );

        // Deserialization.
        let value = serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert!(value.2.is_empty());

        let value =
            serde_json::from_str::<import::MyTupleStruct>(r#"["hello",42,"a","b"]"#).unwrap();
        assert_eq!(value.0, "hello");
        assert_eq!(value.1, 42);
        assert_eq!(value.2, vec!["a", "b"]);

        assert!(serde_json::from_str::<import::MyTupleStruct>(r#"[]"#).is_err());
        assert!(serde_json::from_str::<import::MyTupleStruct>(r#"["hello"]"#).is_err());
    }
}

// Enums: one test covering all four serde tag types.
#[test]
fn test_enums() {
    let configs: &[(&str, EnumTagType)] = &[
        ("External", EnumTagType::External),
        (
            "Internal",
            EnumTagType::Internal {
                tag: "type".to_string(),
            },
        ),
        (
            "Adjacent",
            EnumTagType::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        ),
        ("Untagged", EnumTagType::Untagged),
    ];

    let outputs = configs.iter().map(|(name, tag_type)| {
        let mut builder = TypespaceBuilder::default();

        let string_id = "string".to_string();
        builder.insert(string_id.clone(), Type::String);

        let int_id = "integer".to_string();
        builder.insert(int_id.clone(), Type::Integer("u32".to_string()));

        // Internal tagging doesn't support newtype variants wrapping non-struct
        // types, so we use only unit and struct variants for Internal.
        let variants = match tag_type {
            EnumTagType::Internal { .. } => vec![
                EnumVariant {
                    rust_name: "Unit".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Unit,
                },
                EnumVariant {
                    rust_name: "Named".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Struct(vec![StructProperty::new(
                        format_ident!("x"),
                        StructPropertySerde::None,
                        StructPropertyState::Required,
                        None,
                        int_id.clone(),
                    )]),
                },
            ],
            _ => vec![
                EnumVariant {
                    rust_name: "Unit".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Unit,
                },
                EnumVariant {
                    rust_name: "Item".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Item(string_id.clone()),
                },
                EnumVariant {
                    rust_name: "Named".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Struct(vec![StructProperty::new(
                        format_ident!("x"),
                        StructPropertySerde::None,
                        StructPropertyState::Required,
                        None,
                        int_id.clone(),
                    )]),
                },
            ],
        };

        builder.insert(
            "E".to_string(),
            Type::Enum(TypeEnum::new(
                *name,
                None,
                None,
                tag_type.clone(),
                variants,
                false,
            )),
        );

        builder
            .finalize(
                TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
                no_cycles,
            )
            .unwrap()
            .render()
    });

    let output = quote! { #( #outputs )* };

    #[check_and_include("tests/output/test_enums.rs", output)]
    fn inner() {
        // External tagging
        let v: import::External = serde_json::from_str(r#""Unit""#).unwrap();
        assert!(matches!(v, import::External::Unit));

        let v: import::External = serde_json::from_str(r#"{"Item": "hello"}"#).unwrap();
        assert!(matches!(v, import::External::Item(s) if s == "hello"));

        let v: import::External = serde_json::from_str(r#"{"Named": {"x": 42}}"#).unwrap();
        assert!(matches!(v, import::External::Named { x: 42 }));

        assert_eq!(
            serde_json::to_string(&import::External::Unit).unwrap(),
            r#""Unit""#
        );
        assert_eq!(
            serde_json::to_string(&import::External::Item("hi".to_string())).unwrap(),
            r#"{"Item":"hi"}"#
        );

        // Internal tagging
        let v: import::Internal = serde_json::from_str(r#"{"type": "Unit"}"#).unwrap();
        assert!(matches!(v, import::Internal::Unit));

        let v: import::Internal = serde_json::from_str(r#"{"type": "Named", "x": 7}"#).unwrap();
        assert!(matches!(v, import::Internal::Named { x: 7 }));

        // Adjacent tagging
        let v: import::Adjacent = serde_json::from_str(r#"{"t": "Unit"}"#).unwrap();
        assert!(matches!(v, import::Adjacent::Unit));

        let v: import::Adjacent = serde_json::from_str(r#"{"t": "Item", "c": "hello"}"#).unwrap();
        assert!(matches!(v, import::Adjacent::Item(s) if s == "hello"));

        // Untagged
        let v: import::Untagged = serde_json::from_str(r#"null"#).unwrap();
        assert!(matches!(v, import::Untagged::Unit));

        let v: import::Untagged = serde_json::from_str(r#""hello""#).unwrap();
        assert!(matches!(v, import::Untagged::Item(s) if s == "hello"));

        let v: import::Untagged = serde_json::from_str(r#"{"x": 3}"#).unwrap();
        assert!(matches!(v, import::Untagged::Named { x: 3 }));
    }
}

#[test]
fn test_newtype_struct() {
    let mut builder = TypespaceBuilder::default();

    let string_id = "string".to_string();
    builder.insert(string_id.clone(), Type::String);

    let int_id = "integer".to_string();
    builder.insert(int_id.clone(), Type::Integer("u32".to_string()));

    builder.insert(
        "MyString".to_string(),
        Type::NewtypeStruct(TypeNewtypeStruct::new(
            "MyString",
            Some("A newtype wrapping String.".to_string()),
            None,
            string_id,
            TypeNewtypeConstraints::None,
        )),
    );

    builder.insert(
        "MyInt".to_string(),
        Type::NewtypeStruct(TypeNewtypeStruct::new(
            "MyInt",
            None,
            None,
            int_id,
            TypeNewtypeConstraints::None,
        )),
    );

    let ts = builder
        .finalize(
            TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
            no_cycles,
        )
        .unwrap();

    #[check_and_include("tests/output/test_newtype_struct.rs", ts.render())]
    fn inner() {
        let v = import::MyString("hello".to_string());
        assert_eq!(serde_json::to_string(&v).unwrap(), r#""hello""#);
        assert_eq!(*v, "hello");

        let v: import::MyString = serde_json::from_str(r#""world""#).unwrap();
        assert_eq!(v.0, "world");

        let v = import::MyInt(42);
        assert_eq!(serde_json::to_string(&v).unwrap(), "42");
        assert_eq!(*v, 42u32);

        let v: import::MyInt = serde_json::from_str("7").unwrap();
        assert_eq!(v.0, 7);
    }
}

#[test]
fn test_type_alias() {
    let mut builder = TypespaceBuilder::default();

    let string_id = "string".to_string();
    builder.insert(string_id.clone(), Type::String);

    let vec_string_id = "vec_string".to_string();
    builder.insert(vec_string_id.clone(), Type::Vec(string_id.clone()));

    builder.insert(
        "MyAlias".to_string(),
        Type::TypeAlias(TypeTypeAlias::new("MyAlias", None, string_id)),
    );

    builder.insert(
        "StringList".to_string(),
        Type::TypeAlias(TypeTypeAlias::new(
            "StringList",
            Some("A list of strings.".to_string()),
            vec_string_id,
        )),
    );

    let ts = builder
        .finalize(
            TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
            no_cycles,
        )
        .unwrap();

    #[check_and_include("tests/output/test_type_alias.rs", ts.render())]
    fn inner() {
        let v: import::MyAlias = "hello".to_string();
        assert_eq!(v, "hello");

        let v: import::StringList = vec!["a".to_string(), "b".to_string()];
        assert_eq!(v.len(), 2);
    }
}

#[test]
fn test_struct_serde_rename_flatten() {
    let mut builder = TypespaceBuilder::default();

    let string_id = "string".to_string();
    builder.insert(string_id.clone(), Type::String);

    let int_id = "integer".to_string();
    builder.insert(int_id.clone(), Type::Integer("u32".to_string()));

    // Inner struct that will be flattened.
    builder.insert(
        "Inner".to_string(),
        Type::Struct(TypeStruct::new(
            "Inner",
            None,
            vec![StructProperty::new(
                format_ident!("value"),
                StructPropertySerde::None,
                StructPropertyState::Required,
                None,
                int_id.clone(),
            )],
            false,
        )),
    );

    let inner_id = "Inner".to_string();

    // Outer struct with a renamed field and a flattened inner struct.
    builder.insert(
        "Outer".to_string(),
        Type::Struct(TypeStruct::new(
            "Outer",
            None,
            vec![
                StructProperty::new(
                    format_ident!("my_field"),
                    StructPropertySerde::Rename("my-field".to_string()),
                    StructPropertyState::Required,
                    None,
                    string_id.clone(),
                ),
                StructProperty::new(
                    format_ident!("inner"),
                    StructPropertySerde::Flatten,
                    StructPropertyState::Required,
                    None,
                    inner_id,
                ),
            ],
            false,
        )),
    );

    let ts = builder
        .finalize(
            TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
            no_cycles,
        )
        .unwrap();

    #[check_and_include("tests/output/test_struct_serde_rename_flatten.rs", ts.render())]
    fn inner() {
        let v: import::Outer =
            serde_json::from_str(r#"{"my-field": "hello", "value": 42}"#).unwrap();
        assert_eq!(v.my_field, "hello");
        assert_eq!(v.inner.value, 42);

        let json = serde_json::to_string(&v).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["my-field"], "hello");
        assert_eq!(parsed["value"], 42);
    }
}

#[test]
fn test_native_type() {
    let mut builder = TypespaceBuilder::default();

    let uuid_id = "path".to_string();
    builder.insert(
        uuid_id.clone(),
        Type::Native(TypeNative::new_string_like("std::path::PathBuf")),
    );

    builder.insert(
        "Resource".to_string(),
        Type::Struct(TypeStruct::new(
            "Resource",
            None,
            vec![StructProperty::new(
                format_ident!("location"),
                StructPropertySerde::None,
                StructPropertyState::Required,
                None,
                uuid_id,
            )],
            false,
        )),
    );

    let ts = builder
        .finalize(
            TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
            no_cycles,
        )
        .unwrap();

    #[check_and_include("tests/output/test_native_type.rs", ts.render())]
    fn inner() {}
}

#[test]
fn test_compound_field_types() {
    let mut builder = TypespaceBuilder::default();

    let string_id = "string".to_string();
    builder.insert(string_id.clone(), Type::String);

    let int_id = "integer".to_string();
    builder.insert(int_id.clone(), Type::Integer("u32".to_string()));

    let bool_id = "boolean".to_string();
    builder.insert(bool_id.clone(), Type::Boolean);

    let float_id = "float".to_string();
    builder.insert(float_id.clone(), Type::Float("f64".to_string()));

    let json_id = "json".to_string();
    builder.insert(json_id.clone(), Type::JsonValue);

    let vec_id = "vec_string".to_string();
    builder.insert(vec_id.clone(), Type::Vec(string_id.clone()));

    let map_id = "map".to_string();
    builder.insert(map_id.clone(), Type::Map(string_id.clone(), int_id.clone()));

    let set_id = "set".to_string();
    builder.insert(set_id.clone(), Type::Set(string_id.clone()));

    let array_id = "array".to_string();
    builder.insert(array_id.clone(), Type::Array(int_id.clone(), 3));

    let tuple_id = "tuple".to_string();
    builder.insert(
        tuple_id.clone(),
        Type::Tuple(vec![string_id.clone(), int_id.clone()]),
    );

    builder.insert(
        "All".to_string(),
        Type::Struct(TypeStruct::new(
            "All",
            None,
            vec![
                StructProperty::new(
                    format_ident!("a_bool"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    bool_id,
                ),
                StructProperty::new(
                    format_ident!("an_int"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    int_id,
                ),
                StructProperty::new(
                    format_ident!("a_float"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    float_id,
                ),
                StructProperty::new(
                    format_ident!("a_string"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    string_id,
                ),
                StructProperty::new(
                    format_ident!("a_json"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    json_id,
                ),
                StructProperty::new(
                    format_ident!("a_vec"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    vec_id,
                ),
                StructProperty::new(
                    format_ident!("a_map"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    map_id,
                ),
                StructProperty::new(
                    format_ident!("a_set"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    set_id,
                ),
                StructProperty::new(
                    format_ident!("an_array"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    array_id,
                ),
                StructProperty::new(
                    format_ident!("a_tuple"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    tuple_id,
                ),
            ],
            false,
        )),
    );

    let ts = builder
        .finalize(
            TypespaceSettings::default().with_std(TypespaceSettingsStd::Unqualified),
            no_cycles,
        )
        .unwrap();

    #[check_and_include("tests/output/test_compound_field_types.rs", ts.render())]
    fn inner() {
        let v: import::All = serde_json::from_value(serde_json::json!({
            "a_bool": true,
            "an_int": 7,
            "a_float": 3.14,
            "a_string": "hello",
            "a_json": {"any": "thing"},
            "a_vec": ["x", "y"],
            "a_map": {"k": 1},
            "a_set": ["a", "b"],
            "an_array": [1, 2, 3],
            "a_tuple": ["hi", 99],
        }))
        .unwrap();

        assert_eq!(v.a_bool, true);
        assert_eq!(v.an_int, 7);
        assert_eq!(v.a_string, "hello");
        assert_eq!(v.a_vec, vec!["x", "y"]);
        assert_eq!(v.an_array, [1u32, 2, 3]);
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

    assert!(builder
        .finalize(TypespaceSettings::default(), no_cycles)
        .is_err());
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
        .finalize(TypespaceSettings::default(), |_: &i32| next())
        .expect("finalize typespace");

    #[check_and_include("tests/output/test_cycles.rs", ts.render())]
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
