// Copyright 2021 Oxide Computer Company

use std::collections::BTreeMap;

use log::info;
use proc_macro2::TokenStream;
use quote::quote;
use rustfmt_wrapper::rustfmt;
use schemars::schema::{Metadata, Schema};
use thiserror::Error;
use type_entry::{TypeEntry, TypeEntryDetails, TypeEntryNewtype, VariantDetails};

#[cfg(test)]
mod test_util;

mod convert;
mod enums;
mod structs;
mod type_entry;
mod util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("invalid TypeId")]
    InvalidTypeId,
    #[error("unknown")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Representation of a type which may have a definition or may be built-in.
#[derive(Debug)]
pub struct Type<'a> {
    type_space: &'a TypeSpace,
    type_entry: &'a TypeEntry,
}

/// Type details returned by Type::details() to inspect a type.
pub enum TypeDetails<'a> {
    Enum(TypeEnum<'a>),
    Struct(TypeStruct<'a>),
    Newtype(TypeNewtype<'a>),

    Option(TypeId),
    Array(TypeId),
    Map(TypeId, TypeId),
    Set(TypeId),
    Box(TypeId),
    Tuple(Box<dyn Iterator<Item = TypeId> + 'a>),
    Unit,
    Builtin(&'a str),
}

/// Enum type details.
pub struct TypeEnum<'a> {
    details: &'a type_entry::TypeEntryEnum,
}

/// Enum variant details.
pub enum TypeEnumVariant<'a> {
    Simple,
    Tuple(Vec<TypeId>),
    Struct(Vec<(&'a str, TypeId)>),
}

/// Struct type details.
pub struct TypeStruct<'a> {
    details: &'a type_entry::TypeEntryStruct,
}

/// Newtype details.
pub struct TypeNewtype<'a> {
    details: &'a type_entry::TypeEntryNewtype,
}

/// Type identifier returned from type creation and used to lookup types.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct TypeId(u64);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Name {
    Required(String),
    Suggested(String),
    Unknown,
}

impl Name {
    pub(crate) fn into_option(self) -> Option<String> {
        match self {
            Name::Required(s) | Name::Suggested(s) => Some(s),
            Name::Unknown => None,
        }
    }
}

/// A collection of types.
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
    type_to_id: BTreeMap<TypeEntryDetails, TypeId>,

    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<String, TypeId>,

    uses_chrono: bool,
    uses_uuid: bool,
    uses_serde_json: bool,

    pub(crate) type_mod: Option<String>,
    pub(crate) extra_derives: Vec<TokenStream>,
}

impl Default for TypeSpace {
    fn default() -> Self {
        Self {
            next_id: 1,
            definitions: BTreeMap::new(),
            id_to_entry: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            ref_to_id: BTreeMap::new(),
            type_to_id: BTreeMap::new(),
            uses_chrono: false,
            uses_uuid: false,
            uses_serde_json: false,
            type_mod: None,
            extra_derives: Vec::new(),
        }
    }
}

impl TypeSpace {
    /// Add a collection of types that will be used as references. Regardless
    /// of how these types are defined--*de novo* or built-in--each type will
    /// appear in the final output as a struct, enum or newtype. This method
    /// may be called multiple times, but collections of references must be
    /// self-contained; in other words, a type in one invocation may not refer
    /// to a type in another invocation.
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
        let def_len = definitions.len() as u64;
        self.next_id += def_len;

        for (index, (ref_name, _)) in definitions.iter().enumerate() {
            self.ref_to_id
                .insert(ref_name.to_string(), TypeId(base_id + index as u64));
        }

