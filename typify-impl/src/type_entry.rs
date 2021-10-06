use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;

use crate::{
    enums::{enum_impl, output_variant},
    structs::output_struct_property,
    util::{get_type_name, metadata_description},
    Name, TypeDetails, TypeEntry, TypeEntryIdentifier, TypeSpace,
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
            TypeDetails::Enum {
                tag_type,
                variants,
                deny_unknown_fields,
            } => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);

                let mut serde_options = Vec::new();
                match tag_type {
                    crate::EnumTagType::External => {}
                    crate::EnumTagType::Internal { tag } => {
                        serde_options.push(quote! { tag = #tag });
                    }
                    crate::EnumTagType::Adjacent { tag, content } => {
                        serde_options.push(quote! { tag = #tag });
                        serde_options.push(quote! { content = #content });
                    }
                    crate::EnumTagType::Untagged => {
                        serde_options.push(quote! { untagged });
                    }
                }
                if *deny_unknown_fields {
                    serde_options.push(quote! { deny_unknown_fields });
                }

                let enum_impl = enum_impl(&type_name, variants);

                let variants = variants
                    .iter()
                    .map(|variant| output_variant(variant, type_space))
                    .collect::<Vec<_>>();

                let serde = if serde_options.is_empty() {
                    quote! {}
                } else {
                    quote! { #[serde( #( #serde_options ),* )] }
                };

                quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #serde
                    pub enum #type_name {
                        #(#variants)*
                    }

                    #enum_impl
                }
            }

            TypeDetails::Struct {
                properties,
                deny_unknown_fields,
            } => {
                let type_name = self.name.as_ref().unwrap();
                let type_name = format_ident!("{}", type_name);
                let properties = properties
                    .iter()
                    .map(|prop| output_struct_property(prop, type_space, true))
                    .collect::<Vec<_>>();
                let serde = if *deny_unknown_fields {
                    quote! { #[serde(deny_unknown_fields)]}
                } else {
                    quote! {}
                };
                quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #serde
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
            | TypeDetails::Primitive
            | TypeDetails::String
            | TypeDetails::Option(_)
            | TypeDetails::Array(_)
            | TypeDetails::Map(_, _)
            | TypeDetails::Unit
            | TypeDetails::Tuple(_) => quote! {},

            // We should never get here as reference types should only be used
            // in-flight, but never recorded into the type space.
            TypeDetails::Reference(_) => unreachable!(),
        }
    }

    pub fn type_name(&self, type_space: &TypeSpace) -> String {
        self.type_ident(type_space, false).to_string()
    }

    pub fn type_ident_details(&self, type_space: &TypeSpace) -> TypeEntryIdentifier {
        let stream = self.type_ident(type_space, true);
        match &self.details {
            TypeDetails::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let ident = inner_ty.type_ident_details(type_space);

                // Flatten nested Option types. This would should happen if the
                // schema encoded it; it's an odd construction.
                match inner_ty.details {
                    TypeDetails::Option(_) => ident,
                    _ => {
                        let TypeEntryIdentifier { ident, parameter } = ident;
                        TypeEntryIdentifier {
                            ident: quote! { Option<#ident> },
                            parameter: quote! { Option<#parameter> },
                        }
                    }
                }
            }

            TypeDetails::Unit | TypeDetails::BuiltIn | TypeDetails::Primitive => {
                TypeEntryIdentifier {
                    ident: stream.clone(),
                    parameter: stream,
                }
            }

            // TODO someday perhaps this should be an AsRef<str>
            TypeDetails::String => TypeEntryIdentifier {
                ident: stream,
                parameter: quote! { &str },
            },

            // In the general case, parameters are just a ref to the type name.
            _ => TypeEntryIdentifier {
                parameter: quote! { & #stream },
                ident: stream,
            },
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
                // schema encoded it; it's an odd construction.
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

            TypeDetails::Map(key_id, value_id) => {
                let key_ty = type_space
                    .id_to_entry
                    .get(key_id)
                    .expect("unresolved type id for array")
                    .type_ident(type_space, external);
                let value_ty = type_space
                    .id_to_entry
                    .get(value_id)
                    .expect("unresolved type id for array")
                    .type_ident(type_space, external);

                quote! { std::collections::HashMap<#key_ty, #value_ty> }
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
            TypeDetails::BuiltIn | TypeDetails::String | TypeDetails::Primitive => {
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

    pub fn describe(&self) -> String {
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| "<anonymous>".to_string());
        match &self.details {
            TypeDetails::Enum { .. } => format!("enum {}", name),
            TypeDetails::Struct { .. } => format!("struct {}", name),
            TypeDetails::Unit => "()".to_string(),
            TypeDetails::Option(type_id) => format!("option {}", type_id.0),
            TypeDetails::Array(type_id) => format!("array {}", type_id.0),
            TypeDetails::Map(key_id, value_id) => format!("map {} {}", key_id.0, value_id.0),
            TypeDetails::Tuple(_) => "tuple".to_string(),
            TypeDetails::BuiltIn | TypeDetails::Primitive | TypeDetails::String => name,
            TypeDetails::Newtype(type_id) => format!("newtype {} {}", name, type_id.0),
            TypeDetails::Reference(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{TypeDetails, TypeEntry, TypeSpace};

    #[test]
    fn test_ident() {
        let ts = TypeSpace::default();

        let t = TypeEntry {
            name: Some("u32".to_string()),
            rename: None,
            description: None,
            details: TypeDetails::Primitive,
        };

        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "u32");
        assert_eq!(ident.parameter.to_string(), "u32");

        let t = TypeEntry {
            name: Some("String".to_string()),
            rename: None,
            description: None,
            details: TypeDetails::String,
        };

        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "String");
        assert_eq!(ident.parameter.to_string(), "& str");

        let t = TypeEntry {
            name: None,
            rename: None,
            description: None,
            details: TypeDetails::Unit,
        };

        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "()");
        assert_eq!(ident.parameter.to_string(), "()");

        let t = TypeEntry {
            name: Some("SomeType".to_string()),
            rename: None,
            description: None,
            details: TypeDetails::Struct {
                properties: vec![],
                deny_unknown_fields: false,
            },
        };

        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "SomeType");
        assert_eq!(ident.parameter.to_string(), "& SomeType");
    }
}
