use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    schemalet::SchemaRef,
    typespace::{TypeCommon, Typespace},
};

#[derive(Debug, Clone)]
pub struct TypeTypeAlias {
    pub common: TypeCommon,
    pub target: SchemaRef,
}

impl TypeTypeAlias {
    pub(crate) fn children(&self) -> Vec<SchemaRef> {
        vec![self.target.clone()]
    }

    pub(crate) fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        // TODO 2/4/2026
        // I'm not really sure what to do here; the type alias shouldn't really
        // confer any context to the inner type. I think this is fine, but also
        // may mean that I want to change the name of this pass/fn/concept to
        // indicate that we should only express context insofar as we have it,
        // and that the cardinality of this fn may be different from that of
        // children().
        Vec::new()
    }

    pub(crate) fn render(&self, typespace: &Typespace) -> TokenStream {
        let Self {
            common:
                TypeCommon {
                    name: _,
                    description,
                    built,
                    default: _,
                },
            target,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name = built.as_ref().unwrap().name.to_string();
        let name_ident = format_ident!("{name}");

        let target_ident = typespace.render_ident(target);

        quote! {
            #description
            pub type #name_ident = #target_ident;
        }
    }
}
