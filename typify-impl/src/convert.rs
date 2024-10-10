// Copyright 2024 Oxide Computer Company

use std::collections::BTreeSet;

use crate::merge::{merge_all, try_merge_with_subschemas};
use crate::type_entry::{
    EnumTagType, TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct,
    Variant, VariantDetails,
};
use crate::util::{all_mutually_exclusive, recase, ref_key, Case, StringValidator};
use log::{debug, info};
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    StringValidation, SubschemaValidation,
};

use crate::util::get_type_name;

use crate::{Error, Name, Result, TypeSpace, TypeSpaceImpl};

impl TypeSpace {
    pub(crate) fn convert_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        info!(
            "convert_schema {:?} {}",
            type_name,
            serde_json::to_string_pretty(schema).unwrap()
        );
        match schema {
            Schema::Object(obj) => {
                if let Some(type_entry) = self.cache.lookup(obj) {
                    Ok((type_entry, &obj.metadata))
                } else {
                    self.convert_schema_object(type_name, schema, obj)
                }
            }

            Schema::Bool(true) => self.convert_permissive(&None),
            Schema::Bool(false) => self.convert_never(type_name, schema),
        }
    }

    pub(crate) fn convert_schema_object<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        schema: &'a SchemaObject,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if let Some(type_entry) = self.convert_rust_extension(schema) {
            return Ok((type_entry, &schema.metadata));
        }

        match schema {
            // If we have a schema that has an instance type array that's
            // exactly two elements and one of them is Null, we have the
            // equivalent of an Option<T> where T is the type defined by the
            // rest of the schema.
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Vec(multiple)),
                enum_values,
                ..
            } if multiple.len() == 2 && multiple.contains(&InstanceType::Null) => {
                let only_null = enum_values.as_ref().map_or(false, |values| {
                    values.iter().all(serde_json::Value::is_null)
                });

                if only_null {
                    // If there are enumerated values and they're all null,
                    // it's just a null.
                    self.convert_null(metadata)
                } else if let Some(other_type) = multiple.iter().find(|t| t != &&InstanceType::Null)
                {
                    // In the sensible case where only one of the instance
                    // types is null.
                    let enum_values = enum_values.clone().map(|values| {
                        values
                            .iter()
                            .filter(|&value| !value.is_null())
                            .cloned()
                            .collect()
                    });
                    let ss = Schema::Object(SchemaObject {
                        instance_type: Some(SingleOrVec::from(*other_type)),
                        enum_values,
                        ..schema.clone()
                    });
                    // An Option type won't usually get a name--unless one is
                    // required (in which case we'll generated a newtype
                    // wrapper to give it a name). In such a case, we invent a
                    // new name for the inner type; otherwise, the inner type
                    // can just have this name.
                    let inner_type_name = match &type_name {
                        Name::Required(name) => Name::Suggested(format!("{}Inner", name)),
                        _ => type_name,
                    };
                    self.convert_option(inner_type_name, metadata, &ss)
                } else {
                    // .. otherwise we try again with a simpler type.
                    let new_schema = SchemaObject {
                        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Null))),
                        ..schema.clone()
                    };
                    self.convert_schema_object(type_name, original_schema, &new_schema)
                        .map(|(te, m)| match m {
                            Some(_) if m == metadata => (te, metadata),
                            Some(_) => panic!("unexpected metadata value"),
                            None => (te, &None),
                        })
                }
            }

            // Strings
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: _,
                string,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::String => self.convert_string(
                type_name,
                original_schema,
                metadata,
                format,
                string.as_ref().map(Box::as_ref),
            ),

            // Strings with the type omitted, but validation present
            SchemaObject {
                metadata,
                instance_type: None,
                format,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: string @ Some(_),
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } => self.convert_string(
                type_name,
                original_schema,
                metadata,
                format,
                string.as_ref().map(Box::as_ref),
            ),

            // Enumerated string type
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                // One could imagine wanting to honor the format field in this
                // case, perhaps to generate an impl From<T> for Uuid, say that
                // allowed for fluid conversion from the enum to a type
                // corresponding to the format string. But that seems uncommon
                // enough to ignore for the moment.
                format: _,
                enum_values: Some(enum_values),
                const_value: None,
                subschemas: None,
                number: _,
                string,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::String => self.convert_enum_string(
                type_name,
                original_schema,
                metadata,
                enum_values,
                string.as_ref().map(Box::as_ref),
            ),

            // Integers
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: validation,
                string: _,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Integer => {
                self.convert_integer(metadata, validation, format)
            }

            // Numbers
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: validation,
                string: _,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Number => {
                self.convert_number(metadata, validation, format)
            }

            // Boolean
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: _,
                const_value: None,
                subschemas: None,
                number: _,
                string: _,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Boolean => self.convert_bool(metadata),

            // Object
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: _,
                string: _,
                array: _,
                object: validation,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Object => {
                self.convert_object(type_name, original_schema, metadata, validation)
            }

            // Object with the type omitted, but validation present
            SchemaObject {
                metadata,
                instance_type: None,
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: validation @ Some(_),
                reference: None,
                extensions: _,
            } => self.convert_object(type_name, original_schema, metadata, validation),

            // Array
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: _,
                string: _,
                array: Some(validation),
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Array => {
                self.convert_array(type_name, metadata, validation)
            }

            // Array with the type omitted, but validation present
            SchemaObject {
                metadata,
                instance_type: None,
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
            } => self.convert_array(type_name, metadata, validation),

            // Arrays of anything
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: _,
                string: _,
                array: None,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Array => self.convert_array_of_any(metadata),

            // The permissive schema
            SchemaObject {
                metadata,
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
            } => self.convert_permissive(metadata),

            // Null
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: _,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: _,
                string: _,
                array: _,
                object: _,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Null => self.convert_null(metadata),

            // Reference
            SchemaObject {
                metadata,
                instance_type: None,
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: None,
                reference: Some(reference),
                extensions: _,
            } => self.convert_reference(metadata, &reference),

            // Accept references that... for some reason... include the type.
            // TODO this could be generalized to validate any redundant
            // validation here or could be used to compute a new, more
            // constrained type.
            // TODO the strictest interpretation might be to ignore any fields
            // that appear alongside "$ref" per
            // https://json-schema.org/understanding-json-schema/structuring.html#ref
            SchemaObject {
                metadata,
                instance_type,
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: None,
                reference: Some(reference),
                extensions: _,
            } => {
                let ref_schema = self.definitions.get(&ref_key(reference)).unwrap();
                assert!(matches!(ref_schema, Schema::Object(SchemaObject {
                        instance_type: it, ..
                    }) if instance_type == it));

                self.convert_reference(metadata, reference)
            }

            SchemaObject {
                metadata,
                instance_type: _,
                format: _,
                enum_values: _,
                const_value: _,
                subschemas: _,
                number: _,
                string: _,
                array: _,
                object: _,
                reference: Some(reference),
                extensions: _,
            } => {
                let mut def = self.definitions.get(&ref_key(reference)).unwrap();
                let mut new_schema = Schema::Object(SchemaObject {
                    reference: None,
                    ..schema.clone()
                });
                while let Schema::Object(SchemaObject { reference: r, .. }) = def {
                    let schema_only_ref = Schema::Object(SchemaObject {
                        reference: r.clone(),
                        ..Default::default()
                    });
                    let schema_without_ref = Schema::Object(SchemaObject {
                        reference: None,
                        ..def.clone().into_object()
                    });
                    new_schema = merge_all(
                        &[schema_without_ref, schema_only_ref, new_schema],
                        &self.definitions,
                    );
                    if let Some(r) = r {
                        def = self.definitions.get(&ref_key(r)).unwrap();
                    } else {
                        break;
                    }
                }
                let (type_entry, _) = self.convert_schema(type_name, &new_schema).unwrap();
                Ok((type_entry, metadata))
            }

            // Enum of a single, known, non-String type (strings above).
            SchemaObject {
                instance_type: Some(SingleOrVec::Single(_)),
                enum_values: Some(enum_values),
                ..
            } => self.convert_typed_enum(type_name, original_schema, schema, enum_values),

            // Enum of unknown type
            SchemaObject {
                metadata,
                instance_type: None,
                format: None,
                enum_values: Some(enum_values),
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } => self.convert_unknown_enum(type_name, original_schema, metadata, enum_values),

            // Subschemas
            SchemaObject {
                metadata,
                // TODO we probably shouldn't ignore this...
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
            } => match subschemas.as_ref() {
                SubschemaValidation {
                    all_of: Some(subschemas),
                    any_of: None,
                    one_of: None,
                    not: None,
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_all_of(type_name, original_schema, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: Some(subschemas),
                    one_of: None,
                    not: None,
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_any_of(type_name, original_schema, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: None,
                    one_of: Some(subschemas),
                    not: None,
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_one_of(type_name, original_schema, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: None,
                    one_of: None,
                    not: Some(subschema),
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_not(type_name, original_schema, metadata, subschema),

                // Multiple subschemas may be present at the same time; attempt
                // to merge and then convert.
                subschemas => {
                    // Remove the subschemas so we can merge into the rest.
                    let schema_object = SchemaObject {
                        subschemas: None,
                        ..schema.clone()
                    };
                    let merged_schema = try_merge_with_subschemas(
                        schema_object,
                        Some(subschemas),
                        &self.definitions,
                    );
                    match merged_schema {
                        Ok(s) => {
                            let (type_entry, _) =
                                self.convert_schema_object(type_name, original_schema, &s)?;
                            Ok((type_entry, &None))
                        }
                        // An error indicates that the schema is unresolvable.
                        Err(_) => self.convert_never(type_name, original_schema),
                    }
                }
            },

            // Subschemas with other stuff.
            SchemaObject {
                metadata,
                subschemas: subschemas @ Some(_),
                ..
            } => {
                let without_subschemas = SchemaObject {
                    subschemas: None,
                    metadata: None,
                    ..schema.clone()
                };
                debug!(
                    "pre merged schema {}",
                    serde_json::to_string_pretty(schema).unwrap(),
                );
                match try_merge_with_subschemas(
                    without_subschemas,
                    subschemas.as_deref(),
                    &self.definitions,
                ) {
                    Ok(merged_schema) => {
                        // Preserve metadata from the outer schema.
                        let merged_schema = SchemaObject {
                            metadata: metadata.clone(),
                            ..merged_schema
                        };
                        debug!(
                            "merged schema {}",
                            serde_json::to_string_pretty(&merged_schema).unwrap(),
                        );

                        let (type_entry, _) =
                            self.convert_schema_object(type_name, original_schema, &merged_schema)?;
                        Ok((type_entry, &None))
                    }

                    Err(_) => self.convert_never(type_name, original_schema),
                }
            }

            // TODO let's not bother with const values at the moment. In the
            // future we could create types that have a single value with a
            // newtype wrapper, but it's too much of a mess for too little
            // value at the moment. Instead, we act as though this const_value
            // field were None.
            SchemaObject {
                metadata,
                const_value: Some(_),
                ..
            } => {
                let new_schema = SchemaObject {
                    const_value: None,
                    ..schema.clone()
                };
                self.convert_schema_object(type_name, original_schema, &new_schema)
                    .map(|(te, m)| match m {
                        Some(_) if m == metadata => (te, metadata),
                        Some(_) => panic!("unexpected metadata value"),
                        None => (te, &None),
                    })
            }

            // In actual, not-made-up, in-the-wild specs, I've seen the type
            // field enumerate all possibilities... perhaps to emphasize their
            // seriousness of the schema representing **anything**. In that
            // case, we can strip it out and try again.
            SchemaObject {
                instance_type: Some(SingleOrVec::Vec(instance_types)),
                metadata,
                ..
            } if instance_types.contains(&InstanceType::Null)
                && instance_types.contains(&InstanceType::Boolean)
                && instance_types.contains(&InstanceType::Object)
                && instance_types.contains(&InstanceType::Array)
                && instance_types.contains(&InstanceType::Number)
                && instance_types.contains(&InstanceType::String)
                && instance_types.contains(&InstanceType::Integer) =>
            {
                let (type_entry, _) = self.convert_schema_object(
                    type_name,
                    original_schema,
                    &SchemaObject {
                        instance_type: None,
                        ..schema.clone()
                    },
                )?;
                Ok((type_entry, metadata))
            }

            // Treat a singleton type array like a singleton type.
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Vec(instance_types)),
                ..
            } if instance_types.len() == 1 => {
                let [it] = instance_types.as_slice() else {
                    unreachable!()
                };
                let (type_entry, _) = self.convert_schema_object(
                    type_name,
                    original_schema,
                    &SchemaObject {
                        instance_type: Some(SingleOrVec::Single((*it).into())),
                        ..schema.clone()
                    },
                )?;
                Ok((type_entry, metadata))
            }

            // Turn schemas with multiple types into an untagged enum labeled
            // according to the given type. We associate any validation with
            // the appropriate type. Note that the case of a 2-type list with
            // one of them Null is already handled more specifically above (and
            // rendered into an Option type).
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Vec(instance_types)),
                format,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number,
                string,
                array,
                object,
                reference: None,
                extensions: _,
            } => {
                // Eliminate duplicates (they hold no significance); they
                // aren't supposed to be there, but we can still handle it.
                let unique_types = instance_types.iter().collect::<BTreeSet<_>>();

                // Massage the data into labeled subschemas with the following
                // format:
                //
                // {
                //     "title": <instance type name>,
                //     "allOf": [
                //         {
                //             "type": <instance type>,
                //             <validation relevant to the type>
                //         }
                //     ]
                // }
                //
                // We can then take these and construct an untagged enum. The
                // outer "allOf" schema lets name the variant.
                //
                // Note that we *could* simply copy the full schema, trusting
                // recursive calls to pull out the appropriate components...
                // but why do tomorrow what we could easily to today?
                let subschemas = unique_types
                    .into_iter()
                    .map(|it| {
                        let instance_type = Some(SingleOrVec::Single(Box::new(*it)));
                        let (label, inner_schema) = match it {
                            InstanceType::Null => (
                                "null",
                                SchemaObject {
                                    instance_type,
                                    ..Default::default()
                                },
                            ),
                            InstanceType::Boolean => (
                                "boolean",
                                SchemaObject {
                                    instance_type,
                                    ..Default::default()
                                },
                            ),
                            InstanceType::Object => (
                                "object",
                                SchemaObject {
                                    instance_type,
                                    object: object.clone(),
                                    ..Default::default()
                                },
                            ),
                            InstanceType::Array => (
                                "array",
                                SchemaObject {
                                    instance_type,
                                    array: array.clone(),
                                    ..Default::default()
                                },
                            ),
                            InstanceType::Number => (
                                "number",
                                SchemaObject {
                                    instance_type,
                                    format: format.clone(),
                                    number: number.clone(),
                                    ..Default::default()
                                },
                            ),
                            InstanceType::String => (
                                "string",
                                SchemaObject {
                                    instance_type,
                                    format: format.clone(),
                                    string: string.clone(),
                                    ..Default::default()
                                },
                            ),
                            InstanceType::Integer => (
                                "integer",
                                SchemaObject {
                                    instance_type,
                                    format: format.clone(),
                                    number: number.clone(),
                                    ..Default::default()
                                },
                            ),
                        };
                        // Make the wrapping schema.
                        Schema::Object(SchemaObject {
                            metadata: Some(Box::new(Metadata {
                                title: Some(label.to_string()),
                                ..Default::default()
                            })),
                            subschemas: Some(Box::new(SubschemaValidation {
                                all_of: Some(vec![inner_schema.into()]),
                                ..Default::default()
                            })),
                            ..Default::default()
                        })
                    })
                    .collect::<Vec<_>>();

                let type_entry =
                    self.untagged_enum(type_name, original_schema, metadata, &subschemas)?;
                Ok((type_entry, metadata))
            }

            // Unknown
            SchemaObject { .. } => todo!(
                "invalid (or unexpected) schema:\n{}",
                serde_json::to_string_pretty(schema).unwrap()
            ),
        }
    }

    fn convert_string<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        format: &Option<String>,
        validation: Option<&StringValidation>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match format.as_ref().map(String::as_str) {
            None => match validation {
                // It should not be possible for the StringValidation to be
                // Some, but all its fields to be None, but... just to be sure.
                None
                | Some(schemars::schema::StringValidation {
                    max_length: None,
                    min_length: None,
                    pattern: None,
                }) => Ok((TypeEntryDetails::String.into(), metadata)),

                Some(validation) => {
                    if let Some(pattern) = &validation.pattern {
                        let _ = regress::Regex::new(pattern).map_err(|e| Error::InvalidSchema {
                            type_name: type_name.clone().into_option(),
                            reason: format!("invalid pattern '{}' {}", pattern, e),
                        })?;
                        self.uses_regress = true;
                    }

                    let string = TypeEntryDetails::String.into();
                    let type_id = self.assign_type(string);
                    Ok((
                        TypeEntryNewtype::from_metadata_with_string_validation(
                            self,
                            type_name,
                            metadata,
                            type_id,
                            validation,
                            original_schema.clone(),
                        ),
                        metadata,
                    ))
                }
            },

            Some("uuid") => {
                self.uses_uuid = true;
                Ok((
                    TypeEntry::new_native(
                        "uuid::Uuid",
                        &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                    ),
                    metadata,
                ))
            }

            Some("date") => {
                self.uses_chrono = true;
                Ok((
                    TypeEntry::new_native(
                        "chrono::naive::NaiveDate",
                        &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                    ),
                    metadata,
                ))
            }
            Some("date-time") => {
                self.uses_chrono = true;
                Ok((
                    TypeEntry::new_native(
                        "chrono::DateTime<chrono::offset::Utc>",
                        &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                    ),
                    metadata,
                ))
            }

            Some("ip") => Ok((
                TypeEntry::new_native(
                    "std::net::IpAddr",
                    &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                ),
                metadata,
            )),
            Some("ipv4") => Ok((
                TypeEntry::new_native(
                    "std::net::Ipv4Addr",
                    &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                ),
                metadata,
            )),
            Some("ipv6") => Ok((
                TypeEntry::new_native(
                    "std::net::Ipv6Addr",
                    &[TypeSpaceImpl::Display, TypeSpaceImpl::FromStr],
                ),
                metadata,
            )),

            Some(unhandled) => {
                info!("treating a string format '{}' as a String", unhandled);
                Ok((TypeEntryDetails::String.into(), metadata))
            }
        }
    }

    pub(crate) fn convert_enum_string<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        enum_values: &[serde_json::Value],
        validation: Option<&StringValidation>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        // We expect all enum values to be either a string **or** a null. We
        // gather them all up and then choose to either be an enum of simple
        // variants, or an Option of an enum of string variants depending on if
        // a null is absent or present. Note that it's actually invalid JSON
        // Schema if we do see a null here. In this code path the instance
        // types should exclusively be "string" making null invalid. We
        // intentionally handle instance types of ["string", "null"] prior to
        // this case and strip out the null in both enum values and instance
        // type. Nevertheless, we do our best to interpret even incorrect
        // JSON schema.

        let mut has_null = false;

        let validator = StringValidator::new(&type_name, validation)?;

        let variants = enum_values
            .iter()
            .flat_map(|value| match value {
                // It would be odd to have multiple null values, but we don't
                // need to worry about it.
                serde_json::Value::Null => {
                    has_null = true;
                    None
                }
                serde_json::Value::String(value) if validator.is_valid(value) => {
                    let (name, rename) = recase(value, Case::Pascal);
                    Some(Ok(Variant {
                        name,
                        rename,
                        description: None,
                        details: VariantDetails::Simple,
                    }))
                }

                // Ignore enum variants whose strings don't match the given
                // constraints. If we wanted to get fancy we could include
                // these variants in the enum but exclude them from the FromStr
                // conversion... but that seems like unnecessary swag.
                serde_json::Value::String(_) => None,

                _ => Some(Err(Error::BadValue("string".to_string(), value.clone()))),
            })
            .collect::<Result<Vec<Variant>>>()?;

        if variants.is_empty() {
            if has_null {
                self.convert_null(metadata)
            } else {
                Err(Error::InvalidSchema {
                    type_name: type_name.into_option(),
                    reason: "empty enum array".to_string(),
                })
            }
        } else {
            let mut ty = TypeEntryEnum::from_metadata(
                self,
                type_name,
                metadata,
                EnumTagType::External,
                variants,
                false,
                original_schema.clone(),
            );

            if has_null {
                ty = self.type_to_option(ty);
            }

            Ok((ty, metadata))
        }
    }

    fn convert_integer<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        validation: &Option<Box<schemars::schema::NumberValidation>>,
        format: &Option<String>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let (mut min, mut max, multiple) = if let Some(validation) = validation {
            let min = match (&validation.minimum, &validation.exclusive_minimum) {
                (None, None) => None,
                (None, Some(value)) => Some(value + 1.0),
                (Some(value), None) => Some(*value),
                (Some(min), Some(emin)) => Some(min.max(emin + 1.0)),
            };
            let max = match (&validation.maximum, &validation.exclusive_maximum) {
                (None, None) => None,
                (None, Some(value)) => Some(value - 1.0),
                (Some(value), None) => Some(*value),
                (Some(max), Some(emax)) => Some(max.min(emax - 1.0)),
            };
            (min, max, validation.multiple_of)
        } else {
            (None, None, None)
        };

        // Ordered from most- to least-restrictive.
        let formats: &[(&str, &str, f64, f64)] = &[
            ("int8", "i8", i8::MIN as f64, i8::MAX as f64),
            ("", "std::num::NonZeroU8", 1.0, u8::MAX as f64),
            ("uint8", "u8", u8::MIN as f64, u8::MAX as f64),
            ("int16", "i16", i16::MIN as f64, i16::MAX as f64),
            ("", "std::num::NonZeroU16", 1.0, u16::MAX as f64),
            ("uint16", "u16", u16::MIN as f64, u16::MAX as f64),
            ("int", "i32", i32::MIN as f64, i32::MAX as f64),
            ("int32", "i32", i32::MIN as f64, i32::MAX as f64),
            ("", "std::num::NonZeroU32", 1.0, u32::MAX as f64),
            ("uint", "u32", u32::MIN as f64, u32::MAX as f64),
            ("uint32", "u32", u32::MIN as f64, u32::MAX as f64),
            // TODO all these are wrong as casting to an f64 loses precision.
            // However, schemars stores everything as an f64 so... meh for now.
            ("int64", "i64", i64::MIN as f64, i64::MAX as f64),
            ("", "std::num::NonZeroU64", 1.0, u64::MAX as f64),
            ("uint64", "u64", u64::MIN as f64, u64::MAX as f64),
        ];

        if let Some(format) = format {
            if let Some((_, ty, imin, imax)) = formats
                .iter()
                .find(|(int_format, _, _, _)| int_format == format)
            {
                // If the type matches with other constraints, we're done.
                if multiple.is_none()
                    && (min.is_none() || min == Some(*imin))
                    && (max.is_none() || max == Some(*imax))
                {
                    // If there's a default value and it's either not a number
                    // or outside of the range for this format, return an
                    // error.
                    if let Some(default) = metadata
                        .as_ref()
                        .and_then(|m| m.default.as_ref())
                        .and_then(|v| v.as_f64())
                    {
                        if default < *imin || default > *imax {
                            return Err(Error::InvalidValue);
                        }
                    }
                    return Ok((TypeEntry::new_integer(ty), metadata));
                }

                if min.is_none() {
                    min = Some(*imin);
                }
                if max.is_none() {
                    max = Some(*imax);
                }
            }
        }

        // We check the default value here since we have the min and max
        // close at hand.
        if let Some(default) = metadata.as_ref().and_then(|m| m.default.as_ref()) {
            // TODO it's imprecise (in every sense of the word) to use an
            // f64 here, but we're already constrained by the schemars
            // representation so ... it's probably the best we can do at
            // the moment.
            //
            // I added this because numbers are sometimes specified in double quotes
            let d = match default {
                serde_json::Value::Number(a) => a.as_f64(),
                serde_json::Value::String(a) => a.parse().ok(),
                _ => None,
            };
            match (d, min, max) {
                (Some(_), None, None) => Some(()),
                (Some(value), None, Some(fmax)) if value <= fmax => Some(()),
                (Some(value), Some(fmin), None) if value >= fmin => Some(()),
                (Some(value), Some(fmin), Some(fmax)) if value >= fmin && value <= fmax => Some(()),
                _ => None,
            }
            .ok_or(Error::InvalidValue)?;
        }

        // See if the value bounds fit within a known type.
        let maybe_type = match (min, max) {
            (None, Some(max)) => formats.iter().rev().find_map(|(_, ty, _, imax)| {
                if (imax - max).abs() <= f64::EPSILON {
                    Some(ty.to_string())
                } else {
                    None
                }
            }),
            (Some(min), None) => formats.iter().rev().find_map(|(_, ty, imin, _)| {
                if (imin - min).abs() <= f64::EPSILON {
                    Some(ty.to_string())
                } else {
                    None
                }
            }),
            (Some(min), Some(max)) => formats.iter().rev().find_map(|(_, ty, imin, imax)| {
                if (imax - max).abs() <= f64::EPSILON && (imin - min).abs() <= f64::EPSILON {
                    Some(ty.to_string())
                } else {
                    None
                }
            }),
            (None, None) => None,
        };

        // TODO we should do something with `multiple`
        if let Some(ty) = maybe_type {
            Ok((TypeEntry::new_integer(ty), metadata))
        } else {
            // TODO we could construct a type that itself enforces the various
            // bounds.
            // TODO failing that, we should find the type that most tightly
            // matches these bounds.
            Ok((TypeEntry::new_integer("i64"), metadata))
        }
    }

    // TODO deal with metadata
    fn convert_number<'a>(
        &self,
        _metadata: &'a Option<Box<Metadata>>,
        _validation: &Option<Box<schemars::schema::NumberValidation>>,
        format: &Option<String>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        /*
        See https://github.com/oxidecomputer/typify/issues/169
        if let Some(validation) = validation {
            assert!(validation.multiple_of.is_none());
            assert!(validation.maximum.is_none());
            assert!(validation.exclusive_maximum.is_none());
            assert!(validation.minimum.is_none());
            assert!(validation.exclusive_minimum.is_none());
        }
        */

        match format.as_deref() {
            Some("float") => Ok((TypeEntry::new_float("f32"), &None)),
            _ => Ok((TypeEntry::new_float("f64"), &None)),
        }
    }

    /// If we have a schema that's just the Null instance type, it represents a
    /// solitary value so we model that with the unit type.
    fn convert_null<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((TypeEntryDetails::Unit.into(), metadata))
    }

    /// Determine whether a schema's property name validation constraints can be handled
    fn can_handle_pattern_properties(validation: &ObjectValidation) -> bool {
        if !validation.required.is_empty() {
            return false;
        }

        if !validation.properties.is_empty() {
            return false;
        }

        // Ensure we have at least one pattern property and all pattern property
        // schemas are the same
        let Some(first_schema) = validation.pattern_properties.values().next() else {
            return false;
        };

        if !validation
            .pattern_properties
            .values()
            .all(|schema| schema == first_schema)
        {
            return false;
        }

        // Ensure any additional properties are a false or null schema
        if validation.additional_properties.as_ref().map(AsRef::as_ref) == Some(&Schema::Bool(true))
            || matches!(
                validation.additional_properties.as_ref().map(AsRef::as_ref),
                Some(&Schema::Object(_))
            )
        {
            return false;
        }

        // Ensure there are no additional property names constraints, to avoid a
        // collision between different types of constraints interacting unexpectedly
        if validation.property_names.is_some() {
            return false;
        }

        true
    }

    fn convert_object<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        validation: &Option<Box<ObjectValidation>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match validation.as_ref().map(Box::as_ref) {
            // Maps have an empty properties set, and a non-null schema for the
            // additional_properties field.
            Some(ObjectValidation {
                max_properties: _,
                min_properties: _,
                required,
                properties,
                pattern_properties,
                additional_properties,
                property_names,
            }) if required.is_empty()
                && properties.is_empty()
                && pattern_properties.is_empty()
                && additional_properties.as_ref().map(AsRef::as_ref)
                    != Some(&Schema::Bool(false)) =>
            {
                let type_entry = self.make_map(
                    type_name.into_option(),
                    property_names,
                    additional_properties,
                )?;
                Ok((type_entry, metadata))
            }

            Some(validation) if Self::can_handle_pattern_properties(validation) => {
                let pattern = validation
                    .pattern_properties
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("|");

                // Construct a schema to use for property name validation
                let property_names = Some(Box::new(Schema::Object(SchemaObject {
                    string: Some(Box::new(StringValidation {
                        max_length: None,
                        min_length: None,
                        pattern: Some(pattern),
                    })),
                    ..Default::default()
                })));

                // Construct schema to use for property value validation
                let additional_properties = Some(Box::new(
                    validation
                        .pattern_properties
                        .values()
                        .next()
                        .cloned()
                        .unwrap_or_else(|| unreachable!("pattern_properties cannot be empty here")),
                ));

                let type_entry = self.make_map(
                    type_name.into_option(),
                    &property_names,
                    &additional_properties,
                )?;

                Ok((type_entry, metadata))
            }

            None => {
                let type_entry = self.make_map(type_name.into_option(), &None, &None)?;
                Ok((type_entry, metadata))
            }

            // The typical case
            Some(validation) => {
                let tmp_type_name = get_type_name(&type_name, metadata);
                let (properties, deny_unknown_fields) =
                    self.struct_members(tmp_type_name, validation)?;

                Ok((
                    TypeEntryStruct::from_metadata(
                        self,
                        type_name,
                        metadata,
                        properties,
                        deny_unknown_fields,
                        original_schema.clone(),
                    ),
                    &None,
                ))
            }
        }
    }

    fn convert_reference<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        ref_name: &str,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let key = ref_key(ref_name);
        let type_id = self
            .ref_to_id
            .get(&key)
            .unwrap_or_else(|| panic!("$ref {} is missing", ref_name));
        Ok((
            TypeEntryDetails::Reference(type_id.clone()).into(),
            metadata,
        ))
    }

    fn convert_all_of<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        debug!(
            "all_of {}",
            serde_json::to_string_pretty(subschemas).unwrap()
        );
        if let Some(ty) =
            self.maybe_singleton_subschema(type_name.clone(), original_schema, subschemas)
        {
            return Ok((ty, metadata));
        }

        // In the general case, we merge all schemas in the array. The merged
        // schema will reflect all definitions and constraints, the
        // intersection of all schemas. For example, it will have the union of
        // all properties for an object, recursively merging properties defined
        // in multiple schemas; it will have the union of all required object
        // properties; and it will enforce the greater of all numeric
        // constraints (e.g. the greater of all specified minimum values).
        //
        // Sometimes merging types will produce a result for which no data is
        // valid, the schema is unsatisfiable. Consider this trivial case:
        //
        // {
        //   "allOf": [
        //     { "type": "integer", "minimum": 5 },
        //     { "type": "integer", "maximum": 3 }
        //   ]
        // }
        //
        // No number is >= 5 *and* <= 3! Good luck with that schema! Similarly
        // for this case:
        //
        // {
        //   "allOf": [
        //     { "type": "string" },
        //     { "type": "object" }
        //   ]
        // }
        //
        // A value cannot be both a string and an object!
        //
        // Note that we will effectively "embed" any referenced types. Consider
        // a construction like this:
        //
        // "allOf": [
        //     { "$ref": "#/definitions/SuperClass" },
        //     { "type": "object", "properties": { "another_prop: {} }}
        // ]
        //
        // The resulting merged schema will include all properties of
        // "SuperClass" as well as "another_prop" (which we assume for this
        // example to not have been present in the original). This is
        // suboptimal in that we would like the generated types to reflect some
        // association with the original sub-type.
        //
        // TODO
        // In cases where we have a named type, we would like to provide
        // conversion methods to extract the named type from the merged type.
        // In the "SuperClass" example above, we would provide an
        // Into<SuperClass> implementation to discard the additional properties
        // and produce an instance of "SuperClass".
        //
        // We can do something similar for types that become additionally
        // constrained: a field that becomes required can be converted to an
        // optional field; a number whose value is limited can be converted to
        // the more expansive numeric type.

        let merged_schema = merge_all(subschemas, &self.definitions);
        if let Schema::Bool(false) = &merged_schema {
            self.convert_never(type_name, original_schema)
        } else {
            let mut merged_schema = merged_schema.into_object();
            assert!(merged_schema.metadata.is_none());
            merged_schema.metadata = metadata.clone();

            let (type_entry, _) =
                self.convert_schema_object(type_name, original_schema, &merged_schema)?;
            Ok((type_entry, &None))
        }
    }

    fn convert_any_of<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &'a [Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        // Rust can emit "anyOf":[{"$ref":"#/definitions/C"},{"type":"null"}
        // for Option. We match this here because the mutual exclusion check
        // below may fail for cases such as Option<T> where T is defined to be,
        // say, (). In such a case, both variants are actually null.
        if let Some(ty) = self.maybe_option(type_name.clone(), metadata, subschemas) {
            return Ok((ty, metadata));
        }

        // Check if this could be more precisely handled as a "one-of". This
        // occurs if each subschema is mutually exclusive i.e. so that exactly
        // one of them can match.
        if all_mutually_exclusive(subschemas, &self.definitions) {
            self.convert_one_of(type_name, original_schema, metadata, subschemas)
        } else {
            // We'll want to build a struct that looks like this:
            // struct Name {
            //     #[serde(flatten)]
            //     schema1: Option<Schema1Type>,
            //     #[serde(flatten)]
            //     schema2: Option<Schema2Type>,
            //     ...
            // }

            self.flattened_union_struct(type_name, original_schema, metadata, subschemas, true)
        }
    }

    /// A "one of" may reasonably be converted into a Rust enum, but there are
    /// several cases to consider:
    ///
    /// Options expressed as enums are uncommon since { "type": [ "null",
    /// "<type>"], ... } is a much simpler construction. Nevertheless, an
    /// option may be expressed as a "one of" with two subschemas where one is
    /// null.
    ///
    /// Externally tagged enums are comprised of either an enumerated set of
    /// string values or objects that have a single required member. The
    /// variant is either the enumerated value with no data or the required
    /// member with its type as the associated data. Note that this is the
    /// serde default.
    ///
    /// Internally tagged enums are comprised exclusively of objects where each
    /// object has a required property in common and this required property
    /// must be a string with a single fixed value. The property becomes the
    /// serde tag and the value becomes the variant. Any additional properties
    /// on that object become the data associated with the given variant.
    ///
    /// Adjacently tagged enums are comprised exclusively of objects that have
    /// a tag and content field in common (though the content field will only
    /// be present for variants that include data). The value of the tag
    /// should, as above, be a string with a single enumerated value. The value
    /// of the content field, if it exists, becomes the data payload for the
    /// variant.
    ///
    /// Untagged enums intentionally omit a named tag. There are no constraints
    /// on untagged enums so this is our fallback if the tagging schemes above
    /// don't apply. While untagged enums are not always strictly exclusive by
    /// construction, we know that *these* variants must be mutually exclusive
    /// if we've ended up here. Note that untagged variants are distinguished
    /// by their data, so a single variant may exist with no associated data,
    /// but we'd expect that variant to be null or an empty struct. This case
    /// requires us to invent variant names since that information is not
    /// included in the schema data.
    ///
    /// Note that the order of checking for tagging schemes must be carefully
    /// considered. Adjacent tagging must be checked before internal tagging as
    /// the former is a subset of the latter: the content field could be
    /// interpreted as a struct variant with a single field:
    ///
    /// ```ignore
    /// enum MyEnum {
    ///     Variant1 { content: MyObj },
    ///     Variant2 { content: MyObj },
    /// }
    /// ```
    ///
    /// Fortunately, external tagging can't be confused with internal or
    /// adjacent tagging except in reductive cases such as enums with a single
    /// variant.
    ///
    /// Untagged enums apply to any set of subschemas so must be applied last.
    pub(crate) fn convert_one_of<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<schemars::schema::Metadata>>,
        subschemas: &'a [Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        debug!(
            "one_of {}",
            serde_json::to_string_pretty(subschemas).unwrap()
        );

        // TODO it would probably be smart to do a pass through the schema
        // given to us and either put it into some canonical form or move to
        // some sort of intermediate representation.
        //
        // Each of the enum types does some similar exploration of each
        // variant-schema--it should be possible to do that once. In addition
        // the various enum types rely on some heuristics around how schemas
        // are laid out; it would be nice to eliminate some of the guesswork,
        // but putting schemas into a predictable form.

        let ty = self
            .maybe_option(type_name.clone(), metadata, subschemas)
            .or_else(|| {
                self.maybe_externally_tagged_enum(
                    type_name.clone(),
                    original_schema,
                    metadata,
                    subschemas,
                )
            })
            .or_else(|| {
                self.maybe_adjacently_tagged_enum(
                    type_name.clone(),
                    original_schema,
                    metadata,
                    subschemas,
                )
            })
            .or_else(|| {
                self.maybe_internally_tagged_enum(
                    type_name.clone(),
                    original_schema,
                    metadata,
                    subschemas,
                )
            })
            .or_else(|| {
                self.maybe_singleton_subschema(type_name.clone(), original_schema, subschemas)
            })
            .map_or_else(
                || self.untagged_enum(type_name, original_schema, metadata, subschemas),
                Ok,
            )?;

        Ok((ty, metadata))
    }

    /// The "not" construction is pretty challenging to handle in the general
    /// case: what is the appropriate rust structure for a type that is merely
    /// the exclusion of another? This is tractable, however, in some special
    /// cases that occur frequently enough in the wild to consider them.
    ///
    /// The simplest is for the boolean schemas: true to accept everything;
    /// false to accept nothing. These we can simply invert. Why someone would
    /// specify a type in this fashion... hard to say.
    ///
    /// The next we consider is that of enumerated values: if the schema
    /// explicitly enumerates its valid values, we can construct a type that
    /// disallows those values (just as we have a type that may only be one of
    /// several specific values). We either use the specified type (e.g.
    /// string) or infer the type from the enumerated values. These are
    /// represented as a newtype that contains a deny list (rather than an
    /// allow list as is the case for non-string enumerated values).
    pub(crate) fn convert_not<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<schemars::schema::Metadata>>,
        subschema: &'a Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match subschema {
            // This is a weird construct, but simple enough to handle.
            Schema::Bool(b) => {
                let (type_entry, _) = self.convert_schema(type_name, &Schema::Bool(!b))?;
                Ok((type_entry, metadata))
            }

            // An explicit type and enumerated values.
            Schema::Object(
                schema @ SchemaObject {
                    instance_type: Some(SingleOrVec::Single(_)),
                    enum_values: Some(enum_values),
                    ..
                },
            ) => {
                let type_schema = SchemaObject {
                    enum_values: None,
                    ..schema.clone()
                };

                let (type_entry, _) =
                    self.convert_schema_object(Name::Unknown, original_schema, &type_schema)?;

                // Make sure all the values are valid.
                // TODO this isn't strictly legal since we may not yet have
                // resolved references.
                enum_values
                    .iter()
                    .try_for_each(|value| type_entry.validate_value(self, value).map(|_| ()))?;

                let type_id = self.assign_type(type_entry);

                let newtype_entry = TypeEntryNewtype::from_metadata_with_deny_values(
                    self,
                    type_name,
                    metadata,
                    type_id,
                    enum_values,
                    original_schema.clone(),
                );

                Ok((newtype_entry, metadata))
            }

            // No type so we infer it from the values.
            Schema::Object(SchemaObject {
                metadata,
                instance_type: None,
                format: None,
                enum_values: Some(enum_values),
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            }) => {
                // All the values need to be of the same type.
                let instance_types = enum_values
                    .iter()
                    .map(|v| match v {
                        serde_json::Value::Bool(_) => InstanceType::Boolean,
                        serde_json::Value::Number(_) => InstanceType::Number,
                        serde_json::Value::String(_) => InstanceType::String,

                        serde_json::Value::Null
                        | serde_json::Value::Array(_)
                        | serde_json::Value::Object(_) => {
                            panic!("unhandled type for `not` construction: {}", v)
                        }
                    })
                    .collect::<BTreeSet<_>>();

                match (instance_types.len(), instance_types.iter().next()) {
                    (1, Some(instance_type)) => {
                        let typed_schema = SchemaObject {
                            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                                *instance_type,
                            ))),
                            ..Default::default()
                        };

                        let (type_entry, _) = self.convert_schema_object(
                            Name::Unknown,
                            original_schema,
                            &typed_schema,
                        )?;
                        // Make sure all the values are valid.
                        // TODO this isn't strictly legal since we may not yet
                        // have resolved references.
                        enum_values.iter().try_for_each(|value| {
                            type_entry.validate_value(self, value).map(|_| ())
                        })?;

                        let type_id = self.assign_type(type_entry);

                        let newtype_entry = TypeEntryNewtype::from_metadata_with_deny_values(
                            self,
                            type_name,
                            metadata,
                            type_id,
                            enum_values,
                            original_schema.clone(),
                        );

                        Ok((newtype_entry, metadata))
                    }

                    _ => panic!(
                        "multiple implied types for an un-typed enum {:?} {:?}",
                        instance_types, enum_values,
                    ),
                }
            }

            _ => todo!("unhandled not schema {:#?}", subschema),
        }
    }

    fn convert_array<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        validation: &ArrayValidation,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match validation {
            // Tuples and fixed-length arrays satisfy the condition that the
            // max and min lengths are equal (and greater than zero). When
            // the `item` is an array, we produce a tuple; when it is a single
            // element, we produce a fixed-length array.
            ArrayValidation {
                items,
                additional_items,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
                contains: None,
            } if max_items == min_items && *max_items > 0 => match items {
                // Tuple with fewer types specified than required items.
                Some(SingleOrVec::Vec(items)) if items.len() < *max_items as usize => {
                    let rest_name = type_name.append("additional");
                    let rest_id = if let Some(rest_schema) = additional_items {
                        self.id_for_schema(rest_name, rest_schema)?.0
                    } else {
                        self.id_for_schema(rest_name, &Schema::Bool(true))?.0
                    };
                    let start = items.iter().enumerate().map(|(ii, item_schema)| {
                        let item_name = type_name.append(&format!("item{}", ii));
                        Ok(self.id_for_schema(item_name, item_schema)?.0)
                    });
                    let rest = (items.len()..*max_items as usize).map(|_| Ok(rest_id.clone()));
                    let types = start.chain(rest).collect::<Result<Vec<_>>>()?;
                    Ok((TypeEntryDetails::Tuple(types).into(), metadata))
                }
                // Tuple with at least as many items as required.
                Some(SingleOrVec::Vec(items)) => {
                    let types = items
                        .iter()
                        .take(*max_items as usize)
                        .enumerate()
                        .map(|(ii, item_schema)| {
                            let item_name = type_name.append(&format!("item{}", ii));
                            Ok(self.id_for_schema(item_name, item_schema)?.0)
                        })
                        .collect::<Result<_>>()?;
                    Ok((TypeEntryDetails::Tuple(types).into(), metadata))
                }

                // Array with a schema for the item.
                Some(SingleOrVec::Single(item_schema)) => {
                    let item_id = self.id_for_schema(type_name.append("item"), item_schema)?.0;
                    Ok((
                        TypeEntryDetails::Array(item_id, *max_items as usize).into(),
                        metadata,
                    ))
                }
                // Array with no schema for the item.
                None => {
                    let any_id = self
                        .id_for_schema(type_name.append("item"), &Schema::Bool(true))?
                        .0;
                    Ok((
                        TypeEntryDetails::Array(any_id, *max_items as usize).into(),
                        metadata,
                    ))
                }
            },

            // Arrays and sets.
            ArrayValidation {
                items: Some(SingleOrVec::Single(item)),
                additional_items: _, // By spec: ignored for single items
                max_items: _,        // TODO enforce size limitations
                min_items: _,        // TODO enforce size limitations
                unique_items,
                contains: None,
            } => {
                let item_type_name = match get_type_name(&type_name, metadata) {
                    Some(s) => Name::Suggested(format!("{}Item", s)),
                    None => Name::Unknown,
                };
                let (type_id, _) = self.id_for_schema(item_type_name, item.as_ref())?;

                // If items are unique, this is a Set; otherwise it's an Array.
                match unique_items {
                    Some(true) => Ok((TypeEntryDetails::Set(type_id).into(), metadata)),
                    _ => Ok((TypeEntryDetails::Vec(type_id).into(), metadata)),
                }
            }

            // Arrays and sets with no specified items.
            ArrayValidation {
                items: None,
                additional_items: _, // By spec: ignored for missing items
                max_items: _,        // TODO enforce size limitations
                min_items: _,        // TODO enforce size limitations
                unique_items,
                contains: None,
            } => {
                self.uses_serde_json = true;
                let type_id = self.assign_type(TypeEntryDetails::JsonValue.into());

                // If items are unique, this is a Set; otherwise it's an Array.
                match unique_items {
                    Some(true) => Ok((TypeEntryDetails::Set(type_id).into(), metadata)),
                    _ => Ok((TypeEntryDetails::Vec(type_id).into(), metadata)),
                }
            }

            _ => Err(Error::InvalidSchema {
                type_name: type_name.into_option(),
                reason: format!("unhandled array validation {:#?}", validation),
            }),
        }
    }

    fn convert_array_of_any<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        self.uses_serde_json = true;
        let type_id = self.assign_type(TypeEntryDetails::JsonValue.into());
        Ok((TypeEntryDetails::Vec(type_id).into(), metadata))
    }

    // TODO not sure if I want to deal with enum_values here, but we'll see...
    fn convert_bool<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((TypeEntry::new_boolean(), metadata))
    }

    fn convert_permissive<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        self.uses_serde_json = true;
        Ok((TypeEntryDetails::JsonValue.into(), metadata))
    }

    fn convert_never<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let ty = TypeEntryEnum::from_metadata(
            self,
            type_name,
            &None,
            EnumTagType::External,
            vec![],
            true,
            schema.clone(),
        );
        Ok((ty, &None))
    }

    fn convert_typed_enum<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        schema: &'a SchemaObject,
        enum_values: &[serde_json::Value],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let type_schema = SchemaObject {
            enum_values: None,
            ..schema.clone()
        };

        let inner_type_name = match get_type_name(&type_name, &schema.metadata) {
            Some(s) => Name::Suggested(format!("{}Inner", s)),
            None => Name::Unknown,
        };

        let (type_entry, metadata) =
            self.convert_schema_object(inner_type_name, original_schema, &type_schema)?;

        // Make sure all the values are valid.
        enum_values
            .iter()
            .try_for_each(|value| type_entry.validate_value(self, value).map(|_| ()))?;

        let type_id = self.assign_type(type_entry);

        let newtype_entry = TypeEntryNewtype::from_metadata_with_enum_values(
            self,
            type_name,
            metadata,
            type_id,
            enum_values,
            original_schema.clone(),
        );

        Ok((
            newtype_entry,
            if metadata.is_some() {
                &schema.metadata
            } else {
                &None
            },
        ))
    }

    fn convert_unknown_enum<'a>(
        &mut self,
        type_name: Name,
        original_schema: &'a Schema,
        metadata: &'a Option<Box<Metadata>>,
        enum_values: &[serde_json::Value],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        // We're here because the schema didn't have a type; that's a bummer,
        // but we'll do our best to roll with the punches.
        assert!(!enum_values.is_empty());

        // Let's hope all these values are the same type.
        let mut instance_types = enum_values
            .iter()
            .map(|v| match v {
                serde_json::Value::Null => InstanceType::Null,
                serde_json::Value::Bool(_) => InstanceType::Boolean,
                serde_json::Value::Number(_) => InstanceType::Number,
                serde_json::Value::String(_) => InstanceType::String,
                serde_json::Value::Array(_) => InstanceType::Array,
                serde_json::Value::Object(_) => InstanceType::Object,
            })
            .collect::<BTreeSet<_>>();

        let has_null = instance_types.remove(&InstanceType::Null);

        if has_null {
            // If there's a null-value, recur with the null value removed; then
            // convert the resulting type to be optional.
            let enum_values = enum_values
                .iter()
                .filter(|v| !v.is_null())
                .cloned()
                .collect::<Vec<_>>();

            let (type_entry, metadata) =
                self.convert_unknown_enum(type_name, original_schema, metadata, &enum_values)?;
            let type_entry = self.type_to_option(type_entry);
            Ok((type_entry, metadata))
        } else {
            match (instance_types.len(), instance_types.iter().next()) {
                (1, Some(InstanceType::String)) => self.convert_enum_string(
                    type_name,
                    original_schema,
                    metadata,
                    enum_values,
                    None,
                ),

                // TODO We're ignoring enumerated values for the boolean
                // type--at least for the moment--because some of the tests
                // show that this may require more careful consideration.
                (1, Some(InstanceType::Boolean)) => self.convert_bool(metadata),

                (1, Some(instance_type)) => {
                    let typed_schema = SchemaObject {
                        instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                            *instance_type,
                        ))),
                        ..Default::default()
                    };
                    let (type_entry, new_metadata) = self.convert_typed_enum(
                        type_name,
                        original_schema,
                        &typed_schema,
                        enum_values,
                    )?;
                    Ok((
                        type_entry,
                        if new_metadata.is_some() {
                            metadata
                        } else {
                            &None
                        },
                    ))
                }
                (1, None) => unreachable!(),
                _ => panic!(
                    "multiple implied types for an un-typed enum {:?} {:?}",
                    instance_types, enum_values,
                ),
            }
        }
    }

    pub(crate) fn convert_option<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        schema: &'_ Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let (ty, _) = self.convert_schema(type_name, schema)?;
        let ty = self.type_to_option(ty);

        Ok((ty, metadata))
    }

    /// We'll often see this if the subschema was just to provide an additional
    /// level for annotation such as a "title" or "description".
    pub(crate) fn maybe_singleton_subschema(
        &mut self,
        type_name: Name,
        _original_schema: &Schema,
        subschemas: &[Schema],
    ) -> Option<TypeEntry> {
        match (subschemas.len(), subschemas.first()) {
            (1, Some(subschema)) => Some(self.convert_schema(type_name, subschema).ok()?.0),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

    use paste::paste;
    use quote::{quote, ToTokens};
    use schema::Schema;
    use schemars::{
        schema::{InstanceType, Metadata, NumberValidation, RootSchema, SchemaObject},
        schema_for, JsonSchema,
    };
    use serde_json::json;

    use crate::{
        test_util::validate_output, validate_builtin, Error, Name, TypeSpace, TypeSpaceImpl,
        TypeSpaceSettings,
    };

    #[track_caller]
    fn int_helper<T: JsonSchema>(type_name: &'static str) {
        let schema = schema_for!(T);

        let mut type_space = TypeSpace::default();
        type_space
            .add_ref_types(schema.definitions.clone())
            .unwrap();
        let (ty, _) = type_space
            .convert_schema_object(
                Name::Unknown,
                &schemars::schema::Schema::Object(schema.schema.clone()),
                &schema.schema,
            )
            .unwrap();
        let output = ty.type_name(&type_space);
        let actual = output.split("::").last().unwrap().trim();
        let expected = type_name.split("::").last().unwrap();
        assert_eq!(actual, expected);
    }

    macro_rules! int_test {
        ($t:ty) => {
            paste! {
                #[test]
                fn [<test_int_ $t:lower>]() {
                    int_helper::<$t>(stringify!($t))
                }
            }
        };
    }

    int_test!(u8);
    int_test!(u16);
    int_test!(u32);
    int_test!(u64);
    int_test!(i8);
    int_test!(i16);
    int_test!(i32);
    int_test!(i64);
    int_test!(NonZeroU8);
    int_test!(NonZeroU16);
    int_test!(NonZeroU32);
    int_test!(NonZeroU64);

    #[test]
    fn test_redundant_types() {
        #[derive(JsonSchema)]
        #[allow(dead_code)]
        struct Alphabet {
            a: u32,
            b: u32,
            c: u32,
            d: Option<u32>,
            e: Option<u32>,
            f: (u32, u32, u32, Option<u32>),
        }

        let schema = schema_for!(Alphabet);

        let mut type_space = TypeSpace::default();
        type_space
            .add_ref_types(schema.definitions.clone())
            .unwrap();
        let _ = type_space
            .add_type_with_name(&schema.schema.into(), Some("Alphabet".to_string()))
            .unwrap();

        // We expect a total of 4 types:
        // 1. u32
        // 2. option -> 1
        // 3. tuple -> 1, 1, 1, 2
        // 4. struct -> 1, 1, 1, 2, 2, 3
        assert_eq!(type_space.iter_types().count(), 4);
    }

    #[test]
    fn test_basic_option_flat() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct C {}

        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A {
            a: Option<C>,
        }

        validate_output::<A>();
    }

    #[test]
    fn test_unit_option() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct Foo;

        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct Bar {
            a: Option<Foo>,
        }

        validate_output::<Bar>();
    }

    // TODO we can turn this on once we generate proper sets.
    #[ignore]
    #[test]
    fn test_set() {
        validate_builtin!(std::collections::BTreeSet<u32>);
    }

    #[test]
    fn test_low_default() {
        let schema = SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            format: Some("uint".to_string()),
            metadata: Some(
                Metadata {
                    default: Some(json!(-1i32)),
                    ..Default::default()
                }
                .into(),
            ),
            number: Some(NumberValidation::default().into()),
            ..Default::default()
        };

        let mut type_space = TypeSpace::default();
        match type_space.convert_schema_object(
            Name::Unknown,
            &schemars::schema::Schema::Object(schema.clone()),
            &schema,
        ) {
            Err(Error::InvalidValue) => (),
            _ => panic!("unexpected result"),
        }
    }

    #[test]
    fn test_high_default() {
        let schema = SchemaObject {
            instance_type: Some(InstanceType::Integer.into()),
            metadata: Some(
                Metadata {
                    default: Some(json!(867_5309_u32)),
                    ..Default::default()
                }
                .into(),
            ),
            number: Some(
                NumberValidation {
                    maximum: Some(256.0),
                    ..Default::default()
                }
                .into(),
            ),
            ..Default::default()
        };

        let mut type_space = TypeSpace::default();
        match type_space.convert_schema_object(
            Name::Unknown,
            &schemars::schema::Schema::Object(schema.clone()),
            &schema,
        ) {
            Err(Error::InvalidValue) => (),
            _ => panic!("unexpected result"),
        }
    }

    #[test]
    fn test_null() {
        let schema_json = r#"
        {
            "title": "Null",
            "type": "string",
            "enum": [null]
        }
        "#;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::default();
        let _ = type_space.add_type(&schema.schema.into()).unwrap();

        let actual = type_space.to_stream();
        let file = syn::parse2::<syn::File>(actual).expect("type space should emit a valid file");
        match file.items.as_slice() {
            [syn::Item::Mod(error)] if error.ident == "error" => {}
            _ => panic!("unexpected file contents {}", file.to_token_stream()),
        }
    }

    #[test]
    fn test_overridden_conversion() {
        let schema_json = r#"
        {
            "description": "don't let this fool you",
            "type": "string",
            "format": "uuid"
        }
        "#;

        let schema: RootSchema = serde_json::from_str(schema_json).unwrap();

        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_conversion(
            SchemaObject {
                instance_type: Some(InstanceType::String.into()),
                format: Some("uuid".to_string()),
                ..Default::default()
            },
            "not::a::real::library::Uuid",
            [TypeSpaceImpl::Display].into_iter(),
        ));
        let type_id = type_space.add_type(&schema.schema.into()).unwrap();
        let typ = type_space.get_type(&type_id).unwrap();

        let actual = typ.ident();
        let expected = quote! { not::a::real::library::Uuid };
        assert_eq!(actual.to_string(), expected.to_string());
    }
}
