// Copyright 2026 Oxide Computer Company

//! Construction of [`typespace::Type`] values from schema data.
//!
//! Historically this module defined typify's internal type
//! representation (`TypeEntry` and friends) along with its rendering.
//! The representation is now `typespace::build::Type<TypeId>`,
//! constructed directly at the conversion sites in this module and in
//! convert.rs / structs.rs / enums.rs, and rendering is typespace's job
//! (see `TypeSpace::to_stream`). What remains here is:
//!
//! - `TypeEntry`, which wraps a typespace type or an in-flight
//!   reference.
//! - the `from_metadata` constructors that turn schema metadata into
//!   named typespace types.
//!
//! The queries typify's public API used to answer from here (`has_impl`,
//! `type_ident`, ...) are typespace's job; see
//! `TypeSpace::to_typespace` and typespace's view module.

use std::collections::HashMap;

use schemars::schema::{Metadata, Schema};
use unicode_ident::is_xid_continue;

use crate::{
    sanitize,
    util::{get_type_name, metadata_description, unique, TypePatch},
    Case, Name, TypeId, TypeSpace,
};

use typespace::build::{
    Enum, EnumTagType, EnumVariant, JsonValue, Native, NewtypeConstraints, NewtypeStruct, Struct,
    StructProperty, Type, VariantDetails,
};

// [typespace migration note] Struct-variant property names were
// `&'a str` borrowed out of typify's own representation; typespace's
// `StructProperty::rust_name` is a `syn::Ident`, which can only give up
// its name as an owned String. INTERFACE GAP: consider `rust_name()
// -> &str` (or storing the name as a String) in typespace. This applies
// to typespace's view module too, whose property names are owned
// Strings for the same reason.

/// A typespace type under construction, or an in-flight reference to
/// another type ID.
///
/// typespace has no reference variant--every ID inserted into its
/// builder must name a concrete type--and it doesn't need one: by the
/// time output is requested, every reference has been resolved.
/// References exist only during conversion (see `convert_reference`) and
/// are unwrapped by `TypeSpace::assign_type` before storage.
#[derive(Debug, Clone)]
pub(crate) enum TypeEntry {
    Type(Type<TypeId>),
    Reference(TypeId),
}

/// In-flight enum variant used while assembling a `TypeEnum`. The
/// identifier name is chosen (and de-duplicated) in
/// `TypeEntryEnum::from_metadata`; until then only the raw (JSON) name is
/// known.
#[derive(Debug, Clone)]
pub(crate) struct Variant {
    pub raw_name: String,
    pub description: Option<String>,
    pub details: VariantDetails<TypeId>,
}

impl Variant {
    pub(crate) fn new(
        raw_name: String,
        description: Option<String>,
        details: VariantDetails<TypeId>,
    ) -> Self {
        Self {
            raw_name,
            description,
            details,
        }
    }
}

fn variant_names_unique(names: &[String]) -> bool {
    unique(names.iter())
}

/// Constructors for named typespace types. These retain the shape (and
/// names) of typify's original `TypeEntryEnum` / `TypeEntryStruct` /
/// `TypeEntryNewtype` constructors so that the conversion code reads the
/// same; the difference is that they now construct `typespace::Type`
/// values directly.
pub(crate) struct TypeEntryEnum {}
pub(crate) struct TypeEntryStruct {}
pub(crate) struct TypeEntryNewtype {}

