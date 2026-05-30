use quote::{format_ident, quote};
use typespace::{
    no_cycles, EnumTagType, EnumVariant, StructProperty, StructPropertySerde,
    StructPropertyState, Type, TypeDetails, TypeEnum, TypeEnumVariant, TypeSpaceImpl,
    TypeStruct, TypespaceBuilder, TypespaceSettings, VariantDetails,
};

fn make_typespace() -> typespace::Typespace<String> {
    let mut builder = TypespaceBuilder::default();
    let settings = TypespaceSettings::default();

    let str_id = "str".to_string();
    builder.insert(str_id.clone(), Type::String);

    let u32_id = "u32".to_string();
    builder.insert(u32_id.clone(), Type::Integer("u32".to_string()));

    let bool_id = "bool".to_string();
    builder.insert(bool_id.clone(), Type::Boolean);

    let opt_str_id = "opt_str".to_string();
    builder.insert(opt_str_id.clone(), Type::Option(str_id.clone()));

    // A struct with three properties.
    let struct_id = "MyStruct".to_string();
    builder.insert(
        struct_id.clone(),
        Type::Struct(TypeStruct::new(
            "MyStruct",
            Some("A sample struct".to_string()),
            vec![
                StructProperty::new(
                    format_ident!("name"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    Some("The name field".to_string()),
                    str_id.clone(),
                ),
                StructProperty::new(
                    format_ident!("count"),
                    StructPropertySerde::None,
                    StructPropertyState::Required,
                    None,
                    u32_id.clone(),
                ),
                StructProperty::new(
                    format_ident!("label"),
                    StructPropertySerde::None,
                    StructPropertyState::Optional,
                    None,
                    opt_str_id.clone(),
                ),
            ],
            false,
        )),
    );

    // An enum with three variants: unit, single-item (Item), multi-item (Tuple).
    let enum_id = "MyEnum".to_string();
    builder.insert(
        enum_id.clone(),
        Type::Enum(TypeEnum::new(
            "MyEnum",
            Some("A sample enum".to_string()),
            None,
            EnumTagType::External,
            vec![
                EnumVariant {
                    rust_name: "Nothing".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Unit,
                },
                EnumVariant {
                    rust_name: "Single".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Item(str_id.clone()),
                },
                EnumVariant {
                    rust_name: "Pair".to_string(),
                    rename: None,
                    description: None,
                    details: VariantDetails::Tuple(vec![str_id.clone(), u32_id.clone()]),
                },
            ],
            false,
        )),
    );

    builder.finalize(settings, no_cycles).unwrap()
}

#[test]
#[should_panic(expected = "invalid type id")]
fn get_type_panics_for_unknown_id() {
    let ts = make_typespace();
    ts.get_type(&"nonexistent".to_string());
}

#[test]
fn iter_types_covers_all_inserted_ids() {
    let ts = make_typespace();
    let names: std::collections::BTreeSet<String> =
        ts.iter_types().map(|t| t.name()).collect();
    assert!(names.contains("MyStruct"), "missing MyStruct in {names:?}");
    assert!(names.contains("MyEnum"), "missing MyEnum in {names:?}");
}

#[test]
fn struct_properties_via_get_type() {
    let ts = make_typespace();
    let ti = ts.get_type(&"MyStruct".to_string());

    assert_eq!(ti.name(), "MyStruct");
    assert_eq!(ti.description(), Some("A sample struct"));

    let TypeDetails::Struct(s) = ti.details() else {
        panic!("expected Struct, got something else");
    };

    let props: Vec<_> = s.properties_info().collect();
    assert_eq!(props.len(), 3);

    let name_prop = props.iter().find(|p| p.name == "name").unwrap();
    assert!(name_prop.required);
    assert_eq!(name_prop.description, Some("The name field"));

    let label_prop = props.iter().find(|p| p.name == "label").unwrap();
    assert!(!label_prop.required);
}

#[test]
fn enum_variants_via_get_type() {
    let ts = make_typespace();
    let ti = ts.get_type(&"MyEnum".to_string());

    assert_eq!(ti.name(), "MyEnum");
    assert_eq!(ti.description(), Some("A sample enum"));

    let TypeDetails::Enum(e) = ti.details() else {
        panic!("expected Enum, got something else");
    };

    let variants: Vec<_> = e.variants_info().collect();
    assert_eq!(variants.len(), 3);

    let nothing = variants.iter().find(|v| v.name == "Nothing").unwrap();
    assert!(matches!(nothing.details, TypeEnumVariant::Simple));

    let single = variants.iter().find(|v| v.name == "Single").unwrap();
    assert!(
        matches!(&single.details, TypeEnumVariant::Tuple(ids) if ids.len() == 1),
        "Single should be a one-element Tuple"
    );

    let pair = variants.iter().find(|v| v.name == "Pair").unwrap();
    assert!(
        matches!(&pair.details, TypeEnumVariant::Tuple(ids) if ids.len() == 2),
        "Pair should be a two-element Tuple"
    );
}

#[test]
fn ident_produces_expected_tokens() {
    let ts = make_typespace();

    let struct_ti = ts.get_type(&"MyStruct".to_string());
    let struct_ident = struct_ti.ident();
    assert_eq!(struct_ident.to_string(), quote! { MyStruct }.to_string());

    let param_ident = struct_ti.parameter_ident();
    assert_eq!(param_ident.to_string(), quote! { &MyStruct }.to_string());

    let bool_ti = ts.get_type(&"bool".to_string());
    let bool_param = bool_ti.parameter_ident();
    assert_eq!(bool_param.to_string(), quote! { bool }.to_string());
}

#[test]
fn has_impl_false_for_plain_types() {
    let ts = make_typespace();
    let ti = ts.get_type(&"MyStruct".to_string());
    assert!(!ti.has_impl(TypeSpaceImpl::Display));
    assert!(!ti.has_impl(TypeSpaceImpl::FromStr));
}
