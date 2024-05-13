// Copyright 2023 Oxide Computer Company

use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use schemars::schema::{Metadata, Schema};
use syn::Path;

use crate::{
    enums::output_variant,
    output::{OutputSpace, OutputSpaceMod},
    structs::{generate_serde_attr, DefaultFunction},
    util::{get_type_name, metadata_description, type_patch},
    DefaultImpl, Name, Result, TypeId, TypeSpace, TypeSpaceImpl,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SchemaWrapper(Schema);

impl Eq for SchemaWrapper {}

impl Ord for SchemaWrapper {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}
impl PartialOrd for SchemaWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryEnum {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub tag_type: EnumTagType,
    pub variants: Vec<Variant>,
    pub deny_unknown_fields: bool,
    pub bespoke_impls: BTreeSet<TypeEntryEnumImpl>,
    pub schema: SchemaWrapper,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeEntryEnumImpl {
    AllSimpleVariants,
    UntaggedFromStr,
    UntaggedDisplay,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryStruct {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub properties: Vec<StructProperty>,
    pub deny_unknown_fields: bool,
    pub schema: SchemaWrapper,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryNewtype {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub type_id: TypeId,
    pub constraints: TypeEntryNewtypeConstraints,
    pub schema: SchemaWrapper,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeEntryNewtypeConstraints {
    None,
    EnumValue(Vec<WrappedValue>),
    DenyValue(Vec<WrappedValue>),
    String {
        max_length: Option<u32>,
        min_length: Option<u32>,
        pattern: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryNative {
    pub type_name: String,
    impls: Vec<TypeSpaceImpl>,
    // TODO to support const generics, this can be some sort of TypeOrValue,
    // but note that we may some day need to disambiguate char and &'static str
    // since schemars represents a char as a string of length 1.
    pub parameters: Vec<TypeId>,
}
impl TypeEntryNative {
    pub(crate) fn name_match(&self, type_name: &Name) -> bool {
        let native_name = self.type_name.rsplit("::").next().unwrap();
        !self.parameters.is_empty()
            || matches!(type_name, Name::Required(req) if req == native_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WrappedValue(pub serde_json::Value);
impl WrappedValue {
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl Ord for WrappedValue {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}
impl PartialOrd for WrappedValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// TODO This struct needs to go away (again). The derives should go into the
// generated struct/enum/newtype structs. Same for the impls. Native types will
// also have impls. Builtin generic types such as Box or Vec will delegate to
// their subtypes (while recursive, it is necessarily terminating... though I
// suppose we could memoize it). Builtin simple types such as u64 or String
// have a static list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypeEntry {
    pub details: TypeEntryDetails,
    pub extra_derives: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeEntryDetails {
    Enum(TypeEntryEnum),
    Struct(TypeEntryStruct),
    Newtype(TypeEntryNewtype),

    /// Native types exported from a well-known crate.
    Native(TypeEntryNative),

    // Types from core and std.
    Option(TypeId),
    Box(TypeId),
    Vec(TypeId),
    Map(TypeId, TypeId),
    Set(TypeId),
    Array(TypeId, usize),
    Tuple(Vec<TypeId>),
    Unit,
    Boolean,
    /// Integers
    Integer(String),
    /// Floating point numbers; not Eq, Ord, or Hash
    Float(String),
    /// Strings... which we handle a little specially.
    String,
    /// serde_json::Value which we also handle specially.
    JsonValue,

    /// While these types won't very make their way out to the user, we need
    /// reference types in particular to represent simple type aliases between
    /// types named as reference targets.
    Reference(TypeId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EnumTagType {
    External,
    Internal { tag: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Variant {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub details: VariantDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VariantDetails {
    Simple,
    Item(TypeId),
    Tuple(Vec<TypeId>),
    Struct(Vec<StructProperty>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StructProperty {
    pub name: String,
    pub rename: StructPropertyRename,
    pub state: StructPropertyState,
    pub description: Option<String>,
    pub type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StructPropertyRename {
    None,
    Rename(String),
    Flatten,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StructPropertyState {
    Required,
    Optional,
    Default(WrappedValue),
}

#[derive(Debug)]
pub(crate) enum DefaultKind {
    Intrinsic,
    Specific,
    Generic(DefaultImpl),
}

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
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Enum(Self {
            name,
            rename,
            description,
            default: None,
            tag_type,
            variants,
            deny_unknown_fields,
            bespoke_impls: Default::default(),
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }

    pub(crate) fn finalize(&mut self, type_space: &TypeSpace) {
        self.bespoke_impls = [
            // Not untagged with all simple variants.
            (self.tag_type != EnumTagType::Untagged
                && !self.variants.is_empty()
                && self
                    .variants
                    .iter()
                    .all(|variant| matches!(variant.details, VariantDetails::Simple)))
            .then_some(TypeEntryEnumImpl::AllSimpleVariants),
            // Untagged and all variants impl FromStr.
            untagged_newtype_variants(
                type_space,
                &self.tag_type,
                &self.variants,
                TypeSpaceImpl::FromStr,
            )
            .then_some(TypeEntryEnumImpl::UntaggedFromStr),
            // Untagged and all variants impl Display.
            untagged_newtype_variants(
                type_space,
                &self.tag_type,
                &self.variants,
                TypeSpaceImpl::Display,
            )
            .then_some(TypeEntryEnumImpl::UntaggedDisplay),
        ]
        .into_iter()
        .flatten()
        .collect();
    }
}

impl TypeEntryStruct {
    pub(crate) fn from_metadata(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        properties: Vec<StructProperty>,
        deny_unknown_fields: bool,
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);
        let default = metadata
            .as_ref()
            .and_then(|m| m.default.as_ref())
            .cloned()
            .map(WrappedValue::new);

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Struct(Self {
            name,
            rename,
            description,
            default,
            properties,
            deny_unknown_fields,
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }
}

impl TypeEntryNewtype {
    pub(crate) fn from_metadata(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::None,
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }

    pub(crate) fn from_metadata_with_enum_values(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        enum_values: &[serde_json::Value],
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::EnumValue(
                enum_values.iter().cloned().map(WrappedValue::new).collect(),
            ),
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }

    pub(crate) fn from_metadata_with_deny_values(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        enum_values: &[serde_json::Value],
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::DenyValue(
                enum_values.iter().cloned().map(WrappedValue::new).collect(),
            ),
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }

    pub(crate) fn from_metadata_with_string_validation(
        type_space: &TypeSpace,
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        validation: &schemars::schema::StringValidation,
        schema: Schema,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let schemars::schema::StringValidation {
            max_length,
            min_length,
            pattern,
        } = validation.clone();

        let (name, extra_derives) = type_patch(type_space, name);

        let details = TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::String {
                max_length,
                min_length,
                pattern,
            },
            schema: SchemaWrapper(schema),
        });

        TypeEntry {
            details,
            extra_derives,
        }
    }
}

impl From<TypeEntryDetails> for TypeEntry {
    fn from(details: TypeEntryDetails) -> Self {
        Self {
            details,
            extra_derives: Default::default(),
        }
    }
}

impl TypeEntry {
    pub(crate) fn new_native<S: ToString>(type_name: S, impls: &[TypeSpaceImpl]) -> Self {
        TypeEntry {
            details: TypeEntryDetails::Native(TypeEntryNative {
                type_name: type_name.to_string(),
                impls: impls.to_vec(),
                parameters: Default::default(),
            }),
            extra_derives: Default::default(),
        }
    }
    pub(crate) fn new_native_params<S: ToString>(type_name: S, params: &[TypeId]) -> Self {
        TypeEntry {
            details: TypeEntryDetails::Native(TypeEntryNative {
                type_name: type_name.to_string(),
                impls: Default::default(),
                parameters: params.to_vec(),
            }),
            extra_derives: Default::default(),
        }
    }
    pub(crate) fn new_boolean() -> Self {
        TypeEntry {
            details: TypeEntryDetails::Boolean,
            extra_derives: Default::default(),
        }
    }
    pub(crate) fn new_integer<S: ToString>(type_name: S) -> Self {
        TypeEntryDetails::Integer(type_name.to_string()).into()
    }
    pub(crate) fn new_float<S: ToString>(type_name: S) -> Self {
        TypeEntry {
            details: TypeEntryDetails::Float(type_name.to_string()),
            extra_derives: Default::default(),
        }
    }

    pub(crate) fn finalize(&mut self, type_space: &mut TypeSpace) -> Result<()> {
        if let TypeEntryDetails::Enum(enum_details) = &mut self.details {
            enum_details.finalize(type_space);
        }

        self.check_defaults(type_space)
    }

    pub(crate) fn name(&self) -> Option<&String> {
        match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum { name, .. })
            | TypeEntryDetails::Struct(TypeEntryStruct { name, .. })
            | TypeEntryDetails::Newtype(TypeEntryNewtype { name, .. }) => Some(name),

            _ => None,
        }
    }

    pub(crate) fn has_impl<'a>(
        &'a self,
        type_space: &'a TypeSpace,
        impl_name: TypeSpaceImpl,
    ) -> bool {
        match &self.details {
            TypeEntryDetails::Enum(details) => match impl_name {
                TypeSpaceImpl::Default => details.default.is_some(),

                TypeSpaceImpl::FromStr => {
                    details
                        .bespoke_impls
                        .contains(&TypeEntryEnumImpl::AllSimpleVariants)
                        || details
                            .bespoke_impls
                            .contains(&TypeEntryEnumImpl::UntaggedFromStr)
                }
                TypeSpaceImpl::Display => {
                    details
                        .bespoke_impls
                        .contains(&TypeEntryEnumImpl::AllSimpleVariants)
                        || details
                            .bespoke_impls
                            .contains(&TypeEntryEnumImpl::UntaggedDisplay)
                }
            },

            TypeEntryDetails::Struct(details) => match impl_name {
                TypeSpaceImpl::Default => details.default.is_some(),
                _ => false,
            },
            TypeEntryDetails::Newtype(details) => match (&details.constraints, impl_name) {
                (_, TypeSpaceImpl::Default) => details.default.is_some(),
                (TypeEntryNewtypeConstraints::String { .. }, TypeSpaceImpl::FromStr) => true,
                (TypeEntryNewtypeConstraints::String { .. }, TypeSpaceImpl::Display) => true,
                (TypeEntryNewtypeConstraints::None, _) => {
                    // TODO this is a lucky kludge that will need to be removed
                    // once we have proper handling of reference cycles (i.e.
                    // as opposed to containment cycles... which we also do not
                    // handle correctly). In particular output_newtype calls
                    // this to determine if it should produce a FromStr impl.
                    // This implementation could be infinitely recursive for a
                    // type such as this:
                    //     struct A(Box<A>);
                    // While this type is useless and unusable, we do--
                    // basically--support and test this. On such a type, if one
                    // were to ask `ty.has_impl(TypeSpaceImpl::Default)` it
                    // would be infinitely recursive. Fortunately the type
                    // doesn't occur in the wild (we hope) and generation
                    // doesn't rely on that particular query.

                    let type_entry = type_space.id_to_entry.get(&details.type_id).unwrap();
                    type_entry.has_impl(type_space, impl_name)
                }
                _ => false,
            },
            TypeEntryDetails::Native(details) => details.impls.contains(&impl_name),
            TypeEntryDetails::Box(type_id) => {
                if impl_name == TypeSpaceImpl::Default {
                    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                    type_entry.has_impl(type_space, impl_name)
                } else {
                    false
                }
            }

            TypeEntryDetails::JsonValue => false,

            TypeEntryDetails::Unit
            | TypeEntryDetails::Option(_)
            | TypeEntryDetails::Vec(_)
            | TypeEntryDetails::Map(_, _)
            | TypeEntryDetails::Set(_) => {
                matches!(impl_name, TypeSpaceImpl::Default)
            }

            TypeEntryDetails::Tuple(type_ids) => {
                // Default is implemented for tuples of up to 12 items long.
                matches!(impl_name, TypeSpaceImpl::Default)
                    && type_ids.len() <= 12
                    && type_ids.iter().all(|type_id| {
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                        type_entry.has_impl(type_space, TypeSpaceImpl::Default)
                    })
            }

            TypeEntryDetails::Array(item_id, length) => {
                // Default is implemented for arrays of up to length 32.
                if *length <= 32 && impl_name == TypeSpaceImpl::Default {
                    let type_entry = type_space.id_to_entry.get(item_id).unwrap();
                    type_entry.has_impl(type_space, impl_name)
                } else {
                    false
                }
            }

            TypeEntryDetails::Boolean => true,
            TypeEntryDetails::Integer(_) => true,
            TypeEntryDetails::Float(_) => true,
            TypeEntryDetails::String => true,

            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }

    pub(crate) fn output(&self, type_space: &TypeSpace, output: &mut OutputSpace) {
        let derive_set = ["Serialize", "Deserialize", "Debug", "Clone"]
            .into_iter()
            .collect::<BTreeSet<_>>();

        match &self.details {
            TypeEntryDetails::Enum(enum_details) => {
                self.output_enum(type_space, output, enum_details, derive_set)
            }
            TypeEntryDetails::Struct(struct_details) => {
                self.output_struct(type_space, output, struct_details, derive_set)
            }
            TypeEntryDetails::Newtype(newtype_details) => {
                self.output_newtype(type_space, output, newtype_details, derive_set)
            }

            // We should never get here as reference types should only be used
            // in-flight, but never recorded into the type space.
            TypeEntryDetails::Reference(_) => unreachable!(),

            // Unnamed types require no definition as they're already defined.
            _ => (),
        }
    }

    fn output_enum(
        &self,
        type_space: &TypeSpace,
        output: &mut OutputSpace,
        enum_details: &TypeEntryEnum,
        mut derive_set: BTreeSet<&str>,
    ) {
        let TypeEntryEnum {
            name,
            rename,
            description,
            default,
            tag_type,
            variants,
            deny_unknown_fields,
            bespoke_impls,
            schema: SchemaWrapper(schema),
        } = enum_details;

        let doc = make_doc(name, description.as_ref(), schema);

        // TODO this is a one-off for some useful traits; this should move into
        // the creation of the enum type.
        if variants
            .iter()
            .all(|variant| matches!(variant.details, VariantDetails::Simple))
        {
            derive_set.extend(["Copy", "PartialOrd", "Ord", "PartialEq", "Eq", "Hash"]);
        }

        let mut serde_options = Vec::new();
        if let Some(old_name) = rename {
            serde_options.push(quote! { rename = #old_name });
        }
        match tag_type {
            EnumTagType::External => {}
            EnumTagType::Internal { tag } => {
                serde_options.push(quote! { tag = #tag });
            }
            EnumTagType::Adjacent { tag, content } => {
                serde_options.push(quote! { tag = #tag });
                serde_options.push(quote! { content = #content });
            }
            EnumTagType::Untagged => {
                serde_options.push(quote! { untagged });
            }
        }
        if *deny_unknown_fields {
            serde_options.push(quote! { deny_unknown_fields });
        }

        let serde = (!serde_options.is_empty()).then(|| {
            quote! { #[serde( #( #serde_options ),* )] }
        });

        let type_name = format_ident!("{}", name);

        let variants_decl = variants
            .iter()
            .map(|variant| output_variant(variant, type_space, output, name))
            .collect::<Vec<_>>();

        // It should not be possible to construct an untagged enum
        // with more than one simple variant--it would not be usable.
        if tag_type == &EnumTagType::Untagged {
            assert!(
                variants
                    .iter()
                    .filter(|variant| matches!(variant.details, VariantDetails::Simple))
                    .count()
                    <= 1
            )
        }

        // ToString and FromStr impls for enums that are made exclusively of
        // simple variants (and are not untagged).
        let simple_enum_impl = bespoke_impls
            .contains(&TypeEntryEnumImpl::AllSimpleVariants)
            .then(|| {
                let (match_variants, match_strs): (Vec<_>, Vec<_>) = variants
                    .iter()
                    .map(|variant| {
                        let variant_name = format_ident!("{}", variant.name);
                        let variant_str = match &variant.rename {
                            Some(s) => s,
                            None => &variant.name,
                        };
                        (variant_name, variant_str)
                    })
                    .unzip();

                quote! {
                    impl ToString for #type_name {
                        fn to_string(&self) -> String {
                            match *self {
                                #(Self::#match_variants => #match_strs.to_string(),)*
                            }
                        }
                    }
                    impl std::str::FromStr for #type_name {
                        type Err = self::error::ConversionError;

                        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                            match value {
                                #(#match_strs => Ok(Self::#match_variants),)*
                                _ => Err("invalid value".into()),
                            }
                        }
                    }
                    impl std::convert::TryFrom<&str> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<&String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
                            value.parse()
                        }
                    }
                }
            });

        let default_impl = default.as_ref().map(|value| {
            let default_stream = self.output_value(type_space, &value.0, &quote! {}).unwrap();
            quote! {
                impl Default for #type_name {
                    fn default() -> Self {
                        #default_stream
                    }
                }
            }
        });

        let untagged_newtype_from_string_impl = bespoke_impls
            .contains(&TypeEntryEnumImpl::UntaggedFromStr)
            .then(|| {
                let variant_name = variants
                    .iter()
                    .map(|variant| format_ident!("{}", variant.name));

                quote! {
                    impl std::str::FromStr for #type_name {
                        type Err = self::error::ConversionError;

                        fn from_str(value: &str) ->
                            Result<Self, self::error::ConversionError>
                        {
                            #(
                                // Try to parse() into each variant.
                                if let Ok(v) = value.parse() {
                                    Ok(Self::#variant_name(v))
                                } else
                            )*
                            {
                                Err("string conversion failed for all variants".into())
                            }
                        }
                    }
                    impl std::convert::TryFrom<&str> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &str) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<&String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &String) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: String) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }
                }
            });

        let untagged_newtype_to_string_impl = bespoke_impls
            .contains(&TypeEntryEnumImpl::UntaggedDisplay)
            .then(|| {
                let variant_name = variants
                    .iter()
                    .map(|variant| format_ident!("{}", variant.name));

                quote! {
                    impl ToString for #type_name {
                        fn to_string(&self) -> String {
                            match self {
                                #(Self::#variant_name(x) => x.to_string(),)*
                            }
                        }
                    }
                }
            });

        let convenience_from = {
            // Build a map whose key is the type ID or type IDs of the Item and
            // Tuple variants, and whose value is a tuple of the original index
            // and the variant itself. Any key that is seen multiple times has
            // a value of None.
            // TODO this requires more consideration to handle single-item
            // tuples.
            let unique_variants =
                variants
                    .iter()
                    .enumerate()
                    .fold(BTreeMap::new(), |mut map, (index, variant)| {
                        let key = match &variant.details {
                            VariantDetails::Item(type_id) => vec![type_id],
                            VariantDetails::Tuple(type_ids) => type_ids.iter().collect(),
                            _ => return map,
                        };

                        map.entry(key)
                            .and_modify(|v| *v = None)
                            .or_insert(Some((index, variant)));
                        map
                    });

            // Remove any variants that are duplicates (i.e. the value is None)
            // with the flatten(). Then order a new map according to the
            // original order of variants. The allows for the order to be
            // stable and for impl blocks to appear in the same order as their
            // corresponding variants.
            let ordered_variants = unique_variants
                .into_values()
                .flatten()
                .collect::<BTreeMap<_, _>>();

            // Generate a `From<VariantType>` impl block that converts the type
            // into the appropriate variant of the enum.
            let variant_from =
                ordered_variants
                    .into_values()
                    .map(|variant| match &variant.details {
                        VariantDetails::Item(type_id) => {
                            let variant_type = type_space.id_to_entry.get(type_id).unwrap();

                            // TODO Strings might conflict with the way we're
                            // dealing with TryFrom<String> right now.
                            (variant_type.details != TypeEntryDetails::String).then(|| {
                                let variant_type_ident = variant_type.type_ident(type_space, &None);
                                let variant_name = format_ident!("{}", variant.name);
                                quote! {
                                    impl From<#variant_type_ident> for #type_name {
                                        fn from(value: #variant_type_ident)
                                            -> Self
                                        {
                                            Self::#variant_name(value)
                                        }
                                    }
                                }
                            })
                        }
                        VariantDetails::Tuple(type_ids) => {
                            let variant_type_idents = type_ids.iter().map(|type_id| {
                                type_space
                                    .id_to_entry
                                    .get(type_id)
                                    .unwrap()
                                    .type_ident(type_space, &None)
                            });
                            let variant_type_ident = if type_ids.len() != 1 {
                                quote! { ( #(#variant_type_idents),* ) }
                            } else {
                                // A single-item tuple requires a trailing
                                // comma.
                                quote! { ( #(#variant_type_idents,)* ) }
                            };
                            let variant_name = format_ident!("{}", variant.name);
                            let ii = (0..type_ids.len()).map(syn::Index::from);
                            Some(quote! {
                                impl From<#variant_type_ident> for #type_name {
                                    fn from(value: #variant_type_ident) -> Self {
                                        Self::#variant_name(
                                            #( value.#ii, )*
                                        )
                                    }
                                }
                            })
                        }
                        _ => None,
                    });

            quote! {
                #( #variant_from )*
            }
        };

        let derives = strings_to_derives(
            derive_set,
            &self.extra_derives,
            &type_space.settings.extra_derives,
        );

        let item = quote! {
            #doc
            #[derive(#(#derives),*)]
            #serde
            pub enum #type_name {
                #(#variants_decl)*
            }

            impl From<&#type_name> for #type_name {
                fn from(value: &#type_name) -> Self {
                    value.clone()
                }
            }

            #simple_enum_impl
            #default_impl
            #untagged_newtype_from_string_impl
            #untagged_newtype_to_string_impl
            #convenience_from
        };
        output.add_item(OutputSpaceMod::Crate, name, item);
    }

    fn output_struct(
        &self,
        type_space: &TypeSpace,
        output: &mut OutputSpace,
        struct_details: &TypeEntryStruct,
        derive_set: BTreeSet<&str>,
    ) {
        let TypeEntryStruct {
            name,
            rename,
            description,
            default,
            properties,
            deny_unknown_fields,
            schema: SchemaWrapper(schema),
        } = struct_details;
        let doc = make_doc(name, description.as_ref(), schema);

        // Generate the serde directives as needed.
        let mut serde_options = Vec::new();
        if let Some(old_name) = rename {
            serde_options.push(quote! { rename = #old_name });
        }
        if *deny_unknown_fields {
            serde_options.push(quote! { deny_unknown_fields });
        }
        let serde =
            (!serde_options.is_empty()).then(|| quote! { #[serde( #( #serde_options ),* )] });

        let type_name = format_ident!("{}", name);

        // Gather the various components for all properties.
        let mut prop_doc = Vec::new();
        let mut prop_serde = Vec::new();
        let mut prop_default = Vec::new();
        let mut prop_name = Vec::new();
        let mut prop_error = Vec::new();
        let mut prop_type = Vec::new();
        let mut prop_type_scoped = Vec::new();

        properties.iter().for_each(|prop| {
            prop_doc.push(prop.description.as_ref().map(|d| quote! { #[doc = #d] }));
            prop_name.push(format_ident!("{}", prop.name));
            prop_error.push(format!(
                "error converting supplied value for {}: {{}}",
                prop.name,
            ));

            let prop_type_entry = type_space.id_to_entry.get(&prop.type_id).unwrap();
            prop_type.push(prop_type_entry.type_ident(type_space, &None));
            prop_type_scoped
                .push(prop_type_entry.type_ident(type_space, &Some("super".to_string())));

            let (serde, default_fn) = generate_serde_attr(
                name,
                &prop.name,
                &prop.rename,
                &prop.state,
                prop_type_entry,
                type_space,
                output,
            );

            prop_serde.push(serde);
            prop_default.push(match default_fn {
                DefaultFunction::Default => {
                    quote! {
                        Ok(Default::default())
                    }
                }
                DefaultFunction::Custom(fn_name) => {
                    let default_fn = syn::parse_str::<Path>(&fn_name).unwrap();
                    quote! {
                        Ok(super::#default_fn())
                    }
                }
                DefaultFunction::None => {
                    let err_msg = format!("no value supplied for {}", prop.name);
                    quote! {
                        Err(#err_msg.to_string())
                    }
                }
            });
        });

        let derives = strings_to_derives(
            derive_set,
            &self.extra_derives,
            &type_space.settings.extra_derives,
        );

        output.add_item(
            OutputSpaceMod::Crate,
            name,
            quote! {
                #doc
                #[derive(#(#derives),*)]
                #serde
                pub struct #type_name {
                    #(
                        #prop_doc
                        #prop_serde
                        pub #prop_name: #prop_type,
                    )*
                }

                impl From<&#type_name> for #type_name {
                    fn from(value: &#type_name) -> Self {
                        value.clone()
                    }
                }
            },
        );

        // If there's a default value, generate an impl Default
        if let Some(value) = default {
            let default_stream = self.output_value(type_space, &value.0, &quote! {}).unwrap();
            output.add_item(
                OutputSpaceMod::Crate,
                name,
                quote! {
                    impl Default for #type_name {
                        fn default() -> Self {
                            #default_stream
                        }
                    }
                },
            );
        }

        if type_space.settings.struct_builder {
            output.add_item(
                OutputSpaceMod::Crate,
                name,
                quote! {
                    impl #type_name {
                        pub fn builder() -> builder::#type_name {
                            Default::default()
                        }
                    }
                },
            );

            output.add_item(
                OutputSpaceMod::Builder,
                name,
                quote! {
                    #[derive(Clone, Debug)]
                    pub struct #type_name {
                        #(
                            #prop_name: Result<#prop_type_scoped, String>,
                        )*
                    }

                    impl Default for #type_name {
                        fn default() -> Self {
                            Self {
                                #(
                                    #prop_name: #prop_default,
                                )*
                            }
                        }
                    }

                    impl #type_name {
                        #(
                            pub fn #prop_name<T>(mut self, value: T) -> Self
                                where
                                    T: std::convert::TryInto<#prop_type_scoped>,
                                    T::Error: std::fmt::Display,
                            {
                                self.#prop_name = value.try_into()
                                    .map_err(|e| format!(#prop_error, e));
                                self
                            }
                        )*
                    }

                    // This is how the item is built.
                    impl std::convert::TryFrom<#type_name>
                        for super::#type_name
                    {
                        type Error = super::error::ConversionError;

                        fn try_from(value: #type_name)
                            -> Result<Self, super::error::ConversionError>
                        {
                            Ok(Self {
                                #(
                                    #prop_name: value.#prop_name?,
                                )*
                            })
                        }
                    }

                    // Construct a builder from the item.
                    impl From<super::#type_name> for #type_name {
                        fn from(value: super::#type_name) -> Self {
                            Self {
                                #(
                                    #prop_name: Ok(value.#prop_name),
                                )*
                            }
                        }
                    }
                },
            );
        }
    }

    fn output_newtype(
        &self,
        type_space: &TypeSpace,
        output: &mut OutputSpace,
        newtype_details: &TypeEntryNewtype,
        mut derive_set: BTreeSet<&str>,
    ) {
        let TypeEntryNewtype {
            name,
            rename,
            description,
            default,
            type_id,
            constraints,
            schema: SchemaWrapper(schema),
        } = newtype_details;
        let doc = make_doc(name, description.as_ref(), schema);

        let serde = rename.as_ref().map(|old_name| {
            quote! {
                #[serde(rename = #old_name)]
            }
        });

        let type_name = format_ident!("{}", name);
        let inner_type = type_space.id_to_entry.get(type_id).unwrap();
        let inner_type_name = inner_type.type_ident(type_space, &None);

        let is_str = matches!(inner_type.details, TypeEntryDetails::String);

        // If this is just a wrapper around a string, we can derive some more
        // useful traits.
        if is_str {
            derive_set.extend(["PartialOrd", "Ord", "PartialEq", "Eq", "Hash"]);
        }

        let constraint_impl = match constraints {
            // In the unconstrained case we proxy impls through the inner type.
            TypeEntryNewtypeConstraints::None => {
                let str_impl = is_str.then(|| {
                    quote! {
                        impl std::str::FromStr for #type_name {
                            type Err = std::convert::Infallible;

                            fn from_str(value: &str) ->
                                Result<Self, Self::Err>
                            {
                                Ok(Self(value.to_string()))
                            }
                        }
                    }
                });

                // TODO see the comment in has_impl related to this case.
                let from_str_impl = (inner_type.has_impl(type_space, TypeSpaceImpl::FromStr)
                    && !is_str)
                    .then(|| {
                        quote! {
                            impl std::str::FromStr for #type_name {
                                type Err = <#inner_type_name as
                                    std::str::FromStr>::Err;

                                fn from_str(value: &str) ->
                                    Result<Self, Self::Err>
                                {
                                    Ok(Self(value.parse()?))
                                }
                            }
                            impl std::convert::TryFrom<&str> for #type_name {
                                type Error = <#inner_type_name as
                                    std::str::FromStr>::Err;

                                fn try_from(value: &str) ->
                                    Result<Self, Self::Error>
                                {
                                    value.parse()
                                }
                            }
                            impl std::convert::TryFrom<&String> for #type_name {
                                type Error = <#inner_type_name as
                                    std::str::FromStr>::Err;

                                fn try_from(value: &String) ->
                                    Result<Self, Self::Error>
                                {
                                    value.parse()
                                }
                            }
                            impl std::convert::TryFrom<String> for #type_name {
                                type Error = <#inner_type_name as
                                    std::str::FromStr>::Err;

                                fn try_from(value: String) ->
                                    Result<Self, Self::Error>
                                {
                                    value.parse()
                                }
                            }
                        }
                    });

                let display_impl = inner_type
                    .has_impl(type_space, TypeSpaceImpl::Display)
                    .then(|| {
                        quote! {
                            impl ToString for #type_name {
                                fn to_string(&self) -> String {
                                    self.0.to_string()
                                }
                            }
                        }
                    });

                quote! {
                    impl From<#inner_type_name> for #type_name {
                        fn from(value: #inner_type_name) -> Self {
                            Self(value)
                        }
                    }

                    #str_impl
                    #from_str_impl
                    #display_impl
                }
            }

            TypeEntryNewtypeConstraints::DenyValue(enum_values)
            | TypeEntryNewtypeConstraints::EnumValue(enum_values) => {
                let not = matches!(constraints, TypeEntryNewtypeConstraints::EnumValue(_))
                    .then(|| quote! { ! });
                // Note that string types with enumerated values are converted
                // into simple enums rather than newtypes so we would not
                // expect to see a string as the inner type here.
                assert!(
                    matches!(constraints, TypeEntryNewtypeConstraints::DenyValue(_))
                        || !matches!(&inner_type.details, TypeEntryDetails::String)
                );

                // We're going to impl Deserialize so we can remove it
                // from the set of derived impls.
                derive_set.remove("Deserialize");

                // TODO: if a user were to derive schemars::JsonSchema, it
                // wouldn't be accurate.

                let value_output = enum_values
                    .iter()
                    .map(|value| inner_type.output_value(type_space, &value.0, &quote! {}));
                // TODO if the sub_type is a string we could probably impl
                // TryFrom<&str> as well and FromStr.
                // TODO maybe we want to handle JsonSchema here
                quote! {
                    // This is effectively the constructor for this type.
                    impl std::convert::TryFrom<#inner_type_name> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(
                            value: #inner_type_name
                        ) -> Result<Self, self::error::ConversionError>
                        {
                            if #not [
                                #(#value_output,)*
                            ].contains(&value) {
                                Err("invalid value".into())
                            } else {
                                Ok(Self(value))
                            }
                        }
                    }

                    impl<'de> serde::Deserialize<'de> for #type_name {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            Self::try_from(
                                <#inner_type_name>::deserialize(deserializer)?,
                            )
                            .map_err(|e| {
                                <D::Error as serde::de::Error>::custom(
                                    e.to_string(),
                                )
                            })
                        }
                    }
                }
            }

            TypeEntryNewtypeConstraints::String {
                max_length,
                min_length,
                pattern,
            } => {
                let max = max_length.map(|v| {
                    let v = v as usize;
                    let err = format!("longer than {} characters", v);
                    quote! {
                        if value.len() > #v {
                            return Err(#err.into());
                        }
                    }
                });
                let min = min_length.map(|v| {
                    let v = v as usize;
                    let err = format!("shorter than {} characters", v);
                    quote! {
                        if value.len() < #v {
                            return Err(#err.into());
                        }
                    }
                });
                let pat = pattern.as_ref().map(|p| {
                    let err = format!("doesn't match pattern \"{}\"", p);
                    quote! {
                        if regress::Regex::new(#p).unwrap().find(value).is_none() {
                            return Err(#err.into());
                        }
                    }
                });

                // We're going to impl Deserialize so we can remove it
                // from the set of derived impls.
                derive_set.remove("Deserialize");

                // TODO: if a user were to derive schemars::JsonSchema, it
                // wouldn't be accurate.
                quote! {
                    impl std::str::FromStr for #type_name {
                        type Err = self::error::ConversionError;

                        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                            #max
                            #min
                            #pat

                            Ok(Self(value.to_string()))
                        }
                    }
                    impl std::convert::TryFrom<&str> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &str) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<&String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: &String) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }
                    impl std::convert::TryFrom<String> for #type_name {
                        type Error = self::error::ConversionError;

                        fn try_from(value: String) ->
                            Result<Self, self::error::ConversionError>
                        {
                            value.parse()
                        }
                    }

                    impl<'de> serde::Deserialize<'de> for #type_name {
                        fn deserialize<D>(
                            deserializer: D,
                        ) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            String::deserialize(deserializer)?
                                .parse()
                                .map_err(|e: self::error::ConversionError| {
                                    <D::Error as serde::de::Error>::custom(
                                        e.to_string(),
                                    )
                                })
                        }
                    }
                }
            }
        };

        // If there are no constraints, let consumers directly access the value.
        let vis = match constraints {
            TypeEntryNewtypeConstraints::None => Some(quote! {pub}),
            _ => None,
        };

        let default_impl = default.as_ref().map(|value| {
            let default_stream = self.output_value(type_space, &value.0, &quote! {}).unwrap();
            quote! {
                impl Default for #type_name {
                    fn default() -> Self {
                        #default_stream
                    }
                }
            }
        });

        let derives = strings_to_derives(
            derive_set,
            &self.extra_derives,
            &type_space.settings.extra_derives,
        );

        let item = quote! {
            #doc
            #[derive(#(#derives),*)]
            #serde
            pub struct #type_name(#vis #inner_type_name);

            impl std::ops::Deref for #type_name {
                type Target = #inner_type_name;
                fn deref(&self) -> &#inner_type_name {
                    &self.0
                }
            }

            impl From<#type_name> for #inner_type_name {
                fn from(value: #type_name) -> Self {
                    value.0
                }
            }

            impl From<&#type_name> for #type_name {
                fn from(value: &#type_name) -> Self {
                    value.clone()
                }
            }

            #default_impl
            #constraint_impl
        };
        output.add_item(OutputSpaceMod::Crate, name, item);
    }

    pub(crate) fn type_name(&self, type_space: &TypeSpace) -> String {
        self.type_ident(type_space, &None).to_string()
    }

    pub(crate) fn type_ident(
        &self,
        type_space: &TypeSpace,
        type_mod: &Option<String>,
    ) -> TokenStream {
        match &self.details {
            // Named types.
            TypeEntryDetails::Enum(TypeEntryEnum { name, .. })
            | TypeEntryDetails::Struct(TypeEntryStruct { name, .. })
            | TypeEntryDetails::Newtype(TypeEntryNewtype { name, .. }) => match &type_mod {
                Some(type_mod) => {
                    let type_mod = format_ident!("{}", type_mod);
                    let type_name = format_ident!("{}", name);
                    quote! { #type_mod :: #type_name }
                }
                None => {
                    let type_name = format_ident!("{}", name);
                    quote! { #type_name }
                }
            },

            TypeEntryDetails::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let inner_ident = inner_ty.type_ident(type_space, type_mod);

                // Flatten nested Option types. This would only happen if the
                // schema encoded it; it's an odd construction.
                match &inner_ty.details {
                    TypeEntryDetails::Option(_) => inner_ident,
                    _ => quote! { Option<#inner_ident> },
                }
            }

            TypeEntryDetails::Box(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for box");

                let item = inner_ty.type_ident(type_space, type_mod);

                quote! { Box<#item> }
            }

            TypeEntryDetails::Vec(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for array");
                let item = inner_ty.type_ident(type_space, type_mod);

                quote! { Vec<#item> }
            }

            TypeEntryDetails::Map(key_id, value_id) => {
                let key_ty = type_space
                    .id_to_entry
                    .get(key_id)
                    .expect("unresolved type id for map key");
                let value_ty = type_space
                    .id_to_entry
                    .get(value_id)
                    .expect("unresolved type id for map value");

                if key_ty.details == TypeEntryDetails::String
                    && value_ty.details == TypeEntryDetails::JsonValue
                {
                    quote! { serde_json::Map<String, serde_json::Value> }
                } else {
                    let key_ident = key_ty.type_ident(type_space, type_mod);
                    let value_ident = value_ty.type_ident(type_space, type_mod);
                    quote! { std::collections::HashMap<#key_ident, #value_ident> }
                }
            }

            TypeEntryDetails::Set(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for set");
                let item = inner_ty.type_ident(type_space, type_mod);
                // TODO we'll want this to be a Set of some kind, but we need
                // to get the derives right first.
                quote! { Vec<#item> }
            }

            TypeEntryDetails::Tuple(items) => {
                let type_idents = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .expect("unresolved type id for tuple")
                        .type_ident(type_space, type_mod)
                });

                if items.len() != 1 {
                    quote! { ( #(#type_idents),* ) }
                } else {
                    // A single-item tuple requires a trailing comma.
                    quote! { ( #(#type_idents,)* ) }
                }
            }

            TypeEntryDetails::Array(item_id, length) => {
                let item_ty = type_space
                    .id_to_entry
                    .get(item_id)
                    .expect("unresolved type id for array");
                let item_ident = item_ty.type_ident(type_space, type_mod);

                quote! { [#item_ident; #length]}
            }

            TypeEntryDetails::Native(TypeEntryNative {
                type_name,
                impls: _,
                parameters,
            }) => {
                let path =
                    syn::parse_str::<syn::TypePath>(type_name).expect("type path wasn't valid");

                let type_idents = (!parameters.is_empty()).then(|| {
                    let type_idents = parameters.iter().map(|type_id| {
                        type_space
                            .id_to_entry
                            .get(type_id)
                            .expect("unresolved type id for tuple")
                            .type_ident(type_space, type_mod)
                    });
                    quote! { < #(#type_idents,)* > }
                });

                quote! {
                    #path
                    #type_idents
                }
            }

            TypeEntryDetails::Unit => quote! { () },
            TypeEntryDetails::String => quote! { String },
            TypeEntryDetails::Boolean => quote! { bool },
            TypeEntryDetails::JsonValue => quote! { serde_json::Value },
            TypeEntryDetails::Integer(name) | TypeEntryDetails::Float(name) => {
                syn::parse_str::<syn::TypePath>(name)
                    .unwrap()
                    .to_token_stream()
            }

            TypeEntryDetails::Reference(_) => panic!("references should be resolved by now"),
        }
    }

    pub(crate) fn type_parameter_ident(
        &self,
        type_space: &TypeSpace,
        lifetime_name: Option<&str>,
    ) -> TokenStream {
        let lifetime = lifetime_name.map(|s| {
            vec![
                TokenTree::from(Punct::new('\'', Spacing::Joint)),
                TokenTree::from(format_ident!("{}", s)),
            ]
            .into_iter()
            .collect::<TokenStream>()
        });
        match &self.details {
            // We special-case enums for which all variants are simple to let
            // them be passed as values rather than as references.
            // TODO we should probably cache "simpleness" of all variants
            // rather than iterating every time. We'll know it when the enum is
            // constructed.
            TypeEntryDetails::Enum(TypeEntryEnum { variants, .. })
                if variants
                    .iter()
                    .all(|variant| matches!(&variant.details, VariantDetails::Simple)) =>
            {
                self.type_ident(type_space, &type_space.settings.type_mod)
            }
            TypeEntryDetails::Enum(_)
            | TypeEntryDetails::Struct(_)
            | TypeEntryDetails::Newtype(_)
            | TypeEntryDetails::Vec(_)
            | TypeEntryDetails::Map(..)
            | TypeEntryDetails::Set(_)
            | TypeEntryDetails::Box(_)
            | TypeEntryDetails::Native(_)
            | TypeEntryDetails::Array(..)
            | TypeEntryDetails::JsonValue => {
                let ident = self.type_ident(type_space, &type_space.settings.type_mod);
                quote! {
                    & #lifetime #ident
                }
            }

            TypeEntryDetails::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let inner_ident = inner_ty.type_parameter_ident(type_space, lifetime_name);

                // Flatten nested Option types. This would only happen if the
                // schema encoded it; it's an odd construction.
                match &inner_ty.details {
                    TypeEntryDetails::Option(_) => inner_ident,
                    _ => quote! { Option<#inner_ident> },
                }
            }

            TypeEntryDetails::Tuple(items) => {
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .expect("unresolved type id for tuple")
                        .type_parameter_ident(type_space, lifetime_name)
                });

                if items.len() != 1 {
                    quote! { ( #(#type_streams),* ) }
                } else {
                    // Single-element tuples require special handling. In
                    // particular, they must have a trailing comma or else are
                    // treated as extraneously parenthesized types.
                    quote! { ( #(#type_streams,)* ) }
                }
            }

            TypeEntryDetails::Unit
            | TypeEntryDetails::Boolean
            | TypeEntryDetails::Integer(_)
            | TypeEntryDetails::Float(_) => {
                self.type_ident(type_space, &type_space.settings.type_mod)
            }
            TypeEntryDetails::String => quote! { & #lifetime str },

            TypeEntryDetails::Reference(_) => panic!("references should be resolved by now"),
        }
    }

    pub(crate) fn describe(&self) -> String {
        match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum { name, .. }) => format!("enum {}", name),
            TypeEntryDetails::Struct(TypeEntryStruct { name, .. }) => format!("struct {}", name),
            TypeEntryDetails::Newtype(TypeEntryNewtype { name, type_id, .. }) => {
                format!("newtype {} {}", name, type_id.0)
            }

            TypeEntryDetails::Unit => "()".to_string(),
            TypeEntryDetails::Option(type_id) => format!("option {}", type_id.0),
            TypeEntryDetails::Vec(type_id) => format!("vec {}", type_id.0),
            TypeEntryDetails::Map(key_id, value_id) => {
                format!("map {} {}", key_id.0, value_id.0)
            }
            TypeEntryDetails::Set(type_id) => format!("set {}", type_id.0),
            TypeEntryDetails::Box(type_id) => format!("box {}", type_id.0),
            TypeEntryDetails::Tuple(type_ids) => {
                format!(
                    "tuple ({})",
                    type_ids
                        .iter()
                        .map(|type_id| type_id.0.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypeEntryDetails::Array(type_id, length) => {
                format!("array {}; {}", type_id.0, length)
            }
            TypeEntryDetails::Boolean => "bool".to_string(),
            TypeEntryDetails::Native(TypeEntryNative {
                type_name: name, ..
            })
            | TypeEntryDetails::Integer(name)
            | TypeEntryDetails::Float(name) => name.clone(),
            TypeEntryDetails::String => "string".to_string(),

            TypeEntryDetails::JsonValue => "json value".to_string(),

            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }
}

fn make_doc(name: &str, description: Option<&String>, schema: &Schema) -> TokenStream {
    let desc = description.map_or(name, |desc| desc.as_str());
    let schema_json = serde_json::to_string_pretty(schema).unwrap();
    let schema_lines = schema_json.lines();
    quote! {
        #[doc = #desc]
        ///
        /// <details><summary>JSON schema</summary>
        ///
        /// ```json
        #(
            #[doc = #schema_lines]
        )*
        /// ```
        /// </details>
    }
}

fn strings_to_derives<'a>(
    derive_set: BTreeSet<&'a str>,
    type_derives: &'a BTreeSet<String>,
    extra_derives: &'a [String],
) -> impl Iterator<Item = TokenStream> + 'a {
    let mut combined_derives = derive_set.clone();
    combined_derives.extend(extra_derives.iter().map(String::as_str));
    combined_derives.extend(type_derives.iter().map(String::as_str));
    combined_derives.into_iter().map(|derive| {
        syn::parse_str::<syn::Path>(derive)
            .unwrap()
            .into_token_stream()
    })
}

/// Returns true iff...
/// - the enum is untagged
/// - all variants are 1-item tuple-types (aka newtype variants)
/// - the type of the newtype variant implements the required trait
fn untagged_newtype_variants(
    type_space: &TypeSpace,
    tag_type: &EnumTagType,
    variants: &[Variant],
    req_impl: TypeSpaceImpl,
) -> bool {
    tag_type == &EnumTagType::Untagged
        && variants.iter().all(|variant| {
            // If the variant is a one-element tuple...
            match &variant.details {
                VariantDetails::Item(type_id) => Some(type_id),
                _ => None,
            }
            .map_or_else(
                || false,
                |type_id| {
                    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                    // ... and its type has the required impl
                    type_entry.has_impl(type_space, req_impl)
                },
            )
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        type_entry::{SchemaWrapper, TypeEntry, TypeEntryStruct},
        TypeEntryDetails, TypeSpace,
    };

    #[test]
    fn test_ident() {
        let ts = TypeSpace::default();

        let type_mod = Some("the_mod".to_string());

        let t = TypeEntry::new_integer("u32");
        let ident = t.type_ident(&ts, &type_mod);
        assert_eq!(ident.to_string(), "u32");
        let parameter = t.type_parameter_ident(&ts, None);
        assert_eq!(parameter.to_string(), "u32");

        let t = TypeEntry::from(TypeEntryDetails::String);
        let ident = t.type_ident(&ts, &type_mod);
        assert_eq!(ident.to_string(), "String");
        let parameter = t.type_parameter_ident(&ts, None);
        assert_eq!(parameter.to_string(), "& str");
        let parameter = t.type_parameter_ident(&ts, Some("static"));
        assert_eq!(parameter.to_string(), "& 'static str");

        let t = TypeEntry::from(TypeEntryDetails::Unit);
        let ident = t.type_ident(&ts, &type_mod);
        assert_eq!(ident.to_string(), "()");
        let parameter = t.type_parameter_ident(&ts, None);
        assert_eq!(parameter.to_string(), "()");

        let t = TypeEntry::from(TypeEntryDetails::Struct(TypeEntryStruct {
            name: "SomeType".to_string(),
            rename: None,
            description: None,
            default: None,
            properties: vec![],
            deny_unknown_fields: false,
            schema: SchemaWrapper(schemars::schema::Schema::Bool(false)),
        }));

        let ident = t.type_ident(&ts, &type_mod);
        assert_eq!(ident.to_string(), "the_mod :: SomeType");
        let parameter = t.type_parameter_ident(&ts, None);
        assert_eq!(parameter.to_string(), "& SomeType");
        let parameter = t.type_parameter_ident(&ts, Some("a"));
        assert_eq!(parameter.to_string(), "& 'a SomeType");
    }
}
