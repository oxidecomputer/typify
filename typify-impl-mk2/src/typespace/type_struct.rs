use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    namespace::Name,
    schemalet::SchemaRef,
    typespace::{JsonValue, NameBuilder, Typespace},
};

#[derive(Debug, Clone)]
pub struct TypeStruct {
    pub name: NameBuilder,
    pub description: Option<String>,
    pub default: Option<JsonValue>,
    pub properties: Vec<StructProperty>,
    pub deny_unknown_fields: bool,

    pub(crate) built: Option<TypeStructBuilt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeStructBuilt {
    pub name: Name<SchemaRef>,
}

impl TypeStruct {
    pub fn new(
        name: NameBuilder,
        description: Option<String>,
        default: Option<JsonValue>,
        properties: Vec<StructProperty>,
        deny_unknown_fields: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description,
            default,
            properties,
            deny_unknown_fields,
            built: None,
        }
    }
    pub(crate) fn children(&self) -> Vec<SchemaRef> {
        self.properties
            .iter()
            .map(|StructProperty { type_id, .. }| type_id.clone())
            .collect()
    }

    pub(crate) fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        self.properties
            .iter()
            .map(
                |StructProperty {
                     rust_name, type_id, ..
                 }| (type_id.clone(), format!("{rust_name}")),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructProperty {
    pub rust_name: Ident,
    pub json_name: StructPropertySerde,
    pub state: StructPropertyState,
    pub description: Option<String>,
    pub type_id: SchemaRef,
}

impl StructProperty {
    pub fn new(
        rust_name: Ident,
        json_name: StructPropertySerde,
        state: StructPropertyState,
        description: Option<String>,
        type_id: SchemaRef,
    ) -> Self {
        Self {
            rust_name,
            json_name,
            state,
            description,
            type_id: type_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructPropertySerde {
    None,
    Rename(String),
    Flatten,
}

/// The volitionality of a struct property. Only `Optional` will translate into
/// an `Option<T>` type; the others will be required in Rust. Conversely, only
/// `Required` must be present during deserialization; the others may be
/// omitted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructPropertyState {
    /// The field must be present.
    Required,
    /// The field may be omitted.
    Optional,
    /// The field may be omitted; if it is, its value comes from the type's
    /// intrinsic default. For built-in types, serialization of the default
    /// will be omitted.
    Default,
    /// The field may be omitted; if it is, its value comes from the provided
    /// JSON value. Note that this applies only to deserialization;
    /// serialization will always emit the field.
    DefaultValue(JsonValue),
}

#[derive(Debug, Clone)]
pub struct TypeUnitStruct {
    pub name: NameBuilder,
    pub description: Option<String>,

    pub repr: serde_json::Value,

    pub(crate) built: Option<TypeStructBuilt>,
}
impl TypeUnitStruct {
    pub(crate) fn new(
        name: NameBuilder,
        description: Option<String>,
        repr: serde_json::Value,
    ) -> Self {
        Self {
            name,
            description,
            repr,
            built: None,
        }
    }

    pub(crate) fn render(&self) -> proc_macro2::TokenStream {
        let Self {
            name: _,
            description,
            repr,
            built,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name = built.as_ref().unwrap().name.to_string();
        let name_ident = format_ident!("{name}");

        let repr_tokens = super::value_tokens::value_tokens(repr);
        let repr_string = serde_json::to_string(repr).unwrap();
        quote! {
            #description
            #[derive(::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name_ident;

            impl ::serde::Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    #repr_tokens.serialize(serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let expected = #repr_tokens;
                    let value: serde_json::Value =
                        ::serde::Deserialize::deserialize(deserializer)?;
                    if value != expected {
                        return Err(::serde::de::Error::custom(format!(
                            "expected unit struct value {}, found {}",
                            #repr_string,
                            ::serde_json::to_string(&value).unwrap())));
                    }
                    Ok(#name_ident)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeTupleStruct {
    pub name: NameBuilder,
    pub description: Option<String>,
    /// Fields of the tuple.
    pub fields: Vec<SchemaRef>,

    /// Optional type, which must be represented as an array, that stores
    /// items beyond those in `fields`.
    pub rest: Option<SchemaRef>,

    pub(crate) built: Option<TypeStructBuilt>,
}
impl TypeTupleStruct {
    pub(crate) fn new(
        name: NameBuilder,
        description: Option<String>,
        fields: Vec<SchemaRef>,
        rest: Option<SchemaRef>,
    ) -> Self {
        Self {
            name,
            description,
            fields,
            rest,
            built: None,
        }
    }

    pub(crate) fn children(&self) -> Vec<SchemaRef> {
        let mut children = self.fields.clone();
        if let Some(rest) = &self.rest {
            children.push(rest.clone());
        }

        children
    }

    pub(crate) fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        let mut children = self
            .fields
            .iter()
            .cloned()
            .enumerate()
            .map(|(ii, type_id)| (type_id, ii.to_string()))
            .collect::<Vec<_>>();

        if let Some(rest) = &self.rest {
            children.push((rest.clone(), self.fields.len().to_string()));
        }

        children
    }

    pub(crate) fn contained_children_mut(&mut self) -> Vec<&mut SchemaRef> {
        let mut children = self.fields.iter_mut().collect::<Vec<&mut SchemaRef>>();

        if let Some(rest) = &mut self.rest {
            children.push(rest);
        }

        children
    }
}

#[derive(Debug, Clone)]
pub struct TypeNewtypeStruct {
    pub name: NameBuilder,
    pub description: Option<String>,
    pub inner: SchemaRef,
    // TODO 2/11/2026
    // I think I want to add in some representation of constraints here
    pub(crate) built: Option<TypeStructBuilt>,
}

impl TypeNewtypeStruct {
    pub(crate) fn children(&self) -> Vec<SchemaRef> {
        vec![self.inner.clone()]
    }

    pub(crate) fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        vec![(self.inner.clone(), "0".to_string())]
    }

    pub(crate) fn contained_children_mut(&mut self) -> Vec<&mut SchemaRef> {
        vec![&mut self.inner]
    }

    pub(crate) fn render(&self, typespace: &Typespace) -> proc_macro2::TokenStream {
        let Self {
            name: _,
            description,
            inner,
            built,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name = built.as_ref().unwrap().name.to_string();
        let name_ident = format_ident!("{name}");

        let inner_ident = typespace.render_ident(inner);

        quote! {
            #description
            pub struct #name_ident(#inner_ident)

            impl Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    self.0.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Ok(Self(#inner_ident::deserialize(deserializer)?))
                }
            }
        }
    }
}
