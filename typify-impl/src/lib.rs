// Copyright 2026 Oxide Computer Company

//! typify backend implementation.

#![deny(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use conversions::SchemaCache;
use log::{debug, info};
use proc_macro2::TokenStream;
use quote::ToTokens;
use schemars::schema::{Metadata, RootSchema, Schema};
use thiserror::Error;
use type_entry::{TypeEntry, TypeEntryNewtype};

use crate::util::{sanitize, Case};

pub use crate::util::accept_as_ident;

/// The typespace crate, re-exported for consumers of this one.
///
/// [`TypeSpace::to_typespace`] yields a [`typespace::Typespace`];
/// its view API answers type queries (identifiers, structure, trait
/// impls) that used to live on this crate's own wrapper types.
pub use ::typespace;

#[cfg(test)]
mod test_util;

mod conversions;
mod convert;
mod cycles;
mod defaults;
mod enums;
mod merge;
mod rust_extension;
mod structs;
mod type_entry;
mod typespace_ext;
mod util;
mod validate;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("value does not conform to the given schema")]
    InvalidValue,
    #[error(transparent)]
    Typespace(#[from] typespace::error::Error<TypeId>),
    #[error("invalid schema for {}: {reason}", show_type_name(.type_name.as_deref()))]
    InvalidSchema {
        type_name: Option<String>,
        reason: String,
    },
}

impl Error {
    fn invalid_value() -> Self {
        Self::InvalidValue
    }
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;

fn show_type_name(type_name: Option<&str>) -> &str {
    type_name.unwrap_or("<unknown type>")
}

/// Type identifier returned from type creation and used to lookup types.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
pub struct TypeId(u64);

// typespace requires its Id type to implement Display (for error
// reporting); typify's TypeId is an opaque integer, so this shows the
// number.
impl std::fmt::Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum RefKey {
    Root,
    Def(String),
}

/// A collection of types under conversion from JSON Schema.
///
/// Add schemas with the `add_*` methods. Querying and rendering are
/// typespace's job: [`TypeSpace::to_typespace`] finalizes the
/// collected types into a [`typespace::Typespace`] whose view API
/// answers identifier, structure, and trait-impl queries;
/// [`TypeSpace::to_stream`] renders everything to code.
#[derive(Debug)]
pub struct TypeSpace {
    next_id: u64,

    // TODO we need this in order to inspect the collection of reference types
    // e.g. to do `all_mutually_exclusive`. In the future, we could obviate the
    // need this by keeping a single Map of referenced types whose value was an
    // enum of a "raw" or a "converted" schema.
    definitions: BTreeMap<RefKey, Schema>,

    id_to_entry: BTreeMap<TypeId, TypeEntry>,

    name_to_id: BTreeMap<String, TypeId>,
    ref_to_id: BTreeMap<RefKey, TypeId>,

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
    NZU64,
}

impl DefaultImpl {
    /// The name of the shared function this renders as.
    fn fn_name(&self) -> &'static str {
        match self {
            DefaultImpl::Boolean => "default_bool",
            DefaultImpl::I64 => "default_i64",
            DefaultImpl::U64 => "default_u64",
            DefaultImpl::NZU64 => "default_nzu64",
        }
    }
}

/// Type name to use in generated code.
#[derive(Clone)]
pub struct MapType(pub syn::Type);

impl MapType {
    /// Create a new MapType from a [`str`].
    ///
    /// # Panics
    ///
    /// Panics if `s` cannot be parsed as a Rust type. Prefer
    /// [`str::parse`] (via the [`FromStr`](std::str::FromStr)
    /// implementation) to handle invalid input without panicking.
    pub fn new(s: &str) -> Self {
        let map_type = syn::parse_str::<syn::Type>(s).expect("valid ident");
        Self(map_type)
    }
}

impl std::str::FromStr for MapType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let map_type = syn::parse_str::<syn::Type>(s)
            .map_err(|err| format!("invalid map type {s:?}: {err}"))?;
        Ok(Self(map_type))
    }
}

impl Default for MapType {
    fn default() -> Self {
        Self::new("::std::collections::HashMap")
    }
}