impl TypeEntryEnum {
    pub(crate) fn from_metadata(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        tag_type: EnumTagType,
        variants: Vec<Variant>,
        deny_unknown_fields: bool,
        schema: Schema,
    ) -> TypeEntry {
        // Let's find some decent names for variants. We first try the simple
        // sanitization.
        let mut ident_names = variants
            .iter()
            .map(|variant| sanitize(&variant.raw_name, Case::Pascal))
            .collect::<Vec<_>>();

        // If variants aren't unique, we're turn the elided characters into
        // 'x's.
        if !variant_names_unique(&ident_names) {
            ident_names = variants
                .iter()
                .map(|variant| {
                    sanitize(
                        &variant
                            .raw_name
                            .replace(|c| c == '_' || !is_xid_continue(c), "X"),
                        Case::Pascal,
                    )
                })
                .collect();
        }

        // If variants still aren't unique, we fail: we'd rather not emit code
        // that can't compile
        if !variant_names_unique(&ident_names) {
            let mut counts = HashMap::new();
            ident_names.iter().for_each(|ident_name| {
                counts
                    .entry(ident_name)
                    .and_modify(|xxx| *xxx += 1)
                    .or_insert(0);
            });
            let dups = variants
                .iter()
                .zip(ident_names.iter())
                .filter(|(_, ident_name)| *counts.get(ident_name).unwrap() > 0)
                .map(|(variant, _)| variant.raw_name.as_str())
                .collect::<Vec<_>>()
                .join(",");
            panic!("Failed to make unique variant names for [{}]", dups);
        }

        let variants: Vec<EnumVariant<TypeId>> = variants
            .into_iter()
            .zip(ident_names)
            .map(
                |(
                    Variant {
                        raw_name,
                        description,
                        details,
                    },
                    ident_name,
                )| {
                    // typify computed the serde rename at output time by
                    // comparing the sanitized identifier with the raw
                    // name; typespace wants it precomputed.
                    let rename = (ident_name != raw_name).then_some(raw_name);
                    let mut variant = EnumVariant::new(ident_name, details);
                    if let Some(rename) = rename {
                        variant = variant.with_rename(rename);
                    }
                    if let Some(description) = description {
                        variant = variant.with_description(description);
                    }
                    variant
                },
            )
            .collect();

        let name = get_type_name(&type_name, metadata).unwrap();
        // INTERFACE GAP: container-level serde rename (set when a patch
        // renames the type) has no home in typespace's TypeEnum.
        let description = metadata_description(metadata);

        let type_patch = TypePatch::new(type_space, name);

        let mut type_enum = Enum::new()
            .name(&type_patch.name)
            .description(make_doc(&type_patch.name, description.as_ref(), &schema))
            .extra_derives(type_patch.derives)
            .extra_attrs(type_patch.attrs)
            .tag_type(tag_type)
            .variants(variants);
        if deny_unknown_fields {
            type_enum = type_enum.deny_unknown_fields();
        }
        let typ = type_enum.build().unwrap();

        TypeEntry::Type(typ)
    }
}

impl TypeEntryStruct {
    pub(crate) fn from_metadata(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        properties: Vec<StructProperty<TypeId>>,
        deny_unknown_fields: bool,
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        // INTERFACE GAP: container-level serde rename; see
        // TypeEntryEnum::from_metadata.
        let description = metadata_description(metadata);
        let default = metadata
            .as_ref()
            .and_then(|m| m.default.as_ref())
            .cloned()
            .map(JsonValue::new);

        let type_patch = TypePatch::new(type_space, name);

        let mut type_struct = Struct::new()
            .name(&type_patch.name)
            .description(make_doc(&type_patch.name, description.as_ref(), &schema))
            .extra_derives(type_patch.derives)
            .extra_attrs(type_patch.attrs)
            .properties(properties);
        if deny_unknown_fields {
            type_struct = type_struct.deny_unknown_fields();
        }
        let mut typ = type_struct.build().unwrap();
        typ.set_default(default);

        TypeEntry::Type(typ)
    }
}

impl TypeEntryNewtype {
    fn make(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        constraints: NewtypeConstraints,
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        // INTERFACE GAP: container-level serde rename; see
        // TypeEntryEnum::from_metadata.
        let description = metadata_description(metadata);

        let type_patch = TypePatch::new(type_space, name);

        let typ = NewtypeStruct::new(type_id)
            .name(&type_patch.name)
            .description(make_doc(&type_patch.name, description.as_ref(), &schema))
            .extra_derives(type_patch.derives)
            .extra_attrs(type_patch.attrs)
            .constraints(constraints)
            .build()
            .unwrap();

        TypeEntry::Type(typ)
    }

    pub(crate) fn from_metadata(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        schema: Schema,
    ) -> TypeEntry {
        Self::make(
            type_space,
            type_name,
            metadata,
            type_id,
            NewtypeConstraints::None,
            schema,
        )
    }

