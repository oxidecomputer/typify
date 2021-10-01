use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;

use crate::{
    enums::{enum_impl, output_variant},
    structs::output_struct_property,
    util::{get_type_name, metadata_description},
    Name, TypeDetails, TypeEntry, TypeSpace,
};

impl TypeEntry {
    pub(crate) fn from_metadata(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        details: TypeDetails,
    ) -> Self {
        let name = get_type_name(&type_name, metadata, Case::Pascal);

        Self {
            name,
            // TODO
            rename: None,
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

                let enum_impl = enum_impl(&type_name, variants);

                let variants = variants
                    .iter()
                    .map(|variant| output_variant(variant, type_space))
                    .collect::<Vec<_>>();

                quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #tag
                    pub enum #type_name {
                        #(#variants)*
                    }

                    #enum_impl
                }
            }

            TypeDetails::Struct(props) => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);
                let properties = props
                    .iter()
                    .map(|prop| output_struct_property(prop, type_space, true))
                    .collect::<Vec<_>>();
                quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    pub struct #type_name {
                        #(#properties)*
                    }
                }
            }

            TypeDetails::Newtype(type_id) => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);
                let sub_type = type_space.id_to_entry.get(type_id).unwrap();
                let sub_type_name = sub_type.type_ident(type_space, false);
                quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    pub struct #type_name(#sub_type_name);
                }
            }

            // These types require no definition as they're already defined.
            TypeDetails::BuiltIn
            | TypeDetails::Option(_)
            | TypeDetails::Array(_)
            | TypeDetails::Unit
            | TypeDetails::Tuple(_) => quote! {},

            // We should never get here as reference types should only be used
            // in-flight, but never recorded into the type space.
            TypeDetails::Reference(_) => unreachable!(),
        }
    }

    pub fn type_ident(&self, type_space: &TypeSpace, external: bool) -> TokenStream {
        match &self.details {
            TypeDetails::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let stream = inner_ty.type_ident(type_space, external);

                // Flatten nested Option types. This would should happen if the
                // schema encoded it, and it's an odd construction.
                match inner_ty.details {
                    TypeDetails::Option(_) => stream,
                    _ => quote! { Option<#stream> },
                }
            }

            TypeDetails::Array(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for array");
                let stream = inner_ty.type_ident(type_space, external);

                quote! { Vec<#stream> }
            }

            TypeDetails::Tuple(items) => {
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .expect("unresolved type id for tuple")
                        .type_ident(type_space, external)
                });

                quote! { ( #(#type_streams),* ) }
            }

            TypeDetails::Unit => quote! { () },

            _ if self.name.is_none() => panic!("unnamed type {:#?}", self),

            // Simple built-in types for which the name is the identifier.
            TypeDetails::BuiltIn => {
                let name = self.name.as_ref().unwrap();
                let tok = syn::parse_str::<syn::TypePath>(name).unwrap();
                tok.to_token_stream()
            }

            _ => match &type_space.type_mod {
                Some(type_mod) if external => {
                    let type_mod = format_ident!("{}", type_mod);
                    let type_name = format_ident!("{}", self.name.as_ref().unwrap());
                    quote! { #type_mod :: #type_name }
                }
                _ => {
                    let type_name = format_ident!("{}", self.name.as_ref().unwrap());
                    quote! { #type_name }
                }
            },
        }
    }
}
