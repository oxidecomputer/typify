use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;

use crate::{
    enums::{enum_impl, output_variant},
    structs::output_struct_property,
    util::{metadata_description, metadata_title, recase},
    TypeDetails, TypeEntry, TypeSpace,
};

impl TypeEntry {
    pub(crate) fn from_metadata(
        type_name: Option<&str>,
        metadata: &Option<Box<Metadata>>,
        details: TypeDetails,
    ) -> Self {
        let (name, rename) = match type_name
            .map(ToString::to_string)
            .or_else(|| metadata_title(metadata))
        {
            Some(name) => {
                let (name, rename) = recase(name, Case::Pascal);
                (Some(name), rename)
            }
            None => (None, None),
        };
        Self {
            name,
            rename,
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

            // These types require no definition as they're already defined.
            TypeDetails::BuiltIn | TypeDetails::Option(_) | &TypeDetails::Array(_) => quote! {},

            // TODO we only expect to end up with a reference type here if
            // there's a type defined in the definitions table (i.e. the
            // reference targets) that's a simple alias to another type named
            // in the same table.
            TypeDetails::Reference(_) => todo!(),

            TypeDetails::Unit => todo!(),
            TypeDetails::Tuple(_) => todo!(),

            // TODO remove this; here for development
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }

    pub fn type_ident(&self, type_space: &TypeSpace, external: bool) -> TokenStream {
        match &self.details {
            TypeDetails::Option(id) => {
                let inner_ty = type_space.id_to_entry.get(id).unwrap();
                let stream = inner_ty.type_ident(type_space, external);

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
                let stream = inner_ty.type_ident(type_space, external);

                quote! { Vec<#stream> }
            }

            TypeDetails::Tuple(items) => {
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .unwrap()
                        .type_ident(type_space, external)
                });

                quote! { ( #(#type_streams),* ) }
            }

            TypeDetails::Unit => quote! {()},

            _ if self.name.is_none() => todo!("{:#?}", self),
            TypeDetails::BuiltIn => {
                let name = self.name.as_ref().unwrap();
                let tok = syn::parse_str::<syn::TypePath>(name).unwrap();
                tok.to_token_stream()
            }
            _ => match &type_space.type_mod {
                Some(type_mod) if external => {
                    let tmod = format_ident!("{}", type_mod);
                    let tname = format_ident!("{}", self.name.as_ref().unwrap());
                    quote! { #tmod :: #tname }
                }
                _ => {
                    let tname = format_ident!("{}", self.name.as_ref().unwrap());
                    quote! { #tname }
                }
            },
        }
    }
}
