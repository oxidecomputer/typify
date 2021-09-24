use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::ObjectValidation;

use crate::{util::metadata_description, Result, StructProperty, StructPropertySerde, TypeSpace};

pub(crate) fn struct_members(
    validation: &ObjectValidation,
    type_space: &mut TypeSpace,
) -> Result<Vec<StructProperty>> {
    // These are the fields we don't currently handle
    assert!(validation.max_properties.is_none());
    assert!(validation.min_properties.is_none());
    assert!(validation.pattern_properties.is_empty());
    assert!(validation.additional_properties.is_none());
    assert!(validation.property_names.is_none());

    let mut properties = validation
        .properties
        .iter()
        .map(|(name, ty)| struct_property(validation, name, ty, type_space))
        .collect::<Result<Vec<_>>>()?;

    // Sort parameters by name to ensure a deterministic result.
    properties.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(properties)
}

pub(crate) fn struct_property(
    validation: &ObjectValidation,
    prop_name: &str,
    schema: &schemars::schema::Schema,
    type_space: &mut TypeSpace,
) -> Result<StructProperty> {
    let (mut type_id, metadata) = type_space.id_for_schema(None, schema)?;
    if !validation.required.contains(prop_name) {
        type_id = type_space.id_for_option(type_id);
    }

    let name = prop_name.to_case(Case::Snake);
    let serde_options = if name == prop_name {
        StructPropertySerde::None
    } else {
        StructPropertySerde::Rename(prop_name.to_string())
    };

    Ok(StructProperty {
        name,
        serde_options,
        description: metadata_description(metadata),
        type_id,
    })
}

pub(crate) fn output_struct_property(prop: &StructProperty, type_space: &TypeSpace) -> TokenStream {
    let name = format_ident!("{}", prop.name);
    let doc = match &prop.description {
        Some(s) => quote! {#[doc = #s]},
        None => quote! {},
    };
    let serde = match &prop.serde_options {
        StructPropertySerde::Rename(s) => quote! { #[serde(rename = #s)] },
        StructPropertySerde::None => quote! {},
        StructPropertySerde::Flatten => quote! { #[serde(flatten)] },
    };
    let prop_type = type_space.id_to_entry.get(&prop.type_id).unwrap();
    let type_name = prop_type.type_ident(type_space);
    quote! {
        #doc
        #serde
        #name: #type_name,
    }
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::JsonSchema;
    use serde::Serialize;

    use crate::test_util::validate_output;

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    struct SimpleStruct {
        alpha: u32,
        bravo: String,
        charlie: Vec<(String, u32)>,
        delta: Option<String>,
        echo: Option<(u32, String)>,
    }

    #[test]
    fn test_simple_struct() {
        validate_output::<SimpleStruct>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    struct LessSimpleStruct {
        thing: SimpleStruct,
        things: Vec<SimpleStruct>,
    }

    #[test]
    fn test_less_simple_struct() {
        validate_output::<LessSimpleStruct>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    struct FlattenStruct {
        #[serde(flatten)]
        a_to_e: SimpleStruct,
        foxtrot: String,
    }

    #[test]
    fn test_flatten_struct() {
        validate_output::<FlattenStruct>();
    }
}
