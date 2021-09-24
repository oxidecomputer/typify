use std::collections::BTreeMap;

use convert_case::Case;
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};
use structs::struct_members;
use thiserror::Error;
use util::{all_mutually_exclusive, recase};

use crate::enums::{
    maybe_adjacently_tagged_enum, maybe_externally_tagged_enum, maybe_internally_tagged_enum,
    untagged_enum,
};

mod enums;
mod structs;
#[cfg(test)]
mod test_util;
mod type_entry;
mod util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("unknown")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct TypeEntry {
    name: Option<String>,
    // TODO rename: Option<String>,
    description: Option<String>,
    details: TypeDetails,
    // TODO probably need a bit to say if this is a built-in type
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct TypeId(u64);

#[derive(Debug)]
pub(crate) enum TypeDetails {
    Enum {
        tag_type: EnumTagType,
        variants: Vec<Variant>,
    },
    Struct(Vec<StructProperty>),
    Unit,
    Option(TypeId),
    Array(TypeId),
    Tuple(Vec<TypeId>),
    BuiltIn,

    Reference(TypeId),
}

#[derive(Debug, PartialEq)]
pub(crate) enum EnumTagType {
    External,
    Internal { tag: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Variant {
    name: String,
    rename: Option<String>,
    description: Option<String>,
    details: VariantDetails,
}

#[derive(Debug, PartialEq)]
pub(crate) enum VariantDetails {
    Simple,
    Tuple(Vec<TypeId>),
    Struct(Vec<StructProperty>),
}

// TODO there's actually a subtle difference between properties that are
// required and those that have a nullable type. We're representing both of
// them as an Option<T>, but in some cases we may also want to have a
// `#[serde(skip_serializing_if = "Option::is_none")]`
// required and nullable -> Option<T>
// non-required and nullable -> Option<T> and skip or not (doesn't matter)
// non-required and non-nullable -> Option<T> and skip
// required and non-nullable -> T
#[derive(Debug, PartialEq)]
pub(crate) struct StructProperty {
    name: String,
    serde_options: StructPropertySerde,
    description: Option<String>,
    type_id: TypeId,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StructPropertySerde {
    None,
    Rename(String),
    Flatten,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TypeName {
    /// We don't have a type name hint so this type must not result in a type
    /// that requires a name.
    None,
    /// The type must have a name and should be referred to by this name. This
    /// will be the case for types used by reference into the global type
    /// dictionary.
    Required(String),
    /// The type is not required to have a name, but can use one if necessary.
    /// Built-in types such as Vec, String, or u32 will ignore this whereas
    /// embedded structs or enums may use it.
    Suggestion(String),
}

// TODO we need two String -> Type maps:
// 1. the one for references. these will almost certainly need to be used by
// name
// 2. one for types that we create by necessity; these may not have great
// names. They're types that are embedded within other types and that Rust
// requires us to define as their own type.

#[derive(Debug)]
pub struct TypeSpace {
    next_id: u64,

    definitions: BTreeMap<String, Schema>,

    // TODO
    pub(crate) id_to_entry: BTreeMap<TypeId, TypeEntry>,
    ref_to_id: BTreeMap<String, TypeId>,
    id_to_option_id: BTreeMap<TypeId, TypeId>,
}

impl Default for TypeSpace {
    fn default() -> Self {
        Self {
            next_id: 1,
            definitions: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
            ref_to_id: BTreeMap::new(),
            id_to_option_id: BTreeMap::new(),
        }
    }
}

impl TypeSpace {
    pub fn new(definitions: &BTreeMap<String, Schema>) -> Result<Self> {
        let mut ts = Self {
            next_id: 1,
            definitions: definitions.clone(),
            id_to_entry: BTreeMap::new(),
            ref_to_id: BTreeMap::new(),
            id_to_option_id: BTreeMap::new(),
        };
        ts.refs()?;
        Ok(ts)
    }

    fn refs(&mut self) -> Result<()> {
        let base_id = self.next_id;
        self.next_id += self.definitions.len() as u64;

        for (index, (ref_name, schema)) in self.definitions.iter().enumerate() {
            println!("inserting {}", ref_name);
            self.ref_to_id
                .insert(ref_name.clone(), TypeId(base_id + index as u64));
        }

        let xxx = self
            .definitions
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();

        for (index, (ref_name, schema)) in xxx {
            let type_name = match ref_name.rfind("/") {
                Some(idx) => &ref_name[idx..],
                None => &ref_name,
            };

            let (type_entry, _) = self.convert_schema(Some(type_name), &schema)?;
            self.id_to_entry
                .insert(TypeId(base_id + index as u64), type_entry);
        }

        Ok(())
    }

    pub fn iter_types(&self) -> impl Iterator<Item = &TypeEntry> {
        self.id_to_entry.values()
    }

    pub fn convert_schema<'a>(
        &mut self,
        type_name: Option<&str>,
        schema: &'a Schema,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match schema {
            Schema::Bool(_) => todo!(),
            Schema::Object(obj) => self.convert_schema_object(type_name, obj),
        }
    }

    pub fn convert_schema_object<'a>(
        &mut self,
        type_name: Option<&str>,
        schema: &'a SchemaObject,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match schema {
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
            } if single.as_ref() == &InstanceType::Integer => {
                self.convert_integer(metadata, validation, format)
            }

            // Boolean
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
            } if single.as_ref() == &InstanceType::Boolean => {
                self.convert_bool(type_name, metadata)
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
                object: Some(validation),
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
            } => self.convert_permissive(type_name, metadata),

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
            } if single.as_ref() == &InstanceType::Null => self.convert_null(type_name, metadata),