impl std::fmt::Debug for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapType({})", self.0.to_token_stream())
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_token_stream().fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for MapType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl From<String> for MapType {
    /// # Panics
    ///
    /// Panics if `s` cannot be parsed as a Rust type. Prefer
    /// [`str::parse`] (via the [`FromStr`](std::str::FromStr)
    /// implementation) to handle invalid input without panicking.
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl From<&str> for MapType {
    /// # Panics
    ///
    /// Panics if `s` cannot be parsed as a Rust type. Prefer
    /// [`str::parse`] (via the [`FromStr`](std::str::FromStr)
    /// implementation) to handle invalid input without panicking.
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<syn::Type> for MapType {
    fn from(t: syn::Type) -> Self {
        Self(t)
    }
}

/// Settings that alter type generation.
// [typespace migration note] typify1 had a `type_mod` setting that
// qualified every named-type identifier query with a module path;
// typespace makes scope a per-query argument (`ident_in`,
// `parameter_ident_in`), so the setting is gone and callers pass the
// scope where they ask for identifiers.
#[derive(Default, Debug, Clone)]
pub struct TypeSpaceSettings {
    extra_derives: Vec<String>,
    extra_attrs: Vec<String>,
    struct_builder: bool,

    unknown_crates: UnknownPolicy,
    crates: BTreeMap<String, CrateSpec>,
    map_type: MapType,

    patch: BTreeMap<String, TypeSpacePatch>,
    replace: BTreeMap<String, TypeSpaceReplace>,
    convert: Vec<TypeSpaceConversion>,
}

#[derive(Debug, Clone)]
struct CrateSpec {
    version: CrateVers,
    rename: Option<String>,
}

/// Policy to apply to external types described by schema extensions whose
/// crates are not explicitly specified.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize)]
pub enum UnknownPolicy {
    /// Generate the type rather according to the schema.
    #[default]
    Generate,
    /// Use the specified type by path (this will result in a compile error if
    /// one of the crates is not an existing dependency). Note that this
    /// ignores compatibility requirements specified by the schema extension
    /// and may result in subtle failures if the crate used is incompatible
    /// with the version that produced the schema.
    Allow,
    /// If an unknown crate is encountered, generate a compiler warning
    /// indicating the crate that must be specified to proceed along with
    /// version constraints. This affords users an opportunity to specify the
    /// specific crate version to use (or the user may explicitly deny use of
    /// that crate).
    Deny,
}

/// Specify the version for a named crate to consider for type use (rather than
/// generating types) in the presense of a schema extension.
#[derive(Debug, Clone)]
pub enum CrateVers {
    /// An explicit version.
    Version(semver::Version),
    /// Any version.
    Any,
    /// Never use the given crate.
    Never,
}

impl CrateVers {
    /// Parse from a string
    pub fn parse(s: &str) -> Option<Self> {
        if s == "!" {
            Some(Self::Never)
        } else if s == "*" {
            Some(Self::Any)
        } else {
            Some(Self::Version(semver::Version::parse(s).ok()?))
        }
    }
}

/// Contains a set of modifications that may be applied to an existing type.
#[derive(Debug, Default, Clone)]
pub struct TypeSpacePatch {
    rename: Option<String>,
    derives: Vec<String>,
    attrs: Vec<String>,
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

#[allow(missing_docs)]
// TODO we can currently only address traits for which cycle analysis is not
// required.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TypeSpaceImpl {
    FromStr,
    FromStringIrrefutable,
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

impl TypeSpaceImpl {
    /// Translate consumer-supplied capability markers into the trait
    /// set typespace expects a native to declare.
    ///
    /// This backs `with_conversion` and `with_replacement`, the two
    /// public settings that let a consumer name an opaque native type;
    /// typify cannot verify the consumer's claim the way it verifies
    /// its own built-in natives, so it takes the markers at face
    /// value. typify has always assumed the basic complement (`Clone`,
    /// `Debug`, `Serialize`, `Deserialize`) regardless of what markers
    /// are given, so those are added unconditionally. `JsonSchema`
    /// joins them: a requested derive was always emitted without
    /// consulting the conversion target, so a named native must
    /// satisfy it. `FromStringIrrefutable` has no typespace equivalent
    /// (see typespace_ext.rs gap note 2) and is dropped.
    pub(crate) fn native_traits(impls: &[Self]) -> typespace::TypespaceTraitSet {
        let mut traits = [
            typespace::TypespaceTrait::Clone,
            typespace::TypespaceTrait::Debug,
            typespace::TypespaceTrait::Serialize,
            typespace::TypespaceTrait::Deserialize,
            typespace::TypespaceTrait::JsonSchema,
        ]
        .into_iter()
        .collect::<typespace::TypespaceTraitSet>();
        for impl_ in impls {
            match impl_ {
                Self::FromStr => traits.add(typespace::TypespaceTrait::FromStr),
                Self::Display => traits.add(typespace::TypespaceTrait::Display),
                Self::Default => traits.add(typespace::TypespaceTrait::Default),
                Self::FromStringIrrefutable => {}
            }
        }
        traits
    }
}

impl TypeSpaceSettings {
    /// Add an additional derive macro to apply to all defined types.
    pub fn with_derive(&mut self, derive: String) -> &mut Self {
        if !self.extra_derives.contains(&derive) {
            self.extra_derives.push(derive);
        }
        self
    }

    /// Add an additional attribute to apply to all defined types.
    pub fn with_attr(&mut self, attr: String) -> &mut Self {
        if !self.extra_attrs.contains(&attr) {
            self.extra_attrs.push(attr);
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
    ///
    /// # Examples
    ///
    /// ```
    /// // Setup 'number' json type to be translated into 'rust_decimal::Decimal'
    /// use schemars::schema::{InstanceType, SchemaObject};
    /// use typify_impl::{TypeSpace, TypeSpaceImpl, TypeSpaceSettings};
    /// let mut type_space = TypeSpace::new(
    ///        TypeSpaceSettings::default()
    ///            .with_struct_builder(true)
    ///            .with_conversion(
    ///                SchemaObject {
    ///                    instance_type: Some(InstanceType::Number.into()),
    ///                    ..Default::default()
    ///                },
    ///                "::rust_decimal::Decimal",
    ///                [TypeSpaceImpl::Display].into_iter(),
    ///            ),
    ///    );
    /// ```
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

    /// Type schemas may contain an extension (`x-rust-type`) that indicates
    /// the corresponding Rust type within a particular crate. This function
    /// changes the disposition regarding crates not otherwise specified via
    /// [`Self::with_crate`]. The default value is `false`.
    pub fn with_unknown_crates(&mut self, policy: UnknownPolicy) -> &mut Self {
        self.unknown_crates = policy;
        self
    }

    /// Type schemas may contain an extension (`x-rust-type`) that indicates
    /// the corresponding Rust type within a particular crate. This extension
    /// indicates the crate, version compatibility, type path, and type
    /// parameters. This function modifies settings to use (rather than
    /// generate) types from the given crate and version. The version should
    /// precisely match the version of the crate that you expect as a
    /// dependency.
    pub fn with_crate<S1: ToString>(
        &mut self,
        crate_name: S1,
        version: CrateVers,
        rename: Option<&String>,
    ) -> &mut Self {
        self.crates.insert(
            crate_name.to_string(),
            CrateSpec {
                version,
                rename: rename.cloned(),
            },
        );
        self
    }

    /// Specify the map-like type to be used in generated code.
    ///
    /// ## Requirements
    ///
    /// - An `is_empty` method that returns a boolean
    /// - Two generic parameters, `K` and `V`
    /// - [`Default`] + [`Clone`] + [`Debug`] +
    ///   [`Serialize`][serde::Serialize] + [`Deserialize`][serde::Deserialize]
    ///
    /// ## Examples
    ///
    /// - [`::std::collections::HashMap`]
    /// - [`::std::collections::BTreeMap`]
    /// - [`::indexmap::IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html)
    pub fn with_map_type<T: Into<MapType>>(&mut self, map_type: T) -> &mut Self {
        self.map_type = map_type.into();
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

    /// Specify an additional attribute to apply to the patched type.
    pub fn with_attr<S: ToString>(&mut self, attr: S) -> &mut Self {
        self.attrs.push(attr.to_string());
        self
    }
}

impl TypeSpace {
    /// Create a new TypeSpace with custom settings.
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
        self.add_ref_types_impl(
            type_defs
                .into_iter()
                .map(|(key, schema)| (RefKey::Def(key.as_ref().to_string()), schema)),
        )
    }

    fn add_ref_types_impl<I>(&mut self, type_defs: I) -> Result<()>
    where
        I: IntoIterator<Item = (RefKey, Schema)>,
    {
        // Gather up all types to make things a little more convenient.
        let definitions = type_defs.into_iter().collect::<Vec<_>>();

        // Assign IDs to reference types before actually converting them. We'll
        // need these in the case of forward (or circular) references.
        let base_id = self.next_id;
        let def_len = definitions.len() as u64;
        self.next_id += def_len;

        for (index, (ref_name, schema)) in definitions.iter().enumerate() {
            self.ref_to_id
                .insert(ref_name.clone(), TypeId(base_id + index as u64));
            self.definitions.insert(ref_name.clone(), schema.clone());
        }

        // Convert all types; note that we use the type id assigned from the
        // previous step because each type may create additional types. This
        // effectively is doing the work of `add_type_with_name` but for a
        // batch of types.
        for (index, (ref_name, schema)) in definitions.into_iter().enumerate() {
            info!(
                "converting type: {:?} with schema {}",
                ref_name,
                serde_json::to_string(&schema).unwrap()
            );

            // Check for manually replaced types. Proceed with type conversion
            // if there is none; use the specified type if there is.
            let type_id = TypeId(base_id + index as u64);

            let maybe_replace = match &ref_name {
                RefKey::Root => None,
                RefKey::Def(def_name) => {
                    let check_name = sanitize(def_name, Case::Pascal);
                    self.settings.replace.get(&check_name)
                }
            };

            match maybe_replace {
                None => {
                    let type_name = if let RefKey::Def(name) = ref_name {
                        Name::Required(name.clone())
                    } else {
                        Name::Unknown
                    };
                    self.convert_ref_type(type_name, schema, type_id)?
                }

                Some(replace_type) => {
                    let type_entry = TypeEntry::new_native(
                        replace_type.replace_type.clone(),
                        TypeSpaceImpl::native_traits(&replace_type.impls),
                    );
                    self.id_to_entry.insert(type_id, type_entry);
                }
            }
        }

        // Eliminate cycles. It's sufficient to only start from referenced
        // types as a reference is required to make a cycle.
        self.break_cycles(base_id..base_id + def_len);

        // Finalize all created types.
        for index in base_id..self.next_id {
            let type_id = TypeId(index);
            let mut type_entry = self.id_to_entry.get(&type_id).unwrap().clone();
            debug!("finalizing type entry: {} {:#?}", index, &type_entry);
            type_entry.finalize(self)?;
            self.id_to_entry.insert(type_id, type_entry);
        }

        Ok(())
    }

    fn convert_ref_type(&mut self, type_name: Name, schema: Schema, type_id: TypeId) -> Result<()> {
        let (mut type_entry, metadata) = self.convert_schema(type_name.clone(), &schema)?;
        let default = metadata
            .as_ref()
            .and_then(|m| m.default.as_ref())
            .cloned()
            .map(typespace::build::JsonValue::new);
        let type_entry = match &mut type_entry {
            // The types that are already named are good to go.
            TypeEntry::Type(typ) if typ.is_named() => {
                typ.set_default(default);
                type_entry
            }

            // If the type entry is a reference, then this definition is a
            // simple alias to another type in this list of definitions
            // (which may nor may not have already been converted). We
            // simply create a newtype with that type ID.
            TypeEntry::Reference(type_id) => TypeEntryNewtype::from_metadata(
                self,
                type_name,
                metadata,
                type_id.clone(),
                schema.clone(),
            ),

            TypeEntry::Type(typespace::build::Type::Native(native))
                if native_name_match(native, &type_name) =>
            {
                type_entry
            }

            // For types that don't have names, this is effectively a type
            // alias which we treat as a newtype.
            _ => {
                info!(
                    "type alias {:?} {}\n{:?}",
                    type_name,
                    serde_json::to_string_pretty(&schema).unwrap(),
                    metadata
                );
                let subtype_id = self.assign_type(type_entry);
                TypeEntryNewtype::from_metadata(
                    self,
                    type_name,
                    metadata,
                    subtype_id,
                    schema.clone(),
                )
            }
        };
        // TODO need a type alias?
        if let Some(entry_name) = type_entry.name() {
            self.name_to_id
                .insert(entry_name.to_string(), type_id.clone());
        }
        self.id_to_entry.insert(type_id, type_entry);
        Ok(())
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

    /// Add all the types contained within a RootSchema including any
    /// referenced types and the top-level type (if there is one and it has a
    /// title).
    pub fn add_root_schema(&mut self, schema: RootSchema) -> Result<Option<TypeId>> {
        let RootSchema {
            meta_schema: _,
            schema,
            definitions,
        } = schema;

        let mut defs = definitions
            .into_iter()
            .map(|(key, schema)| (RefKey::Def(key), schema))
            .collect::<Vec<_>>();

        // Does the root type have a name (otherwise... ignore it)
        let root_type = schema
            .metadata
            .as_ref()
            .and_then(|m| m.title.as_ref())
            .is_some();

        if root_type {
            defs.push((RefKey::Root, schema.into()));
        }

        self.add_ref_types_impl(defs)?;

        if root_type {
            Ok(self.ref_to_id.get(&RefKey::Root).cloned())
        } else {
            Ok(None)
        }
    }

    /// Whether the generated code needs `chrono` crate.
    pub fn uses_chrono(&self) -> bool {
        self.uses_chrono
    }

    /// Whether the generated code needs [regress] crate.
    pub fn uses_regress(&self) -> bool {
        self.uses_regress
    }

    /// Whether the generated code needs [serde_json] crate.
    pub fn uses_serde_json(&self) -> bool {
        self.uses_serde_json
    }

    /// Whether the generated code needs `uuid` crate.
    pub fn uses_uuid(&self) -> bool {
        self.uses_uuid
    }

    /// The typespace settings derived from typify's settings.
    ///
    /// typify has always emitted `Serialize`, `Deserialize`, `Clone`, and
    /// `Debug` on every generated type, so those four are requested as
    /// required traits regardless of user settings. Global extra derives
    /// forward to typespace's settings-wide `with_derive`, and the map
    /// type forwards to `with_map_type` along with the traits a map key
    /// requires. `ContainerType::new` accepts an arbitrary type path, so
    /// typify cannot know whether the configured container needs
    /// hashed-lookup traits (`Hash`, `Eq`, `PartialEq`) or ordered-lookup
    /// traits (`Ord`, `PartialOrd`, along with `Eq` and `PartialEq`, which
    /// `Ord` and `PartialOrd` require); the union of both is forwarded as
    /// the key obligation, matching typify's own generated key types,
    /// which always derive all five (see
    /// `typify/tests/schemas/maps_custom.rs`); the value obligation is
    /// empty. `ContainerType::new` also claims only what rendering
    /// assumes of the container itself (`Default` unconditionally, and
    /// `Clone`, `Debug`, `Serialize`, `Deserialize` following the
    /// parameters), which is the conservative answer for a path typify
    /// cannot otherwise reason about. `with_typify_compat` withholds
    /// `Default` from a tuple struct, a unit struct, and a newtype, which
    /// typify never derives it for. Global extra attributes have no
    /// typespace home yet; see the implementation gaps on
    /// [`TypeSpace::to_stream`] and typespace_ext.rs gap note 3.
    fn typespace_settings(&self) -> typespace::settings::Settings {
        let map_key_obligation = [
            typespace::TypespaceTrait::Hash,
            typespace::TypespaceTrait::Eq,
            typespace::TypespaceTrait::PartialEq,
            typespace::TypespaceTrait::Ord,
            typespace::TypespaceTrait::PartialOrd,
        ]
        .into_iter()
        .collect::<typespace::TypespaceTraitSet>();
        let map_type = typespace::settings::ContainerType::new(
            &self.settings.map_type.0.to_token_stream().to_string(),
            [map_key_obligation, typespace::TypespaceTraitSet::empty()],
        )
        .with_provision(
            typespace::TypespaceTrait::JsonSchema,
            typespace::TraitProvision::IfParameters,
        );
        let mut settings = typespace::settings::Settings::minimal()
            .with_required_trait(typespace::TypespaceTrait::Serialize)
            .with_required_trait(typespace::TypespaceTrait::Deserialize)
            .with_required_trait(typespace::TypespaceTrait::Clone)
            .with_required_trait(typespace::TypespaceTrait::Debug)
            .with_desired_trait(typespace::TypespaceTrait::Default)
            .with_desired_trait(typespace::TypespaceTrait::Eq)
            .with_desired_trait(typespace::TypespaceTrait::PartialEq)
            .with_desired_trait(typespace::TypespaceTrait::Ord)
            .with_desired_trait(typespace::TypespaceTrait::PartialOrd)
            .with_desired_trait(typespace::TypespaceTrait::Hash)
            .with_desired_trait(typespace::TypespaceTrait::Display)
            .with_desired_trait(typespace::TypespaceTrait::FromStr)
            .with_desired_trait(typespace::TypespaceTrait::Copy)
            .with_map_type(map_type)
            .with_typify_compat(true);
        for derive in &self.settings.extra_derives {
            match derive.as_str() {
                "::schemars::JsonSchema" | "schemars::JsonSchema" | "JsonSchema" => {
                    settings = settings.with_required_trait(typespace::TypespaceTrait::JsonSchema)
                }
                _ => {
                    settings = settings.with_derive(derive.clone());
                }
            }
        }
        for attr in &self.settings.extra_attrs {
            settings = settings.with_attr(attr.clone());
        }
        settings = settings.with_struct_builder(self.settings.struct_builder);
        settings
    }

    /// Finalize the collected types into a [`typespace::Typespace`].
    ///
    /// The typespace is the query surface for the collected types: use
    /// its `get_type` and `iter_types` to inspect a type's structure,
    /// render its identifier (optionally scoped by a module path), and
    /// ask about trait impls. Conversion may continue after this call;
    /// a later call reflects the additional types.
    pub fn to_typespace(&self) -> Result<typespace::Typespace<TypeId>> {
        let mut builder = typespace::TypespaceBuilder::new(self.typespace_settings());

        for (type_id, type_entry) in &self.id_to_entry {
            match type_entry {
                TypeEntry::Type(typ) => {
                    builder
                        .insert(type_id.clone(), typ.clone())
                        .expect("type IDs are unique by construction");
                }
                // References never land in id_to_entry (assign_type
                // unwraps them); this is defensive.
                TypeEntry::Reference(_) => {}
            }
        }

        // typify breaks containment cycles itself (see cycles.rs) by
        // inserting Box entries before output, so typespace finalization
        // should never need to manufacture a Box of its own. Provide a
        // real generator anyway--if it ever fires we'd rather get a
        // distinctive id than a panic--by mapping the inner id into a
        // range that TypeSpace::assign never produces.
        Ok(builder.finalize(|inner: &TypeId| TypeId(inner.0 | (1 << 63)))?)
    }

    /// All code for processed types.
    ///
    /// Rendering is delegated to typespace: the stored types are
    /// inserted into a `TypespaceBuilder`, finalized, and rendered
    /// through codespace. Finalization errors (dangling references,
    /// name collisions, unsatisfiable trait requirements) surface as
    /// [`Error::Typespace`]. Known IMPLEMENTATION GAPS relative to
    /// typify1's renderer (each of these is a construct typespace can
    /// or should express but does not yet render):
    ///
    /// - no `error` module / `ConversionError` type (nothing typespace
    ///   emits refers to it yet, so the output is self-consistent);
    ///   this method injects the module itself to match typify1's
    ///   unconditional emission;
    /// - no bespoke enum impls: Display/FromStr for all-simple-variant
    ///   enums, untagged Display/FromStr proxies, TryFrom<&str>, and
    ///   the convenience `From<VariantType>` impls;
    /// - no `impl Default` for whole-type default values, and explicit
    ///   property defaults are runtime `serde_json::from_value` calls
    ///   rather than typify1's typed expressions (see value.rs, retained
    ///   unbuilt for reference) with shared `defaults::` helpers;
    /// - no string-constraint validation (min/max length, pattern) on
    ///   newtypes, and no value-list constraints at all;
    /// - no `#[serde(deny_unknown_fields)]` on structs or enums;
    /// - no derives beyond serde and the settings-wide extra derives
    ///   (typify1 derived Debug/Clone always, Copy/Eq/Ord/Hash for
    ///   simple enums, and applied per-type derives and attrs from
    ///   patches);
    /// - no struct builder types (TypeSpaceSettings::with_struct_builder);
    pub fn to_stream(&self) -> Result<TokenStream> {
        let typespace = self.to_typespace()?;

        let codespace = typespace.to_codespace();

        Ok(codespace.into_stream())
    }

    /// Allocated the next TypeId.
    fn assign(&mut self) -> TypeId {
        let id = TypeId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Assign a TypeId for a TypeEntry. This handles resolving references
    /// and checking for duplicate type definitions (e.g. to make sure there
    /// aren't two conflicting types of the same name).
    fn assign_type(&mut self, ty: TypeEntry) -> TypeId {
        if let TypeEntry::Reference(type_id) = ty {
            type_id
        } else if let Some(name) = ty.name().map(str::to_string) {
            // If there's already a type of this name, we make sure it's
            // identical. Note that this covers all user-defined types.

            // TODO there are many different choices we might make here
            // that could differ depending on the texture of the schema.
            // For example, a schema might use the string "Response" in a
            // bunch of places and if that were the case we might expect
            // them to be different and resolve that by renaming or scoping
            // them in some way.
            if let Some(type_id) = self.name_to_id.get(&name) {
                // TODO we'd like to verify that the type is structurally the
                // same, but the types may not be functionally equal. This is a
                // consequence of types being "finalized" after each type
                // addition. This further emphasized the need for a more
                // deliberate, multi-pass approach.
                type_id.clone()
            } else {
                let type_id = self.assign();
                self.name_to_id.insert(name, type_id.clone());
                self.id_to_entry.insert(type_id.clone(), ty);
                type_id
            }
        } else {
            let type_id = self.assign();
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
            let default = metadata
                .default
                .clone()
                .map(typespace::build::JsonValue::new);
            // Only named types carry a default; set_default is a no-op
            // otherwise.
            if let Some(typ) = type_entry.as_type_mut() {
                typ.set_default(default);
            }
        }
        let type_id = self.assign_type(type_entry);
        Ok((type_id, metadata))
    }

    /// Create an Option<T> from a pre-assigned TypeId and assign it an ID.
    ///
    /// typify1 tolerated nested Option types internally and flattened
    /// them when rendering identifiers; typespace renders exactly the
    /// types it is given, so we avoid constructing Option<Option<T>> in
    /// the first place. (A forward reference won't have an entry yet,
    /// but references always name named types, never raw Options, so
    /// wrapping is correct in that case.)
    fn id_to_option(&mut self, id: &TypeId) -> TypeId {
        if let Some(entry) = self.id_to_entry.get(id) {
            if matches!(entry, TypeEntry::Type(typespace::build::Type::Option(_))) {
                return id.clone();
            }
        }
        self.assign_type(typespace::build::Type::Option(id.clone()).into())
    }

    // Create an Option<T> from a TypeEntry by assigning it type.
    fn type_to_option(&mut self, ty: TypeEntry) -> TypeEntry {
        // As with id_to_option, don't nest Options.
        if matches!(&ty, TypeEntry::Type(typespace::build::Type::Option(_))) {
            return ty;
        }
        typespace::build::Type::Option(self.assign_type(ty)).into()
    }

    /// Create a Box<T> from a pre-assigned TypeId and assign it an ID.
    fn id_to_box(&mut self, id: &TypeId) -> TypeId {
        self.assign_type(typespace::build::Type::Box(id.clone()).into())
    }
}

/// Whether a native type's name matches the required name for a
/// reference type (or the native type has parameters and so couldn't
/// simply be aliased).
fn native_name_match(native: &typespace::build::Native<TypeId>, type_name: &Name) -> bool {
    // typespace answers a native's path as a syn::Type, so read the
    // last segment rather than splitting the rendered tokens, which
    // carry spaces around their separators.
    let native_name = match native.path() {
        syn::Type::Path(path) => path.path.segments.last().map(|seg| seg.ident.to_string()),
        _ => None,
    };
    !native.parameters().is_empty()
        || matches!(
            (type_name, native_name.as_deref()),
            (Name::Required(req), Some(name)) if req == name
        )
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::{schema_for, JsonSchema};
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashSet;

    use crate::{
        test_util::validate_output, type_entry::TypeEntry, MapType, Name, TypeSpace,
        TypeSpaceSettings,
    };
    use typespace::build::{Type, VariantDetails};

    #[test]
    fn test_map_type_from_str() {
        let map_type = "::std::collections::BTreeMap".parse::<MapType>().unwrap();
        assert_eq!(map_type.to_string(), ":: std :: collections :: BTreeMap");

        "not a valid!!type".parse::<MapType>().unwrap_err();
        "".parse::<MapType>().unwrap_err();
    }

    #[test]
    fn test_map_type_deserialize() {
        let map_type: MapType =
            serde_json::from_value(json!("::std::collections::BTreeMap")).unwrap();
        assert_eq!(map_type.to_string(), ":: std :: collections :: BTreeMap");

        // Strings with escape sequences require owned deserialization; make
        // sure that works.
        let map_type: MapType =
            serde_json::from_str("\"::std::collections::\\u0042TreeMap\"").unwrap();
        assert_eq!(map_type.to_string(), ":: std :: collections :: BTreeMap");

        // ... and invalid types must produce an error rather than a panic.
        serde_json::from_value::<MapType>(json!("not a valid!!type")).unwrap_err();
    }

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
            .convert_schema_object(
                Name::Unknown,
                &schemars::schema::Schema::Object(schema.schema.clone()),
                &schema.schema,
            )
            .unwrap();

        println!("{:#?}", ty);

        // Render everything (including the newly converted type) via
        // typespace.
        let _ = type_space.assign_type(ty);
        println!("{}", type_space.to_stream().unwrap());
    }

    #[test]
    fn test_external_references() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-04/schema#",
            "definitions": {
                "somename": {
                    "$ref": "#/definitions/someothername",
                    "required": [ "someproperty" ]
                },
                "someothername": {
                    "type": "object",
                    "properties": {
                        "someproperty": {
                            "type": "string"
                        }
                    }
                }
            }
        });
        let schema = serde_json::from_value(schema).unwrap();
        println!("{:#?}", schema);
        let settings = TypeSpaceSettings::default();
        let mut type_space = TypeSpace::new(&settings);
        type_space.add_root_schema(schema).unwrap();
        let tokens = type_space.to_stream().unwrap().to_string();
        println!("{}", tokens);
        assert!(tokens
            .contains(" pub struct Somename { pub someproperty : :: std :: string :: String , }"))
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
            .convert_schema_object(
                Name::Unknown,
                &schemars::schema::Schema::Object(schema.schema.clone()),
                &schema.schema,
            )
            .unwrap();

        match &ty {
            TypeEntry::Type(Type::Enum(type_enum)) => {
                let variants = type_enum.get_variants();
                for variant in variants {
                    assert_eq!(variant.details(), &VariantDetails::Unit);
                }
                let var_names = variants
                    .iter()
                    .map(|variant| variant.rust_name().to_string())
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
                panic!("unexpected type entry {:#?}", ty);
            }
        }
    }

    #[test]
    fn test_string_enum_with_null() {
        let original_schema = json!({ "$ref": "xxx"});
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
                &serde_json::from_value(original_schema).unwrap(),
                &None,
                &enum_values,
                None,
            )
            .unwrap();

        if let TypeEntry::Type(Type::Option(id)) = &te {
            let ote = type_space.id_to_entry.get(id).unwrap();
            if let Type::Enum(type_enum) = ote.as_type() {
                let variants = type_enum
                    .get_variants()
                    .iter()
                    .map(|v| match v.details() {
                        VariantDetails::Unit => v.rust_name().to_string(),
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
        #[allow(dead_code)]
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
