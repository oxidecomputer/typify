use crate::type_entry::{
    EnumTagType, TypeEntry, TypeEntryEnum, TypeEntryStruct, Variant, VariantDetails,
};
use crate::util::{all_mutually_exclusive, recase};
use convert_case::Case;
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};

use crate::util::get_type_name;

use crate::{Error, Name, Result, TypeSpace};

impl TypeSpace {
    pub(crate) fn convert_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match schema {
            Schema::Bool(true) => self.convert_permissive(&None),
            Schema::Object(obj) => self.convert_schema_object(type_name, obj),

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
                    self.convert_option(type_name, metadata, &ss)
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
                string: validation,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::String => {
                self.convert_string(metadata, format, validation)
            }

            // Simple string enum
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Single(single)),
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
            } if single.as_ref() == &InstanceType::String => {
                self.convert_enum_string(type_name, metadata, enum_values)
            }

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
                enum_values,
                const_value: None,
                subschemas: None,
                number: None,
                string: None,
                array: None,
                object: None,
                reference: None,
                extensions: _,
            } if single.as_ref() == &InstanceType::Boolean => {
                self.convert_bool(metadata, enum_values)
            }

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

            // Arrays
            SchemaObject {
                metadata,
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
            } if single.as_ref() == &InstanceType::Array => {
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

            // Unknown
            SchemaObject { .. } => todo!("{:#?}", schema),
        }
    }

    fn convert_string<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
        format: &Option<String>,
        _validation: &Option<Box<schemars::schema::StringValidation>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        trait OptionIsNoneOrDefault {
            fn is_none_or_default(&self) -> bool;
        }

        impl<T> OptionIsNoneOrDefault for Option<T>
        where
            T: Default + PartialEq,
        {
            fn is_none_or_default(&self) -> bool {
                match self {
                    Some(t) => t == &T::default(),
                    None => true,
                }
            }
        }
        match format.as_ref().map(String::as_str) {
            None => {
                // TODO we'll need to deal with strings with lengths and
                // patterns, but it seems like a pain in the neck so I'm
                // punting for now.
                // assert!(validation.is_none_or_default(), "{:#?}", validation);
                Ok((TypeEntry::String, metadata))
            }

            Some("uuid") => {
                self.uses_uuid = true;
                Ok((TypeEntry::new_builtin("uuid::Uuid"), metadata))
            }

            Some("date") => {
                self.uses_chrono = true;
                Ok((
                    TypeEntry::new_builtin("chrono::Date<chrono::offset::Utc>"),
                    metadata,
                ))
            }

            Some("date-time") => {
                self.uses_chrono = true;
                Ok((
                    TypeEntry::new_builtin("chrono::DateTime<chrono::offset::Utc>"),
                    metadata,
                ))
            }

            // TODO random types I'm not sure what to do with
            Some("uri" | "uri-template" | "email") => Ok((TypeEntry::String, metadata)),

            unhandled => todo!("{:#?}", unhandled),
        }
    }

    pub(crate) fn convert_enum_string<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        enum_values: &[serde_json::Value],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        // We expect all enum values to be either a string **or** a null. We
        // gather them all up and then choose to either be an enum of simple
        // variants, or an Option of an enum of string variants depending on if
        // a null is absent or present. Note that it's actually invalid JSON
        // Schema if we do see a null here. In this code path the instance
        // types should exclusively be "string" making null invalid. We
        // intentionally handle instance types of ["string", "null"] prior to
        // this case and strip out the null in both enum values and instance
        // type. Nevertheless, we do our best to interpret even somewhat janky
        // JSON schema.
        let mut has_null = false;

        let variants = enum_values
            .iter()
            .flat_map(|value| match value {
                // It would be odd to have multiple null values, but we don't
                // need to worry about it.
                serde_json::Value::Null => {
                    has_null = true;
                    None
                }
                serde_json::Value::String(value) => {
                    let (name, rename) = recase(value.clone(), Case::Pascal);
                    Some(Ok(Variant {
                        name,
                        rename,
                        description: None,
                        details: VariantDetails::Simple,
                    }))
                }
                _ => Some(Err(Error::BadValue("string".to_string(), value.clone()))),
            })
            .collect::<Result<Vec<Variant>>>()?;
        let mut ty = TypeEntryEnum::from_metadata(
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
                    return Ok((TypeEntry::new_primitive(ty), metadata));
                }

                if min.is_none() {
                    min = Some(*imin);
                }
                if max.is_none() {
                    max = Some(*imax);
                }
            }
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
            Ok((TypeEntry::new_primitive(ty), metadata))
        } else {
            // TODO we could construct a type that itself enforces the various
            // bounds.
            // TODO failing that we should find the type that most tightly
            // matches these bounds.
            Ok((TypeEntry::new_primitive("i64"), metadata))
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

        Ok((TypeEntry::new_primitive("f64"), &None))
    }

    /// If we have a schema that's just the Null instance type, it represents a
    /// solitary value so we model that with the unit type.
    fn convert_null<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((TypeEntry::Unit, metadata))
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
                self.make_map(None, additional_properties)
            }
            None => self.make_map(None, &None),

            // The typical case
            Some(validation) => {
                let tmp_type_name = get_type_name(&type_name, metadata, Case::Pascal);
                let (properties, deny_unknown_fields) =
                    self.struct_members(tmp_type_name, validation)?;
                let ty = TypeEntryStruct::from_metadata(
                    type_name,
                    metadata,
                    properties,
                    deny_unknown_fields,
                );
                Ok((ty, &None))
            }
        }
    }

    fn convert_reference<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        ref_name: &str,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let key = match ref_name.rfind('/') {
            Some(idx) => &ref_name[idx + 1..],
            None => ref_name,
        };
        let type_id = self.ref_to_id.get(key).unwrap();
        let ty = TypeEntry::Reference(type_id.clone());
        Ok((ty, metadata))
    }

    fn convert_all_of<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
            return Ok((ty, metadata));
        }

        // TODO make this look more like the other maybe clauses
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
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
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
    /// "xxx"], ... } is a much simpler construction. Nevertheless, an option
    /// may be expressed as a one of with two subschemas where one is null.
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
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
            return Ok((ty, metadata));
        }
        let ty = self
            .maybe_option_as_enum(type_name.clone(), metadata, subschemas)
            .or_else(|| self.maybe_externally_tagged_enum(type_name.clone(), metadata, subschemas))
            .or_else(|| self.maybe_adjacently_tagged_enum(type_name.clone(), metadata, subschemas))
            .or_else(|| self.maybe_internally_tagged_enum(type_name.clone(), metadata, subschemas))
            .map_or_else(|| self.untagged_enum(type_name, metadata, subschemas), Ok)?;

        Ok((ty, metadata))
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

                Ok((TypeEntry::Tuple(types), metadata))
            }

            // Normal, vanilla array with no funny business.
            ArrayValidation {
                items: Some(SingleOrVec::Single(item)),
                additional_items: None,
                max_items: _, // TODO enforce size limitations
                min_items: _, // TODO enforce size limitations
                unique_items: None,
                contains: None,
            } => {
                let tmp_type_name = match get_type_name(&type_name, metadata, Case::Pascal) {
                    Some(s) => Name::Suggested(format!("{}Item", s)),
                    None => Name::Unknown,
                };
                let (type_id, _) = self.id_for_schema(tmp_type_name, item.as_ref())?;

                Ok((TypeEntry::Array(type_id), metadata))
            }

            _ => todo!("{:#?}", validation),
        }
    }
    fn convert_array_of_any<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let any = TypeEntry::new_builtin("serde_json::Value");
        let type_id = self.assign_type(any);
        let ty = TypeEntry::Array(type_id);

        Ok((ty, metadata))
    }

    // TODO not sure if I want to deal with enum_values here, but we'll see...
    fn convert_bool<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        _enum_values: &Option<Vec<serde_json::Value>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((TypeEntry::new_primitive("bool"), metadata))
    }

    fn convert_permissive<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        self.uses_serde_json = true;
        Ok((TypeEntry::new_builtin("serde_json::Value"), metadata))
    }

    fn convert_unknown_enum<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        enum_values: &[serde_json::Value],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        // We're here because the schema didn't have a type; that seems busted,
        // but we'll do our best to roll with the punches.
        assert!(!enum_values.is_empty());

        // Let's hope all these values are the same type.
        let instance_types = enum_values
            .iter()
            .map(|v| match v {
                serde_json::Value::Null => InstanceType::Null,
                serde_json::Value::Bool(_) => InstanceType::Boolean,
                serde_json::Value::Number(_) => InstanceType::Number,
                serde_json::Value::String(_) => InstanceType::String,
                serde_json::Value::Array(_) => InstanceType::Array,
                serde_json::Value::Object(_) => InstanceType::Object,
            })
            .collect::<Vec<_>>();

        match (instance_types.len(), instance_types.first()) {
            (1, Some(InstanceType::String)) => {
                self.convert_enum_string(type_name, metadata, enum_values)
            }
            (1, Some(InstanceType::Boolean)) => {
                self.convert_bool(metadata, &Some(enum_values.into()))
            }
            _ => panic!(),
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
}

#[cfg(test)]
mod tests {
    use std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

    use schemars::{schema_for, JsonSchema};

    use crate::{Name, TypeSpace};
    use paste::paste;

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
}