            // If we have a schema that has an instance type array that's
            // exactly two elements and one of them is Null, we have the
            // equivalent of an Option<T> where T is the type defined by the
            // rest of the schema.
            SchemaObject {
                metadata,
                instance_type: Some(SingleOrVec::Vec(multiple)),
                ..
            } if multiple.len() == 2 && multiple.contains(&InstanceType::Null) => {
                let other_type = multiple.iter().find(|t| t != &&InstanceType::Null).unwrap();
                let ss = SchemaObject {
                    instance_type: Some(SingleOrVec::from(*other_type)),
                    ..schema.clone()
                };

                let (ty, _) = self.convert_schema_object(type_name, &ss)?;
                let ty = self.type_to_option(ty);

                Ok((ty, metadata))
            }

            // Reference
            SchemaObject {
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
                reference: Some(reference),
                extensions: _,
            } => self.convert_reference(reference),

            // Subschemas
            SchemaObject {
                metadata,
                instance_type: None,
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

            // Unknown
            SchemaObject { .. } => todo!("{:#?}", schema),
        }
    }

    pub(crate) fn convert_string<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        format: &Option<String>,
        validation: &Option<Box<schemars::schema::StringValidation>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match format.as_ref().map(String::as_str) {
            None => {
                // TODO we'll need to deal with strings with lengths and
                // patterns, but it seems like a pain in the neck so I'm
                // punting for now.
                // assert!(validation.is_none_or_default(), "{:#?}", validation);
                Ok((
                    TypeEntry {
                        name: Some("String".to_string()),
                        description: None,
                        details: TypeDetails::BuiltIn,
                    },
                    metadata,
                ))
            }

            Some("uuid") => Ok((
                TypeEntry {
                    name: Some("uuid::Uuid".to_string()),
                    description: None,
                    details: TypeDetails::BuiltIn,
                },
                metadata,
            )),

            Some("date-time") => Ok((
                TypeEntry {
                    name: Some("chrono::DateTime<Utc>".to_string()),
                    description: None,
                    details: TypeDetails::BuiltIn,
                },
                metadata,
            )),

            unhandled => todo!("{:#?}", unhandled),
        }
    }

    pub(crate) fn convert_enum_string<'a>(
        &self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
        enum_values: &[serde_json::Value],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let variants = enum_values
            .iter()
            .map(|value| {
                let (name, rename) = recase(
                    value
                        .as_str()
                        .ok_or_else(|| Error::BadValue("string".to_string(), value.clone()))?
                        .to_string(),
                    Case::Pascal,
                );
                Ok(Variant {
                    name,
                    rename,
                    description: None,
                    details: VariantDetails::Simple,
                })
            })
            .collect::<Result<_>>()?;
        let ty = TypeEntry::from_metadata(
            type_name,
            metadata,
            TypeDetails::Enum {
                tag_type: EnumTagType::External,
                variants,
            },
        );
        Ok((ty, metadata))
    }

    pub(crate) fn convert_integer<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        validation: &Option<Box<schemars::schema::NumberValidation>>,
        format: &Option<String>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if let Some(validation) = validation {
            assert!(validation.multiple_of.is_none());
            assert!(validation.maximum.is_none());
            assert!(validation.exclusive_maximum.is_none());
            // TODO
            //assert!(validation.minimum.is_none());
            assert!(validation.exclusive_minimum.is_none());
        }

        let ty = match format.as_ref().map(|s| s.as_str()) {
            Some("int8") => TypeEntry {
                name: Some("i8".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint8") => TypeEntry {
                name: Some("u8".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int16") => TypeEntry {
                name: Some("i16".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint16") => TypeEntry {
                name: Some("u16".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int32") => TypeEntry {
                name: Some("i32".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint32") => TypeEntry {
                name: Some("u32".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int64") => TypeEntry {
                name: Some("i64".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint64") => TypeEntry {
                name: Some("u64".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },

            _ => todo!("{:#?} {:#?}", validation, format),
        };

        Ok((ty, metadata))
    }

    /// If we have a schema that's just the Null instance type, it represents a
    /// solitary value so we model that with the unit type.
    pub(crate) fn convert_null<'a>(
        &self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Unit);
        Ok((ty, metadata))
    }

    pub(crate) fn convert_object<'a>(
        &mut self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
        validation: &ObjectValidation,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let ty = TypeEntry::from_metadata(
            type_name,
            metadata,
            TypeDetails::Struct(struct_members(validation, self)?),
        );
        Ok((ty, &None))
    }

    pub(crate) fn convert_reference<'a>(
        &self,
        ref_name: &str,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let key = match ref_name.rfind('/') {
            Some(idx) => &ref_name[idx + 1..],
            None => ref_name,
        };
        let type_id = self.ref_to_id.get(key).unwrap();
        let ty = TypeEntry {
            name: None,
            description: None,
            details: TypeDetails::Reference(type_id.clone()),
        };
        Ok((ty, &None))
    }

    pub(crate) fn convert_all_of<'a>(
        &mut self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
            return Ok((ty, metadata));
        }

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

    pub(crate) fn convert_any_of<'a>(
        &mut self,
        type_name: Option<&str>,
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
    fn convert_one_of<'a>(
        &mut self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
            return Ok((ty, metadata));
        }

        let ty = maybe_externally_tagged_enum(type_name, metadata, subschemas, self)
            .map(Ok)
            .or_else(|| maybe_adjacently_tagged_enum(type_name, metadata, subschemas, self).map(Ok))
            .or_else(|| maybe_internally_tagged_enum(type_name, metadata, subschemas, self).map(Ok))
            .unwrap_or_else(|| untagged_enum(type_name, metadata, subschemas, self))?;

        Ok((ty, &None))
    }

    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    pub(crate) fn id_for_type<'a>(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeDetails::Reference(type_id) = ty.details {
            type_id
        } else {
            let type_id = self.assign();
            self.id_to_entry.insert(type_id.clone(), ty);
            type_id
        }
    }

    pub(crate) fn id_for_schema<'a>(
        &mut self,
        type_name: Option<&str>,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (ty, meta) = self.convert_schema(type_name, schema)?;
        let type_id = self.id_for_type(ty);
        Ok((type_id, meta))
    }

    pub(crate) fn id_for_option(&mut self, id: TypeId) -> TypeId {
        if let Some(id) = self.id_to_option_id.get(&id) {
            id.clone()
        } else {
            let ty = TypeEntry {
                name: None,
                description: None,
                details: TypeDetails::Option(id.clone()),
            };
            let type_id = self.assign();
            self.id_to_option_id.insert(id, type_id.clone());
            self.id_to_entry.insert(type_id.clone(), ty);

            type_id
        }
    }

    pub(crate) fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        let type_id = self.id_for_type(ty);

        // TODO: this is bad b/c I'm not recording this option in `id_to_option`
        TypeEntry {
            name: None,
            description: None,
            details: TypeDetails::Option(type_id),
        }
    }

    pub(crate) fn convert_array<'a>(
        &mut self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
        validation: &ArrayValidation,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match validation {
            // Normal, vanilla array with no funny business.
            ArrayValidation {
                items: Some(SingleOrVec::Single(item)),
                additional_items: None,
                max_items: None,
                min_items: None,
                unique_items: None,
                contains: None,
            } => {
                let (type_id, _) = self.id_for_schema(None, item.as_ref())?;

                let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Array(type_id));

                Ok((ty, metadata))
            }

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
                    .map(|schema| Ok(self.id_for_schema(None, schema)?.0))
                    .collect::<Result<Vec<_>>>()?;

                let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Tuple(types));

                Ok((ty, metadata))
            }

            _ => todo!("{:#?}", validation),
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
    fn flattened_union_struct<'a>(
        &mut self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
        subschemas: &[Schema],
        optional: bool,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let properties = subschemas
            .iter()
            .enumerate()
            .map(|(idx, schema)| {
                // TODO this should take a name hint of some kind
                let type_name = type_name.map(|s| format!("{}Variant{}", s, idx));

                let (mut type_id, _) = self.id_for_schema(type_name.as_deref(), schema)?;
                if optional {
                    type_id = self.id_for_option(type_id);
                }

                // TODO we need a reasonable name that could be derived
                // from the name of the type
                let name = format!("variant_{}", idx);

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

    pub(crate) fn convert_bool<'a>(
        &self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((
            TypeEntry {
                name: Some("bool".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            metadata,
        ))
    }

    pub(crate) fn convert_permissive<'a>(
        &self,
        type_name: Option<&str>,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((
            TypeEntry {
                name: Some("serde_json::Value".to_string()),
                description: None,
                details: TypeDetails::BuiltIn,
            },
            metadata,
        ))
    }
}

#[cfg(test)]
mod tests {
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use std::collections::HashSet;

    use crate::{TypeDetails, TypeEntry, TypeSpace, VariantDetails};

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema)]
    struct Blah {
        blah: String,
    }

    #[allow(dead_code)]
    #[derive(Serialize, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    //#[serde(untagged)]
    //#[serde(tag = "type", content = "content")]
    enum E {
        /// aaa
        A,
        /// bee
        B,
        /// cee
        //C(Vec<String>),
        C(Blah),
        /// dee
        D {
            /// double D
            dd: String,
        },
        // /// eff
        // F(
        //     /// eff.0
        //     u32,
        //     /// eff.1
        //     u32,
        // ),
    }

    #[allow(dead_code)]
    #[derive(JsonSchema)]
    #[serde(rename_all = "camelCase")]
    struct Foo {
        /// this is bar
        #[serde(default)]
        bar: Option<String>,
        baz_baz: i32,
        /// eeeeee!
        e: E,
    }

    #[test]
    fn test_simple() {
        let schema = schema_for!(Foo);
        println!("{:#?}", schema);
        let mut type_space = TypeSpace::new(&schema.definitions).unwrap();
        let (ty, _) = type_space
            .convert_schema_object(None, &schema.schema)
            .unwrap();

        println!("{:#?}", ty);

        let out = ty.output(&type_space);
        println!("{}", out);

        for ty in type_space.id_to_entry.values() {
            println!("{:#?}", ty);
            let out = ty.output(&type_space);
            println!("{}", out);
        }
    }

    #[test]
    fn test_convert_enum_string() {
        #[allow(dead_code)]
        #[derive(JsonSchema)]
        #[serde(rename_all = "camelCase")]
        enum SimpleEnum {
            DotCom,
            Grizz,
            Kenneth,
        }

        let schema = schema_for!(SimpleEnum);
        println!("{:#?}", schema);

        let mut type_space = TypeSpace::new(&schema.definitions).unwrap();
        let (ty, _) = type_space
            .convert_schema_object(None, &schema.schema)
            .unwrap();

        match ty {
            TypeEntry {
                details: TypeDetails::Enum { variants, .. },
                ..
            } => {
                for variant in &variants {
                    assert_eq!(variant.details, VariantDetails::Simple);
                }
                let var_names = variants
                    .iter()
                    .map(|variant| variant.name.clone())
                    .collect::<HashSet<_>>();
                assert_eq!(
                    var_names,
                    ["DotCom", "Grizz", "Kenneth",]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<HashSet<_>>()
                );
            }
            _ => {
                let out = ty.output(&type_space);
                println!("{}", out);
                panic!();
            }
        }
    }
}

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