        // Convert all types; note that we use the type id assigned from the
        // previous step because each type may create additional types.
        for (index, (ref_name, schema)) in definitions.into_iter().enumerate() {
            let type_name = match ref_name.rfind('/') {
                Some(idx) => &ref_name[idx..],
                None => &ref_name,
            };

            info!(
                "converting type: {} with schema {}",
                type_name,
                serde_json::to_string(&schema).unwrap()
            );

            let (type_entry, metadata) =
                self.convert_schema(Name::Required(type_name.to_string()), &schema)?;
            let type_entry = match type_entry.details {
                // The types that are already named are good to go.
                TypeEntryDetails::Enum(_)
                | TypeEntryDetails::Struct(_)
                | TypeEntryDetails::Newtype(_) => type_entry,

                // If the type entry is a reference, then this definition is a
                // simple alias to another type in this list of definitions
                // (which may nor may not have already been converted). We
                // simply create a newtype with that type ID.
                TypeEntryDetails::Reference(type_id) => TypeEntryNewtype::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    type_id,
                )
                .into(),

                // For types that don't have names, this is effectively a type
                // alias which we treat as a newtype (though we could probably
                // handle it as a type alias).
                _ => TypeEntryNewtype::from_metadata(
                    Name::Required(type_name.to_string()),
                    metadata,
                    self.assign_type(type_entry),
                )
                .into(),
            };
            self.definitions.insert(ref_name, schema);
            self.id_to_entry
                .insert(TypeId(base_id + index as u64), type_entry);
        }

        // TODO compute appropriate derives, taking care to account for
        // dependency cycles. Currently we're using a more minimal--safe--set
        // of derives than we might otherwise. This notably prevents us from
        // using a HashSet or BTreeSet type where we might like to.

        // Once all ref types are in, look for containment cycles that we need
        // to break with a Box<T>.
        for index in 0..def_len {
            let type_id = TypeId(base_id + index);

            let mut box_id = None;

            let mut type_entry = self.id_to_entry.get_mut(&type_id).unwrap().clone();
            self.break_trivial_cyclic_refs(&type_id, &mut type_entry, &mut box_id);
            let _ = self.id_to_entry.insert(type_id, type_entry);
        }

        Ok(())
    }

    /// If a type refers to itself, this creates a cycle that will eventually be
    /// emit as a Rust struct that cannot be constructed. Break those cycles
    /// here.
    ///
    /// While we aren't yet handling the general case of type containment
    /// cycles, it's not that bad to look at trivial cycles such as:
    ///
    ///   1) A type refering to itself: A -> A
    ///   2) A type optionally referring to itself: A -> Option<A>
    ///   3) An enum variant referring to itself, either optionally or directly.
    ///
    /// TODO currently only trivial cycles are broken. A more generic solution
    /// may be required, but it may also a point to ask oneself why such a
    /// complicated type is required :) A generic solution is difficult because
    /// certain cycles introduce a question of *where* to Box to break the
    /// cycle, and there's no one answer to this.
    ///
    fn break_trivial_cyclic_refs(
        &mut self,
        parent_type_id: &TypeId,
        type_entry: &mut TypeEntry,
        box_id: &mut Option<TypeId>,
    ) {
        match &mut type_entry.details {
            // Look for the case where an option refers to the parent type
            TypeEntryDetails::Option(option_type_id) => {
                if *option_type_id == *parent_type_id {
                    *option_type_id = box_id
                        .get_or_insert_with(|| self.id_to_box(parent_type_id))
                        .clone();
                }
            }

            // Look for the case where a struct property refers to the parent
            // type
            TypeEntryDetails::Struct(s) => {
                for prop in &mut s.properties {
                    if prop.type_id == *parent_type_id {
                        // A struct property directly refers to the parent type
                        prop.type_id = box_id
                            .get_or_insert_with(|| self.id_to_box(parent_type_id))
                            .clone();
                    } else {
                        // A struct property optionally refers to the parent type
                        let mut prop_type_entry =
                            self.id_to_entry.get_mut(&prop.type_id).unwrap().clone();
                        self.break_trivial_cyclic_refs(
                            &parent_type_id,
                            &mut prop_type_entry,
                            box_id,
                        );
                        let _ = self
                            .id_to_entry
                            .insert(prop.type_id.clone(), prop_type_entry);
                    }
                }
            }

            // Look for the cases where an enum variant refers to the parent
            // type
            TypeEntryDetails::Enum(type_entry_enum) => {
                for variant in &mut type_entry_enum.variants {
                    match &mut variant.details {
                        // Simple variants will not refer to anything
                        VariantDetails::Simple => {}
                        // Look for a tuple entry that refers to the parent type
                        VariantDetails::Tuple(vec_type_id) => {
                            for tuple_type_id in vec_type_id {
                                // A tuple entry directly refers to the parent type
                                if *tuple_type_id == *parent_type_id {
                                    *tuple_type_id = box_id
                                        .get_or_insert_with(|| self.id_to_box(parent_type_id))
                                        .clone();
                                } else {
                                    // A tuple entry optionally refers to the parent type
                                    let mut tuple_type_entry =
                                        self.id_to_entry.get_mut(&tuple_type_id).unwrap().clone();
                                    self.break_trivial_cyclic_refs(
                                        &parent_type_id,
                                        &mut tuple_type_entry,
                                        box_id,
                                    );
                                    let _ = self
                                        .id_to_entry
                                        .insert(tuple_type_id.clone(), tuple_type_entry);
                                }
                            }
                        }
                        // Look for a struct property that refers to the parent type
                        VariantDetails::Struct(vec_struct_property) => {
                            for struct_property in vec_struct_property {
                                let vec_type_id = &mut struct_property.type_id;
                                // A struct property refers to the parent type
                                if *vec_type_id == *parent_type_id {
                                    *vec_type_id = box_id
                                        .get_or_insert_with(|| self.id_to_box(parent_type_id))
                                        .clone();
                                } else {
                                    // A struct property optionally refers to
                                    // the parent type
                                    let mut prop_type_entry =
                                        self.id_to_entry.get_mut(vec_type_id).unwrap().clone();
                                    self.break_trivial_cyclic_refs(
                                        &parent_type_id,
                                        &mut prop_type_entry,
                                        box_id,
                                    );
                                    let _ = self
                                        .id_to_entry
                                        .insert(vec_type_id.clone(), prop_type_entry);
                                }
                            }
                        }
                    }
                }
            }
            // Containers that can be size 0 are *not* cyclic references for that type
            TypeEntryDetails::Array(_) => {}
            TypeEntryDetails::Set(_) => {}
            TypeEntryDetails::Map(_, _) => {}
            // Everything else can be ignored
            _ => {}
        }
    }

    /// Add a new type and return a type identifier that may be used in
    /// function signatures or embedded within other types.
    pub fn add_type(&mut self, schema: &Schema) -> Result<TypeId> {
        let (type_entry, _) = self.convert_schema(Name::Unknown, schema)?;
        Ok(self.assign_type(type_entry))
    }

    /// Add a new type with a name hint and return a the components necessary
    /// to use the type for various components of a function signature.
    pub fn add_type_with_name(
        &mut self,
        schema: &Schema,
        name_hint: Option<String>,
    ) -> Result<TypeId> {
        let name = match name_hint {
            Some(s) => Name::Suggested(s),
            None => Name::Unknown,
        };
        let (type_entry, _) = self.convert_schema(name, schema)?;
        Ok(self.assign_type(type_entry))
    }

    /// Get a type given its ID.
    pub fn get_type(&self, type_id: &TypeId) -> Result<Type> {
        let type_entry = self.id_to_entry.get(type_id).ok_or(Error::InvalidTypeId)?;
        Ok(Type {
            type_space: self,
            type_entry,
        })
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

    /// Set the name of the path prefix for types defined in this [TypeSpace].
    pub fn set_type_mod<S: AsRef<str>>(&mut self, type_mod: S) {
        self.type_mod = Some(type_mod.as_ref().to_string());
    }

    /// Add an additional derive macro to apply to all defined types.
    pub fn add_derive(&mut self, derive: TokenStream) {
        self.extra_derives.push(derive);
    }

    /// Iterate over all types including those defined in this [TypeSpace] and
    /// those referred to by those types.
    pub fn iter_types(&self) -> impl Iterator<Item = Type> {
        self.id_to_entry.values().map(move |type_entry| Type {
            type_space: self,
            type_entry,
        })
    }

    pub fn to_stream(&self) -> TokenStream {
        let type_defs = self.iter_types().map(|t| t.definition());

        quote! {
            #(#type_defs)*
        }
    }

    /// Allocated the next TypeId.
    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Assign a TypeId for a TypeEntry. This handles resolving references,
    /// checking for duplicate type definitions (e.g. to make sure there aren't
    /// two conflicting types of the same name), and deduplicates various
    /// flavors of built-in types.
    fn assign_type(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeEntryDetails::Reference(type_id) = ty.details {
            type_id
        } else if let Some(name) = ty.name() {
            // If there's already a type of this name, we make sure it's
            // identical. Note that this covers all user-defined types.

            // TODO there are many different choices we might make here
            // that could differ depending on the texture of the schema.
            // For example, a schema might use the string "Response" in a
            // bunch of places and if that were the case we might expect
            // them to be different and resolve that by renaming or scoping
            // them in some way.
            if let Some(type_id) = self.name_to_id.get(name) {
                let existing_ty = self.id_to_entry.get(type_id).unwrap();
                assert_eq!(existing_ty, &ty);
                type_id.clone()
            } else {
                let type_id = self.assign();
                self.name_to_id.insert(name.clone(), type_id.clone());
                self.id_to_entry.insert(type_id.clone(), ty);
                type_id
            }
        } else if let Some(type_id) = self.type_to_id.get(&ty.details) {
            type_id.clone()
        } else {
            let type_id = self.assign();
            self.type_to_id.insert(ty.details.clone(), type_id.clone());
            self.id_to_entry.insert(type_id.clone(), ty);
            type_id
        }
    }

    /// Convert a schema to a TypeEntry and assign it a TypeId.
    fn id_for_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (ty, meta) = self.convert_schema(type_name, schema)?;
        let type_id = self.assign_type(ty);
        Ok((type_id, meta))
    }

    /// Create an Option<T> from a pre-assigned TypeId and assign it an ID.
    fn id_to_option(&mut self, id: &TypeId) -> TypeId {
        self.assign_type(TypeEntryDetails::Option(id.clone()).into())
    }

    // Create an Option<T> from a TypeEntry by assigning it type.
    fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        TypeEntryDetails::Option(self.assign_type(ty)).into()
    }

    /// Create a Box<T> from a pre-assigned TypeId and assign it an ID.
    fn id_to_box(&mut self, id: &TypeId) -> TypeId {
        self.assign_type(TypeEntryDetails::Box(id.clone()).into())
    }
}

