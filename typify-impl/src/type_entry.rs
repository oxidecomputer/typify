use convert_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schemars::schema::Metadata;

use crate::{
    enums::{enum_impl, output_variant},
    structs::output_struct_property,
    util::{get_type_name, metadata_description},
    EnumTagType, Name, StructProperty, TypeEntry, TypeEntryEnum, TypeEntryIdentifier,
    TypeEntryNewtype, TypeEntryStruct, TypeId, TypeSpace, Variant, VariantDetails,
};

impl TypeEntryEnum {
    pub(crate) fn from_metadata(
        type_name: Name,
        metadata: &Option<Box<Metadata>>,
        tag_type: EnumTagType,
        variants: Vec<Variant>,
        deny_unknown_fields: bool,
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata, Case::Pascal).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntry::Enum(Self {
            name,
            rename,
            description,
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
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata, Case::Pascal).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntry::Struct(Self {
            name,
            rename,
            description,
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
    ) -> TypeEntry {
        let name = get_type_name(&type_name, metadata, Case::Pascal).unwrap();
        let rename = None;
        let description = metadata_description(metadata);

        TypeEntry::Newtype(Self {
            name,
            rename,
            description,
            type_id,
        })
    }
}

impl TypeEntry {
    pub(crate) fn new_builtin<S: ToString>(type_name: S) -> Self {
        TypeEntry::BuiltIn(type_name.to_string())
    }
    pub(crate) fn new_primitive<S: ToString>(type_name: S) -> Self {
        TypeEntry::Primitive(type_name.to_string())
    }

    pub(crate) fn name(&self) -> Option<&String> {
        match self {
            TypeEntry::Enum(TypeEntryEnum { name, .. })
            | TypeEntry::Struct(TypeEntryStruct { name, .. })
            | TypeEntry::Newtype(TypeEntryNewtype { name, .. }) => Some(name),

            _ => None,
        }
    }

    pub fn output(&self, type_space: &TypeSpace) -> TokenStream {
        match self {
            TypeEntry::Enum(TypeEntryEnum {
                name,
                rename,
                description,
                tag_type,
                variants,
                deny_unknown_fields,
            }) => {
                let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

                let mut serde_options = Vec::new();
                if let Some(old_name) = rename {
                    serde_options.push(quote! { rename = #old_name });
                }
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
                let serde = if serde_options.is_empty() {
                    quote! {}
                } else {
                    quote! { #[serde( #( #serde_options ),* )] }
                };

                let type_name = format_ident!("{}", name);

                let variants_decl = variants
                    .iter()
                    .map(|variant| output_variant(variant, type_space))
                    .collect::<Vec<_>>();

                let enum_impl = enum_impl(&type_name, variants);

                quote! {
                    #doc
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #serde
                    pub enum #type_name {
                        #(#variants_decl)*
                    }

                    #enum_impl
                }
            }

            TypeEntry::Struct(TypeEntryStruct {
                name,
                rename,
                description,
                properties,
                deny_unknown_fields,
            }) => {
                let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

                let mut serde_options = Vec::new();
                if let Some(old_name) = rename {
                    serde_options.push(quote! { rename = #old_name });
                }
                if *deny_unknown_fields {
                    serde_options.push(quote! { deny_unknown_fields });
                }
                let serde = if serde_options.is_empty() {
                    quote! {}
                } else {
                    quote! { #[serde( #( #serde_options ),* )] }
                };

                let type_name = format_ident!("{}", name);
                let properties = properties
                    .iter()
                    .map(|prop| output_struct_property(prop, type_space, true))
                    .collect::<Vec<_>>();

                quote! {
                    #doc
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #serde
                    pub struct #type_name {
                        #(#properties)*
                    }
                }
            }

            TypeEntry::Newtype(TypeEntryNewtype {
                name,
                rename,
                description,
                type_id,
            }) => {
                let doc = description.as_ref().map(|desc| quote! { #[doc = #desc] });

                let serde = rename.as_ref().map(|old_name| {
                    quote! {
                        #[serde(rename = #old_name)]
                    }
                });

                let type_name = format_ident!("{}", name);
                let sub_type = type_space.id_to_entry.get(type_id).unwrap();
                let sub_type_name = sub_type.type_ident(type_space, false);

                quote! {
                    #doc
                    #[derive(Serialize, Deserialize, Debug, Clone)]
                    #serde
                    pub struct #type_name(#sub_type_name);
                }
            }

            // These types require no definition as they're already defined.
            TypeEntry::BuiltIn(_)
            | TypeEntry::Primitive(_)
            | TypeEntry::String
            | TypeEntry::Option(_)
            | TypeEntry::Array(_)
            | TypeEntry::Map(_, _)
            | TypeEntry::Unit
            | TypeEntry::Tuple(_) => quote! {},

            // We should never get here as reference types should only be used
            // in-flight, but never recorded into the type space.
            TypeEntry::Reference(_) => unreachable!(),
        }
    }

    pub fn type_name(&self, type_space: &TypeSpace) -> String {
        self.type_ident(type_space, false).to_string()
    }

    pub fn type_ident_details(&self, type_space: &TypeSpace) -> TypeEntryIdentifier {
        let stream = self.type_ident(type_space, true);
        match self {
            TypeEntry::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let ident = inner_ty.type_ident_details(type_space);

                // Flatten nested Option types. This would only happen if the
                // schema encoded it; it's an odd construction.
                match inner_ty{
                    TypeEntry::Option(_) => ident,
                    _ => {
                        let TypeEntryIdentifier { ident, parameter } = ident;
                        TypeEntryIdentifier {
                            ident: quote! { Option<#ident> },
                            parameter: quote! { Option<#parameter> },
                        }
                    }
                }
            }

            TypeEntry::Unit | TypeEntry::BuiltIn(_) | TypeEntry::Primitive(_) => {
                TypeEntryIdentifier {
                    ident: stream.clone(),
                    parameter: stream,
                }
            }

            // TODO someday perhaps this should be an AsRef<str>
            TypeEntry::String => TypeEntryIdentifier {
                ident: stream,
                parameter: quote! { &str },
            },

            // We special-case enums for which all variants are simple to let
            // them be passed as values rather than as references.
            TypeEntry::Enum(TypeEntryEnum{ variants, .. })
                // TODO we should probably cache this rather than iterating
                // every time. We'll know it when the enum is constructed.
                if variants
                    .iter()
                    .all(|variant| matches!(variant.details, VariantDetails::Simple)) =>
            {
                TypeEntryIdentifier {
                    parameter: stream.clone(),
                    ident: stream,
                }
            }

            // In the general case, parameters are just a ref to the type name.
            _ => TypeEntryIdentifier {
                parameter: quote! { & #stream },
                ident: stream,
            },
        }
    }

    pub fn type_ident(&self, type_space: &TypeSpace, external: bool) -> TokenStream {
        match self {
            // Named types.
            TypeEntry::Enum(TypeEntryEnum { name, .. })
            | TypeEntry::Struct(TypeEntryStruct { name, .. })
            | TypeEntry::Newtype(TypeEntryNewtype { name, .. }) => match &type_space.type_mod {
                Some(type_mod) if external => {
                    let type_mod = format_ident!("{}", type_mod);
                    let type_name = format_ident!("{}", name);
                    quote! { #type_mod :: #type_name }
                }
                _ => {
                    let type_name = format_ident!("{}", name);
                    quote! { #type_name }
                }
            },

            TypeEntry::Option(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for option");
                let inner_ident = inner_ty.type_ident(type_space, external);

                // Flatten nested Option types. This would should happen if the
                // schema encoded it; it's an odd construction.
                match inner_ty {
                    TypeEntry::Option(_) => inner_ident,
                    _ => quote! { Option<#inner_ident> },
                }
            }

            TypeEntry::Array(id) => {
                let inner_ty = type_space
                    .id_to_entry
                    .get(id)
                    .expect("unresolved type id for array");
                let item = inner_ty.type_ident(type_space, external);

                quote! { Vec<#item> }
            }

            TypeEntry::Map(key_id, value_id) => {
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

            TypeEntry::Tuple(items) => {
                let type_streams = items.iter().map(|item| {
                    type_space
                        .id_to_entry
                        .get(item)
                        .expect("unresolved type id for tuple")
                        .type_ident(type_space, external)
                });

                quote! { ( #(#type_streams),* ) }
            }

            TypeEntry::Unit => quote! { () },
            TypeEntry::String => quote! { String },
            TypeEntry::BuiltIn(name) | TypeEntry::Primitive(name) => {
                syn::parse_str::<syn::TypePath>(name)
                    .unwrap()
                    .to_token_stream()
            }

            TypeEntry::Reference(_) => panic!("references should be resolved by now"),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            TypeEntry::Enum(TypeEntryEnum { name, .. }) => format!("enum {}", name),
            TypeEntry::Struct(TypeEntryStruct { name, .. }) => format!("struct {}", name),
            TypeEntry::Newtype(TypeEntryNewtype { name, type_id, .. }) => {
                format!("newtype {} {}", name, type_id.0)
            }

            TypeEntry::Unit => "()".to_string(),
            TypeEntry::Option(type_id) => format!("option {}", type_id.0),
            TypeEntry::Array(type_id) => format!("array {}", type_id.0),
            TypeEntry::Map(key_id, value_id) => format!("map {} {}", key_id.0, value_id.0),
            TypeEntry::Tuple(type_ids) => {
                format!(
                    "tuple ({})",
                    type_ids
                        .iter()
                        .map(|type_id| type_id.0.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            TypeEntry::BuiltIn(name) | TypeEntry::Primitive(name) => name.clone(),
            TypeEntry::String => "string".to_string(),

            TypeEntry::Reference(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{TypeEntry, TypeEntryStruct, TypeSpace};

    #[test]
    fn test_ident() {
        let ts = TypeSpace::default();

        let t = TypeEntry::new_primitive("u32");
        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "u32");
        assert_eq!(ident.parameter.to_string(), "u32");

        let t = TypeEntry::String;
        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "String");
        assert_eq!(ident.parameter.to_string(), "& str");

        let t = TypeEntry::Unit;
        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "()");
        assert_eq!(ident.parameter.to_string(), "()");

        let t = TypeEntry::Struct(TypeEntryStruct {
            name: "SomeType".to_string(),
            rename: None,
            description: None,
            properties: vec![],
            deny_unknown_fields: false,
        });

        let ident = t.type_ident_details(&ts);
        assert_eq!(ident.ident.to_string(), "SomeType");
        assert_eq!(ident.parameter.to_string(), "& SomeType");
    }
}
