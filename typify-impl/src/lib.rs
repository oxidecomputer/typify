use std::collections::BTreeMap;

use convert_case::Case;
use proc_macro2::TokenStream;
use schemars::schema::{
    ArrayValidation, InstanceType, Metadata, ObjectValidation, Schema, SchemaObject, SingleOrVec,
    SubschemaValidation,
};
use structs::{flattened_union_struct, maybe_all_of_subclass, struct_members};
use thiserror::Error;
use util::{all_mutually_exclusive, recase};

use crate::{
    enums::{
        maybe_adjacently_tagged_enum, maybe_externally_tagged_enum, maybe_internally_tagged_enum,
        maybe_option_as_enum, untagged_enum,
    },
    structs::make_map,
    util::get_type_name,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeEntry {
    name: Option<String>,
    rename: Option<String>,
    description: Option<String>,
    details: TypeDetails,
    // TODO probably need a bit to say if this is a built-in type
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct TypeId(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypeDetails {
    Enum {
        tag_type: EnumTagType,
        variants: Vec<Variant>,
        deny_unknown_fields: bool,
    },
    Struct {
        properties: Vec<StructProperty>,
        deny_unknown_fields: bool,
    },
    Unit,
    Option(TypeId),
    Array(TypeId),
    Map(TypeId, TypeId),
    Tuple(Vec<TypeId>),
    BuiltIn,
    Newtype(TypeId),

    // While these types won't very make their way out to the user, we need
    // reference types in particular to represent simple type aliases between
    // types named as reference targets.
    Reference(TypeId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumTagType {
    External,
    Internal { tag: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Variant {
    name: String,
    rename: Option<String>,
    description: Option<String>,
    details: VariantDetails,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StructProperty {
    name: String,
    serde_naming: SerdeNaming,
    serde_rules: SerdeRules,
    description: Option<String>,
    type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SerdeNaming {
    None,
    Rename(String),
    Flatten,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SerdeRules {
    None,
    Optional,
}

#[derive(Debug, Clone)]
pub enum Name {
    Required(String),
    Suggested(String),
    Unknown,
}

// TODO we need two String -> Type maps:
// 1. the one for references. these will almost certainly need to be used by
// name
// 2. one for types that we create by necessity; these may not have great
// names. They're types that are embedded within other types and that Rust
// requires us to define as their own type.
// 3. allow for formatting configuration such as the derive macros that are
// included, and the destination for types e.g. into their own mod with a given
// name.

#[derive(Debug)]
pub struct TypeSpace {
    next_id: u64,

    // TODO we need this in order to inspect the collection of reference types
    // e.g. to do `all_mutually_exclusive`. In the future, we could obviate the
    // need this by keeping a single Map of referenced types whose value was an
    // enum of a "raw" or a "converted" schema.
    definitions: BTreeMap<String, Schema>,

    // TODO needs an API
    pub(crate) id_to_entry: BTreeMap<TypeId, TypeEntry>,
    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<String, TypeId>,
    id_to_option_id: BTreeMap<TypeId, TypeId>,

    uses_chrono: bool,
    uses_uuid: bool,
    uses_serde_json: bool,
    pub(crate) type_mod: Option<String>,
}

impl Default for TypeSpace {
    fn default() -> Self {
        Self {
            next_id: 1,
            definitions: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            ref_to_id: BTreeMap::new(),
            id_to_option_id: BTreeMap::new(),
            uses_chrono: false,
            uses_uuid: false,
            uses_serde_json: false,
            type_mod: None,
        }
    }
}

impl TypeSpace {
    // Working on a public interface

    /// Add a collection of types that will be used as references. Regardless
    /// of how these types are defined--*de novo* or built-in--these types will
    /// appear in the final output in some form. This method may be called
    /// multiple times, but collections of references must be self-contained;
    /// in other words, a type in one invocation may not refer to a type in
    /// another invocation.
    // TODO on an error the TypeSpace is in a weird state; we, perhaps, create
    // a child TypeSpace and then merge it in once all conversions hae
    // succeeded.
    pub fn add_ref_types<I, S>(&mut self, type_defs: I) -> Result<()>
    where
        I: IntoIterator<Item = (S, Schema)>,
        S: AsRef<str>,
    {
        // Gather up all types to make things a little more convenient.
        let definitions = type_defs
            .into_iter()
            .map(|(name, schema)| (name.as_ref().to_string(), schema))
            .collect::<Vec<(String, Schema)>>();

        // Assign IDs to reference types before actually converting them. We'll
        // need these in the case of forward (or circular) references.
        let base_id = self.next_id;
        self.next_id += definitions.len() as u64;

        for (index, (ref_name, _)) in definitions.iter().enumerate() {
            self.ref_to_id
                .insert(ref_name.to_string(), TypeId(base_id + index as u64));
        }

        // Convert all types; note that we use the type assigned from the
        // previous step because each type may create additional types.
        for (index, (ref_name, schema)) in definitions.into_iter().enumerate() {
            let type_name = match ref_name.rfind('/') {
                Some(idx) => &ref_name[idx..],
                None => &ref_name,
            };

            let (type_entry, metadata) =
                self.convert_schema(Name::Required(type_name.to_string()), &schema)?;
            let type_entry = match type_entry {
                TypeEntry {
                    name: None,
                    rename: None,
                    description: None,
                    details: TypeDetails::Reference(type_id),
                } => TypeEntry::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    TypeDetails::Newtype(type_id),
                ),
                _ => type_entry,
            };
            self.definitions.insert(ref_name, schema);
            self.id_to_entry
                .insert(TypeId(base_id + index as u64), type_entry);
        }
        Ok(())
    }

    /// Add a new type and return a type identifier that may be used in
    /// function signatures or embedded within other types.
    pub fn add_type(&mut self, schema: &Schema) -> Result<TokenStream> {
        let (type_entry, _) = self.convert_schema(Name::Unknown, schema)?;

        let type_id = self.assign_type(type_entry);
        let type_entry = self.id_to_entry.get(&type_id).unwrap();
        Ok(type_entry.type_ident(self, true))
    }

    pub fn uses_chrono(&self) -> bool {
        self.uses_chrono
    }

    pub fn uses_uuid(&self) -> bool {
        self.uses_uuid
    }

    pub fn uses_serde_json(&self) -> bool {
        self.uses_serde_json
    }

    pub fn set_type_mod<S: AsRef<str>>(&mut self, type_mod: S) {
        self.type_mod = Some(type_mod.as_ref().to_string());
    }

    // Private interface?

    pub fn new(definitions: &BTreeMap<String, Schema>) -> Result<Self> {
        let mut ts = Self::default();
        ts.add_ref_types(definitions.clone())?;
        Ok(ts)
    }

    pub fn iter_types(&self) -> impl Iterator<Item = &TypeEntry> {
        self.id_to_entry.values()
    }

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

    pub fn convert_schema_object<'a>(
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
                let other_type = multiple.iter().find(|t| t != &&InstanceType::Null).unwrap();
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
            } if single.as_ref() == &InstanceType::Null => self.convert_null(type_name, metadata),

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

    pub(crate) fn convert_string<'a>(
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
                Ok((
                    TypeEntry {
                        name: Some("String".to_string()),
                        rename: None,
                        description: None,
                        details: TypeDetails::BuiltIn,
                    },
                    metadata,
                ))
            }

            Some("uuid") => {
                self.uses_uuid = true;
                Ok((
                    TypeEntry {
                        name: Some("uuid::Uuid".to_string()),
                        rename: None,
                        description: None,
                        details: TypeDetails::BuiltIn,
                    },
                    metadata,
                ))
            }

            Some("date-time") => {
                self.uses_chrono = true;
                Ok((
                    TypeEntry {
                        name: Some("chrono::DateTime<chrono::offset::Utc>".to_string()),
                        rename: None,
                        description: None,
                        details: TypeDetails::BuiltIn,
                    },
                    metadata,
                ))
            }

            // TODO random types I'm not sure what to do with
            Some("uri" | "uri-template" | "email") => Ok((
                TypeEntry {
                    name: Some("String".to_string()),
                    rename: None,
                    description: None,
                    details: TypeDetails::BuiltIn,
                },
                metadata,
            )),

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
        let mut ty = TypeEntry::from_metadata(
            type_name,
            metadata,
            TypeDetails::Enum {
                tag_type: EnumTagType::External,
                variants,
                deny_unknown_fields: false,
            },
        );

        if has_null {
            ty = self.type_to_option(ty);
        }

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
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint8") => TypeEntry {
                name: Some("u8".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int16") => TypeEntry {
                name: Some("i16".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint16") => TypeEntry {
                name: Some("u16".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int32" | "int") => TypeEntry {
                name: Some("i32".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint32" | "uint") => TypeEntry {
                name: Some("u32".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("int64") => TypeEntry {
                name: Some("i64".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            Some("uint64") => TypeEntry {
                name: Some("u64".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },

            // TODO is this the right default? Should we be looking at the
            // validation e.g. for max and min?
            None => TypeEntry {
                name: Some("u64".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },

            _ => todo!("{:#?} {:#?}", validation, format),
        };

        Ok((ty, metadata))
    }

    // TODO deal with metadata and format
    pub(crate) fn convert_number<'a>(
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

        Ok((
            TypeEntry {
                name: Some("f64".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            &None,
        ))
    }

    /// If we have a schema that's just the Null instance type, it represents a
    /// solitary value so we model that with the unit type.
    pub(crate) fn convert_null<'a>(
        &self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Unit);
        Ok((ty, metadata))
    }

    pub(crate) fn convert_object<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<Metadata>>,
        validation: &ObjectValidation,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        match validation {
            // Maps have an empty properties set, and a non-null schema for the
            // additional_properties field.
            ObjectValidation {
                max_properties: None,
                min_properties: None,
                required,
                properties,
                pattern_properties,
                additional_properties,
                property_names: None,
            } if required.is_empty()
                && properties.is_empty()
                && pattern_properties.is_empty()
                && additional_properties.as_ref().map(AsRef::as_ref)
                    != Some(&Schema::Bool(false)) =>
            {
                make_map(None, additional_properties, self)
            }

            // The typical case
            _ => {
                let tmp_type_name = get_type_name(&type_name, metadata, Case::Pascal);
                let (properties, deny_unknown_fields) =
                    struct_members(tmp_type_name, validation, self)?;
                let ty = TypeEntry::from_metadata(
                    type_name,
                    metadata,
                    TypeDetails::Struct {
                        properties,
                        deny_unknown_fields,
                    },
                );
                Ok((ty, &None))
            }
        }
    }

    pub(crate) fn convert_reference<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        ref_name: &str,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        let key = match ref_name.rfind('/') {
            Some(idx) => &ref_name[idx + 1..],
            None => ref_name,
        };
        let type_id = self.ref_to_id.get(key).unwrap();
        let ty = TypeEntry {
            name: None,
            rename: None,
            description: None,
            details: TypeDetails::Reference(type_id.clone()),
        };
        Ok((ty, metadata))
    }

    pub(crate) fn convert_all_of<'a>(
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
        if let Some(ty) = maybe_all_of_subclass(type_name.clone(), metadata, subschemas, self) {
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
        flattened_union_struct(type_name, metadata, subschemas, false, self)
    }

    pub(crate) fn convert_any_of<'a>(
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

            flattened_union_struct(type_name, metadata, subschemas, true, self)
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
    fn convert_one_of<'a>(
        &mut self,
        type_name: Name,
        metadata: &'a Option<Box<schemars::schema::Metadata>>,
        subschemas: &[Schema],
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        if subschemas.len() == 1 {
            let (ty, _) = self.convert_schema(type_name, subschemas.first().unwrap())?;
            return Ok((ty, metadata));
        }
        let ty = maybe_option_as_enum(type_name.clone(), metadata, subschemas, self)
            .or_else(|| maybe_externally_tagged_enum(type_name.clone(), metadata, subschemas, self))
            .or_else(|| maybe_adjacently_tagged_enum(type_name.clone(), metadata, subschemas, self))
            .or_else(|| maybe_internally_tagged_enum(type_name.clone(), metadata, subschemas, self))
            .map_or_else(|| untagged_enum(type_name, metadata, subschemas, self), Ok)?;

        Ok((ty, metadata))
    }

    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    fn assign_type(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeDetails::Reference(type_id) = &ty.details {
            // The underlying type is already assigned so we just need to
            // return that type ID.
            type_id.clone()
        } else if let Some(name) = ty.name.clone() {
            // If there's already a type of this name, we make sure it's
            // identical.
            // TODO there are many different choices we might make here that
            // could differ depending on the texture of the schema.
            if let Some(type_id) = self.name_to_id.get(&name) {
                let existing_ty = self.id_to_entry.get(type_id).unwrap();
                assert_eq!(existing_ty, &ty);
                type_id.clone()
            } else {
                let type_id = self.assign();
                self.id_to_entry.insert(type_id.clone(), ty);
                self.name_to_id.insert(name.clone(), type_id.clone());
                type_id
            }
        } else {
            let type_id = self.assign();
            self.id_to_entry.insert(type_id.clone(), ty);
            type_id
        }
    }

    pub(crate) fn id_for_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (ty, meta) = self.convert_schema(type_name, schema)?;
        let type_id = self.assign_type(ty);
        Ok((type_id, meta))
    }

    fn id_for_option(&mut self, id: &TypeId) -> TypeId {
        if let Some(id) = self.id_to_option_id.get(id) {
            id.clone()
        } else {
            let ty = TypeEntry {
                name: None,
                rename: None,
                description: None,
                details: TypeDetails::Option(id.clone()),
            };
            let type_id = self.assign();
            self.id_to_option_id.insert(id.clone(), type_id.clone());
            self.id_to_entry.insert(type_id.clone(), ty);

            type_id
        }
    }

    pub(crate) fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        let type_id = self.assign_type(ty);

        // TODO: this is bad b/c I'm not recording this option in `id_to_option`
        TypeEntry {
            name: None,
            rename: None,
            description: None,
            details: TypeDetails::Option(type_id),
        }
    }

    pub(crate) fn convert_array<'a>(
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

                let ty = TypeEntry::from_metadata(type_name, metadata, TypeDetails::Tuple(types));

                Ok((ty, metadata))
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

                // We don't need a name for an array
                let ty =
                    TypeEntry::from_metadata(Name::Unknown, metadata, TypeDetails::Array(type_id));

                Ok((ty, metadata))
            }

            _ => todo!("{:#?}", validation),
        }
    }

    // TODO not sure if I want to deal with enum_values here, but we'll see...
    pub(crate) fn convert_bool<'a>(
        &self,
        metadata: &'a Option<Box<Metadata>>,
        _enum_values: &Option<Vec<serde_json::Value>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        Ok((
            TypeEntry {
                name: Some("bool".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            metadata,
        ))
    }

    pub(crate) fn convert_permissive<'a>(
        &mut self,
        metadata: &'a Option<Box<Metadata>>,
    ) -> Result<(TypeEntry, &'a Option<Box<Metadata>>)> {
        self.uses_serde_json = true;
        Ok((
            TypeEntry {
                name: Some("serde_json::Value".to_string()),
                rename: None,
                description: None,
                details: TypeDetails::BuiltIn,
            },
            metadata,
        ))
    }

    pub(crate) fn convert_unknown_enum<'a>(
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
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashSet;

    use crate::{Name, TypeDetails, TypeEntry, TypeSpace, VariantDetails};

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
            .convert_schema_object(Name::Unknown, &schema.schema)
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
            .convert_schema_object(Name::Unknown, &schema.schema)
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

    #[test]
    fn test_string_enum_with_null() {
        let enum_values = vec![
            json!("Shadrach"),
            json!("Meshach"),
            json!("Abednego"),
            json!(null),
        ];

        let mut type_space = TypeSpace::default();
        let (te, _) = type_space
            .convert_enum_string(Name::Unknown, &None, &enum_values)
            .unwrap();

        if let TypeDetails::Option(id) = &te.details {
            let ote = type_space.id_to_entry.get(id).unwrap();
            if let TypeDetails::Enum { variants, .. } = &ote.details {
                let variants = variants
                    .iter()
                    .map(|v| match v.details {
                        VariantDetails::Simple => v.name.clone(),
                        _ => panic!("unexpected variant type"),
                    })
                    .collect::<HashSet<_>>();

                assert_eq!(
                    variants,
                    enum_values
                        .iter()
                        .flat_map(|j| j.as_str().map(ToString::to_string))
                        .collect::<HashSet<_>>()
                );
            } else {
                panic!("not the sub-type we expected {:#?}", te)
            }
        } else {
            panic!("not the type we expected {:#?}", te)
        }
    }
}
