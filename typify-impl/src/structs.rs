use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    util::{get_type_name, metadata_description, recase, schema_is_named},
    Name, Result, SerdeNaming, SerdeRules, StructProperty, TypeDetails, TypeEntry, TypeSpace,
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
        .map(|(name, ty)| {
            struct_property(
                type_name.clone(),
                &validation.required,
                name,
                ty,
                type_space,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    // Sort parameters by name to ensure a deterministic result.
    properties.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(properties)
}

pub(crate) fn struct_property(
    type_name: Option<String>,
    required: &schemars::Set<String>,
    prop_name: &str,
    schema: &schemars::schema::Schema,
    type_space: &mut TypeSpace,
) -> Result<StructProperty> {
    let sub_type_name = match type_name {
        Some(name) => Name::Suggested(format!("{}{}", name, prop_name.to_case(Case::Pascal))),
        None => Name::Unknown,
    };
    let (mut type_id, metadata) = type_space.id_for_schema(sub_type_name, schema)?;

    let serde_rules = if required.contains(prop_name) {
        SerdeRules::None
    } else {
        // See if this type is an option or array; note that the type id lookup
        // will fail only for references (and only during initial reference
        // processing).
        let is_option_or_array = type_space.id_to_entry.get(&type_id).map_or_else(
            || false,
            |ty| matches!(&ty.details, TypeDetails::Option(_) | TypeDetails::Array(_)),
        );

        // We can use serde's `skip_serializing_of` construction for options
        // and arrays; otherwise we need to turn this into an option in order
        // to represent the field as non-required.
        if !is_option_or_array {
            type_id = type_space.id_for_option(&type_id);
        }
        SerdeRules::Optional
    };

    let (name, rename) = recase(prop_name.to_string(), Case::Snake);
    let serde_naming = match rename {
        Some(old_name) => SerdeNaming::Rename(old_name),
        None => SerdeNaming::None,
    };

    Ok(StructProperty {
        name,
        serde_naming,
        serde_rules,
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

    let prop_type = type_space.id_to_entry.get(&prop.type_id).unwrap();
    let type_name = prop_type.type_ident(type_space, false);
    let pub_token = if make_pub {
        quote! { pub }
    } else {
        quote! {}
    };
    let serde = generate_serde_attr(&prop.serde_naming, &prop.serde_rules, prop_type);
    quote! {
        #doc
        #serde
        #pub_token #name: #type_name,
    }
}

fn generate_serde_attr(
    serde_naming: &SerdeNaming,
    serde_rules: &SerdeRules,
    prop_type: &TypeEntry,
) -> TokenStream {
    let mut serde_options = Vec::new();
    match serde_naming {
        SerdeNaming::Rename(s) => serde_options.push(quote! { rename = #s }),
        SerdeNaming::Flatten => serde_options.push(quote! { flatten }),
        SerdeNaming::None => (),
    }

    match (serde_rules, &prop_type.details) {
        (SerdeRules::Optional, TypeDetails::Option(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Option::is_none" });
        }
        (SerdeRules::Optional, TypeDetails::Array(_)) => {
            serde_options.push(quote! { default });
            serde_options.push(quote! { skip_serializing_if = "Vec::is_empty" });
        }
        (SerdeRules::Optional, _) => unreachable!(),
        (SerdeRules::None, _) => (),
    }

    if serde_options.is_empty() {
        quote! {}
    } else {
        quote! {
            #[serde( #(#serde_options),*)]
        }
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
                serde_naming: SerdeNaming::Flatten,
                serde_rules: if optional {
                    SerdeRules::Optional
                } else {
                    SerdeRules::None
                },
                description: None,
                type_id,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Struct(properties));

    Ok((ty, metadata))
}

/// This handles the case where an anyOf is used to effect inheritance: the
/// subschemas consist of one or more "super classes" that have names with a
/// final, anonymous object.
///
/// ```text
/// "allOf": [
///     { "$ref": "#/definitions/SuperClass" },
///     { "type": "object", "properties": { "prop_a": .., "prop_b": .. }}
/// ]
/// ```
///
/// This turns into a struct of this form:
/// ```compile_fail
/// struct MyType {
///     #[serde(flatten)]
///     super_class: SuperClass,
///     prop_a: (),
///     prop_b: (),
/// }
/// ```
///
/// Note that the super class member names are derived from the type and are
/// flattened into the struct; the subclass properties are simply included
/// alongside.
pub(crate) fn maybe_all_of_subclass(
    type_name: Name,
    metadata: &Option<Box<Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Option<TypeEntry> {
    assert!(subschemas.len() > 1);

    // Split the subschemas into named (superclass) and unnamed (subclass)
    // schemas.
    let (named, unnamed): (Vec<_>, Vec<_>) = subschemas
        .iter()
        .map(|schema| (schema, schema_is_named(schema)))
        .partition(|(_, name)| name.is_some());

    // We required exactly one unnamed subschema for this special case. Note
    // that zero unnamed subschemas would be trivial to handle, but the generic
    // case already does so albeit slightly differently.
    if unnamed.len() != 1 {
        return None;
    }

    // Get the object validation (or fail to match this special case).
    let unnamed_schema = unnamed.first()?.0;
    let validation = match unnamed_schema {
        Schema::Object(SchemaObject {
            metadata: _,
            instance_type: Some(SingleOrVec::Single(single)),
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: Some(validation),
            reference: None,
            extensions: _,
        }) if single.as_ref() == &InstanceType::Object => Some(validation),
        _ => None,
    }?;
    let tmp_type_name = get_type_name(&type_name, metadata, Case::Pascal);
    let unnamed_properties = struct_members(tmp_type_name, validation, type_space).ok()?;

    let named_properties = named
        .iter()
        .map(|(schema, property_name)| {
            let (type_id, metadata) = type_space.id_for_schema(type_name.clone(), schema)?;
            Ok(StructProperty {
                name: property_name.as_ref().unwrap().to_case(Case::Snake),
                serde_naming: SerdeNaming::Flatten,
                serde_rules: SerdeRules::None,
                description: metadata_description(metadata),
                type_id,
            })
        })
        .collect::<Result<Vec<_>>>()
        .ok()?;

    let ty = TypeEntry::from_metadata(
        type_name,
        metadata,
        TypeDetails::Struct(
            named_properties
                .into_iter()
                .chain(unnamed_properties.into_iter())
                .collect(),
        ),
    );

    Some(ty)
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