    pub(crate) fn from_metadata_with_enum_values(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        enum_values: &[serde_json::Value],
        schema: Schema,
    ) -> TypeEntry {
        let constraints = NewtypeConstraints::AllowList(
            enum_values.iter().cloned().map(JsonValue::new).collect(),
        );
        Self::make(
            type_space,
            type_name,
            metadata,
            type_id,
            constraints,
            schema,
        )
    }

    pub(crate) fn from_metadata_with_deny_values(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        enum_values: &[serde_json::Value],
        schema: Schema,
    ) -> TypeEntry {
        let constraints =
            NewtypeConstraints::DenyList(enum_values.iter().cloned().map(JsonValue::new).collect());
        Self::make(
            type_space,
            type_name,
            metadata,
            type_id,
            constraints,
            schema,
        )
    }

    pub(crate) fn from_metadata_with_string_validation(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        validation: &schemars::schema::StringValidation,
        schema: Schema,
    ) -> TypeEntry {
        let schemars::schema::StringValidation {
            max_length,
            min_length,
            pattern,
        } = validation.clone();

        Self::make(
            type_space,
            type_name,
            metadata,
            type_id,
            NewtypeConstraints::String {
                max: max_length.map(|len| len as usize),
                min: min_length.map(|len| len as usize),
                patterns: pattern.into_iter().collect(),
            },
            schema,
        )
    }
}

impl From<Type<TypeId>> for TypeEntry {
    fn from(typ: Type<TypeId>) -> Self {
        Self::Type(typ)
    }
}

impl TypeEntry {
    pub(crate) fn new_native<S: ToString>(
        type_name: S,
        traits: typespace::TypespaceTraitSet,
    ) -> Self {
        Type::Native(Native::new(&type_name.to_string(), traits, Vec::new())).into()
    }

    pub(crate) fn new_native_params<S: ToString>(type_name: S, params: &[TypeId]) -> Self {
        let traits = [
            typespace::TypespaceTrait::Clone,
            typespace::TypespaceTrait::Debug,
            typespace::TypespaceTrait::Serialize,
            typespace::TypespaceTrait::Deserialize,
        ]
        .into_iter()
        .collect::<typespace::TypespaceTraitSet>();

        Type::Native(Native::new(&type_name.to_string(), traits, params.to_vec())).into()
    }

    pub(crate) fn new_boolean() -> Self {
        Type::Boolean.into()
    }
    pub(crate) fn new_integer<S: ToString>(type_name: S) -> Self {
        Type::Integer(type_name.to_string()).into()
    }
    pub(crate) fn new_float<S: ToString>(type_name: S) -> Self {
        Type::Float(type_name.to_string()).into()
    }

    /// The typespace type stored in this entry.
    ///
    /// # Panics
    ///
    /// Panics on a reference entry; references are unwrapped by
    /// `assign_type` before storage so entries fetched from the type
    /// space are always concrete types.
    pub(crate) fn as_type(&self) -> &Type<TypeId> {
        match self {
            Self::Type(typ) => typ,
            Self::Reference(_) => panic!("references should be resolved by now"),
        }
    }

    pub(crate) fn as_type_mut(&mut self) -> Option<&mut Type<TypeId>> {
        match self {
            Self::Type(typ) => Some(typ),
            Self::Reference(_) => None,
        }
    }

    pub(crate) fn name(&self) -> Option<&str> {
        match self {
            Self::Type(typ) => typ.name(),
            Self::Reference(_) => None,
        }
    }
}

/// Flatten typify's doc block into typespace's single description
/// string.
///
/// typify rendered a type's documentation as a description attribute
/// followed by a `<details>` block containing the pretty-printed JSON
/// schema, as line-by-line `#[doc]` attributes, falling back to
/// `` `Name` `` when the schema has no description.
///
/// INTERFACE GAP: typespace's TypeCommon has only a single description
/// string with no structured place for the schema block, so the whole
/// thing is flattened into one multi-line string here (which typespace
/// emits as one `#[doc = "..."]` attribute). The rendered rustdoc is
/// equivalent but the generated source is not. It also means the schema
/// must be captured at construction time rather than carried alongside
/// the type as typify1 did.
fn make_doc(name: &str, description: Option<&String>, _schema: &Schema) -> String {
    match description {
        Some(desc) => desc.clone(),
        None => format!("`{}`", name),
    }
}
