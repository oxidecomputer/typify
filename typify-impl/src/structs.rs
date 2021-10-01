use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{Metadata, ObjectValidation, Schema};

use crate::{
    util::{get_type_name, metadata_description, recase},
    Name, Result, StructProperty, StructPropertySerde, TypeDetails, TypeEntry, TypeSpace,
};

pub(crate) fn struct_members(
    type_name: Option<String>,
    validation: &ObjectValidation,
    type_space: &mut TypeSpace,
) -> Result<Vec<StructProperty>> {
    // These are the fields we don't currently handle
    assert!(validation.max_properties.is_none());
    assert!(validation.min_properties.is_none());
    assert!(validation.pattern_properties.is_empty());
    assert!(
        validation.additional_properties.is_none()
            || matches!(
                validation.additional_properties.as_ref().map(Box::as_ref),
                Some(Schema::Bool(false))
            )
    );
    assert!(validation.property_names.is_none());

    let mut properties = validation
        .properties
        .iter()
        .map(|(name, ty)| struct_property(type_name.clone(), validation, name, ty, type_space))
        .collect::<Result<Vec<_>>>()?;

    // Sort parameters by name to ensure a deterministic result.
    properties.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(properties)
}

pub(crate) fn struct_property(
    type_name: Option<String>,
    validation: &ObjectValidation,
    prop_name: &str,
    schema: &schemars::schema::Schema,
    type_space: &mut TypeSpace,
) -> Result<StructProperty> {
    let sub_type_name = match type_name {
        Some(name) => Name::Suggested(format!("{}{}", name, prop_name.to_case(Case::Pascal))),
        None => Name::Unknown,
    };
    let (mut type_id, metadata) = type_space.id_for_schema(sub_type_name, schema)?;
    if !validation.required.contains(prop_name) {
        type_id = type_space.id_for_option(&type_id);
    }

    let (name, rename) = recase(prop_name.to_string(), Case::Snake);
    let serde_options = match rename {
        Some(old_name) => StructPropertySerde::Rename(old_name),
        None => StructPropertySerde::None,
    };

    Ok(StructProperty {
        name,
        serde_options,
        description: metadata_description(metadata),
        type_id,
    })
}

pub(crate) fn output_struct_property(
    prop: &StructProperty,
    type_space: &TypeSpace,
    make_pub: bool,
) -> TokenStream {
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
    let type_name = prop_type.type_ident(type_space, false);
    let pub_token = if make_pub {
        quote! { pub }
    } else {
        quote! {}
    };
    quote! {
        #doc
        #serde
        #pub_token #name: #type_name,
    }
}

/// This is used by both any-of and all-of subschema processing. This
/// produces a struct type whose members are the subschemas (flattened).
///
/// ```ignore
/// struct Name {
///     #[serde(flatten)]
///     schema1: Schema1Type,
///     #[serde(flatten)]
///     schema2: Schema2Type
///     ...
/// }
/// ```
///
/// The only difference between any-of and all-of is that where the latter
/// has type T_N for each member of the struct, the former has Option<T_N>.
pub(crate) fn flattened_union_struct<'a>(
    type_name: Name,
    metadata: &'a Option<Box<Metadata>>,
    subschemas: &[Schema],
    optional: bool,
    type_space: &mut TypeSpace,
) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
    let properties = subschemas
        .iter()
        .enumerate()
        .map(|(idx, schema)| {
            let type_name = match get_type_name(&type_name, metadata, Case::Pascal) {
                Some(name) => Name::Suggested(format!("{}Subtype{}", name, idx)),
                None => Name::Unknown,
            };

            let (mut type_id, _) = type_space.id_for_schema(type_name, schema)?;
            if optional {
                type_id = type_space.id_for_option(&type_id);
            }

            // TODO we need a reasonable name that could be derived
            // from the name of the type
            let name = format!("subtype_{}", idx);

            Ok(StructProperty {
                name,
                serde_options: StructPropertySerde::Flatten,
                description: None,
                type_id,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Struct(properties));

    Ok((ty, metadata))
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
}
