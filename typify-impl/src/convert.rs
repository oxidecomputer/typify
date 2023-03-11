// Copyright 2022 Oxide Computer Company

use std::collections::HashSet;

use crate::type_entry::{
    EnumTagType, TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct,
    Variant, VariantDetails,
};
use crate::util::{all_mutually_exclusive, none_or_single, recase, ref_key, Case, StringValidator};
use log::info;
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
        match schema {
            Schema::Object(obj) => {
                if let Some(type_entry) = self.cache.lookup(obj) {
                    Ok((type_entry, &obj.metadata))
                } else {
                    self.convert_schema_object(type_name, obj)
                }
            }

            Schema::Bool(true) => self.convert_permissive(&None),
            // TODO Not sure what to do here... need to return something toxic?
            Schema::Bool(false) => todo!(),
        }
    }

    pub(crate) fn convert_schema_object<'a>(
        &mut self,
        type_name: Name,
        schema: &'a SchemaObject,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
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
                if let Some(other_type) = multiple.iter().find(|t| t != &&InstanceType::Null) {
                    // In the sensible case where only one of the instance
                    // types is null.
                    let enum_values = enum_values.clone().map(|values| {
                        values
                            .iter()
                            .cloned()
                            .filter(|value| !value.is_null())
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
                    self.convert_schema_object(type_name, &new_schema)
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
                number: None,
                string,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::String => self.convert_string(
                type_name,
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
                number: None,
                string,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::String => self.convert_enum_string(
                type_name,
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
                string: None,
                array: None,
                object: None,
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
                string: None,
                array: None,
                object: None,
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
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Boolean => self.convert_bool(metadata),

            // Structs
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
                format: None,
                enum_values: None,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: validation,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Object => {
                self.convert_object(type_name, metadata, validation)
            }

            // Structs with the type omitted, but validation present
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
            } => self.convert_object(type_name, metadata, validation),

            // Arrays
            SchemaObject {
                metadata,
                instance_type,
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
            } if none_or_single(instance_type, &InstanceType::Array) => {
                self.convert_array(type_name, metadata, validation)
            }

            // Arrays of anything
            SchemaObject {
                metadata,
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
            } => self.convert_reference(metadata, reference),

            // Accept references that... for some reason... include the type.
            // TODO this could be generalized to validate any redundant
            // validation here or could be used to compute a new, more
            // constrained type.
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
                let ref_schema = self.definitions.get(ref_key(reference)).unwrap();
                assert!(matches!(ref_schema, Schema::Object(SchemaObject {
                        instance_type: it, ..
                    }) if instance_type == it));

                self.convert_reference(metadata, reference)
            }

            // Enum of a single, known, non-String type (strings above).
            SchemaObject {
                instance_type: Some(SingleOrVec::Single(_)),
                enum_values: Some(enum_values),
                ..
            } => self.convert_typed_enum(type_name, schema, enum_values),

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
            } => self.convert_unknown_enum(type_name, metadata, enum_values),

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
                } => self.convert_all_of(type_name, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: Some(subschemas),
                    one_of: None,
                    not: None,
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_any_of(type_name, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: None,
                    one_of: Some(subschemas),
                    not: None,
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_one_of(type_name, metadata, subschemas),
                SubschemaValidation {
                    all_of: None,
                    any_of: None,
                    one_of: None,
                    not: Some(subschema),
                    if_schema: None,
                    then_schema: None,
                    else_schema: None,
                } => self.convert_not(type_name, metadata, subschema),

                // Unknown
                _ => todo!("{:#?}", subschemas),
            },

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
                self.convert_schema_object(type_name, &new_schema)
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
                    &SchemaObject {
                        instance_type: None,
                        ..schema.clone()
                    },
                )?;
                Ok((type_entry, metadata))
            }

            // If we have a simple type--complicated only by accepting multiple
            // types (none duplicated)--we can create an enum; note that the
            // case of a 2-type list with one of them Null is already handled
            // above (and rendered into an Option).
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Vec(instance_types)),
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
            } if instance_types.len() == instance_types.iter().collect::<HashSet<_>>().len() => {
                let variants = instance_types
                    .iter()
                    .map(|it| {
                        let (name, maybe_ty) = match it {
                            InstanceType::Null => ("Null", None),
                            InstanceType::Boolean => ("Boolean", Some(TypeEntry::new_boolean())),
                            InstanceType::Object => {
                                let (ty, _) = self.make_map(None, &None)?;
                                ("Object", Some(ty))
                            }
                            InstanceType::Array => {
                                let (ty, _) = self.convert_array_of_any(&None)?;
                                ("Array", Some(ty))
                            }
                            InstanceType::Number => ("Number", Some(TypeEntry::new_float("f64"))),
                            InstanceType::String => {
                                ("String", Some(TypeEntry::from(TypeEntryDetails::String)))
                            }
                            InstanceType::Integer => {
                                ("Integer", Some(TypeEntry::new_integer("i64")))
                            }
                        };
                        let details = match maybe_ty {
                            Some(ty) => VariantDetails::Item(self.assign_type(ty)),
                            None => VariantDetails::Simple,
                        };
                        Ok(Variant {
                            name: name.to_string(),
                            rename: None,
                            description: None,
                            details,
                        })
                    })
                    .collect::<Result<_>>()?;
                let type_entry = TypeEntryEnum::from_metadata(
                    self,
                    type_name,
                    metadata,
                    EnumTagType::Untagged,
                    variants,
                    false,
                );
                Ok((type_entry, metadata))
            }

            // Unknown
            SchemaObject { .. } => todo!("invalid (or unexpected) schema:\n{:#?}", schema),
        }
    }

    fn convert_string<'a>(
        &mut self,
        type_name: Name,
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
                        let _ = regress::Regex::new(pattern).map_err(|e| {
                            Error::InvalidSchema(format!("invalid pattern '{}' {}", pattern, e))
                        })?;
                        self.uses_regress = true;
                    }

                    let string = TypeEntryDetails::String.into();
                    let type_id = self.assign_type(string);
                    Ok((
                        TypeEntryNewtype::from_metadata_with_string_validation(
                            self, type_name, metadata, type_id, validation,
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
                        "chrono::Date<chrono::offset::Utc>",
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

        let validator = StringValidator::new(validation)?;

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
                Err(Error::InvalidSchema("empty enum array".to_string()))
            }
        } else {
            let mut ty = TypeEntryEnum::from_metadata(
                self,
                type_name,
                metadata,
                EnumTagType::External,
                variants,
                false,
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
            match (default.as_f64(), min, max) {
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
            (None, Some(max)) => formats.iter().find_map(|(_, ty, _, imax)| {
                if imax + f64::EPSILON >= max {
                    Some(ty.to_string())
                } else {
                    None
                }
            }),
            (Some(min), None) => formats.iter().find_map(|(_, ty, imin, _)| {
                if imin - f64::EPSILON <= min {
                    Some(ty.to_string())
                } else {
                    None
                }
            }),
            (Some(min), Some(max)) => formats.iter().find_map(|(_, ty, imin, imax)| {
                if imax + f64::EPSILON >= max && imin - f64::EPSILON <= min {
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

    // TODO deal with metadata and format
    fn convert_number<'a>(
        &self,
        _metadata: &'a Option<Box<Metadata>>,
        validation: &Option<Box<schemars::schema::NumberValidation>>,
        _format: &Option<String>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if let Some(validation) = validation {
            assert!(validation.multiple_of.is_none());
            assert!(validation.maximum.is_none());
            assert!(validation.exclusive_maximum.is_none());
            assert!(validation.minimum.is_none());
            assert!(validation.exclusive_minimum.is_none());
        }

        Ok((TypeEntry::new_float("f64"), &None))
    }

    /// If we have a schema that's just the Null instance type, it represents a
    /// solitary value so we model that with the unit type.
    fn convert_null<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((TypeEntryDetails::Unit.into(), metadata))
    }

    fn convert_object<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        validation: &Option<Box<ObjectValidation>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match validation.as_ref().map(Box::as_ref) {
            // Maps have an empty properties set, and a non-null schema for the
            // additional_properties field.
            Some(ObjectValidation {
                max_properties: None,
                min_properties: None,
                required,
                properties,
                pattern_properties,
                additional_properties,
                property_names: None,
            }) if required.is_empty()
                && properties.is_empty()
                && pattern_properties.is_empty()
                && additional_properties.as_ref().map(AsRef::as_ref)
                    != Some(&Schema::Bool(false)) =>
            {
                self.make_map(type_name.into_option(), additional_properties)
            }
            None => self.make_map(type_name.into_option(), &None),

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
            .get(key)
            .expect(format!("key {} is missing", key).as_str());
        Ok((
            TypeEntryDetails::Reference(type_id.clone()).into(),
            metadata,
        ))
    }

    fn convert_all_of<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if let Some(ty) = self.maybe_singleton_subschema(type_name.clone(), subschemas) {
            return Ok((ty, metadata));
        }

        if let Some(ty) = self.maybe_all_of_constraints(type_name.clone(), subschemas) {
            return Ok((ty, metadata));
        }

        if let Some(ty) = self.maybe_all_of_subclass(type_name.clone(), metadata, subschemas) {
            return Ok((ty, metadata));
        }

        // TODO JSON schema is annoying. In particular, "allOf" means that all
        // schemas must validate. So for us to construct the schema below, each
        // type must actually be "open" i.e. it must permit arbitrary
        // properties. If it does not, the schemas would not validate i.e. a
        // value (object) could not satisfy both Schema1 and Schema2. To do
        // this as accurately as possible, we would need to validate that each
        // subschema was "open", pull out the "extra" item from each one, etc.

        // We'll want to build a struct that looks like this:
        // struct Name {
        //     #[serde(flatten)]
        //     schema1: Schema1Type,
        //     #[serde(flatten)]
        //     schema2: Schema2Type,
        //     ...
        // }
        self.flattened_union_struct(type_name, metadata, subschemas, false)
    }

    fn convert_any_of<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
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
            self.convert_one_of(type_name, metadata, subschemas)
        } else {
            // We'll want to build a struct that looks like this:
            // struct Name {
            //     #[serde(flatten)]
            //     schema1: Option<Schema1Type>,
            //     #[serde(flatten)]
            //     schema2: Option<Schema2Type>,
            //     ...
            // }

            self.flattened_union_struct(type_name, metadata, subschemas, true)
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
        metadata: &'a Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let ty = self
            .maybe_option(type_name.clone(), metadata, subschemas)
            .or_else(|| self.maybe_externally_tagged_enum(type_name.clone(), metadata, subschemas))
            .or_else(|| self.maybe_adjacently_tagged_enum(type_name.clone(), metadata, subschemas))
            .or_else(|| self.maybe_internally_tagged_enum(type_name.clone(), metadata, subschemas))
            .or_else(|| self.maybe_singleton_subschema(type_name.clone(), subschemas))
            .map_or_else(|| self.untagged_enum(type_name, metadata, subschemas), Ok)?;

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

                let (type_entry, _) = self.convert_schema_object(Name::Unknown, &type_schema)?;

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
                    .collect::<HashSet<_>>();

                match (instance_types.len(), instance_types.iter().next()) {
                    (1, Some(instance_type)) => {
                        let typed_schema = SchemaObject {
                            instance_type: Some(schemars::schema::SingleOrVec::Single(Box::new(
                                *instance_type,
                            ))),
                            ..Default::default()
                        };

                        let (type_entry, _) =
                            self.convert_schema_object(Name::Unknown, &typed_schema)?;
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
            // A tuple.
            ArrayValidation {
                items: Some(SingleOrVec::Vec(items)),
                additional_items: None,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
                contains: None,
            } if max_items == min_items && *max_items as usize == items.len() => {
                let types = items
                    .iter()
                    .map(|schema| Ok(self.id_for_schema(Name::Unknown, schema)?.0))
                    .collect::<Result<Vec<_>>>()?;

                Ok((TypeEntryDetails::Tuple(types).into(), metadata))
            }

            // Arrays and sets.
            ArrayValidation {
                items: Some(SingleOrVec::Single(item)),
                additional_items: None,
                max_items: _, // TODO enforce size limitations
                min_items: _, // TODO enforce size limitations
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
                    _ => Ok((TypeEntryDetails::Array(type_id).into(), metadata)),
                }
            }

            _ => Err(Error::InvalidSchema(format!(
                "unhandled array validation {:#?}",
                validation
            ))),
        }
    }

    fn convert_array_of_any<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let any = TypeEntry::new_native("serde_json::Value", &[]);
        let type_id = self.assign_type(any);
        Ok((TypeEntryDetails::Array(type_id).into(), metadata))
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
        Ok((TypeEntry::new_native("serde_json::Value", &[]), metadata))
    }

    fn convert_typed_enum<'a>(
        &mut self,
        type_name: Name,
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

        let (type_entry, metadata) = self.convert_schema_object(inner_type_name, &type_schema)?;

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
            .collect::<HashSet<_>>();

        let has_null = instance_types.remove(&InstanceType::Null);

        if has_null {
            // If there's a null-value, recur with the null value removed; then
            // convert the resulting type to be optional.
            let enum_values = enum_values
                .iter()
                .cloned()
                .filter(|v| !v.is_null())
                .collect::<Vec<_>>();

            let (type_entry, metadata) =
                self.convert_unknown_enum(type_name, metadata, &enum_values)?;
            let type_entry = self.type_to_option(type_entry);
            Ok((type_entry, metadata))
        } else {
            match (instance_types.len(), instance_types.iter().next()) {
                (1, Some(InstanceType::String)) => {
                    self.convert_enum_string(type_name, metadata, enum_values, None)
                }

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
                    let (type_entry, new_metadata) =
                        self.convert_typed_enum(type_name, &typed_schema, enum_values)?;
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

    pub(crate) fn convert_option<'a, 'b>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        schema: &'b Schema,
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
    use quote::quote;
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
    fn int_helper<T: JsonSchema>() {
        let schema = schema_for!(T);

        let mut type_space = TypeSpace::default();
        type_space
            .add_ref_types(schema.definitions.clone())
            .unwrap();
        let (ty, _) = type_space
            .convert_schema_object(Name::Unknown, &schema.schema)
            .unwrap();
        let output = ty.type_name(&type_space);
        let actual = output
            .rsplit_once("::")
            .map(|(_, x)| x.trim())
            .unwrap_or(&output);
        let expected = std::any::type_name::<T>()
            .rsplit_once("::")
            .map(|(_, x)| x.trim())
            .unwrap_or(&output);
        assert_eq!(actual, expected);
    }

    macro_rules! int_test {
        ($t:ty) => {
            paste! {
                #[test]
                fn [<test_int_ $t:lower>]() {
                    int_helper::<$t>()
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
    fn test_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A {
            a: Box<A>,
        }

        validate_output::<A>();
    }

    #[test]
    fn test_optional_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A {
            a: Option<Box<A>>,
        }

        validate_output::<A>();
    }

    #[test]
    fn test_enum_trivial_cycles() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        enum A {
            Variant0(u64),
            Variant1 {
                a: u64,
                b: Vec<A>,
                rop: Option<Box<A>>,
            },
            Variant2 {
                a: Box<A>,
            },
            Variant3(u64, Box<A>),
            Variant4(Option<Box<A>>, String),
        }

        validate_output::<A>();
    }

    #[test]
    fn test_newtype_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A(Box<A>);

        validate_output::<A>();
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
        match type_space.convert_schema_object(Name::Unknown, &schema) {
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
        match type_space.convert_schema_object(Name::Unknown, &schema) {
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
        let expected = quote! {};
        assert_eq!(actual.to_string(), expected.to_string());
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
