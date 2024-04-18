// Copyright 2024 Oxide Computer Company

use std::collections::{BTreeMap, BTreeSet, HashSet};

use heck::{ToKebabCase, ToPascalCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{
    InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
};

use crate::{
    output::OutputSpace,
    structs::generate_serde_attr,
    type_entry::{
        EnumTagType, TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryStruct, Variant,
        VariantDetails,
    },
    util::{
        constant_string_value, get_object, get_type_name, metadata_description,
        metadata_title_and_description, recase, schema_is_named, Case,
    },
    Name, Result, TypeSpace,
};

impl TypeSpace {
    pub(crate) fn maybe_option(
        &mut self,
        type_name: Name,
        metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        if subschemas.len() == 1 {
            return None;
        }
        // Let's be as general as possible and consider the possibility that
        // more than one subschema is the simple null.
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

        let (type_entry, _) = self.convert_option(type_name, metadata, non_null).ok()?;

        Some(type_entry)
    }

    pub(crate) fn maybe_externally_tagged_enum(
        &mut self,
        type_name: Name,
        original_schema: &Schema,
        enum_metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        enum ProtoVariant<'a> {
            Simple {
                name: &'a str,
                description: Option<String>,
            },
            Typed {
                name: &'a str,
                schema: &'a Schema,
                description: Option<String>,
            },
        }

        // Verify that this matches the shape of an externally tagged enum
        // before we do any type conversion.
        let proto_variants = subschemas
            .iter()
            .map(|schema| -> Option<Vec<ProtoVariant<'_>>> {
                match schema {
                    // It shouldn't be possible to encounter the "match
                    // anything" schema here.
                    Schema::Bool(true) => unreachable!(),
                    // It would be odd to see the "match nothing" schema here.
                    // Let's abort for now, but we could implement this as a
                    // variant that we'd never use... I guess.
                    Schema::Bool(false) => todo!(),

                    // Strings must be simple enumerations or constants.
                    Schema::Object(SchemaObject {
                        metadata,
                        instance_type,
                        format: None,
                        enum_values: Some(values),
                        const_value: None,
                        subschemas: None,
                        number: _,
                        string: _,
                        array: _,
                        object: _,
                        reference: None,
                        extensions: _,
                    }) => {
                        match instance_type {
                            Some(SingleOrVec::Single(single))
                                if single.as_ref() == &InstanceType::String => {}
                            None => {}
                            _ => return None,
                        };
                        // Confirm that all values are, in fact, simple strings.
                        // Simple strings become simple variants. If any is not
                        // a string, we'll end up returning None
                        values
                            .iter()
                            .map(|value| {
                                value.as_str().map(|variant_name| ProtoVariant::Simple {
                                    name: variant_name,
                                    description: metadata_description(metadata),
                                })
                            })
                            .collect()
                    }
                    Schema::Object(SchemaObject {
                        metadata,
                        instance_type,
                        format: None,
                        enum_values: None,
                        const_value: Some(value),
                        subschemas: None,
                        number: _,
                        string: _,
                        array: _,
                        object: _,
                        reference: None,
                        extensions: _,
                    }) => {
                        match instance_type {
                            Some(SingleOrVec::Single(single))
                                if single.as_ref() == &InstanceType::String => {}
                            None => {}
                            _ => return None,
                        };
                        std::iter::once(value.as_str().map(|variant_name| ProtoVariant::Simple {
                            name: variant_name,
                            description: metadata_description(metadata),
                        }))
                        .collect()
                    }

                    other => match get_object(other) {
                        // Objects must have a single property, and that
                        // property must be required. The type of that lone
                        // property determines the type associated with the
                        // variant.
                        Some((
                            metadata,
                            ObjectValidation {
                                max_properties: None,
                                min_properties: None,
                                required,
                                properties,
                                pattern_properties,
                                additional_properties: _,
                                property_names: None,
                            },
                        )) if required.len() == 1
                            && properties.len() == 1
                            && pattern_properties.is_empty() =>
                        {
                            let (prop_name, prop_type) = properties.iter().next().unwrap();
                            // If required and properties both have length 1
                            // then the following must be true for a
                            // well-constructed schema.
                            assert!(required.contains(prop_name));

                            Some(vec![ProtoVariant::Typed {
                                name: prop_name,
                                schema: prop_type,
                                description: metadata_description(metadata),
                            }])
                        }
                        _ => None,
                    },
                }
            })
            .flat_map(|x| match x {
                Some(v) => v.into_iter().map(Some).collect::<Vec<_>>(),
                None => vec![None],
            })
            .collect::<Option<Vec<_>>>()?;

        let variant_names = proto_variants
            .iter()
            .map(|proto| match proto {
                ProtoVariant::Simple { name, .. } | ProtoVariant::Typed { name, .. } => name,
            })
            .collect::<HashSet<_>>();

        // We can't have duplicate variant names in an enum.
        if variant_names.len() != proto_variants.len() {
            return None;
        }

        let mut deny_unknown_fields = false;
        let variants = proto_variants
            .into_iter()
            .map(|proto| match proto {
                ProtoVariant::Simple {
                    name: variant_name,
                    description,
                } => {
                    let (name, rename) = recase(variant_name, Case::Pascal);
                    Some(Variant {
                        name,
                        rename,
                        description,
                        details: VariantDetails::Simple,
                    })
                }

                ProtoVariant::Typed {
                    name: variant_name,
                    schema,
                    description,
                } => {
                    // Append the variant name to the type_name for our new
                    // type name hint.
                    let (details, deny) = self
                        .external_variant(type_name.append(variant_name), schema)
                        .ok()?;
                    deny_unknown_fields |= deny;

                    let (name, rename) = recase(variant_name, Case::Pascal);
                    Some(Variant {
                        name,
                        rename,
                        description,
                        details,
                    })
                }
            })
            .collect::<Option<Vec<_>>>()?;

        Some(TypeEntryEnum::from_metadata(
            self,
            type_name,
            enum_metadata,
            EnumTagType::External,
            variants,
            deny_unknown_fields,
            original_schema.clone(),
        ))
    }

    /// Return the variant details and a bool indicating if the schema denies
    /// unknown fields.
    fn external_variant(
        &mut self,
        prop_type_name: Name,
        variant_schema: &Schema,
    ) -> Result<(VariantDetails, bool)> {
        let (ty, _) = self.convert_schema(prop_type_name, variant_schema)?;

        match ty {
            TypeEntry {
                details: TypeEntryDetails::Tuple(types),
                ..
            } => {
                let details = VariantDetails::Tuple(types);
                Ok((details, false))
            }
            TypeEntry {
                details: TypeEntryDetails::Unit,
                ..
            } => {
                let details = VariantDetails::Simple;
                Ok((details, false))
            }
            TypeEntry {
                details:
                    TypeEntryDetails::Struct(TypeEntryStruct {
                        name: _,
                        rename: _,
                        description: _,
                        default: _, // TODO arguably we should look at this
                        properties,
                        deny_unknown_fields,
                        schema: _,
                    }),
                ..
            } => {
                let details = VariantDetails::Struct(properties);
                Ok((details, deny_unknown_fields))
            }

            ty => {
                let type_id = self.assign_type(ty);

                let details = VariantDetails::Item(type_id);
                Ok((details, false))
            }
        }
    }

    pub(crate) fn maybe_internally_tagged_enum(
        &mut self,
        type_name: Name,
        original_schema: &Schema,
        metadata: &Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        // All subschemas must be objects and all objects must have a
        // required, *fixed-value* property in common. To detect this, we look
        // at all such properties along with the specific values.
        let constant_value_properties_sets = subschemas
            .iter()
            .map(|schema| match get_object(schema) {
                Some((_, validation)) => {
                    validation
                        .properties
                        .iter()
                        .filter(|(prop_name, _)| validation.required.contains(*prop_name))
                        .filter_map(|(prop_name, prop_type)| {
                            constant_string_value(prop_type).map(|value| {
                                // Tuple consisting of the name and a set
                                // with a single value
                                (prop_name.clone(), BTreeSet::from([value.to_string()]))
                            })
                        })
                        .collect()
                }

                // For non-objects, there are no such properties; return
                // the empty set. Note that in the next pass this will
                // result in a None value and exiting the outer function.
                None => BTreeMap::new(),
            })
            // Reduce these sets down to those A. that are common among all
            // subschemas and B. for which the values are unique.
            .reduce(|a, b| {
                a.into_iter()
                    .filter_map(|(prop, mut a_values)| match b.get(&prop) {
                        // If the values are non-disjoint it means that there
                        // are two subschemas that have constant values for a
                        // given property but that those values are identical.
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

        let mut deny_unknown_fields = false;
        let variants = subschemas
            .iter()
            .map(|schema| {
                // We've already validated this; we just need to pluck out the
                // pieces we need to construct the variant.
                let Some((metadata, validation)) = get_object(schema) else {
                    unreachable!();
                };

                match validation.additional_properties.as_ref().map(Box::as_ref) {
                    Some(Schema::Bool(false)) => {
                        deny_unknown_fields = true;
                    }
                    None => (),
                    _ => unreachable!(),
                }

                self.internal_variant(type_name.clone(), metadata, validation, tag)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntryEnum::from_metadata(
            self,
            type_name,
            metadata,
            EnumTagType::Internal { tag: tag.clone() },
            variants,
            deny_unknown_fields,
            original_schema.clone(),
        ))
    }

    fn internal_variant(
        &mut self,
        enum_type_name: Name,
        metadata: &Option<Box<schemars::schema::Metadata>>,
        validation: &ObjectValidation,
        tag: &str,
    ) -> Result<Variant> {
        if validation.properties.len() == 1 {
            let (tag_name, schema) = validation.properties.iter().next().unwrap();
            let variant_name = constant_string_value(schema).unwrap();
            let (name, rename) = recase(variant_name, Case::Pascal);

            // The lone property must be our tag.
            assert_eq!(tag_name, tag);
            assert_eq!(validation.required.len(), 1);

            let variant = Variant {
                name,
                rename,
                description: None,
                details: VariantDetails::Simple,
            };
            Ok(variant)
        } else {
            let tag_schema = validation.properties.get(tag).unwrap();
            let variant_name = constant_string_value(tag_schema).unwrap();
            let (name, rename) = recase(variant_name, Case::Pascal);

            // Make a new object validation that omits the tag.
            let mut new_validation = validation.clone();
            new_validation.properties.remove(tag);
            new_validation.required.remove(tag);

            let (properties, _) =
                self.struct_members(enum_type_name.into_option(), &new_validation)?;
            let variant = Variant {
                name,
                rename,
                description: metadata_title_and_description(metadata),
                details: VariantDetails::Struct(properties),
            };
            Ok(variant)
        }
    }

    pub(crate) fn maybe_adjacently_tagged_enum(
        &mut self,
        type_name: Name,
        original_schema: &Schema,
        metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        // All subschemas need to be objects with at most two properties: a
        // constant, required property (the tag) and an optional property for
        // content. Naturally, those two properties need to be the same for each
        // subschema.
        let prop_sets = subschemas
            .iter()
            .map(|schema| match get_object(schema) {
                Some((_, validation))
                    if validation.properties.len() == validation.required.len() =>
                {
                    let constants = validation
                        .properties
                        .iter()
                        .filter_map(|(prop_name, prop_type)| {
                            constant_string_value(prop_type).map(|_| prop_name.clone())
                        })
                        .collect::<BTreeSet<_>>();
                    let properties = validation
                        .properties
                        .keys()
                        .cloned()
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

        let content = content_props.difference(&tag_props).next().cloned()?;
        let tag = tag_props.into_iter().next()?;

        let mut deny_unknown_fields = false;

        let variants = subschemas
            .iter()
            .map(|schema| {
                // We've already validated this; we just need to pluck out the
                // pieces we need to construct the variant.
                let Some((metadata, validation)) = get_object(schema) else {
                    unreachable!();
                };
                let (variant, deny) =
                    self.adjacent_variant(type_name.clone(), metadata, validation, &tag, &content)?;
                deny_unknown_fields |= deny;
                Ok(variant)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntryEnum::from_metadata(
            self,
            type_name,
            metadata,
            EnumTagType::Adjacent { tag, content },
            variants,
            deny_unknown_fields,
            original_schema.clone(),
        ))
    }

    fn adjacent_variant(
        &mut self,
        enum_type_name: Name,
        metadata: &Option<Box<schemars::schema::Metadata>>,
        validation: &ObjectValidation,
        tag: &str,
        content: &str,
    ) -> Result<(Variant, bool)> {
        if validation.properties.len() == 1 {
            let (tag_name, schema) = validation.properties.iter().next().unwrap();
            let variant_name = constant_string_value(schema).unwrap();
            let (name, rename) = recase(variant_name, Case::Pascal);

            // The lone property must be our tag.
            assert_eq!(tag_name, tag);
            assert_eq!(validation.required.len(), 1);

            let variant = Variant {
                name,
                rename,
                description: None,
                details: VariantDetails::Simple,
            };
            Ok((variant, false))
        } else {
            let tag_schema = validation.properties.get(tag).unwrap();
            let variant_name = constant_string_value(tag_schema).unwrap();
            let (name, rename) = recase(variant_name, Case::Pascal);

            let sub_type_name = match enum_type_name {
                // If the type name is known (required) we append the name of
                // the content (i.e. the struct member); because this type is
                // required (i.e. a named reference) it will be generated as a
                // struct as well. This naming ensures that any inferred
                // subtypes match between this variant and the independent
                // struct type.
                //
                // Note that below we include the variant name to ensure
                // uniqueness, but that is not required here as the required
                // type name already ensures uniqueness.
                name @ Name::Required(_) => name.append(content),

                // Otherwise we simply append the variant name for
                // disambiguation.
                name => name.append(variant_name),
            };

            let content_schema = validation.properties.get(content).unwrap();
            let (details, deny) = self.external_variant(sub_type_name, content_schema)?;

            let variant = Variant {
                name,
                rename,
                description: metadata_title_and_description(metadata),
                details,
            };
            Ok((variant, deny))
        }
    }

    /// Produce an enum with each subschema as a variant. There isn't an
    /// explicit name for each variant so we default to `VariantNN`.
    ///
    /// ```compile_fail
    /// enum MyEnum {
    ///     Variant1(ThingsOfYours),
    ///     Variant2(ThingsOfMine),
    /// }
    /// ```
    ///
    /// We can, however, in some cases infer better names: if each variant is a
    /// tuple of cardinality 1 with a named type, we case use those names
    /// instead for the variants. For example:
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
        &mut self,
        type_name: Name,
        original_schema: &Schema,
        metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Result<TypeEntry> {
        let tmp_type_name = get_type_name(&type_name, metadata);

        let mut deny_unknown_fields = false;

        let variant_names = subschemas
            .iter()
            // Try to get a good name for each variant. Note that this doesn't
            // account for types such as Uuid whose names come from outside of
            // the schema... but you can't win them all.
            .map(schema_is_named)
            .collect::<Option<Vec<_>>>()
            // Prune the common prefixes from all variant names. If this
            // results in any of them being empty, we don't use these names.
            .and_then(|variant_names| {
                let common_prefix = variant_names
                    .iter()
                    .cloned()
                    .reduce(|a, b| get_common_prefix(&a, &b))
                    .unwrap();
                variant_names
                    .into_iter()
                    .map(|var_name| {
                        let var_name = &var_name[common_prefix.len()..];
                        if var_name.is_empty() {
                            None
                        } else {
                            Some(var_name.to_string())
                        }
                    })
                    .collect::<Option<Vec<_>>>()
            })
            // Fall back to `VariantN` naming.
            .unwrap_or_else(|| {
                (0..subschemas.len())
                    .map(|idx| format!("Variant{}", idx))
                    .collect()
            });

        // Gather the variant details along with its name.
        let variant_details = subschemas
            .iter()
            .zip(variant_names)
            .map(|(schema, variant_name)| {
                // We provide a suggested name for the variant value's type
                // simply by appending the variant name to the type name we've
                // inferred for this enum.
                let prop_type_name = match &tmp_type_name {
                    Some(name) => Name::Suggested(name.clone()),
                    None => Name::Unknown,
                }
                .append(&variant_name);

                let (details, deny) = self.external_variant(prop_type_name, schema)?;
                // Note that this is really only relevant for in-line schemas;
                // referenced schemas will enforce their own policy on their
                // generated types.
                deny_unknown_fields |= deny;

                Ok((details, variant_name))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = variant_details
            .into_iter()
            .map(|(details, name)| {
                assert!(!name.is_empty());
                Variant {
                    name,
                    rename: None,
                    description: None,
                    details,
                }
            })
            .collect();

        Ok(TypeEntryEnum::from_metadata(
            self,
            // TODO should this be tmp_type_name?
            type_name,
            metadata,
            EnumTagType::Untagged,
            variants,
            deny_unknown_fields,
            original_schema.clone(),
        ))
    }
}

/// Get the string that represents the common prefix, considering only
/// case-relevant boundaries.
fn get_common_prefix(name: &str, prefix: &str) -> String {
    // Convert both to kebab case, split by '-' and zip them together. Pick up
    // components in order while they're equal, join, and convert to Pascal
    // case for a variant name.
    name.to_kebab_case()
        .split('-')
        .zip(prefix.to_kebab_case().split('-'))
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect::<Vec<&str>>()
        .join("-")
        .to_pascal_case()
}

pub(crate) fn output_variant(
    variant: &Variant,
    type_space: &TypeSpace,
    output: &mut OutputSpace,
    type_name: &str,
) -> TokenStream {
    let name = format_ident!("{}", variant.name);
    let doc = variant.description.as_ref().map(|s| {
        quote! { #[doc = #s] }
    });
    let serde = variant.rename.as_ref().map(|s| {
        quote! { #[serde(rename = #s)] }
    });
    match &variant.details {
        VariantDetails::Simple => quote! {
            #doc
            #serde
            #name,
        },
        VariantDetails::Item(type_id) => {
            let item_type_ident = type_space
                .id_to_entry
                .get(type_id)
                .unwrap()
                .type_ident(type_space, &None);

            quote! {
                #doc
                #serde
                #name(#item_type_ident),
            }
        }

        VariantDetails::Tuple(tuple) => {
            let types = tuple.iter().map(|type_id| {
                type_space
                    .id_to_entry
                    .get(type_id)
                    .unwrap()
                    .type_ident(type_space, &None)
            });

            if tuple.len() != 1 {
                quote! {
                    #doc
                    #serde
                    #name(#(#types),*),
                }
            } else {
                // A tuple variant with a single element requires special
                // handling lest its "tuple-ness" be lost. This is important to
                // ensure correct serialization and deserialization behavior.
                // Note in particular the extra parentheses and trailing comma.
                quote! {
                    #doc
                    #serde
                    #name((#(#types,)*)),
                }
            }
        }

        VariantDetails::Struct(props) => {
            let prop_streams = props.iter().map(|prop| {
                let prop_doc = prop.description.as_ref().map(|s| quote! { #[doc = #s] });

                let prop_type_entry = type_space.id_to_entry.get(&prop.type_id).unwrap();
                let (prop_serde, _) = generate_serde_attr(
                    &format!("{}{}", type_name, &variant.name),
                    &prop.name,
                    &prop.rename,
                    &prop.state,
                    prop_type_entry,
                    type_space,
                    output,
                );

                let prop_name = format_ident!("{}", prop.name);
                let prop_type = prop_type_entry.type_ident(type_space, &None);

                quote! {
                    #prop_doc
                    #prop_serde
                    #prop_name: #prop_type,
                }
            });
            quote! {
                #doc
                #serde
                #name {
                    #(#prop_streams)*
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use quote::quote;
    use schema::Schema;
    use schemars::{
        schema::{InstanceType, RootSchema, SchemaObject, SingleOrVec},
        schema_for, JsonSchema,
    };
    use serde::Serialize;

    use crate::{
        output::OutputSpace,
        test_util::{validate_output, validate_output_for_untagged_enm},
        type_entry::{EnumTagType, TypeEntryEnum, Variant, VariantDetails},
        Name, RefKey, TypeEntryDetails, TypeId, TypeSpace, TypeSpaceSettings,
    };

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(deny_unknown_fields)]
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
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_none());
        assert!(type_space
            .maybe_internally_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_none());
    }

    #[test]
    fn test_externally_tagged_enum_output() {
        validate_output::<ExternallyTaggedEnum>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(tag = "tag", content = "content", deny_unknown_fields)]
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
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("AdjacentlyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("AdjacentlyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
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
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_internally_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_none());
        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .is_none());
    }

    #[test]
    fn test_internally_tagged_enum_output() {
        validate_output::<InternallyTaggedEnum>();
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(untagged, deny_unknown_fields)]
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
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().any_of.unwrap();
        let ty = type_space
            .untagged_enum(
                Name::Required("UntaggedEnum".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .unwrap();

        match &ty.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                name,
                rename: None,
                description: None,
                default: None,
                tag_type: EnumTagType::Untagged,
                variants,
                deny_unknown_fields: _,
                bespoke_impls: _,
                schema: _,
            }) => {
                assert_eq!(name, "UntaggedEnum");
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
                        details: VariantDetails::Item(_),
                        ..
                    }
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
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().any_of.unwrap();

        let (ty, _) = type_space
            .convert_one_of(
                Name::Required("Xyz".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .unwrap();

        // This confirms in particular that the tag type is untagged and
        // therefore that the other enum tagging regimes did not match.
        assert!(matches!(
            &ty.details,
            TypeEntryDetails::Enum(TypeEntryEnum {
                rename: None,
                description: None,
                tag_type: EnumTagType::Untagged,
                ..
            })
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
            .convert_schema_object(
                Name::Unknown,
                &schemars::schema::Schema::Object(schema.schema.clone()),
                &schema.schema,
            )
            .unwrap();

        if let TypeEntryDetails::Enum(TypeEntryEnum {
            variants,
            tag_type,
            deny_unknown_fields,
            ..
        }) = &type_entry.details
        {
            let variant_names = variants
                .iter()
                .map(|variant| variant.name.clone())
                .collect::<HashSet<_>>();
            assert_eq!(variant_names.len(), variants.len());
            assert_eq!(tag_type, &EnumTagType::Untagged);
            assert_eq!(deny_unknown_fields, &true);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_maybe_option() {
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
        let type_entry = type_space
            .maybe_option(Name::Unknown, &None, &subschemas)
            .unwrap();

        assert_eq!(type_entry.details, TypeEntryDetails::Option(TypeId(1)))
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
                        "conclusion",
                        "number",
                        "started_at",
                        "completed_at"
                    ],
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
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
                        "conclusion",
                        "number",
                        "started_at",
                        "completed_at"
                    ],
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
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

        let type_id = type_space
            .ref_to_id
            .get(&RefKey::Def("workflow-step".to_string()))
            .unwrap();
        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

        match &type_entry.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                tag_type,
                variants,
                deny_unknown_fields: _,
                ..
            }) => {
                assert_eq!(tag_type, &EnumTagType::Untagged);
                //assert_eq!(deny_unknown_fields, &true);
                for variant in variants {
                    match &variant.details {
                        VariantDetails::Item(item) => {
                            let variant_type = type_space.id_to_entry.get(item).unwrap();
                            assert!(variant_type.name().unwrap().ends_with(&variant.name));
                        }
                        _ => panic!("{:#?}", type_entry),
                    }
                }
            }
            _ => panic!("{:#?}", type_entry),
        }
    }

    #[test]
    fn test_tricky_untagged_enum() {
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
                        "status": { "type": "string", "enum": ["completed_x"] },
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
                        "status": { "type": "string", "enum": ["in_progress_y"] },
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

        let type_id = type_space
            .ref_to_id
            .get(&RefKey::Def("workflow-step".to_string()))
            .unwrap();
        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

        match &type_entry.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                tag_type: EnumTagType::Untagged,
                ..
            }) => {}
            _ => panic!("{:#?}", type_entry),
        }
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema, Schema)]
    #[serde(tag = "tag", deny_unknown_fields)]
    enum InternalSimple {
        Shadrach,
        Meshach,
        Abednego,
    }

    #[test]
    fn test_internal_deny_simple() {
        validate_output::<InternalSimple>();
    }

    #[test]
    fn test_result() {
        let mut type_space = TypeSpace::default();
        let schema = schema_for!(Result<u32, String>);
        let original_schema = schemars::schema::Schema::Object(schema.schema.clone());
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();
        let type_entry = type_space
            .maybe_externally_tagged_enum(
                Name::Required("ResultX".to_string()),
                &original_schema,
                &None,
                &subschemas,
            )
            .unwrap();
        let mut output = OutputSpace::default();
        type_entry.output(&type_space, &mut output);
        let actual = output.into_stream();
        let schema_json = serde_json::to_string_pretty(&original_schema).unwrap();
        let schema_lines = schema_json.lines();
        let expected = quote! {
            #[doc = "ResultX"]
            ///
            /// <details><summary>JSON schema</summary>
            ///
            /// ```json
            #(
                #[doc = #schema_lines]
            )*
            /// ```
            /// </details>
            #[derive(Clone, Debug, Deserialize, Serialize)]
            pub enum ResultX {
                Ok(u32),
                Err(String),
            }

            impl From<&ResultX> for ResultX {
                fn from(value: &ResultX) -> Self {
                    value.clone()
                }
            }

            impl From<u32> for ResultX {
                fn from(value: u32) -> Self {
                    Self::Ok(value)
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_result_derives() {
        let mut type_space = TypeSpace::new(
            TypeSpaceSettings::default()
                .with_derive("A".to_string())
                .with_derive("B".to_string())
                .with_derive("C".to_string())
                .with_derive("D".to_string()),
        );
        let schema = schema_for!(Result<u32, String>);
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();
        let type_entry = type_space
            .maybe_externally_tagged_enum(
                Name::Required("ResultX".to_string()),
                &schemars::schema::Schema::Bool(true),
                &None,
                &subschemas,
            )
            .unwrap();
        let mut output = OutputSpace::default();
        type_entry.output(&type_space, &mut output);
        let actual = output.into_stream();
        let expected = quote! {
            #[doc = "ResultX"]
            ///
            /// <details><summary>JSON schema</summary>
            ///
            /// ```json
            #[doc = "true"]
            /// ```
            /// </details>
            #[derive(A, B, C, Clone, D, Debug, Deserialize, Serialize)]
            pub enum ResultX {
                Ok(u32),
                Err(String),
            }

            impl From<&ResultX> for ResultX {
                fn from(value: &ResultX) -> Self {
                    value.clone()
                }
            }

            impl From<u32> for ResultX {
                fn from(value: u32) -> Self {
                    Self::Ok(value)
                }
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_singleton_enum() {
        #[allow(dead_code)]
        #[derive(Serialize, JsonSchema, Schema)]
        #[serde(tag = "type", content = "data")]
        enum Hobsons {
            Choice(String),
        }

        validate_output::<Hobsons>();
    }
}