impl ToString for TypeSpace {
    fn to_string(&self) -> String {
        rustfmt(self.to_stream().to_string()).unwrap()
    }
}

impl<'a> Type<'a> {
    /// The name of the type as a String.
    pub fn name(&self) -> String {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.type_name(type_space)
    }

    /// The identifier for the type as might be used for a function return or
    /// defining the type of a member of a struct..
    pub fn ident(&self) -> TokenStream {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.type_ident(type_space, true)
    }

    /// The identifier for the type as might be used for a parameter in a
    /// function signature. In general: simple types are the same as
    /// [Type::ident] and complex types prepend a `&`.
    pub fn parameter_ident(&self) -> TokenStream {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.type_parameter_ident(type_space, None)
    }

    /// The identifier for the type as might be used for a parameter in a
    /// function signature along with a lifetime parameter. In general: simple
    /// types are the same as [Type::ident] and complex types prepend a
    /// `&'<lifetime>`.
    pub fn parameter_ident_with_lifetime(&self, lifetime: &str) -> TokenStream {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.type_parameter_ident(type_space, Some(lifetime))
    }

    /// The definition for this type. This will be empty for types that are
    /// already defined such as `u32` or `uuid::Uuid`.
    pub fn definition(&self) -> TokenStream {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.output(type_space)
    }

