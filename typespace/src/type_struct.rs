use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{JsonValue, TypeCommon, TypespaceRenderer};

#[derive(Debug, Clone)]
pub struct TypeStruct<Id> {
    pub common: TypeCommon,
    pub properties: Vec<StructProperty<Id>>,
    pub deny_unknown_fields: bool,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeStruct<Id> {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        properties: Vec<StructProperty<Id>>,
        deny_unknown_fields: bool,
    ) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            properties,
            deny_unknown_fields,
        }
    }

    pub(crate) fn children(&self) -> Vec<Id> {
        self.properties.iter().map(|p| p.type_id.clone()).collect()
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });
        let name_ident = format_ident!("{}", self.common.name);

        let mut helper_fns: Vec<TokenStream> = Vec::new();
        let properties: Vec<TokenStream> = self
            .properties
            .iter()
            .map(|p| {
                let default_fn_name =
                    if let StructPropertyState::DefaultValue(json_value) = &p.state {
                        let fn_name =
                            format!("__default_{}_{}", self.common.name, p.rust_name);
                        let fn_ident = format_ident!("{}", fn_name);
                        let ty_ident = typespace.render_ident(&p.type_id);
                        let json_str = json_value.0.to_string();
                        helper_fns.push(quote! {
                            fn #fn_ident() -> #ty_ident {
                                ::serde_json::from_str(#json_str).unwrap()
                            }
                        });
                        Some(fn_name)
                    } else {
                        None
                    };
                typespace.render_struct_property(p, true, default_fn_name.as_deref())
            })
            .collect();

        quote! {
            #description
            #( #helper_fns )*
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            pub struct #name_ident {
                #( #properties, )*
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructProperty<Id> {
    pub rust_name: Ident,
    pub json_name: StructPropertySerde,
    pub state: StructPropertyState<Id>,
    pub description: Option<String>,
    pub type_id: Id,
}

impl<Id: Clone> StructProperty<Id> {
    pub fn new(
        rust_name: Ident,
        json_name: StructPropertySerde,
        state: StructPropertyState<Id>,
        description: Option<String>,
        type_id: Id,
    ) -> Self {
        Self {
            rust_name,
            json_name,
            state,
            description,
            type_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructPropertySerde {
    None,
    Rename(String),
    Flatten,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructPropertyState<Id> {
    Required,
    Optional,
    Default,
    DefaultValue(JsonValue),
    // Phantom to allow Id in the type even if unused today.
    #[doc(hidden)]
    _Phantom(std::marker::PhantomData<Id>),
}

#[derive(Debug, Clone)]
pub struct TypeUnitStruct {
    pub common: TypeCommon,
    pub repr: serde_json::Value,
}

impl TypeUnitStruct {
    pub fn new(name: impl Into<String>, description: Option<String>, repr: serde_json::Value) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            repr,
        }
    }

    pub(crate) fn render(&self) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });
        let name_ident = format_ident!("{}", self.common.name);
        let repr_string = serde_json::to_string(&self.repr).unwrap();

        quote! {
            #description
            #[derive(::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name_ident;

            impl ::serde::Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer {
                    let value: ::serde_json::Value =
                        ::serde_json::from_str(#repr_string).unwrap();
                    value.serialize(serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de> {
                    let expected: ::serde_json::Value =
                        ::serde_json::from_str(#repr_string).unwrap();
                    let value: ::serde_json::Value =
                        ::serde::Deserialize::deserialize(deserializer)?;
                    if value != expected {
                        return Err(::serde::de::Error::custom(format!(
                            "expected {}, found {}",
                            #repr_string,
                            ::serde_json::to_string(&value).unwrap()
                        )));
                    }
                    Ok(#name_ident)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeTupleStruct<Id> {
    pub common: TypeCommon,
    pub fields: Vec<Id>,
    pub rest: Option<Id>,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeTupleStruct<Id> {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        fields: Vec<Id>,
        rest: Option<Id>,
    ) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            fields,
            rest,
        }
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });
        let name_ident = format_ident!("{}", self.common.name);

        let field_types = self.fields.iter().map(|id| typespace.render_type(id));
        let rest_field = self
            .rest
            .as_ref()
            .map(|id| {
                let t = typespace.render_type(id);
                quote! { pub #t, }
            });

        quote! {
            #description
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            pub struct #name_ident(
                #( pub #field_types, )*
                #rest_field
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeNewtypeStruct<Id> {
    pub common: TypeCommon,
    pub type_id: Id,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeNewtypeStruct<Id> {
    pub fn new(name: impl Into<String>, description: Option<String>, type_id: Id) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            type_id,
        }
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });
        let name_ident = format_ident!("{}", self.common.name);
        let inner = typespace.render_type(&self.type_id);

        quote! {
            #description
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            pub struct #name_ident(pub #inner);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeTypeAlias<Id> {
    pub common: TypeCommon,
    pub type_id: Id,
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeTypeAlias<Id> {
    pub fn new(name: impl Into<String>, description: Option<String>, type_id: Id) -> Self {
        Self {
            common: TypeCommon::new(name, description),
            type_id,
        }
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> TokenStream {
        let description = self
            .common
            .description
            .as_ref()
            .map(|d| quote! { #[doc = #d] });
        let name_ident = format_ident!("{}", self.common.name);
        let target = typespace.render_type(&self.type_id);

        quote! {
            #description
            pub type #name_ident = #target;
        }
    }
}
