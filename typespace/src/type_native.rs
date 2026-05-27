use proc_macro2::TokenStream;
use quote::quote;

use crate::{TypespaceTrait, TypespaceTraitSet};

#[derive(Debug, Clone)]
pub struct TypeNative<Id = ()> {
    pub name: String,
    pub impls: TypespaceTraitSet,
    pub parameters: Vec<Id>,
}

impl<Id> TypeNative<Id> {
    pub fn new_string_like(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            impls: [
                TypespaceTrait::Clone,
                TypespaceTrait::Debug,
                TypespaceTrait::Serialize,
                TypespaceTrait::Deserialize,
                TypespaceTrait::JsonSchema,
                TypespaceTrait::Ord,
                TypespaceTrait::PartialOrd,
                TypespaceTrait::Eq,
                TypespaceTrait::PartialEq,
                TypespaceTrait::Hash,
                TypespaceTrait::Display,
                TypespaceTrait::FromStr,
            ]
            .into_iter()
            .collect(),
            parameters: Default::default(),
        }
    }

    pub(crate) fn render(&self) -> TokenStream {
        // Native types are referenced by name, not rendered as declarations.
        let name = quote::format_ident!("{}", self.name);
        quote! { #name }
    }
}
