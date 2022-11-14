// Copyright 2022 Oxide Computer Company

use std::collections::{BTreeMap, BTreeSet, HashSet};

use heck::{ToKebabCase, ToPascalCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};

use crate::{
    output::OutputSpace,
    structs::generate_serde_attr,
    type_entry::{EnumTagType, TypeEntry, TypeEntryEnum, Variant, VariantDetails},
    util::{
        constant_string_value, get_type_name, metadata_description, metadata_title_and_description,
        none_or_single, recase, ref_key, schema_is_named, Case,
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
        metadata: &Option<Box<schemars::schema::Metadata>>,
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
                                value.as_str().map(|variant_name| ProtoVariant::Simple {
                                    name: variant_name,
                                    description: metadata_description(metadata),
                                })
                            })
                            .collect()
                    }

                    // Objects must have a single required member. The type of
                    // that lone member determines the type associated with the
                    // variant.
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
                        if let ObjectValidation {
                            max_properties: None,
                            min_properties: None,
                            required,
                            properties,
                            pattern_properties,
                            additional_properties: _,
                            property_names: None,
                        } = validation.as_ref()
                        {
                            if required.len() == 1
                                && properties.len() == 1
                                && pattern_properties.is_empty()
                            {
                                let (prop_name, prop_type) = properties.iter().next().unwrap();
                                // If required and properties both have length 1
                                // then this must be true for a well-constructed
                                // schema.
                                assert!(required.contains(prop_name));

                                Some(vec![ProtoVariant::Typed {
                                    name: prop_name,
                                    schema: prop_type,
                                    description: metadata_description(metadata),
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
            .collect::<Option<Vec<_>>>()?;

        let variant_names = proto_variants
            .iter()
            .map(|proto| match proto {
                ProtoVariant::Simple { name, .. } | ProtoVariant::Typed { name, .. } => name,
            })
            .collect::<HashSet<_>>();

        // We can't have duplicate names in an enum.
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
            metadata,
            EnumTagType::External,
            variants,
            deny_unknown_fields,
        ))
    }

    fn external_variant(
        &mut self,
        prop_type_name: Name,
        variant_schema: &Schema,
    ) -> Result<(VariantDetails, bool)> {
        // Arrays (tuples) must have a fixed size (max_items == min_items).
        //
        // Per the JSON Schema specification, if the array.items is an array
        // (rather than a single element), then:
        //   'validation succeeds if each element of the instance validates
        //   against the schema at the same position, if any.'
        //
        // Accordingly we require the length of the items array to match the
        // fixed size (max_items). Note that array.additionalItems is
        // irrelevant due to this portion of the spec:
        //   'If "items" is present, and its annotation result is a number,
        //   validation succeeds if every instance element at an index greater
        //   than that number validates against "additionalItems".'
        //
        // Note that this is not part fo the match below due to the nested
        // conditions and destructuring.
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
        }) = variant_schema
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
                    if *max_items >= 2 && max_items == min_items && *max_items == items.len() as u32
                    {
                        let details = VariantDetails::Tuple(
                            items
                                .iter()
                                .map(|item_type| {
                                    Ok(self.id_for_schema(prop_type_name.clone(), item_type)?.0)
                                })
                                .collect::<Result<Vec<_>>>()?,
                        );
                        return Ok((details, false));
                    }
                }
            }
        }

        match variant_schema {
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
            }) if single.as_ref() == &InstanceType::Null => Ok((VariantDetails::Simple, false)),

            // Anonymous (i.e. those where metadata.title is None) structs are
            // embedded within the variant as the struct type.
            Schema::Object(SchemaObject {
                metadata,
                instance_type,
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
            }) if none_or_single(instance_type, &InstanceType::Object)
                && metadata
                    .as_ref()
                    .map(|m| m.as_ref().title.as_ref())
                    .is_none() =>
            {
                let tmp_type_name = match prop_type_name {
                    Name::Required(name) | Name::Suggested(name) => Some(name),
                    Name::Unknown => None,
                };
                let (properties, deny) = self.struct_members(tmp_type_name, validation)?;
                Ok((VariantDetails::Struct(properties), deny))
            }

            // Otherwise we create a single-element tuple variant with the given type.
            prop_type => {
                let (type_id, _) = self.id_for_schema(prop_type_name, prop_type)?;
                // TODO We'd ideally look at the type itself to determine if
                // they represent a "closed" struct in which case we'd return
                // "true". However these may be yet-unresolved references so to
                // do this properly we'd need to go through the JSON schema
                // itself rather than our intermediate representation.
                let details = VariantDetails::Tuple(vec![type_id]);
                Ok((details, false))
            }
        }
    }

    pub(crate) fn maybe_internally_tagged_enum(
        &mut self,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        // All subschemas must be objects and all objects must have a *fixed-value*
        // required property in common. To detect this, we look at all such
        // properties along with the specific values.
        let constant_value_properties_sets = subschemas
            .iter()
            .map(
                |schema| match get_object(type_name.clone(), schema, &self.definitions) {
                    None => BTreeMap::<String, BTreeSet<String>>::new(),
                    Some((_, _, validation)) => {
                        validation
                            .properties
                            .iter()
                            .filter_map(|(prop_name, prop_type)| {
                                constant_string_value(prop_type).map(|value| {
                                    // Tuple with the name and a set with a single value
                                    (
                                        prop_name.clone(),
                                        [value.to_string()].iter().cloned().collect(),
                                    )
                                })
                            })
                            .collect()
                    }
                },
            )
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

        let mut deny_unknown_fields = false;
        let variants = subschemas
            .iter()
            .map(|schema| {
                // We've already validated this; we just need to pluck out the
                // pieces we need to construct the variant.
                match get_object(type_name.clone(), schema, &self.definitions) {
                    None => unreachable!(),
                    Some((sub_type_name, metadata, validation)) => {
                        match validation.additional_properties.as_ref().map(Box::as_ref) {
                            Some(Schema::Bool(false)) => {
                                deny_unknown_fields = true;
                            }
                            None => {}
                            _ => unreachable!(),
                        }
                        // Release our borrow of self.
                        let validation = validation.clone();
                        let metadata = metadata.clone();
                        Ok(self.internal_variant(sub_type_name, &metadata, &validation, tag)?)
                    }
                }
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
        ))
    }

    fn internal_variant(
        &mut self,
        type_name: Name,
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

            let (properties, _) = self.struct_members(type_name.into_option(), &new_validation)?;
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
        metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        // All subschemas need to be objects with at most two properties: a
        // constant, required property (the tag) and an optional property for
        // content. Naturally, those two properties need to be the same for each
        // subschema.
        let prop_sets = subschemas
            .iter()
            .map(
                |schema| match get_object(type_name.clone(), schema, &self.definitions) {
                    Some((_, _, validation))
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
                            .iter()
                            .map(|(prop_name, _)| prop_name.clone())
                            .collect::<BTreeSet<_>>();

                        Some((constants, properties))
                    }

                    _ => None,
                },
            )
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
                match get_object(type_name.clone(), schema, &self.definitions) {
                    None => unreachable!(),
                    Some((sub_type_name, metadata, validation)) => {
                        let metadata = metadata.clone();
                        let validation = validation.clone();
                        let (variant, deny) = self.adjacent_variant(
                            sub_type_name,
                            &metadata,
                            &validation,
                            &tag,
                            &content,
                        )?;
                        deny_unknown_fields |= deny;
                        Ok(variant)
                    }
                }
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
        ))
    }

    fn adjacent_variant(
        &mut self,
        type_name: Name,
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

            let sub_type_name = match type_name {
                // If the type name is known (required) we append the name of
                // the content (i.e. the struct member); because this type is
                // required (i.e. a named reference) it will be genetated as a
                // struct as well. This naming ensures that any inferred
                // subtypes match between ths variant and the independent
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
        metadata: &Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Result<TypeEntry> {
        let tmp_type_name = get_type_name(&type_name, metadata);

        let mut names_from_variants = true;
        let mut common_prefix = None;

        let mut deny_unknown_fields = false;

        // Gather the variant details along with an Option of its "good" name.
        let variant_details = subschemas
            .iter()
            .enumerate()
            .map(|(idx, schema)| {
                let variant_name = format!("Variant{}", idx);
                // We provide a suggested name for the variant value's type
                // simply by appending the variant name to the type name we've
                // inferred for this enum.
                let prop_type_name = match &tmp_type_name {
                    Some(name) => Name::Suggested(name.clone()),
                    None => Name::Unknown,
                }
                .append(&variant_name);
                let (details, deny) = self.external_variant(prop_type_name, schema)?;
                deny_unknown_fields |= deny;
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
                    good_name.unwrap()[common_prefix_index..].to_string()
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

        Ok(TypeEntryEnum::from_metadata(
            self,
            // TODO should this be tmp_type_name?
            type_name,
            metadata,
            EnumTagType::Untagged,
            variants,
            deny_unknown_fields,
        ))
    }
}

/// Internally and adjacently tagged enums expect their subschemas to be
/// objects. Return the object data or None if it's not an object (or reference
/// to an object) or doesn't conform to the objects we know how to handle.
pub(crate) fn get_object<'a>(
    type_name: Name,
    schema: &'a Schema,
    definitions: &'a BTreeMap<String, Schema>,
) -> Option<(Name, &'a Option<Box<Metadata>>, &'a ObjectValidation)> {
    match schema {
        // Objects
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
            && schema_none_or_false(&validation.additional_properties)
            && validation.max_properties.is_none()
            && validation.min_properties.is_none()
            && validation.pattern_properties.is_empty()
            && validation.property_names.is_none() =>
        {
            Some((type_name, metadata, validation.as_ref()))
        }

        // References
        Schema::Object(SchemaObject {
            metadata: None,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: None,
            reference: Some(ref_name),
            extensions: _,
        }) => {
            let ref_key = ref_key(ref_name);
            get_object(
                Name::Required(ref_key.to_string()),
                definitions.get(ref_key).unwrap(),
                definitions,
            )
        }

        // Trivial (n == 1) subschemas
        Schema::Object(SchemaObject {
            metadata,
            instance_type: _,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: Some(subschemas),
            number: None,
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: _,
        }) => match subschemas.as_ref() {
            SubschemaValidation {
                all_of: Some(subschemas),
                any_of: None,
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } if subschemas.len() == 1 => subschemas.first(),
            SubschemaValidation {
                all_of: None,
                any_of: Some(subschemas),
                one_of: None,
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } if subschemas.len() == 1 => subschemas.first(),
            SubschemaValidation {
                all_of: None,
                any_of: None,
                one_of: Some(subschemas),
                not: None,
                if_schema: None,
                then_schema: None,
                else_schema: None,
            } if subschemas.len() == 1 => subschemas.first(),
            _ => None,
        }
        .and_then(|sub_schema| {
            get_object(type_name, sub_schema, definitions).map(|(name, m, validation)| match m {
                Some(_) => (name, metadata, validation),
                None => (name, &None, validation),
            })
        }),

        // None if the schema doesn't match the shape we expect.
        _ => None,
    }
}

// We infer from a Some(Schema::Bool(false)) or None value that either nothing
// or nothing of importance is in the additional properties.
fn schema_none_or_false(additional_properties: &Option<Box<Schema>>) -> bool {
    matches!(
        additional_properties.as_ref().map(Box::as_ref),
        None | Some(Schema::Bool(false))
    )
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

        VariantDetails::Tuple(tuple) => {
            let types = tuple.iter().map(|type_id| {
                type_space
                    .id_to_entry
                    .get(type_id)
                    .unwrap()
                    .type_ident(type_space, &None)
            });

            quote! {
                #doc
                #serde
                #name(#(#types),*),
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
        Name, TypeEntryDetails, TypeId, TypeSpace, TypeSpaceSettings,
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
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
                &None,
                &subschemas,
            )
            .is_none());
        assert!(type_space
            .maybe_internally_tagged_enum(
                Name::Required("ExternallyTaggedEnum".to_string()),
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
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("AdjacentlyTaggedEnum".to_string()),
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("AdjacentlyTaggedEnum".to_string()),
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
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();

        assert!(type_space
            .maybe_internally_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
                &None,
                &subschemas,
            )
            .is_some());
        assert!(type_space
            .maybe_adjacently_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
                &None,
                &subschemas,
            )
            .is_none());
        assert!(type_space
            .maybe_externally_tagged_enum(
                Name::Required("InternallyTaggedEnum".to_string()),
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
        let subschemas = schema.schema.subschemas.unwrap().any_of.unwrap();
        let ty = type_space
            .untagged_enum(
                Name::Required("UntaggedEnum".to_string()),
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
            .convert_one_of(Name::Required("Xyz".to_string()), &None, &subschemas)
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
            .convert_schema_object(Name::Unknown, &schema.schema)
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

        let type_id = type_space.ref_to_id.get("workflow-step").unwrap();
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
                        VariantDetails::Tuple(items) if items.len() == 1 => {
                            let variant_type =
                                type_space.id_to_entry.get(items.first().unwrap()).unwrap();
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
    fn test_tricky_internally_tagged_enum() {
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

        let type_id = type_space.ref_to_id.get("workflow-step").unwrap();
        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

        match &type_entry.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                tag_type,
                deny_unknown_fields,
                ..
            }) => {
                assert_eq!(
                    tag_type,
                    &EnumTagType::Internal {
                        tag: "status".to_string()
                    }
                );
                assert_eq!(deny_unknown_fields, &true);
            }
            _ => panic!("{:#?}", type_entry),
        }
    }

    #[test]
    fn test_untagged_enum_nice_names() {
        let schema_json = r##"
        {
            "definitions": {
                "IpNet": {
                    "oneOf": [
                        {
                            "title": "V4",
                            "allOf": [
                                {
                                    "$ref": "#/components/schemas/Ipv4Net"
                                }
                            ]
                        },
                        {
                            "title": "V6",
                            "allOf": [
                                {
                                    "$ref": "#/components/schemas/Ipv4Net"
                                }
                            ]
                        }
                    ]
                },
                "Ipv4Net": {
                    "type": "string"
                },
                "Ipv6Net": {
                    "type": "string"
                }
            }
        }
        "##;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();

        let type_id = type_space.ref_to_id.get("IpNet").unwrap();
        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

        match &type_entry.details {
            TypeEntryDetails::Enum(TypeEntryEnum {
                tag_type,
                variants,
                deny_unknown_fields,
                ..
            }) => {
                assert_eq!(tag_type, &EnumTagType::Untagged);
                assert_eq!(deny_unknown_fields, &false);
                let variant_names = variants
                    .iter()
                    .map(|variant| variant.name.clone())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    variant_names,
                    ["V4", "V6"]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<HashSet<_>>()
                );
            }
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
        let subschemas = schema.schema.subschemas.unwrap().one_of.unwrap();
        let type_entry = type_space
            .maybe_externally_tagged_enum(Name::Required("ResultX".to_string()), &None, &subschemas)
            .unwrap();
        let mut output = OutputSpace::default();
        type_entry.output(&type_space, &mut output);
        let actual = output.into_stream();
        let expected = quote! {
            #[derive(Clone, Debug, Deserialize, Serialize)]
            pub enum ResultX {
                Ok(u32),
                Err(String),
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
            .maybe_externally_tagged_enum(Name::Required("ResultX".to_string()), &None, &subschemas)
            .unwrap();
        let mut output = OutputSpace::default();
        type_entry.output(&type_space, &mut output);
        let actual = output.into_stream();
        let expected = quote! {
            #[derive(Clone, Debug, Deserialize, Serialize, A, B, C, D)]
            pub enum ResultX {
                Ok(u32),
                Err(String),
            }
        };
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_untagged_untyped_unnamed_struct_variants() {
        let schema_json = r#"
        {
            "$schema": "http://json-schema.org/draft-04/schema#",
            "title": "one-of-types",
            "type": "object",
            "oneOf": [
                {
                    "properties": {
                        "bar": {
                            "type": "integer"
                        }
                    },
                    "required": [
                        "bar"
                    ]
                },
                {
                    "properties": {
                        "foo": {
                            "type": "string"
                        }
                    },
                    "required": [
                        "foo"
                    ]
                }
            ]
        }
        "#;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::default();
        let _ = type_space.add_type(&schema.schema.into()).unwrap();

        let actual = type_space.to_stream();
        let expected = quote! {
            #[derive(Clone, Debug, Deserialize, Serialize)]
            #[serde(untagged)]
            pub enum OneOfTypes {
                Variant0 {
                    bar: i64,
                },
                Variant1 {
                    foo: String,
                },
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
