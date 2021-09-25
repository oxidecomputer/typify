use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;

use crate::{
    enums::output_variant,
    structs::output_struct_property,
    util::{metadata_description, metadata_title},
    TypeDetails, TypeEntry, TypeSpace,
};

impl TypeEntry {
    pub(crate) fn from_metadata(
        type_name: Option<&str>,
        metadata: &Option<Box<Metadata>>,
        details: TypeDetails,
    ) -> Self {
        Self {
            name: metadata_title(metadata).or_else(|| type_name.map(ToString::to_string)),
            description: metadata_description(metadata),
            details,
        }
    }

    pub fn output(&self, type_space: &TypeSpace) -> TokenStream {
        match &self.details {
            TypeDetails::Enum { tag_type, variants } => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);

                let tag = match tag_type {
                    crate::EnumTagType::External => quote! {},
                    crate::EnumTagType::Internal { tag } => {
                        quote! {#[serde(tag = #tag)]}
                    }
                    crate::EnumTagType::Adjacent { tag, content } => {
                        quote! {#[serde(tag = #tag, content = #content)]}
                    }
                    crate::EnumTagType::Untagged => {
                        quote! {#[serde(untagged)]}
                    }
                };

                let variants = variants
                    .iter()
                    .map(|variant| output_variant(variant, type_space))
                    .collect::<Vec<_>>();

                quote! {
                    #[derive(serde::Serialize, serde::Deserialize)]
                    #tag
                    enum #type_name {
                        #(#variants)*
                    }
                }
            }

            TypeDetails::Struct(props) => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);
                let properties = props
                    .iter()
                    .map(|prop| output_struct_property(prop, type_space))
                    .collect::<Vec<_>>();
                quote! {
                    #[derive(serde::Serialize, serde::Deserialize)]
                    struct #type_name {
                        #(#properties)*
                    }
                }
            }

            // These types require no definition as they're already defined.
            TypeDetails::BuiltIn | TypeDetails::Option(_) | &TypeDetails::Array(_) => quote! {},

            // It shouldn't be possible to have a type's final state be a
            // reference.
            TypeDetails::Reference(_) => unreachable!(),

            TypeDetails::Unit => todo!(),
            TypeDetails::Tuple(_) => todo!(),

            // TODO remove this; here for development
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }

    pub fn type_ident(&self, type_space: &TypeSpace) -> TokenStream {
        match &self.details {
            TypeDetails::Option(id) => {
                let inner_ty = type_space.id_to_entry.get(id).unwrap();
                let stream = inner_ty.type_ident(type_space);

                // Flatten nested Option types. This happens commonly because
                // we treat both Null-type alternatives as well as non-required
                // object properties by wrapping them in this optional type.
                match inner_ty.details {
                    TypeDetails::Option(_) => stream,
                    _ => quote! { Option<#stream> },
                }
            }

            TypeDetails::Array(id) => {
                let inner_ty = type_space.id_to_entry.get(id).unwrap();
                let stream = inner_ty.type_ident(type_space);

                quote! { Vec<#stream> }
            }

            TypeDetails::Tuple(items) => {
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .unwrap()
                        .type_ident(type_space)
                });

                quote! { ( #(#type_streams),* ) }
            }

            TypeDetails::Unit => quote! {()},

            _ if self.name.is_none() => todo!("{:#?}", self),
            _ => {
                let name = self.name.clone().unwrap();
                let tok = syn::parse_str::<syn::TypePath>(&name).unwrap();
                tok.to_token_stream()
            }
        }
    }
}
