use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    util::{get_type_name, metadata_description, recase, schema_is_named},
    Name, Result, SerdeNaming, SerdeRules, StructProperty, TypeDetails, TypeEntry, TypeId,
    TypeSpace,
};

pub(crate) fn struct_members(
    type_name: Option<String>,
    validation: &ObjectValidation,
    type_space: &mut TypeSpace,
) -> Result<(Vec<StructProperty>, bool)> {
    // These are the fields we don't currently handle
    assert!(validation.max_properties.is_none());
    assert!(validation.min_properties.is_none());
    assert!(validation.pattern_properties.is_empty());
    assert!(validation.property_names.is_none());

    let mut properties = validation
        .properties
        .iter()
        .map(|(name, ty)| {
            let prop_name = name.to_case(Case::Snake);
            let sub_type_name = type_name
                .as_ref()
                .map(|base| format!("{}_{}", base, prop_name));
            struct_property(sub_type_name, &validation.required, name, ty, type_space)
        })
        .collect::<Result<Vec<_>>>()?;

    // Sort parameters by name to ensure a deterministic result.
    properties.sort_by(|a, b| a.name.cmp(&b.name));

    // If there are additional properties tack them on, flattened, at the end.
    // Note that a `None` value for additional_properties is equivalent to the
    // permissive schema (Schema::Bool(true)) for reasons best known to the
    // JSON Schema authors.
    let deny_unknown_fields = match &validation.additional_properties {
        // No additional properties allowed; we'll tag the struct with
        // #[serde(deny_unknown_fields)]
        Some(a) if a.as_ref() == &Schema::Bool(false) => true,

        // We have a permissive schema so all additional properties are
        // allowed (None is equivalent to the permissive schema).
        Some(a)
            if matches!(
                a.as_ref(),
                Schema::Bool(true)
                    | Schema::Object(SchemaObject {
                        metadata: _,
                        instance_type: None,
                        format: None,
                        enum_values: None,
                        const_value: None,
                        subschemas: None,
                        number: None,
                        string: None,
                        array: None,
                        object: None,
                        reference: None,
                        extensions: _,
                    })
            ) =>
        {
            false
        }
        None => false,

        // Only particular additional properties are allowed.
        additional_properties @ Some(_) => {
            let sub_type_name = type_name.as_ref().map(|base| format!("{}_extra", base));
            let (map_type, _) = make_map(sub_type_name, additional_properties, type_space)?;
            let map_type_id = type_space.assign_type(map_type);
            let extra_prop = StructProperty {
                name: "extra".to_string(),
                serde_naming: SerdeNaming::Flatten,
                serde_rules: SerdeRules::None,
                description: None,
                type_id: map_type_id,
            };

            properties.push(extra_prop);
            true
        }
    };

    Ok((properties, deny_unknown_fields))
}

pub(crate) fn struct_property(
    type_name: Option<String>,
    required: &schemars::Set<String>,
    prop_name: &str,
    schema: &schemars::schema::Schema,
    type_space: &mut TypeSpace,
) -> Result<StructProperty> {
    let sub_type_name = match type_name {
        Some(name) => Name::Suggested(name),
        None => Name::Unknown,
    };
    let (mut type_id, metadata) = type_space.id_for_schema(sub_type_name, schema)?;

    let serde_rules = if required.contains(prop_name) {
        SerdeRules::None
    } else {
        // We can use serde's `skip_serializing_of` construction for options
        // and arrays; otherwise we need to turn this into an option in order
        // to represent the field as non-required.
        if !is_skippable(type_space, &type_id) {
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

pub(crate) fn make_map<'a>(
    type_name: Option<String>,
    additional_properties: &Option<Box<Schema>>,
    type_space: &mut TypeSpace,
) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
    let (value_type_id, _) = match additional_properties {
        Some(schema) => {
            let sub_type_name = match type_name {
                Some(name) => Name::Suggested(format!("{}Extra", name)),
                None => Name::Unknown,
            };
            type_space.id_for_schema(sub_type_name, schema)?
        }

        None => type_space.id_for_schema(Name::Unknown, &Schema::Bool(true))?,
    };

    // TODO this is jank; we should be looking up the String type
    let string_type_id = type_space.assign_type(TypeEntry {
        name: Some("String".to_string()),
        rename: None,
        description: None,
        details: TypeDetails::BuiltIn,
    });

    Ok((
        TypeEntry::from_metadata(
            Name::Unknown,
            &None,
            TypeDetails::Map(string_type_id, value_type_id),
        ),
        &None,
    ))
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
        (SerdeRules::Optional, TypeDetails::Map(_, _)) => {
            serde_options.push(quote! { default });
            serde_options
                .push(quote! { skip_serializing_if = "std::collections::BTreeMap::is_empty" });
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

/// See if this type is a type that we can omit with a serde directive; note
/// that the type id lookup will fail only for references (and only during
/// initial reference processing).
fn is_skippable(type_space: &TypeSpace, type_id: &TypeId) -> bool {
    type_space.id_to_entry.get(type_id).map_or_else(
        || false,
        |ty| {
            matches!(
                &ty.details,
                TypeDetails::Option(_) | TypeDetails::Array(_) | TypeDetails::Map(_, _)
            )
        },
    )
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

    let ty = TypeEntry::from_metadata(
        type_name,
        metadata,
        TypeDetails::Struct {
            properties,
            deny_unknown_fields: false,
        },
    );

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
    let (unnamed_properties, deny) = struct_members(tmp_type_name, validation, type_space).ok()?;

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
        TypeDetails::Struct {
            properties: named_properties
                .into_iter()
                .chain(unnamed_properties.into_iter())
                .collect(),
            deny_unknown_fields: deny,
        },
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
    #[serde(deny_unknown_fields)]
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
    struct SomeMaps {
        strings: std::collections::BTreeMap<String, String>,
        things: std::collections::BTreeMap<String, serde_json::Value>,
    }

    #[test]
    fn test_some_maps() {
        validate_output::<SomeMaps>();
    }
}
