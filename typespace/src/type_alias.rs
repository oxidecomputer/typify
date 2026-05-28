use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{TypeCommon, TypespaceRenderer};

#[derive(Debug, Clone)]
pub struct TypeTypeAlias<Id> {
    pub common: TypeCommon,
    pub target: Id,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeTypeAlias<Id> {
    pub fn new(name: impl Into<String>, description: Option<String>, target: Id) -> Self {
        Self {
            common: TypeCommon {
                name: name.into(),
                description,
                default: None,
                built: None,
            },
            target,
        }
    }

    pub(crate) fn children(&self) -> Vec<Id> {
        vec![self.target.clone()]
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let Self {
            common:
                TypeCommon {
                    name: _,
                    description,
                    built,
                    default: _,
                },
            target: type_id,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name = built.as_ref().unwrap().name.to_string();
        let name_ident = format_ident!("{name}");

        let target_ident = typespace.render_ident(type_id);

        quote! {
            #description
            pub type #name_ident = #target_ident;
        }
    }
}
