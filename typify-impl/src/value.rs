// Copyright 2022 Oxide Computer Company

use std::{collections::BTreeMap, str::FromStr};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    type_entry::{
        EnumTagType, StructProperty, StructPropertyRename, TypeEntry, TypeEntryDetails,
        TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct, Variant, VariantDetails,
    },
    TypeId, TypeSpace,
};

impl TypeEntry {
    /// Emit a [`TokenStream`] for the given [`Value`]
    ///
    /// This returns an Option for programming convenience, but we do not
    /// expect it to fail. All validation should already have been done by
    /// [`validate_default()`].
    ///
    /// [`Value`]: serde_json::Value
    pub fn output_value(
        &self,
        type_space: &TypeSpace,
        value: &serde_json::Value,
        scope: &TokenStream,
    ) -> Option<TokenStream> {
        let v = match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                name,
                tag_type,
                variants,
                ..
            }) => match tag_type {
                EnumTagType::External => {
                    value_for_external_enum(type_space, name, variants, value, scope)?
                }
                EnumTagType::Internal { tag } => {
                    value_for_internal_enum(type_space, name, variants, value, tag, scope)?
                }
                EnumTagType::Adjacent { tag, content } => {
                    value_for_adjacent_enum(type_space, name, variants, value, tag, content, scope)?
                }
                EnumTagType::Untagged => {
                    value_for_untagged_enum(type_space, name, variants, value, scope)?
                }
            },
            TypeEntryDetails::Struct(TypeEntryStruct {
                name, properties, ..
            }) => {
                let props = value_for_struct_props(properties, value, type_space, scope)?;
                let ident = format_ident!("{}", name);
                quote! { #scope #ident { #( #props ),* }}
            }
            TypeEntryDetails::Newtype(TypeEntryNewtype { name, type_id, .. }) => {
                let inner = type_space
                    .id_to_entry
                    .get(type_id)
                    .unwrap()
                    .output_value(type_space, value, scope);
                let ident = format_ident!("{}", name);
                quote! { #scope #ident ( #inner )}
            }

            TypeEntryDetails::Option(type_id) => {
                if let serde_json::Value::Null = value {
                    quote! { None }
                } else {
                    let inner = type_space
                        .id_to_entry
                        .get(type_id)
                        .unwrap()
                        .output_value(type_space, value, scope)?;
                    quote! { Some(#inner) }
                }
            }
            TypeEntryDetails::Box(type_id) => {
                let inner = type_space
                    .id_to_entry
                    .get(type_id)
                    .unwrap()
                    .output_value(type_space, value, scope)?;
                quote! { Box::new(#inner) }
            }
            // TODO: this should become a HashSet<_> once we figure out the
            // derives more precisely.
            TypeEntryDetails::Set(type_id) | TypeEntryDetails::Array(type_id) => {
                let arr = value.as_array()?;
                let inner = type_space.id_to_entry.get(type_id).unwrap();
                let values = arr
                    .iter()
                    .map(|arr_value| inner.output_value(type_space, arr_value, scope))
                    .collect::<Option<Vec<_>>>()?;
                quote! { vec![#(#values),*] }
            }
            TypeEntryDetails::Map(type_id) => {
                let obj = value.as_object()?;
                let inner = type_space.id_to_entry.get(type_id).unwrap();
                let kvs = obj
                    .iter()
                    .map(|(name, obj_value)| {
                        Some((name, inner.output_value(type_space, obj_value, scope)?))
                    })
                    .collect::<Option<Vec<_>>>()?;
                let (keys, values): (Vec<_>, Vec<_>) = kvs.into_iter().unzip();
                quote! {
                   [#( (#keys, #values) ),*]
                       .into_iter()
                       .collect()
                }
            }
            TypeEntryDetails::Tuple(types) => {
                let tup = value_for_tuple(type_space, value, types, scope)?;
                quote! { ( #( #tup ),* )}
            }
            TypeEntryDetails::Unit => {
                value.as_null()?;
                quote! { () }
            }
            TypeEntryDetails::BuiltIn(type_name) => {
                // Serialize value to a string... not hard.
                let text = value.to_string();
                let type_path = type_name
                    .split("::")
                    .map(|component| format_ident!("{}", component));

                // Deserialize the string to the type; the unwrap() is
                // unfortunate, but unavoidable without getting in the
                // underpants of the serialized form of these built-in types.
                quote! {
                    serde_json::from_str::< #( #type_path )::* >(#text).unwrap()
                }
            }
            TypeEntryDetails::Boolean => {
                let v = value.as_bool()?;
                quote! { #v }
            }
            TypeEntryDetails::Integer(type_name) | TypeEntryDetails::Float(type_name) => {
                value.is_number().then(|| ())?;
                let val = match proc_macro2::Literal::from_str(&format!("{}_{}", value, type_name))
                {
                    Ok(v) => v,
                    Err(_) => unreachable!(),
                };
                TokenStream::from(proc_macro2::TokenTree::from(val))
            }
            TypeEntryDetails::String => {
                let s = value.as_str()?;
                quote! { #s.to_string() }
            }
            TypeEntryDetails::Reference(_) => unreachable!(),
        };
        Some(v)
    }
}

fn value_for_external_enum(
    type_space: &TypeSpace,
    type_name: &str,
    variants: &[Variant],
    value: &serde_json::Value,
    scope: &TokenStream,
) -> Option<TokenStream> {
    if let Some(simple_name) = value.as_str() {
        let variant = variants
            .iter()
            .find(|variant| simple_name == variant.rename.as_ref().unwrap_or(&variant.name))?;
        matches!(&variant.details, VariantDetails::Simple).then(|| ())?;

        let var_ident = format_ident!("{}", &variant.name);
        let type_ident = format_ident!("{}", type_name);
        Some(quote! { #scope #type_ident::#var_ident })
    } else {
        let map = value.as_object()?;
        (map.len() == 1).then(|| ())?;

        let (name, var_value) = map.iter().next()?;

        let variant = variants
            .iter()
            .find(|variant| name == variant.rename.as_ref().unwrap_or(&variant.name))?;

        let var_ident = format_ident!("{}", &variant.name);
        let type_ident = format_ident!("{}", type_name);
        match &variant.details {
            VariantDetails::Simple => None,
            VariantDetails::Item(type_id) => {
                let item = value_for_item(type_space, var_value, type_id, scope);
                Some(quote! { #scope #type_ident::#var_ident ( #item ::default() ) })
            }
            VariantDetails::Tuple(types) => {
                let tup = value_for_tuple(type_space, var_value, types, scope)?;
                Some(quote! { #scope #type_ident::#var_ident ( #( #tup ),* ) })
            }
            VariantDetails::Struct(props) => {
                let props = value_for_struct_props(props, var_value, type_space, scope)?;
                Some(quote! { #scope #type_ident::#var_ident { #( #props ),* } })
            }
        }
    }
}

fn value_for_internal_enum(
    type_space: &TypeSpace,
    type_name: &str,
    variants: &[Variant],
    value: &serde_json::Value,
    tag: &str,
    scope: &TokenStream,
) -> Option<TokenStream> {
    let map = value.as_object()?;
    let ser_name = map.get(tag).and_then(serde_json::Value::as_str)?;
    let variant = variants
        .iter()
        .find(|variant| ser_name == variant.rename.as_ref().unwrap_or(&variant.name))?;
    let var_ident = format_ident!("{}", &variant.name);
    let type_ident = format_ident!("{}", type_name);

    match &variant.details {
        VariantDetails::Simple => Some(quote! { #scope #type_ident::#var_ident }),
        VariantDetails::Struct(props) => {
            // Make an object without the tag.
            let inner_value = serde_json::Value::Object(
                map.clone()
                    .into_iter()
                    .filter(|(name, _)| name != tag)
                    .collect(),
            );

            let props = value_for_struct_props(props, &inner_value, type_space, scope)?;
            Some(quote! { #scope #type_ident::#var_ident { #( #props ),* } })
        }

        VariantDetails::Item(_) | VariantDetails::Tuple(_) => unreachable!(),
    }
}

fn value_for_adjacent_enum(
    type_space: &TypeSpace,
    type_name: &str,
    variants: &[Variant],
    value: &serde_json::Value,
    tag: &str,
    content: &str,
    scope: &TokenStream,
) -> Option<TokenStream> {
    let map = value.as_object()?;

    let (tag_value, content_value) = match (
        map.len(),
        map.get(tag).and_then(serde_json::Value::as_str),
        map.get(content),
    ) {
        (1, Some(tag_value), None) => (tag_value, None),
        (2, Some(tag_value), content_value @ Some(_)) => (tag_value, content_value),
        _ => return None,
    };

    let variant = variants
        .iter()
        .find(|variant| tag_value == variant.rename.as_ref().unwrap_or(&variant.name))?;
    let type_ident = format_ident!("{}", type_name);
    let var_ident = format_ident!("{}", &variant.name);
    match (&variant.details, content_value) {
        (VariantDetails::Simple, None) => Some(quote! { #scope #type_ident::#var_ident}),
        (VariantDetails::Tuple(types), Some(content_value)) => {
            let tup = value_for_tuple(type_space, content_value, types, scope)?;
            Some(quote! { #scope #type_ident::#var_ident ( #( #tup ),* ) })
        }
        (VariantDetails::Struct(props), Some(content_value)) => {
            let props = value_for_struct_props(props, content_value, type_space, scope)?;
            Some(quote! { #scope #type_ident::#var_ident { #( #props ),* } })
        }
        _ => None,
    }
}

fn value_for_untagged_enum(
    type_space: &TypeSpace,
    type_name: &str,
    variants: &[Variant],
    value: &serde_json::Value,
    scope: &TokenStream,
) -> Option<TokenStream> {
    let type_ident = format_ident!("{}", type_name);
    variants.iter().find_map(|variant| {
        let var_ident = format_ident!("{}", &variant.name);
        match &variant.details {
            VariantDetails::Simple => {
                value.as_null()?;
                Some(quote! { #scope #type_ident::#var_ident })
            }
            VariantDetails::Item(type_id) => {
                let item = value_for_item(type_space, value, type_id, scope)?;
                Some(quote! { #scope #type_ident::#var_ident ( #item ) })
            }
            VariantDetails::Tuple(types) => {
                let tup = value_for_tuple(type_space, value, types, scope)?;
                Some(quote! { #scope #type_ident::#var_ident ( #( #tup ),* ) })
            }
            VariantDetails::Struct(props) => {
                let props = value_for_struct_props(props, value, type_space, scope)?;
                Some(quote! { #scope #type_ident::#var_ident { #( #props ),* } })
            }
        }
    })
}

fn value_for_item(
    type_space: &TypeSpace,
    value: &serde_json::Value,
    type_id: &TypeId,
    scope: &TokenStream,
) -> Option<TokenStream> {
    type_space
        .id_to_entry
        .get(type_id)
        .unwrap()
        .output_value(type_space, value, scope)
}

fn value_for_tuple(
    type_space: &TypeSpace,
    value: &serde_json::Value,
    types: &[TypeId],
    scope: &TokenStream,
) -> Option<Vec<TokenStream>> {
    let arr = value.as_array()?;
    (arr.len() == types.len()).then(|| ())?;
    types
        .iter()
        .zip(arr)
        .map(|(type_id, tup_value)| {
            type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .output_value(type_space, tup_value, scope)
        })
        .collect()
}

fn value_for_struct_props(
    properties: &[StructProperty],
    value: &serde_json::Value,
    type_space: &TypeSpace,
    scope: &TokenStream,
) -> Option<Vec<TokenStream>> {
    let map = value.as_object()?;

    let prop_map = properties
        .iter()
        .filter_map(|prop| {
            let name = match &prop.rename {
                StructPropertyRename::None => &prop.name,
                StructPropertyRename::Rename(rename) => rename,
                StructPropertyRename::Flatten => return None,
            };

            Some((name, prop))
        })
        .collect::<BTreeMap<_, _>>();

    let direct_props = map.iter().filter_map(|(name, value)| {
        // It's okay if the property isn't in the prop_map... it must be part
        // of one of the flattened properties.
        let prop = prop_map.get(name)?;
        let type_entry = type_space.id_to_entry.get(&prop.type_id).unwrap();
        let prop_value = type_entry.output_value(type_space, value, scope)?;
        let name_ident = format_ident!("{}", name);

        Some(quote! { #name_ident: #prop_value })
    });

    let extra_value = serde_json::Value::Object(
        map.clone()
            .into_iter()
            .filter(|(name, _)| prop_map.get(name).is_none())
            .collect(),
    );

    let flat_props = properties.iter().filter_map(|prop| match &prop.rename {
        StructPropertyRename::Flatten => {
            let type_entry = type_space.id_to_entry.get(&prop.type_id).unwrap();

            // The flattened type must be a struct, map or option for a struct.
            match &type_entry.details {
                TypeEntryDetails::Struct(_)
                | TypeEntryDetails::Option(_)
                | TypeEntryDetails::Map(_) => (),
                _ => unreachable!(),
            }

            let flat_value = type_entry.output_value(type_space, &extra_value, scope)?;
            let name = &prop.name;
            Some(quote! { #name: #flat_value })
        }
        _ => None,
    });

    Some(direct_props.chain(flat_props).collect())
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use schemars::JsonSchema;
    use serde_json::json;
    use std::collections::HashMap;
    use uuid::Uuid;

    use crate::{test_util::get_type, type_entry::TypeEntry};

    #[test]
    fn test_value_option() {
        let (type_space, type_id) = get_type::<Option<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(null), &quote! {})
                .map(|x| x.to_string()),
            Some("None".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(42), &quote! {})
                .map(|x| x.to_string()),
            Some("Some (42_u32)".to_string()),
        );
    }

    #[test]
    fn test_value_box() {
        let (type_space, type_id) = get_type::<Option<u32>>();

        let type_entry = TypeEntry {
            details: crate::type_entry::TypeEntryDetails::Box(type_id),
            derives: Default::default(),
            impls: Default::default(),
        };

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(null), &quote! {})
                .map(|x| x.to_string()),
            Some("Box :: new (None)".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(42), &quote! {})
                .map(|x| x.to_string()),
            Some("Box :: new (Some (42_u32))".to_string()),
        );
    }

    #[test]
    fn test_value_array() {
        let (type_space, type_id) = get_type::<Vec<u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!([]), &quote! {})
                .map(|x| x.to_string()),
            Some("vec ! []".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!([1, 2, 5]), &quote! {})
                .map(|x| x.to_string()),
            Some("vec ! [1_u32 , 2_u32 , 5_u32]".to_string()),
        );
    }

    #[test]
    fn test_value_map() {
        let (type_space, type_id) = get_type::<HashMap<String, u32>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!({}), &quote! {})
                .map(|x| x.to_string()),
            Some("[] . into_iter () . collect ()".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!({"a": 1, "b": 2}), &quote! {})
                .map(|x| x.to_string()),
            Some(r#"[("a" , 1_u32) , ("b" , 2_u32)] . into_iter () . collect ()"#.to_string()),
        );
    }

    #[test]
    fn test_value_tuple() {
        let (type_space, type_id) = get_type::<(u32, u32, String)>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!([1, 2, "three"]), &quote! {})
                .map(|x| x.to_string()),
            Some(r#"(1_u32 , 2_u32 , "three" . to_string ())"#.to_string()),
        );
    }

    #[test]
    fn test_value_builtin() {
        let (type_space, type_id) = get_type::<Uuid>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!("not-a-uuid"), &quote! {})
                .map(|x| x.to_string()),
            Some(
                quote! {
                    serde_json::from_str::<uuid::Uuid>("\"not-a-uuid\"").unwrap()
                }
                .to_string()
            ),
        );
    }

    #[test]
    fn test_value_bool() {
        let (type_space, type_id) = get_type::<Option<bool>>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(true), &quote! {})
                .map(|x| x.to_string()),
            Some("Some (true)".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(false), &quote! {})
                .map(|x| x.to_string()),
            Some("Some (false)".to_string()),
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(null), &quote! {})
                .map(|x| x.to_string()),
            Some("None".to_string()),
        );
    }

    #[test]
    fn test_value_numbers_and_string() {
        let (type_space, type_id) = get_type::<(u32, i64, f64, String)>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!([0, 0, 0, "zero"]), &quote! {})
                .map(|x| x.to_string()),
            Some(r#"(0_u32 , 0_i64 , 0_f64 , "zero" . to_string ())"#.to_string()),
        );
    }

    #[test]
    fn test_struct_simple_super_scoped() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        struct Test {
            a: String,
            b: u32,
            c: Option<String>,
            d: Option<f64>,
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!(
                        {
                            "a": "aaaa",
                            "b": 7,
                            "c": "cccc"
                        }
                    ),
                    &quote! {super::}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test {
                        a: "aaaa".to_string(),
                        b: 7_u32,
                        c: Some("cccc".to_string())
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_struct_simple() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        struct Test {
            a: String,
            b: u32,
            c: Option<String>,
            d: Option<f64>,
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!(
                        {
                            "a": "aaaa",
                            "b": 7,
                            "c": "cccc"
                        }
                    ),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test {
                        a: "aaaa".to_string(),
                        b: 7_u32,
                        c: Some("cccc".to_string())
                    }
                }
                .to_string()
            )
        );
    }
    #[test]
    fn test_enum_external_super_scoped() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!("A"), &quote! { super:: })
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "B": ["xx", "yy"]
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::B("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "C": { "cc": "xx", "dd": "yy" }
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_external() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!("A"), &quote! {})
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "B": ["xx", "yy"]
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::B("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "C": { "cc": "xx", "dd": "yy" }
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_internal_super_scoped() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag")]
        enum Test {
            A,
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "A"
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "C",
                        "cc": "xx",
                        "dd": "yy"
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_internal() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag")]
        enum Test {
            A,
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "A"
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "C",
                        "cc": "xx",
                        "dd": "yy"
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_adjacent_super_scoped() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag", content = "content")]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "A"
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "B",
                        "content": ["xx", "yy"]
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::B("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "C",
                        "content": { "cc": "xx", "dd": "yy" }
                    }),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_adjacent() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(tag = "tag", content = "content")]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "A"
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::A
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "B",
                        "content": ["xx", "yy"]
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::B("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!({
                        "tag": "C",
                        "content": { "cc": "xx", "dd": "yy" }
                    }),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::C {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_untagged_super_scoped() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(untagged)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(null), &quote! { super:: })
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::Variant0
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(["xx", "yy"]), &quote! { super:: })
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::Variant1("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!(
                         { "cc": "xx", "dd": "yy" }
                    ),
                    &quote! { super:: }
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    super::Test::Variant2 {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }

    #[test]
    fn test_enum_untagged() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        #[serde(untagged)]
        enum Test {
            A,
            B(String, String),
            C { cc: String, dd: String },
        }

        let (type_space, type_id) = get_type::<Test>();
        let type_entry = type_space.id_to_entry.get(&type_id).unwrap();

        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(null), &quote! {})
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::Variant0
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(&type_space, &json!(["xx", "yy"]), &quote! {})
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::Variant1("xx".to_string(), "yy".to_string())
                }
                .to_string()
            )
        );
        assert_eq!(
            type_entry
                .output_value(
                    &type_space,
                    &json!(
                         { "cc": "xx", "dd": "yy" }
                    ),
                    &quote! {}
                )
                .map(|x| x.to_string()),
            Some(
                quote! {
                    Test::Variant2 {
                        cc: "xx".to_string(),
                        dd: "yy".to_string()
                    }
                }
                .to_string()
            )
        );
    }
}
