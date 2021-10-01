use std::collections::{BTreeMap, BTreeSet};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    structs::{output_struct_property, struct_members, struct_property},
    util::{constant_string_value, get_type_name, metadata_description, recase, schema_is_named},
    EnumTagType, Name, Result, TypeDetails, TypeEntry, TypeSpace, Variant, VariantDetails,
};

pub(crate) fn maybe_option_as_enum(
    type_name: Name,
    metadata: &Option<Box<schemars::schema::Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Option<TypeEntry> {
    if subschemas.len() == 1 {
        return None;
    }
    // Let's be as general as possible and consider the possibility that more
    // than one subschema is the simple null.
    let non_nulls = subschemas
        .iter()
        .filter(|schema| {
            !matches!(schema, Schema::Object(SchemaObject {
                instance_type: Some(SingleOrVec::Single(single)),
                ..
            }) if single.as_ref() == &InstanceType::Null)
        })
        .collect::<Vec<_>>();

    if non_nulls.len() != 1 {
        return None;
    }

    let non_null = non_nulls.into_iter().next()?;

    let (type_entry, _) = type_space
        .convert_option(type_name, metadata, non_null)
        .ok()?;

    Some(type_entry)
}

// TODO these maybe_* functions need to not create new types until we're past
// that point at which they might return None.
pub(crate) fn maybe_externally_tagged_enum(
    type_name: Name,
    metadata: &Option<Box<schemars::schema::Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Option<TypeEntry> {
    let variants = subschemas
        .iter()
        .map(|schema| -> Option<Vec<Variant>> {
            match schema {
                // It shouldn't be possible to encounter the "match anything"
                // schema here.
                Schema::Bool(true) => unreachable!(),
                // TODO It would be odd to see the "match nothing" schema here.
                // Let's abort for now, but we could implement this as a variant
                // that we'd never use... I guess.
                Schema::Bool(false) => unreachable!(),

                // Strings must be simple enumerations.
                Schema::Object(SchemaObject {
                    metadata,
                    instance_type: Some(SingleOrVec::Single(single)),
                    format: None,
                    enum_values: Some(values),
                    const_value: None,
                    subschemas: None,
                    number: None,
                    string: None,
                    array: None,
                    object: None,
                    reference: None,
                    extensions: _,
                }) if single.as_ref() == &InstanceType::String => {
                    // Confirm that all values are, in fact, simple strings.
                    // Simple strings become simple variants. If any is not
                    // a string, we'll end up returning None
                    values
                        .iter()
                        .map(|value| {
                            value.as_str().map(|variant_name| {
                                let name = variant_name.to_case(Case::Pascal);
                                let rename = if variant_name == name {
                                    None
                                } else {
                                    Some(variant_name.to_string())
                                };
                                Variant {
                                    name,
                                    rename,
                                    description: metadata_description(metadata),
                                    details: VariantDetails::Simple,
                                }
                            })
                        })
                        .collect()
                }

                // Objects must have a single required member. The type of that
                // lone member determines the type associated with the variant.
                Schema::Object(SchemaObject {
                    metadata,
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
                }) if single.as_ref() == &InstanceType::Object => {
                    let validation = validation.as_ref();
                    if let ObjectValidation {
                        max_properties: None,
                        min_properties: None,
                        required,
                        properties,
                        pattern_properties,
                        additional_properties: Some(additional_properties),
                        property_names: None,
                    } = validation
                    {
                        if required.len() == 1
                            && properties.len() == 1
                            && pattern_properties.is_empty()
                            && additional_properties.as_ref() == &Schema::Bool(false)
                        {
                            let (prop_name, prop_type) = properties.iter().next().unwrap();

                            let name = prop_name.to_case(Case::Pascal);
                            let rename = if *prop_name == name {
                                None
                            } else {
                                Some(prop_name.clone())
                            };

                            // If required and properties both have length 1
                            // then this must be true for a well-constructed
                            // schema.
                            assert!(required.contains(prop_name));

                            // TODO should I be doing something different with the error below?
                            Some(vec![Variant {
                                name,
                                rename,
                                description: metadata_description(metadata),
                                // TODO type name
                                details: external_variant(Name::Unknown, prop_type, type_space)
                                    .ok()?,
                            }])
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .flat_map(|x| match x {
            Some(v) => v.into_iter().map(Some).collect::<Vec<_>>(),
            None => vec![None],
        })
        .collect::<Option<Vec<_>>>();

    variants.map(|variants| {
        TypeEntry::from_metadata(
            type_name,
            metadata,
            TypeDetails::Enum {
                variants,
                tag_type: EnumTagType::External,
            },
        )
    })
}

fn external_variant(
    type_name: Name,
    prop_type: &Schema,
    type_space: &mut TypeSpace,
) -> Result<VariantDetails> {
    // Arrays (tuples) must have a fixed size (max_items == min_items).
    // Per the JSON Schema specification, if the array.items is an array
    // (rather than a single element), then:
    //   "validation succeeds if each element of the instance validates against
    //   the schema at the same position, if any.""
    // Accordingly we require the length if the items array to match the fixed
    // size (max_items). Note that array.additionalItems is irrelevant due to
    // this portion of the spec:
    //   "If "items" is present, and its annotation result is a number,
    //   validation succeeds if every instance element at an index greater than
    //   that number validates against "additionalItems"."
    if let Schema::Object(SchemaObject {
        metadata: _,
        instance_type: Some(SingleOrVec::Single(single)),
        format: None,
        enum_values: None,
        const_value: None,
        subschemas: None,
        number: None,
        string: None,
        array: Some(validation),
        object: None,
        reference: None,
        extensions: _,
    }) = prop_type
    {
        if single.as_ref() == &InstanceType::Array {
            if let ArrayValidation {
                items: Some(SingleOrVec::Vec(items)),
                additional_items: _, // irrelevant; see above
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
                contains: None,
            } = validation.as_ref()
            {
                if *max_items >= 2 && max_items == min_items && *max_items == items.len() as u32 {
                    return Ok(VariantDetails::Tuple(
                        items
                            .iter()
                            .map(|item_type| {
                                Ok(type_space.id_for_schema(type_name.clone(), item_type)?.0)
                            })
                            .collect::<Result<Vec<_>>>()?,
                    ));
                }
            }
        }
    }

    match prop_type {
        // Null instance type equates to a simple variant
        Schema::Object(SchemaObject {
            metadata: None,
            instance_type: Some(SingleOrVec::Single(single)),
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
        }) if single.as_ref() == &InstanceType::Null => Ok(VariantDetails::Simple),

        // Anonymous (i.e. those where metadata.title is None) structs are
        // embedded within the variant as the struct type.
        Schema::Object(SchemaObject {
            metadata,
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
        }) if single.as_ref() == &InstanceType::Object
            && metadata
                .as_ref()
                .map(|m| m.as_ref().title.as_ref())
                .is_none()
            && schema_none_or_false(&validation.additional_properties) =>
        {
            let sub_type_name = match type_name {
                Name::Required(name) | Name::Suggested(name) => Some(name),
                Name::Unknown => None,
            };
            Ok(VariantDetails::Struct(struct_members(
                sub_type_name,
                validation,
                type_space,
            )?))
        }

        // Otherwise we create a single-element tuple variant with the given type.
        prop_type => Ok(VariantDetails::Tuple(vec![
            type_space.id_for_schema(type_name, prop_type)?.0,
        ])),
    }
}

pub(crate) fn maybe_internally_tagged_enum(
    type_name: Name,
    metadata: &Option<Box<Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Option<TypeEntry> {
    // All subschemas must be objects and all objects must have a *fixed-value*
    // required property in common. To detect this, we look at all such
    // properties along with the specific values.
    let constant_value_properties_sets = subschemas
        .iter()
        .map(|schema| match get_object(schema) {
            None => BTreeMap::<String, BTreeSet<String>>::new(),
            Some((_, validation)) => {
                validation
                    .properties
                    .iter()
                    .filter_map(|(prop_name, prop_type)| {
                        constant_string_value(prop_type).map(|value| {
                            // Tuple with the name and a set with a single value
                            (prop_name.clone(), [value].iter().cloned().collect())
                        })
                    })
                    .collect()
            }
        })
        // Reduce these sets down to those A. that are common among all
        // subschemas and B. for which the values for each is unique.
        .reduce(|a, b| {
            a.into_iter()
                .filter_map(|(prop, mut a_values)| match b.get(&prop) {
                    // If the values are non-disjoint it means that there are
                    // two subschemas that have constant values for a given
                    // property but that those values are identical.
                    Some(b_values) if a_values.is_disjoint(b_values) => {
                        a_values.extend(b_values.iter().cloned());
                        Some((prop, a_values))
                    }
                    _ => None,
                })
                .collect()
        })?;

    // It would be odd to have more than a single common, constant value,
    // but it would be fine. We sort the properties to choose one
    // deterministically.
    let mut constant_value_properties = constant_value_properties_sets
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    constant_value_properties.sort();
    let tag = constant_value_properties.first()?;

    let variants = subschemas
        .iter()
        .map(|schema| {
            // We've already validated this; we just need to pluck out the
            // pieces we need to construct the variant.
            if let Schema::Object(SchemaObject {
                object: Some(validation),
                ..
            }) = schema
            {
                internal_variant(validation, tag, type_space)
            } else {
                unreachable!();
            }
        })
        .collect::<Result<Vec<_>>>()
        .ok()?;

    Some(TypeEntry::from_metadata(
        type_name,
        metadata,
        TypeDetails::Enum {
            variants,
            tag_type: EnumTagType::Internal { tag: tag.clone() },
        },
    ))
}

fn internal_variant(
    validation: &ObjectValidation,
    tag: &str,
    type_space: &mut TypeSpace,
) -> Result<Variant> {
    if validation.properties.len() == 1 {
        let (tag_name, schema) = validation.properties.iter().next().unwrap();
        let variant_name = constant_string_value(schema).unwrap();
        let (name, rename) = recase(variant_name, Case::Pascal);

        // The lone property must be our tag.
        assert_eq!(tag_name, tag);
        assert_eq!(validation.required.len(), 1);

        Ok(Variant {
            name,
            rename,
            description: None,
            details: VariantDetails::Simple,
        })
    } else {
        let tag_schema = validation.properties.get(tag).unwrap();
        let variant_name = constant_string_value(tag_schema).unwrap();
        let (name, rename) = recase(variant_name, Case::Pascal);

        let properties = validation
            .properties
            .iter()
            .filter_map(|(prop_name, prop_type)| {
                // Include all properties except the tag.
                if prop_name != tag {
                    Some(struct_property(
                        None,
                        &validation.required,
                        prop_name,
                        prop_type,
                        type_space,
                    ))
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Variant {
            name,
            rename,
            description: None,
            details: VariantDetails::Struct(properties),
        })
    }
}

pub(crate) fn maybe_adjacently_tagged_enum(
    type_name: Name,
    metadata: &Option<Box<schemars::schema::Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Option<TypeEntry> {
    // All subschemas need to be objects with at most two properties: a
    // constant, required property (the tag) and an optional property for
    // content. Naturally, those two properties need to be the same for each
    // subschema.
    let prop_sets = subschemas
        .iter()
        .map(|schema| match get_object(schema) {
            Some((_, validation)) if validation.properties.len() == validation.required.len() => {
                let constants = validation
                    .properties
                    .iter()
                    .filter_map(|(prop_name, prop_type)| {
                        constant_string_value(prop_type).map(|_| prop_name.clone())
                    })
                    .collect::<BTreeSet<_>>();
                let properties = validation
                    .properties
                    .iter()
                    .map(|(prop_name, _)| prop_name.clone())
                    .collect::<BTreeSet<_>>();

                Some((constants, properties))
            }

            _ => None,
        })
        .collect::<Option<Vec<_>>>()?;

    // We take the intersection of all tag properties and the union of all
    // properties.
    let (tag_props, content_props) =
        prop_sets
            .into_iter()
            .reduce(|(a_const, a_props), (b_const, b_props)| {
                (
                    a_const
                        .intersection(&b_const)
                        .cloned()
                        .collect::<BTreeSet<_>>(),
                    a_props.union(&b_props).cloned().collect::<BTreeSet<_>>(),
                )
            })?;

    if tag_props.len() != 1 || content_props.len() != 2 {
        return None;
    }

    let content = content_props.difference(&tag_props).cloned().next()?;
    let tag = tag_props.into_iter().next()?;

    let variants = subschemas
        .iter()
        .map(|schema| {
            // We've already validated this; we just need to pluck out the
            // pieces we need to construct the variant.
            if let Schema::Object(SchemaObject {
                object: Some(validation),
                ..
            }) = schema
            {
                adjacent_variant(validation, &tag, &content, type_space)
            } else {
                unreachable!();
            }
        })
        .collect::<Result<Vec<_>>>()
        .ok()?;

    Some(TypeEntry::from_metadata(
        type_name,
        metadata,
        TypeDetails::Enum {
            variants,
            tag_type: EnumTagType::Adjacent { tag, content },
        },
    ))
}

fn adjacent_variant(
    validation: &ObjectValidation,
    tag: &str,
    content: &str,
    type_space: &mut TypeSpace,
) -> Result<Variant> {
    if validation.properties.len() == 1 {
        let (tag_name, schema) = validation.properties.iter().next().unwrap();
        let variant_name = constant_string_value(schema).unwrap();
        let (name, rename) = recase(variant_name, Case::Pascal);

        // The lone property must be our tag.
        assert_eq!(tag_name, tag);
        assert_eq!(validation.required.len(), 1);

        Ok(Variant {
            name,
            rename,
            description: None,
            details: VariantDetails::Simple,
        })
    } else {
        let tag_schema = validation.properties.get(tag).unwrap();
        let variant_name = constant_string_value(tag_schema).unwrap();
        let (name, rename) = recase(variant_name, Case::Pascal);

        let content_schema = validation.properties.get(content).unwrap();

        Ok(Variant {
            name,
            rename,
            description: None,
            // TODO type name
            details: external_variant(Name::Unknown, content_schema, type_space)?,
        })
    }
}

/// Internally and adjacently tagged enums expect their subschemas to be
/// objects. Return the object data or None if it's not an object or doesn't
/// conform to the objects we know how to handle.
fn get_object(schema: &Schema) -> Option<(Option<&Metadata>, &ObjectValidation)> {
    match schema {
        Schema::Object(SchemaObject {
            metadata,
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
        }) if single.as_ref() == &InstanceType::Object
            && schema_none_or_false(&validation.additional_properties) =>
        {
            // These are the fields we don't currently handle
            assert!(validation.max_properties.is_none());
            assert!(validation.min_properties.is_none());
            assert!(validation.pattern_properties.is_empty());
            assert!(validation.property_names.is_none());

            Some((metadata.as_ref().map(|m| m.as_ref()), validation.as_ref()))
        }

        // None if the schema doesn't match the shape we expect.
        _ => None,
    }
}

// TODO: https://github.com/GREsau/schemars/pull/99
// Really this should be Schema::Bool(false), but additional_properties is used
// inconsistently... in schemars, but also generally.
fn schema_none_or_false(additional_properties: &Option<Box<Schema>>) -> bool {
    matches!(
        additional_properties.as_ref().map(Box::as_ref),
        None | Some(Schema::Bool(false))
    )
}

/// Produce an enum with each subschema as a variant. There isn't an explicit
/// name for each variant so we default to `VariantNN`.
///
/// ```compile_fail
/// enum MyEnum {
///     Variant1(ThingsOfYours),
///     Variant2(ThingsOfMine),
/// }
/// ```
///
/// We can, however, in some cases infer better names: if each variant is a
/// tuple of cardinality 1 with a named type, we case use those names instead
/// for the variants. For example:
///
/// ```compile_fail
/// enum MyEnum {
///     ThingsOfYours(ThingsOfYours),
///     ThingsOfMine(ThingsOfMine),
/// }
/// ```
///
/// We even do a step better by eliminating common prefixes:
///
/// ```compile_fail
/// enum MyEnum {
///     Yours(ThingsOfYours),
///     Mine(ThingsOfMine),
/// }
/// ```
pub(crate) fn untagged_enum(
    type_name: Name,
    metadata: &Option<Box<schemars::schema::Metadata>>,
    subschemas: &[Schema],
    type_space: &mut TypeSpace,
) -> Result<TypeEntry> {
    let tmp_type_name = get_type_name(&type_name, metadata, Case::Pascal);

    let mut names_from_variants = true;
    let mut common_prefix = None;

    // Gather the variant details along with an Option of its "good" name.
    let variant_details = subschemas
        .iter()
        .enumerate()
        .map(|(idx, schema)| {
            let sub_type_name = match &tmp_type_name {
                Some(name) => Name::Suggested(format!("{}Variant{}", name, idx)),
                None => Name::Unknown,
            };
            let details = external_variant(sub_type_name, schema, type_space)?;
            let good_name = schema_is_named(schema);
            match (&good_name, common_prefix.as_ref()) {
                (None, _) => {
                    names_from_variants = false;
                }
                (Some(name), None) => {
                    common_prefix = Some(name.clone());
                }
                (Some(name), Some(prefix)) => {
                    common_prefix = Some(get_common_prefix(name, prefix));
                }
            }

            Ok((details, good_name))
        })
        .collect::<Result<Vec<_>>>()?;

    let common_prefix_index = match &common_prefix {
        Some(prefix) => prefix.len(),
        None => 0,
    };

    let variants = variant_details
        .into_iter()
        .enumerate()
        .map(|(idx, (details, good_name))| {
            let name = if names_from_variants {
                (&good_name.unwrap()[common_prefix_index..]).to_string()
            } else {
                format!("Variant{}", idx)
            };
            Variant {
                name,
                rename: None,
                description: None,
                details,
            }
        })
        .collect();

    Ok(TypeEntry::from_metadata(
        type_name,
        metadata,
        TypeDetails::Enum {
            tag_type: EnumTagType::Untagged,
            variants,
        },
    ))
}

/// Get the string that represents the common prefix, considering only
/// case-relevant boundaries.
fn get_common_prefix(name: &str, prefix: &str) -> String {
    name.to_case(Case::Kebab)
        .split('-')
        .zip(prefix.to_case(Case::Kebab).split('-'))
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect::<Vec<&str>>()
        .join("-")
        .to_case(Case::Pascal)
}

pub(crate) fn output_variant(variant: &Variant, type_space: &TypeSpace) -> TokenStream {
    let name = format_ident!("{}", variant.name);
    let doc = match &variant.description {
        Some(s) => quote! {#[doc = #s]},
        None => quote! {},
    };
    let rename = match &variant.rename {
        Some(s) => quote! { #[serde(rename = #s)]},
        None => quote! {},
    };
    match &variant.details {
        VariantDetails::Simple => quote! {
            #doc
            #rename
            #name,
        },

        VariantDetails::Tuple(tuple) => {
            let types = tuple
                .iter()
                .map(|type_id| {
                    let item_type = type_space.id_to_entry.get(type_id).unwrap();
                    let type_name = item_type.type_ident(type_space, false);
                    quote! { #type_name }
                })
                .collect::<Vec<_>>();
            quote! {
                #doc
                #rename
                #name(#(#types),*),
            }
        }

        VariantDetails::Struct(props) => {
            let properties = props
                .iter()
                .map(|prop| output_struct_property(prop, type_space, false))
                .collect::<Vec<_>>();
            quote! {
                #doc
                #rename
                #name {
                    #(#properties)*
                },
            }
        }
    }
}

pub(crate) fn enum_impl(type_name: &Ident, variants: &[Variant]) -> TokenStream {
    let maybe_simple_variants = variants
        .iter()
        .map(|variant| {
            if let VariantDetails::Simple = variant.details {
                Some(variant)
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>();

    match maybe_simple_variants {
        Some(simple_variants) => {
            let match_variants = simple_variants.iter().map(|variant| {
                let variant_name = format_ident!("{}", variant.name);
                let variant_str = match &variant.rename {
                    Some(s) => s,
                    None => &variant.name,
                };
                quote! {
                    #type_name::#variant_name => #variant_str.to_string()
                }
            });

            quote! {
                impl ToString for #type_name {
                    fn to_string(&self) -> String {
                        match self {
                            #(#match_variants),*
                        }
                    }
                }
            }
        }

        // Not all of the variants were simple
        None => quote! {},
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use schema::Schema;
    use schemars::{
        schema::{InstanceType, RootSchema, SchemaObject, SingleOrVec},
        schema_for, JsonSchema,
    };
    use serde::Serialize;

    use crate::{
        enums::{
            maybe_adjacently_tagged_enum, maybe_externally_tagged_enum,
            maybe_internally_tagged_enum, maybe_option_as_enum, untagged_enum,
        },
        test_util::{validate_output, validate_output_for_untagged_enm},
        EnumTagType, Name, TypeDetails, TypeEntry, TypeId, TypeSpace, Variant, VariantDetails,
    };

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    enum ExternallyTaggedEnum {
        Alpha,
        #[serde(rename_all = "camelCase")]
        Bravo {
            bee_bee: String,
            foo_foo: String,
        },
        Charlie(Vec<u32>),
        Delta(u32, String),
        Echo(Option<String>, String, String),
    }

    #[test]
    fn test_externally_tagged_enum() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(ExternallyTaggedEnum);
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(maybe_externally_tagged_enum(
            Name::Required("ExternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_some());
        assert!(maybe_adjacently_tagged_enum(
            Name::Required("ExternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_none());
        assert!(maybe_internally_tagged_enum(
            Name::Required("ExternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_none());
    }

    #[test]
    fn test_externally_tagged_enum_output() {
        validate_output::<ExternallyTaggedEnum>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(tag = "tag", content = "content")]
    enum AdjacentlyTaggedEnum {
        Alpha,
        #[serde(rename_all = "camelCase")]
        Bravo {
            bee_bee: String,
            foo_foo: String,
        },
        Charlie(Vec<u32>),
        Delta(u32, String),
        Echo(Option<String>, String, String),
    }

    #[test]
    fn test_adjacently_tagged_enum() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(AdjacentlyTaggedEnum);
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(maybe_adjacently_tagged_enum(
            Name::Required("AdjacentlyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_some());
        assert!(maybe_externally_tagged_enum(
            Name::Required("AdjacentlyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_none());
    }
    #[test]
    fn test_adjacently_tagged_enum_output() {
        validate_output::<AdjacentlyTaggedEnum>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(tag = "tag")]
    enum InternallyTaggedEnum {
        Alpha,
        #[serde(rename_all = "camelCase")]
        Bravo {
            bee_bee: String,
            foo_foo: String,
        },
        // All variants must be simple or structs with internally tagged enums
        // Charlie(Vec<u32>),
        // Delta(u32, String),
        // Echo(Option<String>, String, String),
    }

    #[test]
    fn test_internally_tagged_enum() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(InternallyTaggedEnum);
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(maybe_internally_tagged_enum(
            Name::Required("InternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_some());
        assert!(maybe_adjacently_tagged_enum(
            Name::Required("InternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_none());
        assert!(maybe_externally_tagged_enum(
            Name::Required("InternallyTaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space
        )
        .is_none());
    }

    #[test]
    fn test_internally_tagged_enum_output() {
        validate_output::<InternallyTaggedEnum>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(untagged)]
    enum UntaggedEnum {
        Alpha,
        #[serde(rename_all = "camelCase")]
        Bravo {
            bee_bee: String,
            foo_foo: String,
        },
        Charlie(Vec<u32>),
        Delta(u32, String),
        Echo(Option<String>, String, String),
    }

    #[test]
    fn test_untagged_enum() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(UntaggedEnum);
        let subschemas = schema.schema.subschemas.unwrap().any_of.unwrap();
        let ty = untagged_enum(
            Name::Required("UntaggedEnum".to_string()),
            &None,
            &subschemas,
            &mut type_space,
        )
        .unwrap();

        match ty {
            TypeEntry {
                name,
                rename: None,
                description: None,
                details:
                    TypeDetails::Enum {
                        tag_type: EnumTagType::Untagged,
                        variants,
                    },
            } => {
                assert_eq!(name, Some("UntaggedEnum".to_string()));
                assert_eq!(variants.len(), 5);

                assert!(matches!(
                    variants.get(0).unwrap(),
                    Variant {
                        details: VariantDetails::Simple,
                        ..
                    }
                ));
                assert!(matches!(
                    variants.get(1).unwrap(),
                    Variant {
                        details: VariantDetails::Struct(_),
                        ..
                    }
                ));
                assert!(matches!(
                    variants.get(2).unwrap(),
                    Variant {
                        details: VariantDetails::Tuple(tup),
                        ..
                    } if tup.len() == 1
                ));
                assert!(matches!(
                    variants.get(3).unwrap(),
                    Variant {
                        details: VariantDetails::Tuple(tup),
                        ..
                    } if tup.len() == 2
                ));
                assert!(matches!(
                    variants.get(4).unwrap(),
                    Variant {
                        details: VariantDetails::Tuple(tup),
                        ..
                    } if tup.len() == 3
                ));
            }

            _ => panic!(),
        }
    }
    #[test]
    fn test_untagged_enum_output() {
        validate_output_for_untagged_enm::<UntaggedEnum>();
    }

    #[test]
    fn test_enum_detection_untagged() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(UntaggedEnum);
        let subschemas = schema.schema.subschemas.unwrap().any_of.unwrap();

        let (ty, _) = type_space
            .convert_one_of(Name::Unknown, &None, &subschemas)
            .unwrap();

        // This confirms in particular that the tag type is untagged and
        // therefore that the other enum tagging regimes did not match.
        assert!(matches!(
            ty,
            TypeEntry {
                name: None,
                rename: None,
                description: None,
                details: TypeDetails::Enum {
                    tag_type: EnumTagType::Untagged,
                    variants: _,
                },
            }
        ));
    }

    #[test]
    fn test_head_fake_tagged_enum() {
        let schema_json = r##"
        {
            "$schema": "http://json-schema.org/draft-07/schema",
            "$id": "pull_request$review_request_removed",
            "oneOf": [
              {
                "type": "object",
                "required": [
                  "action",
                  "number",
                  "pull_request",
                  "requested_reviewer",
                  "repository",
                  "sender"
                ],
                "properties": {
                  "action": { "type": "string", "enum": ["review_request_removed"] },
                  "number": {
                    "type": "integer",
                    "description": "The pull request number."
                  },
                  "pull_request": { "$ref": "#/definitions/pull-request" },
                  "requested_reviewer": { "$ref": "#/definitions/user" },
                  "repository": { "$ref": "#/definitions/repository" },
                  "installation": { "$ref": "#/definitions/installation-lite" },
                  "organization": { "$ref": "#/definitions/organization" },
                  "sender": { "$ref": "#/definitions/user" }
                },
                "additionalProperties": false
              },
              {
                "type": "object",
                "required": [
                  "action",
                  "number",
                  "pull_request",
                  "requested_team",
                  "repository",
                  "sender"
                ],
                "properties": {
                  "action": { "type": "string", "enum": ["review_request_removed"] },
                  "number": {
                    "type": "integer",
                    "description": "The pull request number."
                  },
                  "pull_request": { "$ref": "#/definitions/pull-request" },
                  "requested_team": { "$ref": "#/definitions/team" },
                  "repository": { "$ref": "#/definitions/repository" },
                  "installation": { "$ref": "#/definitions/installation-lite" },
                  "organization": { "$ref": "#/definitions/organization" },
                  "sender": { "$ref": "#/definitions/user" }
                },
                "additionalProperties": false
              }
            ],
            "title": "pull_request review_request_removed event",

            "definitions": {
                "pull-request": { "type": "string" },
                "user": { "type": "string" },
                "team": { "type": "string" },
                "repository": { "type": "string" },
                "installation-lite": { "type": "string" },
                "organization": { "type": "string" }
            }
          }
        "##;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();

        let (type_entry, _) = type_space
            .convert_schema_object(Name::Unknown, &schema.schema)
            .unwrap();

        if let TypeDetails::Enum { variants, tag_type } = &type_entry.details {
            let variant_names = variants
                .iter()
                .map(|variant| variant.name.clone())
                .collect::<HashSet<_>>();
            assert_eq!(variant_names.len(), variants.len());
            assert_eq!(tag_type, &EnumTagType::Untagged);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_maybe_option_as_enum() {
        let subschemas = vec![
            SchemaObject {
                instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                ..Default::default()
            }
            .into(),
            SchemaObject {
                instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Null))),
                ..Default::default()
            }
            .into(),
        ];

        let mut type_space = TypeSpace::default();
        let type_entry =
            maybe_option_as_enum(Name::Unknown, &None, &subschemas, &mut type_space).unwrap();

        assert_eq!(
            type_entry,
            TypeEntry {
                name: None,
                rename: None,
                description: None,
                details: TypeDetails::Option(TypeId(1))
            }
        )
    }

    #[test]
    fn test_simple_untagged_enum() {
        let schema_json = r##"
        {
            "definitions": {
                "workflow-step-completed": {
                "$schema": "http://json-schema.org/draft-07/schema",
                "required": [
                    "name",
                    "status",
                    "conclusion",
                    "number",
                    "started_at",
                    "completed_at"
                ],
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "status": { "type": "string", "enum": ["completed"] },
                    "conclusion": {
                    "type": "string",
                    "enum": ["failure", "skipped", "success"]
                    },
                    "number": { "type": "integer" },
                    "started_at": { "type": "string" },
                    "completed_at": { "type": "string" }
                },
                "additionalProperties": false,
                "title": "Workflow Step (Completed)"
                },
                "workflow-step-in_progress": {
                "$schema": "http://json-schema.org/draft-07/schema",
                "required": [
                    "name",
                    "status",
                    "conclusion",
                    "number",
                    "started_at",
                    "completed_at"
                ],
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "status": { "type": "string", "enum": ["in_progress"] },
                    "conclusion": { "type": "null" },
                    "number": { "type": "integer" },
                    "started_at": { "type": "string" },
                    "completed_at": { "type": "null" }
                },
                "additionalProperties": false,
                "title": "Workflow Step (In Progress)"
                },
                "workflow-step": {
                "$schema": "http://json-schema.org/draft-07/schema",
                "type": "object",
                "oneOf": [
                    { "$ref": "#/definitions/workflow-step-in_progress" },
                    { "$ref": "#/definitions/workflow-step-completed" }
                ],
                "title": "Workflow Step"
                }
            }
        }
        "##;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();

        let type_id = type_space.ref_to_id.get("workflow-step").unwrap();
        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

        println!("{:#?}", type_entry);

        match &type_entry.details {
            TypeDetails::Enum { tag_type, variants } => {
                assert_eq!(tag_type, &EnumTagType::Untagged);
                for variant in variants {
                    match &variant.details {
                        VariantDetails::Tuple(items) if items.len() == 1 => {
                            let variant_type =
                                type_space.id_to_entry.get(items.first().unwrap()).unwrap();
                            assert!(variant_type.name.as_ref().unwrap().ends_with(&variant.name));
                        }
                        _ => panic!("{:#?}", type_entry),
                    }
                }
            }
            _ => panic!("{:#?}", type_entry),
        }
    }
}
