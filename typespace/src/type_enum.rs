use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{type_struct::StructProperty, TypeCommon, TypespaceRenderer};

#[derive(Debug, Clone)]
pub struct TypeEnum<Id> {
    pub common: TypeCommon,
    pub tag_type: EnumTagType,
    pub variants: Vec<EnumVariant<Id>>,
    pub deny_unknown_fields: bool,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeEnum<Id> {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        tag_type: EnumTagType,
        variants: Vec<EnumVariant<Id>>,
        deny_unknown_fields: bool,
    ) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            tag_type,
            variants,
            deny_unknown_fields,
        }
    }

    pub(crate) fn children(&self) -> Vec<Id> {
        self.variants
            .iter()
            .flat_map(|v| v.children())
            .collect()
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });

        let serde = match &self.tag_type {
            EnumTagType::External => TokenStream::new(),
            EnumTagType::Internal { tag } => quote! { #[serde(tag = #tag)] },
            EnumTagType::Adjacent { tag, content } => {
                quote! { #[serde(tag = #tag, content = #content)] }
            }
            EnumTagType::Untagged => quote! { #[serde(untagged)] },
        };

        let variants = self.variants.iter().map(|variant| {
            let name = format_ident!("{}", variant.rust_name);
            let variant_serde = variant
                .rename
                .as_ref()
                .map(|n| quote! { #[serde(rename = #n)] });
            let description = variant
                .description
                .as_ref()
                .map(|d| quote! { #[doc = #d] });

            let data = match &variant.details {
                VariantDetails::Unit => TokenStream::new(),
                VariantDetails::Item(id) => {
                    let t = typespace.render_type(id);
                    quote! { (#t) }
                }
                VariantDetails::Tuple(ids) => {
                    let items = ids.iter().map(|id| typespace.render_type(id));
                    quote! { ( #( #items, )* ) }
                }
                VariantDetails::Struct(props) => {
                    let props = props
                        .iter()
                        .map(|p| typespace.render_struct_property(p, false, None));
                    quote! { { #( #props, )* } }
                }
            };

            quote! {
                #description
                #variant_serde
                #name #data
            }
        });

        let name_ident = format_ident!("{}", self.common.name);

        quote! {
            #description
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            #serde
            pub enum #name_ident {
                #( #variants, )*
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnumTagType {
    External,
    Internal { tag: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumVariant<Id> {
    pub rust_name: String,
    pub rename: Option<String>,
    pub description: Option<String>,
    pub details: VariantDetails<Id>,
}

impl<Id: Clone> EnumVariant<Id> {
    pub(crate) fn children(&self) -> Vec<Id> {
        match &self.details {
            VariantDetails::Unit => vec![],
            VariantDetails::Item(id) => vec![id.clone()],
            VariantDetails::Tuple(ids) => ids.clone(),
            VariantDetails::Struct(props) => props.iter().map(|p| p.type_id.clone()).collect(),
        }
    }

    pub(crate) fn contained_children(&self) -> Vec<Id> {
        self.children()
    }

    pub(crate) fn rewrite_id(&mut self, from: &Id, to: &Id)
    where
        Id: PartialEq,
    {
        let rewrite = |id: &mut Id| {
            if id == from {
                *id = to.clone();
            }
        };
        match &mut self.details {
            VariantDetails::Unit => {}
            VariantDetails::Item(id) => rewrite(id),
            VariantDetails::Tuple(ids) => ids.iter_mut().for_each(rewrite),
            VariantDetails::Struct(props) => props.iter_mut().for_each(|p| rewrite(&mut p.type_id)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariantDetails<Id> {
    Unit,
    Item(Id),
    Tuple(Vec<Id>),
    Struct(Vec<StructProperty<Id>>),
}