    /// A textual description of the type appropriate for debug output.
    pub fn describe(&self) -> String {
        self.type_entry.describe()
    }

    /// Get details about the type.
    pub fn details(&self) -> TypeDetails {
        match &self.type_entry.details {
            // Named user-defined types
            TypeEntryDetails::Enum(details) => TypeDetails::Enum(TypeEnum { details }),
            TypeEntryDetails::Struct(details) => TypeDetails::Struct(TypeStruct { details }),
            TypeEntryDetails::Newtype(details) => TypeDetails::Newtype(TypeNewtype { details }),

            // Compound types
            TypeEntryDetails::Option(type_id) => TypeDetails::Option(type_id.clone()),
            TypeEntryDetails::Array(type_id) => TypeDetails::Array(type_id.clone()),
            TypeEntryDetails::Map(key_id, value_id) => {
                TypeDetails::Map(key_id.clone(), value_id.clone())
            }
            TypeEntryDetails::Set(type_id) => TypeDetails::Set(type_id.clone()),
            TypeEntryDetails::Box(type_id) => TypeDetails::Box(type_id.clone()),
            TypeEntryDetails::Tuple(types) => TypeDetails::Tuple(Box::new(types.iter().cloned())),

            // Builtin types
            TypeEntryDetails::Unit => TypeDetails::Unit,
            TypeEntryDetails::BuiltIn(name)
            | TypeEntryDetails::Integral(name)
            | TypeEntryDetails::Float(name) => TypeDetails::Builtin(name.as_str()),
            TypeEntryDetails::String => TypeDetails::Builtin("String"),

            // Only used during processing; shouldn't be visible at this point
            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }
}

impl<'a> TypeEnum<'a> {
    pub fn variants(&'a self) -> impl Iterator<Item = (&'a str, TypeEnumVariant<'a>)> {
        self.details.variants.iter().map(move |variant| {
            let v = match &variant.details {
                type_entry::VariantDetails::Simple => TypeEnumVariant::Simple,
                type_entry::VariantDetails::Tuple(types) => TypeEnumVariant::Tuple(types.clone()),
                type_entry::VariantDetails::Struct(properties) => TypeEnumVariant::Struct(
                    properties
                        .iter()
                        .map(|prop| (prop.name.as_str(), prop.type_id.clone()))
                        .collect(),
                ),
            };
            (variant.name.as_str(), v)
        })
    }
}

impl<'a> TypeStruct<'a> {
    pub fn properties(&'a self) -> impl Iterator<Item = (&'a str, TypeId)> {
        self.details
            .properties
            .iter()
            .map(move |prop| (prop.name.as_str(), prop.type_id.clone()))
    }
}

impl<'a> TypeNewtype<'a> {
    pub fn subtype(&self) -> TypeId {
        self.details.type_id.clone()
    }
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashSet;

    use crate::{
        test_util::validate_output,
        type_entry::{TypeEntryEnum, VariantDetails},
        Name, TypeEntryDetails, TypeSpace,
    };

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
        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();
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

        let mut type_space = TypeSpace::default();
        type_space.add_ref_types(schema.definitions).unwrap();
        let (ty, _) = type_space
            .convert_schema_object(Name::Unknown, &schema.schema)
            .unwrap();

        match ty.details {
            TypeEntryDetails::Enum(TypeEntryEnum { variants, .. }) => {
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
            .convert_enum_string(Name::Required("OnTheGo".to_string()), &None, &enum_values)
            .unwrap();

        if let TypeEntryDetails::Option(id) = &te.details {
            let ote = type_space.id_to_entry.get(id).unwrap();
            if let TypeEntryDetails::Enum(TypeEntryEnum { variants, .. }) = &ote.details {
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

    #[test]
    fn test_alias() {
        #[derive(JsonSchema, Schema)]
        struct Stuff(Vec<String>);

        #[allow(dead_code)]
        #[derive(JsonSchema, Schema)]
        struct Things {
            a: String,
            b: Stuff,
        }

        validate_output::<Things>();
    }
}
