// Copyright 2022 Oxide Computer Company

use std::collections::BTreeSet;

use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;
use syn::Path;

use crate::{
    enums::output_variant,
    output::{OutputSpace, OutputSpaceMod},
    structs::{generate_serde_attr, DefaultFunction},
    util::{get_type_name, metadata_description},
    DefaultImpl, Name, TypeId, TypeSpace,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryEnum {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub tag_type: EnumTagType,
    pub variants: Vec<Variant>,
    pub deny_unknown_fields: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryStruct {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub properties: Vec<StructProperty>,
    pub deny_unknown_fields: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TypeEntryNewtype {
    pub name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub default: Option<WrappedValue>,
    pub type_id: TypeId,
    pub constraints: TypeEntryNewtypeConstraints,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeEntryNewtypeConstraints {
    None,
    EnumValue(Vec<WrappedValue>),
    String {
        max_length: Option<u32>,
        min_length: Option<u32>,
        pattern: Option<String>,
    },
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
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypeEntry {
    pub details: TypeEntryDetails,
    pub derives: Option<BTreeSet<&'static str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TypeEntryDetails {
    Enum(TypeEntryEnum),
    Struct(TypeEntryStruct),
    Newtype(TypeEntryNewtype),

    Option(TypeId),
    Box(TypeId),
    Array(TypeId),
    Map(TypeId),
    Set(TypeId),
    Tuple(Vec<TypeId>),
    Unit,
    /// Built-in complex types with no type generics such as Uuid
    BuiltIn(String),
    /// Boolean
    Boolean,
    /// Integers
    Integer(String),
    /// Floating point numbers; not Eq, Ord, or Hash
    Float(String),
    /// Strings... which we handle a little specially.
    String,

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
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        tag_type: EnumTagType,
        variants: Vec<Variant>,
        deny_unknown_fields: bool,
    ) -> TypeEntryDetails {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntryDetails::Enum(Self {
            name,
            rename,
            description,
            default: None,
            tag_type,
            variants,
            deny_unknown_fields,
        })
    }
}

impl TypeEntryStruct {
    pub(crate) fn from_metadata(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        properties: Vec<StructProperty>,
        deny_unknown_fields: bool,
    ) -> TypeEntryDetails {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);
        let default = metadata
            .as_ref()
            .and_then(|m| m.default.as_ref())
            .cloned()
            .map(WrappedValue::new);

        TypeEntryDetails::Struct(Self {
            name,
            rename,
            description,
            default,
            properties,
            deny_unknown_fields,
        })
    }
}

impl TypeEntryNewtype {
    pub(crate) fn from_metadata(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
    ) -> TypeEntryDetails {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::None,
        })
    }

    pub(crate) fn from_metadata_with_enum_values(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        enum_values: &[serde_json::Value],
    ) -> TypeEntryDetails {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntryDetails::Newtype(Self {
            name,
            rename,
            description,
            default: None,
            type_id,
            constraints: TypeEntryNewtypeConstraints::EnumValue(
                enum_values.iter().cloned().map(WrappedValue::new).collect(),
            ),
        })
    }

    pub(crate) fn from_metadata_with_string_validation(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        type_id: TypeId,
        validation: &schemars::schema::StringValidation,
    ) -> TypeEntryDetails {
        let name = get_type_name(&type_name, metadata).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        let schemars::schema::StringValidation {
            max_length,
            min_length,
            pattern,
        } = validation.clone();

        TypeEntryDetails::Newtype(Self {
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
        })
    }
}

impl From<TypeEntryDetails> for TypeEntry {
    fn from(details: TypeEntryDetails) -> Self {
        Self {
            details,
            derives: None,
        }
    }
}

impl TypeEntry {
    pub(crate) fn new_builtin<S: ToString>(type_name: S) -> Self {
        TypeEntry {
            details: TypeEntryDetails::BuiltIn(type_name.to_string()),
            derives: None,
        }
    }
    pub(crate) fn new_boolean() -> Self {
        TypeEntry {
            details: TypeEntryDetails::Boolean,
            derives: None,
        }
    }
    pub(crate) fn new_integer<S: ToString>(type_name: S) -> Self {
        TypeEntry {
            details: TypeEntryDetails::Integer(type_name.to_string()),
            derives: None,
        }
    }
    pub(crate) fn new_float<S: ToString>(type_name: S) -> Self {
        TypeEntry {
            details: TypeEntryDetails::Float(type_name.to_string()),
            derives: None,
        }
    }

    pub(crate) fn name(&self) -> Option<&String> {
        match &self.details {
            TypeEntryDetails::Enum(TypeEntryEnum { name, .. })
            | TypeEntryDetails::Struct(TypeEntryStruct { name, .. })
            | TypeEntryDetails::Newtype(TypeEntryNewtype { name, .. }) => Some(name),

            _ => None,
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
        } = enum_details;

        let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

        // TODO this is a one-off for some useful traits
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

        // ToString impl for enums that are made exclusively of simple variants.
        let simple_enum_impl = variants
            .iter()
            .map(|variant| {
                if let VariantDetails::Simple = variant.details {
                    Some(variant)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()
            .map(|simple_variants| {
                // It should not be possible to construct an untagged enum
                // exclusively of simple variants--it would not be usable.
                assert!(tag_type != &EnumTagType::Untagged);

                let (match_variants, match_strs): (Vec<_>, Vec<_>) = simple_variants
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
                        type Err = &'static str;

                        fn from_str(
                            value: &str
                        ) -> Result<Self, Self::Err> {
                            match value {
                                #(#match_strs => Ok(Self::#match_variants),)*
                                _ => Err("invalid value"),
                            }
                        }
                    }
                }
            });

        let default_impl = default.as_ref().map(|value| {
            let default_stream = self.output_value(type_space, &value.0).unwrap();
            quote! {
                impl Default for #type_name {
                    fn default() -> Self {
                        #default_stream
                    }
                }
            }
        });

        let untagged_newtype_impl =
            all_variants_support_from_string(type_space, tag_type, variants).then(|| {
                let (variant_name, variant_type): (Vec<_>, Vec<_>) = variants
                    .iter()
                    .map(|variant| {
                        let type_id = match &variant.details {
                            VariantDetails::Tuple(types) if types.len() == 1 => {
                                types.first().unwrap()
                            }
                            _ => unreachable!(),
                        };
                        let type_entry = type_space.id_to_entry.get(type_id).unwrap();

                        (
                            format_ident!("{}", variant.name),
                            type_entry.type_ident(type_space, &None),
                        )
                    })
                    .unzip();

                // Implement From<String> by doing a try_from() for each
                // variant.
                quote! {
                    impl std::convert::TryFrom<&str> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: &str) -> Result<Self, Self::Error> {
                            // Seed with an error to make successive cases more
                            // consistent; this will never reach the user.
                            Err("")
                            #(
                                .or_else(|_: Self::Error| {
                                    Ok(Self::#variant_name(
                                        #variant_type::try_from(value)?,
                                    ))
                                })
                            )*
                            .map_err(|_: Self::Error| {
                                "string conversion failed for all variants"
                            })
                        }
                    }
                    impl std::convert::TryFrom<&String> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: &String) -> Result<Self, Self::Error> {
                            Self::try_from(value.as_str())
                        }
                    }
                    impl std::convert::TryFrom<String> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: String) -> Result<Self, Self::Error> {
                            Self::try_from(value.as_str())
                        }
                    }
                }
            });

        let derives = strings_to_derives(derive_set, &type_space.settings.extra_derives);

        let item = quote! {
            #doc
            #[derive(#(#derives),*)]
            #serde
            pub enum #type_name {
                #(#variants_decl)*
            }

            #simple_enum_impl
            #default_impl
            #untagged_newtype_impl
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
        } = struct_details;
        let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

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

        let derives =
            strings_to_derives(derive_set, &type_space.settings.extra_derives).collect::<Vec<_>>();

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
            },
        );

        // If there's a default value, generate an impl Default
        if let Some(value) = default {
            let default_stream = self.output_value(type_space, &value.0).unwrap();
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
                            builder::#type_name::default()
                        }
                    }
                },
            );

            output.add_item(
                OutputSpaceMod::Builder,
                name,
                quote! {
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

                    impl std::convert::TryFrom<#type_name> for super::#type_name {
                        type Error = String;

                        fn try_from(value: #type_name)
                            -> Result<Self, Self::Error>
                        {
                            Ok(Self {
                                #(
                                    #prop_name: value.#prop_name?,
                                )*
                            })
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
        } = newtype_details;
        let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

        let serde = rename.as_ref().map(|old_name| {
            quote! {
                #[serde(rename = #old_name)]
            }
        });

        let type_name = format_ident!("{}", name);
        let sub_type = type_space.id_to_entry.get(type_id).unwrap();
        let sub_type_name = sub_type.type_ident(type_space, &None);

        let constraint_impl = match constraints {
            TypeEntryNewtypeConstraints::None => None,

            TypeEntryNewtypeConstraints::EnumValue(enum_values) => {
                let value_output = enum_values
                    .iter()
                    .map(|value| sub_type.output_value(type_space, &value.0));
                // TODO if the sub_type is a string we could probably impl
                // TryFrom<&str> as well
                Some(quote! {
                    impl std::convert::TryFrom<#sub_type_name> for #type_name {
                        type Error = &'static str;

                        fn try_from(
                            value: #sub_type_name
                        ) -> Result<Self, Self::Error> {
                            if ![
                                #(#value_output,)*
                            ].contains(&value) {
                                Err("invalid value")
                            } else {
                                Ok(Self(value))
                            }
                        }
                    }
                })
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
                            return Err(#err);
                        }
                    }
                });
                let min = min_length.map(|v| {
                    let v = v as usize;
                    let err = format!("shorter than {} characters", v);
                    quote! {
                        if value.len() < #v {
                            return Err(#err);
                        }
                    }
                });
                let pat = pattern.as_ref().map(|p| {
                    let err = format!("doesn't match pattern \"{}\"", p);
                    quote! {
                        if regress::Regex::new(#p).unwrap().find(value).is_none() {
                            return Err(#err);
                        }
                    }
                });

                // We're going to impl Deserialize so we can remove it
                // from the set of derived impls.
                derive_set.remove("Deserialize");

                Some(quote! {
                    impl std::convert::TryFrom<&str> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: &str) -> Result<Self, Self::Error> {
                            #max
                            #min
                            #pat

                            Ok(Self(value.to_string()))
                        }
                    }
                    impl std::convert::TryFrom<&String> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: &String) -> Result<Self, Self::Error> {
                            Self::try_from(value.as_str())
                        }
                    }
                    impl std::convert::TryFrom<String> for #type_name {
                        type Error = &'static str;

                        fn try_from(value: String) -> Result<Self, Self::Error> {
                            Self::try_from(value.as_str())
                        }
                    }
                    impl<'de> serde::Deserialize<'de> for #type_name {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            Self::try_from(String::deserialize(deserializer)?)
                                .map_err(|e| {
                                    <D::Error as serde::de::Error>::custom(
                                        e.to_string(),
                                    )
                                })
                        }
                    }
                })
            }
        };

        // If there are no constraints, let folks directly access the
        // value.
        let vis = match constraints {
            TypeEntryNewtypeConstraints::None => Some(quote! {pub}),
            _ => None,
        };

        let default_impl = default.as_ref().map(|value| {
            let default_stream = self.output_value(type_space, &value.0).unwrap();
            quote! {
                impl Default for #type_name {
                    fn default() -> Self {
                        #default_stream
                    }
                }
            }
        });

        let derives = strings_to_derives(derive_set, &type_space.settings.extra_derives);

        let item = quote! {
            #doc
            #[derive(#(#derives),*)]
            #serde
            pub struct #type_name(#vis #sub_type_name);

            impl std::ops::Deref for #type_name {
                type Target = #sub_type_name;
                fn deref(&self) -> &Self::Target {
                    &self.0
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

            TypeEntryDetails::Array(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for array");
                let item = inner_ty.type_ident(type_space, type_mod);

                quote! { Vec<#item> }
            }

            TypeEntryDetails::Map(type_id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(type_id)
                    .expect("unresolved type id for map")
                    .type_ident(type_space, type_mod);

                quote! { std::collections::HashMap<String, #inner_ty> }
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
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .expect("unresolved type id for tuple")
                        .type_ident(type_space, type_mod)
                });

                quote! { ( #(#type_streams),* ) }
            }

            TypeEntryDetails::Unit => quote! { () },
            TypeEntryDetails::String => quote! { String },
            TypeEntryDetails::Boolean => quote! { bool },
            TypeEntryDetails::BuiltIn(name)
            | TypeEntryDetails::Integer(name)
            | TypeEntryDetails::Float(name) => syn::parse_str::<syn::TypePath>(name)
                .unwrap()
                .to_token_stream(),

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
            TypeEntryDetails::Enum(TypeEntryEnum{ variants, .. })
                // TODO we should probably cache this rather than iterating
                // every time. We'll know it when the enum is constructed.
                if variants
                    .iter()
                    .all(|variant| matches!(variant.details, VariantDetails::Simple)) =>
            {
                self.type_ident(type_space, &type_space.settings.type_mod)
            }

            TypeEntryDetails::Enum(_)
            | TypeEntryDetails::Struct(_)
            | TypeEntryDetails::Newtype(_)
            | TypeEntryDetails::Array(_)
            | TypeEntryDetails::Map(_)
            | TypeEntryDetails::Set(_)
            | TypeEntryDetails::Box(_)
            | TypeEntryDetails::BuiltIn(_) => {
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

                quote! { ( #(#type_streams),* ) }
            }

            TypeEntryDetails::Unit | TypeEntryDetails::Boolean|TypeEntryDetails::Integer(_) | TypeEntryDetails::Float(_) => {
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
            TypeEntryDetails::Array(type_id) => format!("array {}", type_id.0),
            TypeEntryDetails::Map(type_id) => format!("map {}", type_id.0),
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
            TypeEntryDetails::Boolean => "bool".to_string(),
            TypeEntryDetails::BuiltIn(name)
            | TypeEntryDetails::Integer(name)
            | TypeEntryDetails::Float(name) => name.clone(),
            TypeEntryDetails::String => "string".to_string(),

            TypeEntryDetails::Reference(_) => unreachable!(),
        }
    }
}

fn strings_to_derives<'a>(
    derive_set: BTreeSet<&'a str>,
    extra_derives: &'a [String],
) -> impl Iterator<Item = TokenStream> + 'a {
    derive_set
        .into_iter()
        .chain(extra_derives.iter().map(String::as_str))
        .map(|derive| {
            syn::parse_str::<syn::Path>(derive)
                .unwrap()
                .into_token_stream()
        })
}

fn all_variants_support_from_string(
    type_space: &TypeSpace,
    tag_type: &EnumTagType,
    variants: &[Variant],
) -> bool {
    tag_type == &EnumTagType::Untagged
        && variants.iter().all(|variant| {
            // If the variant is a tuple...
            match &variant.details {
                VariantDetails::Tuple(types) if types.len() == 1 => types.first(),
                _ => None,
            }
            .map_or_else(
                || false,
                |type_id| {
                    let type_entry = type_space.id_to_entry.get(type_id).unwrap();
                    matches!(
                        &type_entry.details,
                        // ... and its type is either a string
                        TypeEntryDetails::String
                        // ... or a newtype wrapper around a constrained string
                            | TypeEntryDetails::Newtype(TypeEntryNewtype {
                                constraints: TypeEntryNewtypeConstraints::String { .. },
                                ..
                            })
                    )
                },
            )
        })
}

#[cfg(test)]
mod tests {
    use crate::{
        type_entry::{TypeEntry, TypeEntryStruct},
        TypeEntryDetails, TypeSpace,
    };

    #[test]
    fn test_ident() {
        let ts = TypeSpace::default();

        let type_mod = Some("tha_mod".to_string());

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
        }));

        let ident = t.type_ident(&ts, &type_mod);
        assert_eq!(ident.to_string(), "tha_mod :: SomeType");
        let parameter = t.type_parameter_ident(&ts, None);
        assert_eq!(parameter.to_string(), "& SomeType");
        let parameter = t.type_parameter_ident(&ts, Some("a"));
        assert_eq!(parameter.to_string(), "& 'a SomeType");
    }
}
