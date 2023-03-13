// Copyright 2023 Oxide Computer Company

use std::collections::{BTreeMap, BTreeSet};

use conversions::SchemaCache;
use log::info;
use output::OutputSpace;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rustfmt_wrapper::rustfmt;
use schemars::schema::{Metadata, Schema};
use thiserror::Error;
use type_entry::{
    StructPropertyState, TypeEntry, TypeEntryDetails, TypeEntryNative, TypeEntryNewtype,
    VariantDetails, WrappedValue,
};

use crate::util::{sanitize, Case};

#[cfg(test)]
mod test_util;

mod conversions;
mod convert;
mod defaults;
mod enums;
mod output;
mod structs;
mod type_entry;
mod util;
mod value;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("invalid TypeId")]
    InvalidTypeId,
    #[error("value does not conform to the given schema")]
    InvalidValue,
    #[error("schema invalid: {0}")]
    InvalidSchema(String),
}

impl Error {
    fn invalid_value() -> Self {
        Self::InvalidValue
    }
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
    Map(TypeId),
    Set(TypeId),
    Box(TypeId),
    Tuple(Box<dyn Iterator<Item = TypeId> + 'a>),
    Builtin(&'a str),

    Unit,
    String,
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

pub struct TypeStructPropInfo<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub required: bool,
    pub type_id: TypeId,
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
    pub fn into_option(self) -> Option<String> {
        match self {
            Name::Required(s) | Name::Suggested(s) => Some(s),
            Name::Unknown => None,
        }
    }

    pub fn append(&self, s: &str) -> Self {
        match self {
            Name::Required(prefix) | Name::Suggested(prefix) => {
                Self::Suggested(format!("{}_{}", prefix, s))
            }
            Name::Unknown => Name::Unknown,
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

    id_to_entry: BTreeMap<TypeId, TypeEntry>,
    type_to_id: BTreeMap<TypeEntryDetails, TypeId>,

    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<String, TypeId>,

    uses_chrono: bool,
    uses_uuid: bool,
    uses_serde_json: bool,
    uses_regress: bool,

    settings: TypeSpaceSettings,

    cache: SchemaCache,

    // Shared functions for generating default values
    defaults: BTreeSet<DefaultImpl>,
}

impl Default for TypeSpace {
    fn default() -> Self {
        Self {
            next_id: 1,
            definitions: Default::default(),
            id_to_entry: Default::default(),
            type_to_id: Default::default(),
            name_to_id: Default::default(),
            ref_to_id: Default::default(),
            uses_chrono: Default::default(),
            uses_uuid: Default::default(),
            uses_serde_json: Default::default(),
            uses_regress: Default::default(),
            settings: Default::default(),
            cache: Default::default(),
            defaults: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DefaultImpl {
    Boolean,
    I64,
    U64,
}

/// Settings that alter type generation.
#[derive(Debug, Default, Clone)]
pub struct TypeSpaceSettings {
    type_mod: Option<String>,
    extra_derives: Vec<String>,
    struct_builder: bool,

    patch: BTreeMap<String, TypeSpacePatch>,
    replace: BTreeMap<String, TypeSpaceReplace>,
    convert: Vec<TypeSpaceConversion>,
}

/// Contains a set of modifications that may be applied to an existing type.
#[derive(Debug, Default, Clone)]
pub struct TypeSpacePatch {
    rename: Option<String>,
    derives: Vec<String>,
}

/// Contains the attributes of a replacement of an existing type.
#[derive(Debug, Default, Clone)]
pub struct TypeSpaceReplace {
    replace_type: String,
    impls: Vec<TypeSpaceImpl>,
}

/// Defines a schema which will be replaced, and the attributes of the
/// replacement.
#[derive(Debug, Clone)]
struct TypeSpaceConversion {
    schema: schemars::schema::SchemaObject,
    type_name: String,
    impls: Vec<TypeSpaceImpl>,
}

// TODO we can currently only address traits for which cycle analysis is not
// required.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TypeSpaceImpl {
    FromStr,
    Display,
    Default,
}

impl std::str::FromStr for TypeSpaceImpl {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "FromStr" => Ok(Self::FromStr),
            "Display" => Ok(Self::Display),
            "Default" => Ok(Self::Default),
            _ => Err(format!("{} is not a valid trait specifier", s)),
        }
    }
}

impl TypeSpaceSettings {
    /// Set the name of the path prefix for types defined in this [TypeSpace].
    pub fn with_type_mod<S: AsRef<str>>(&mut self, type_mod: S) -> &mut Self {
        self.type_mod = Some(type_mod.as_ref().to_string());
        self
    }

    /// Add an additional derive macro to apply to all defined types.
    pub fn with_derive(&mut self, derive: String) -> &mut Self {
        if !self.extra_derives.contains(&derive) {
            self.extra_derives.push(derive);
        }
        self
    }

    /// For structs, include a "builder" type that can be used to construct it.
    pub fn with_struct_builder(&mut self, struct_builder: bool) -> &mut Self {
        self.struct_builder = struct_builder;
        self
    }

    /// Replace a referenced type with a named type. This causes the referenced
    /// type *not* to be generated. If the same `type_name` is specified multiple times,
    /// the last one is honored.
    pub fn with_replacement<TS: ToString, RS: ToString, I: Iterator<Item = TypeSpaceImpl>>(
        &mut self,
        type_name: TS,
        replace_type: RS,
        impls: I,
    ) -> &mut Self {
        self.replace.insert(
            type_name.to_string(),
            TypeSpaceReplace {
                replace_type: replace_type.to_string(),
                impls: impls.collect(),
            },
        );
        self
    }

    /// Modify a type with the given name. Note that specifying a type not
    /// created by the input JSON schema does **not** result in an error and is
    /// silently ignored. If the same `type_name` is specified multiple times,
    /// the last one is honored.
    pub fn with_patch<S: ToString>(
        &mut self,
        type_name: S,
        type_patch: &TypeSpacePatch,
    ) -> &mut Self {
        self.patch.insert(type_name.to_string(), type_patch.clone());
        self
    }

    /// Replace a given schema with a named type. The given schema must precisely
    /// match the schema from the input, including fields such as `description`.
    /// Typical usage is to map a schema definition to a builtin type or type
    /// provided by a crate, such as `'rust_decimal::Decimal'`. If the same schema
    /// is specified multiple times, the first one is honored.
    pub fn with_conversion<S: ToString, I: Iterator<Item = TypeSpaceImpl>>(
        &mut self,
        schema: schemars::schema::SchemaObject,
        type_name: S,
        impls: I,
    ) -> &mut Self {
        self.convert.push(TypeSpaceConversion {
            schema,
            type_name: type_name.to_string(),
            impls: impls.collect(),
        });
        self
    }
}

impl TypeSpacePatch {
    /// Specify the new name for patched type.
    pub fn with_rename<S: ToString>(&mut self, rename: S) -> &mut Self {
        self.rename = Some(rename.to_string());
        self
    }

    /// Specify an additional derive to apply to the patched type.
    pub fn with_derive<S: ToString>(&mut self, derive: S) -> &mut Self {
        self.derives.push(derive.to_string());
        self
    }
}

impl TypeSpace {
    /// Create a new TypeSpace with custom settings
    pub fn new(settings: &TypeSpaceSettings) -> Self {
        let mut cache = SchemaCache::default();

        settings.convert.iter().for_each(
            |TypeSpaceConversion {
                 schema,
                 type_name,
                 impls,
             }| {
                cache.insert(schema, type_name, impls);
            },
        );

        Self {
            settings: settings.clone(),
            cache,
            ..Default::default()
        }
    }

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

        for (index, (ref_name, schema)) in definitions.iter().enumerate() {
            self.ref_to_id
                .insert(ref_name.to_string(), TypeId(base_id + index as u64));
            self.definitions.insert(ref_name.clone(), schema.clone());
        }

        // Convert all types; note that we use the type id assigned from the
        // previous step because each type may create additional types. This
        // effectively is doing the work of `add_type_with_name` but for a
        // batch of types.
        for (index, (type_name, schema)) in definitions.into_iter().enumerate() {
            info!(
                "converting type: {} with schema {}",
                type_name,
                serde_json::to_string(&schema).unwrap()
            );

            // Check for manually replaced types. Proceed with type conversion
            // if there is none; use the specified type if there is.
            let type_id = TypeId(base_id + index as u64);
            let check_name = sanitize(&type_name, Case::Pascal);
            match self.settings.replace.get(&check_name) {
                None => self.convert_ref_type(&type_name, schema, type_id)?,

                Some(replace_type) => {
                    let type_entry = TypeEntry::new_native(
                        replace_type.replace_type.clone(),
                        &replace_type.impls.clone(),
                    );
                    self.id_to_entry.insert(type_id, type_entry);
                }
            }
        }

        // Now that all references have been processed, we can do some
        // additional validation and processing.
        for index in 0..def_len {
            // This is slightly inefficient, but we make a copy of the type in
            // order to manipulate it without holding a reference on self.
            let type_id = TypeId(base_id + index);
            let mut type_entry = self.id_to_entry.get(&type_id).unwrap().clone();

            // TODO compute appropriate derives, taking care to account for
            // dependency cycles. Currently we're using a more minimal--safe--
            // set of derives than we might otherwise. This notably prevents us
            // from using a HashSet or BTreeSet type where we might like to.

            // Once all ref types are in, look for containment cycles that we
            // need to break with a Box<T>. Note that we unconditionally
            // replace the type entry at the given ID regardless of whether the
            // type changes.

            // TODO: we've declared box_id here to avoid allocating it in the
            // ID space twice, but the dedup logic in assign_type() should
            // already address this. There's room to simplify here...
            let mut box_id = None;
            self.break_trivial_cyclic_refs(&type_id, &mut type_entry, &mut box_id);

            // Overwrite the entry regardless of whether we modified it.
            self.id_to_entry.insert(type_id, type_entry);
        }

        // Finalize all created types.
        for index in base_id..self.next_id {
            let type_id = TypeId(index);
            let mut type_entry = self.id_to_entry.get(&type_id).unwrap().clone();
            type_entry.finalize(self)?;
            self.id_to_entry.insert(type_id, type_entry);
        }

        Ok(())
    }

    fn convert_ref_type(&mut self, type_name: &str, schema: Schema, type_id: TypeId) -> Result<()> {
        let (mut type_entry, metadata) =
            self.convert_schema(Name::Required(type_name.to_string()), &schema)?;
        let default = metadata
            .as_ref()
            .and_then(|m| m.default.as_ref())
            .cloned()
            .map(WrappedValue::new);
        let type_entry = match &mut type_entry.details {
            // The types that are already named are good to go.
            TypeEntryDetails::Enum(details) => {
                details.default = default;
                type_entry
            }
            TypeEntryDetails::Struct(details) => {
                details.default = default;
                type_entry
            }
            TypeEntryDetails::Newtype(details) => {
                details.default = default;
                type_entry
            }

            // If the type entry is a reference, then this definition is a
            // simple alias to another type in this list of definitions
            // (which may nor may not have already been converted). We
            // simply create a newtype with that type ID.
            TypeEntryDetails::Reference(type_id) => TypeEntryNewtype::from_metadata(
                self,
                Name::Required(type_name.to_string()),
                metadata,
                type_id.clone(),
            ),

            // For types that don't have names, this is effectively a type
            // alias which we treat as a newtype.
            _ => {
                let subtype_id = self.assign_type(type_entry);

                TypeEntryNewtype::from_metadata(
                    self,
                    Name::Required(type_name.to_string()),
                    metadata,
                    subtype_id,
                )
            }
        };
        let entry_name = type_entry.name().unwrap().clone();
        self.name_to_id.insert(entry_name, type_id.clone());
        self.id_to_entry.insert(type_id, type_entry);
        Ok(())
    }

    /// If a type refers to itself, this creates a cycle that will eventually
    /// be emit as a Rust struct that cannot be constructed. Break those cycles
    /// here.
    ///
    /// While we aren't yet handling the general case of type containment
    /// cycles, it's not that bad to look at trivial cycles such as:
    ///
    ///   1) A type referring to itself: A -> A
    ///   2) A type optionally referring to itself: A -> Option<A>
    ///   3) An enum variant referring to itself, either optionally or directly
    ///   
    /// TODO currently only trivial cycles are broken. A more generic solution
    /// may be required, but it may also a point to ask oneself why such a
    /// complicated type is required :) A generic solution is difficult because
    /// certain cycles introduce a question of *where* to Box to break the
    /// cycle, and there's no one answer to this.
    fn check_for_cyclic_ref(
        &mut self,
        parent_type_id: &TypeId,
        child_type_id: &mut TypeId,
        box_id: &mut Option<TypeId>,
    ) {
        if *child_type_id == *parent_type_id {
            *child_type_id = box_id
                .get_or_insert_with(|| self.id_to_box(parent_type_id))
                .clone();
        } else {
            let mut child_type_entry = self.id_to_entry.get_mut(child_type_id).unwrap().clone();

            if let TypeEntryDetails::Option(option_type_id) = &mut child_type_entry.details {
                if *option_type_id == *parent_type_id {
                    *option_type_id = box_id
                        .get_or_insert_with(|| self.id_to_box(parent_type_id))
                        .clone();
                }
            }

            let _ = self
                .id_to_entry
                .insert(child_type_id.clone(), child_type_entry);
        }
    }

    fn break_trivial_cyclic_refs(
        &mut self,
        parent_type_id: &TypeId,
        type_entry: &mut TypeEntry,
        box_id: &mut Option<TypeId>,
    ) {
        match &mut type_entry.details {
            // Look for the case where a struct property refers to the parent
            // type
            TypeEntryDetails::Struct(s) => {
                for prop in &mut s.properties {
                    self.check_for_cyclic_ref(parent_type_id, &mut prop.type_id, box_id);
                }
            }

            // Look for the cases where an enum variant refers to the parent
            // type
            TypeEntryDetails::Enum(type_entry_enum) => {
                for variant in &mut type_entry_enum.variants {
                    match &mut variant.details {
                        // Simple variants will not refer to anything
                        VariantDetails::Simple => {}
                        // Look for a single-item tuple that refers to the
                        // parent type.
                        VariantDetails::Item(item_type_id) => {
                            self.check_for_cyclic_ref(parent_type_id, item_type_id, box_id);
                        }
                        // Look for a tuple entry that refers to the parent
                        // type.
                        VariantDetails::Tuple(vec_type_id) => {
                            for tuple_type_id in vec_type_id {
                                self.check_for_cyclic_ref(parent_type_id, tuple_type_id, box_id);
                            }
                        }
                        // Look for a struct property that refers to the parent
                        // type.
                        VariantDetails::Struct(vec_struct_property) => {
                            for struct_property in vec_struct_property {
                                let vec_type_id = &mut struct_property.type_id;
                                self.check_for_cyclic_ref(parent_type_id, vec_type_id, box_id);
                            }
                        }
                    }
                }
            }

            // Look for cases where a newtype refers to a parent type
            TypeEntryDetails::Newtype(new_type_entry) => {
                self.check_for_cyclic_ref(parent_type_id, &mut new_type_entry.type_id, box_id);
            }

            // Containers that can be size 0 are *not* cyclic references for that type
            TypeEntryDetails::Array(_) => {}
            TypeEntryDetails::Set(_) => {}
            TypeEntryDetails::Map(_) => {}

            // Everything else can be ignored
            _ => {}
        }
    }

    /// Add a new type and return a type identifier that may be used in
    /// function signatures or embedded within other types.
    pub fn add_type(&mut self, schema: &Schema) -> Result<TypeId> {
        self.add_type_with_name(schema, None)
    }

    /// Add a new type with a name hint and return a the components necessary
    /// to use the type for various components of a function signature.
    pub fn add_type_with_name(
        &mut self,
        schema: &Schema,
        name_hint: Option<String>,
    ) -> Result<TypeId> {
        let base_id = self.next_id;

        let name = match name_hint {
            Some(s) => Name::Suggested(s),
            None => Name::Unknown,
        };
        let (type_id, _) = self.id_for_schema(name, schema)?;

        // Finalize all created types.
        for index in base_id..self.next_id {
            let type_id = TypeId(index);
            let mut type_entry = self.id_to_entry.get(&type_id).unwrap().clone();
            type_entry.finalize(self)?;
            self.id_to_entry.insert(type_id, type_entry);
        }

        Ok(type_id)
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

    pub fn uses_regress(&self) -> bool {
        self.uses_regress
    }

    pub fn uses_serde_json(&self) -> bool {
        self.uses_serde_json
    }

    pub fn uses_uuid(&self) -> bool {
        self.uses_uuid
    }

    /// Iterate over all types including those defined in this [TypeSpace] and
    /// those referred to by those types.
    pub fn iter_types(&self) -> impl Iterator<Item = Type> {
        self.id_to_entry.values().map(move |type_entry| Type {
            type_space: self,
            type_entry,
        })
    }

    /// Common code, shared by types.
    pub fn common_code(&self) -> TokenStream {
        if self.defaults.is_empty() {
            quote! {}
        } else {
            let fns = self.defaults.iter().map(TokenStream::from);
            quote! {
                mod defaults {
                    #(#fns)*
                }
            }
        }
    }
    /// All code for processed types.
    pub fn to_stream(&self) -> TokenStream {
        let mut output = OutputSpace::default();

        // Add all types.
        self.id_to_entry
            .values()
            .for_each(|type_entry| type_entry.output(self, &mut output));

        // Add all shared default functions.
        self.defaults
            .iter()
            .for_each(|x| output.add_item(output::OutputSpaceMod::Defaults, "", x.into()));

        output.into_stream()
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
                // TODO we'd like to verify that the type is structurally the
                // same, but the types may not be functionally equal. This is a
                // consequence of types being "finalized" after each type
                // addition. This further emphasized the need for a more
                // deliberate, multi-pass approach.
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
    ///
    /// This is used for sub-types such as the type of an array or the types of
    /// properties of a struct.
    fn id_for_schema<'a>(
        &mut self,
        type_name: Name,
        schema: &'a Schema,
    ) -> Result<(TypeId, &'a Option<Box<Metadata>>)> {
        let (mut type_entry, metadata) = self.convert_schema(type_name, schema)?;
        if let Some(metadata) = metadata {
            let default = metadata.default.clone().map(WrappedValue::new);
            match &mut type_entry.details {
                TypeEntryDetails::Enum(details) => {
                    details.default = default;
                }
                TypeEntryDetails::Struct(details) => {
                    details.default = default;
                }
                TypeEntryDetails::Newtype(details) => {
                    details.default = default;
                }
                _ => (),
            }
        }
        let type_id = self.assign_type(type_entry);
        Ok((type_id, metadata))
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

impl ToTokens for TypeSpace {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_stream())
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
        type_entry.type_ident(type_space, &type_space.settings.type_mod)
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
            TypeEntryDetails::Map(type_id) => TypeDetails::Map(type_id.clone()),
            TypeEntryDetails::Set(type_id) => TypeDetails::Set(type_id.clone()),
            TypeEntryDetails::Box(type_id) => TypeDetails::Box(type_id.clone()),
            TypeEntryDetails::Tuple(types) => TypeDetails::Tuple(Box::new(types.iter().cloned())),

            // Builtin types
            TypeEntryDetails::Unit => TypeDetails::Unit,
            TypeEntryDetails::Native(TypeEntryNative {
                type_name: name, ..
            })
            | TypeEntryDetails::Integer(name)
            | TypeEntryDetails::Float(name) => TypeDetails::Builtin(name.as_str()),
            TypeEntryDetails::Boolean => TypeDetails::Builtin("bool"),
            TypeEntryDetails::String => TypeDetails::String,

            // Only used during processing; shouldn't be visible at this point
            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }

    /// Checks if the type has the associated impl.
    pub fn has_impl(&self, impl_name: TypeSpaceImpl) -> bool {
        let Type {
            type_space,
            type_entry,
        } = self;
        type_entry.has_impl(type_space, impl_name)
    }
}

impl<'a> TypeEnum<'a> {
    pub fn variants(&'a self) -> impl Iterator<Item = (&'a str, TypeEnumVariant<'a>)> {
        self.details.variants.iter().map(move |variant| {
            let v = match &variant.details {
                type_entry::VariantDetails::Simple => TypeEnumVariant::Simple,
                // The distinction between a lone item variant and a tuple
                // variant with a single item is only relevant internally.
                type_entry::VariantDetails::Item(type_id) => {
                    TypeEnumVariant::Tuple(vec![type_id.clone()])
                }
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

    pub fn properties_info(&'a self) -> impl Iterator<Item = TypeStructPropInfo> {
        self.details
            .properties
            .iter()
            .map(move |prop| TypeStructPropInfo {
                name: prop.name.as_str(),
                description: prop.description.as_deref(),
                required: matches!(&prop.state, StructPropertyState::Required),
                type_id: prop.type_id.clone(),
            })
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
        output::OutputSpace,
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

        let mut output = OutputSpace::default();
        ty.output(&type_space, &mut output);
        println!("{}", output.into_stream());

        for ty in type_space.id_to_entry.values() {
            println!("{:#?}", ty);
            let mut output = OutputSpace::default();
            ty.output(&type_space, &mut output);
            println!("{}", output.into_stream());
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
                let mut output = OutputSpace::default();
                ty.output(&type_space, &mut output);
                println!("{}", output.into_stream());
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
            .convert_enum_string(
                Name::Required("OnTheGo".to_string()),
                &None,
                &enum_values,
                None,
            )
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
