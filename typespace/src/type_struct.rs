use log::debug;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{JsonValue, TypeCommon, TypeCommonBuilt, TypespaceRenderer};

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
            common: TypeCommon {
                name: name.into(),
                description,
                default: None,
                built: None,
            },
            properties,
            deny_unknown_fields,
        }
    }
    pub(crate) fn render(
        &self,
        typespace: &TypespaceRenderer<'_, Id>,
        cs: &mut codespace::Codespace,
    ) -> proc_macro2::TokenStream {
        let Self {
            common:
                TypeCommon {
                    name,
                    description,
                    default: _,
                    built: _,
                },
            properties,
            deny_unknown_fields: _,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc] });
        let name_ident = format_ident!("{name}");
        let snake_name = heck::AsSnakeCase(name.as_str()).to_string();

        let mut rendered_properties = Vec::new();
        for prop in properties {
            rendered_properties.push(typespace.render_struct_property(prop, true, &snake_name, cs));
        }

        quote! {
            #description
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            pub struct #name_ident {
                #( #rendered_properties, )*
            }
        }
    }

    pub(crate) fn children(&self) -> Vec<Id> {
        self.properties
            .iter()
            .map(|StructProperty { type_id, .. }| type_id.clone())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructProperty<Id> {
    pub rust_name: Ident,
    pub json_name: StructPropertySerde,
    pub state: StructPropertyState,
    pub description: Option<String>,
    pub type_id: Id,
}

impl<Id> StructProperty<Id> {
    pub fn new(
        rust_name: Ident,
        json_name: StructPropertySerde,
        state: StructPropertyState,
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
    pub common: TypeCommon,

    pub repr: serde_json::Value,
}
impl TypeUnitStruct {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        repr: serde_json::Value,
    ) -> Self {
        Self {
            common: TypeCommon {
                name: name.into(),
                description,
                default: None,
                built: None,
            },
            repr,
        }
    }

    pub(crate) fn render(&self) -> proc_macro2::TokenStream {
        let Self {
            common:
                TypeCommon {
                    name,
                    description,
                    built: _,
                    default: _,
                },
            repr,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name_ident = format_ident!("{name}");

        let repr_tokens = crate::value_tokens::value_tokens(repr);
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
pub struct TypeTupleStruct<Id> {
    pub common: TypeCommon,
    /// Fields of the tuple.
    pub fields: Vec<Id>,

    /// Optional type, which must be represented as an array, that stores
    /// items beyond those in `fields`.
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
            common: TypeCommon {
                name: name.into(),
                description,
                default: None,
                built: None,
            },
            fields,
            rest,
        }
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> proc_macro2::TokenStream {
        let Self {
            common:
                TypeCommon {
                    name,
                    description,
                    default: _,
                    built: _,
                },
            fields,
            rest,
        } = self;
        let description = description.as_ref().map(|desc| quote! { #[doc = #desc] });

        let name_ident = format_ident!("{name}");

        let field_ident = fields
            .iter()
            .map(|field_id| typespace.render_ident(field_id));
        let rest_ident = rest
            .as_ref()
            .map(|rest_id| typespace.render_ident(rest_id))
            .into_iter();

        let field_index = (0..fields.len()).map(syn::Index::from);
        let rest_index = rest
            .as_ref()
            .map(|_| syn::Index::from(fields.len()))
            .into_iter();

        let field_var = (0..fields.len())
            .map(|ii| format_ident!("field_{ii}"))
            .collect::<Vec<_>>();
        let field_int = (0..fields.len()).collect::<Vec<_>>();
        let rest_var = rest
            .as_ref()
            .map(|_| format_ident!("rest"))
            .into_iter()
            .collect::<Vec<_>>();
        let expected = format!("a tuple of size {} or more", fields.len());

        quote! {
            #description
            #[derive(::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name_ident(
                #( pub #field_ident, )*
                #( pub #rest_ident, )*
            );

            impl ::serde::Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeSeq;
                    let mut seq = serializer.serialize_seq(None)?;
                    #(
                        seq.serialize_element(&self.#field_index)?;
                    )*
                    #(
                        self.#rest_index.serialize(
                            ::json_serde::FlattenedSequenceSerializer::new(&mut seq)
                        )?;
                    )*
                    seq.end()
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> ::serde::de::Visitor<'de> for Visitor {
                        type Value = #name_ident;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            // TODO could we specify the type here?
                            formatter.write_str("a sequence")
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                        where
                            A: ::serde::de::SeqAccess<'de>,
                        {
                            // Strictly speaking, we don't need to
                            // store each tuple element in a
                            // variable, but as a practical matter,
                            // it makes the generated code much
                            // easier to follow and less indented.
                            #(
                                let #field_var = seq
                                    .next_element()?
                                    .ok_or_else(|| ::serde::de::Error::invalid_length(
                                        #field_int,
                                        &#expected
                                    ))?;
                            )*
                            #(
                                let #rest_var = ::serde::Deserialize::deserialize(
                                    ::json_serde::FlattenedSequenceDeserializer::new(&mut seq)
                                )?;
                            )*
                            Ok(#name_ident(
                                #( #field_var, )*
                                #( #rest_var, )*
                            ))
                        }
                    }

                    deserializer.deserialize_seq(Visitor)
                }
            }
        }
    }

    pub(crate) fn children(&self) -> Vec<Id> {
        let mut children = self.fields.clone();
        if let Some(rest) = &self.rest {
            children.push(rest.clone());
        }

        children
    }

    pub(crate) fn contained_children_mut(&mut self) -> Vec<&mut Id> {
        let mut children = self.fields.iter_mut().collect::<Vec<&mut Id>>();

        if let Some(rest) = &mut self.rest {
            children.push(rest);
        }

        children
    }
}

#[derive(Debug, Clone)]
pub struct TypeNewtypeStruct<Id> {
    pub common: TypeCommon,
    // pub name: NameBuilder,
    // pub description: Option<String>,
    // pub default: Option<JsonValue>,
    pub inner: Id,
    pub constraints: TypeNewtypeConstraints,
    // pub(crate) built: Option<TypeStructBuilt>,
}

// TODO 3/7/2026
// I'm ambivalent as to whether the constrained form of a newtype should be
// it's own, fundamentally distinct entity. However for now I'm going to just
// shove it into the existing newtype representation.
#[derive(Debug, Clone)]
pub enum TypeNewtypeConstraints {
    None,
    String {
        min: Option<usize>,
        max: Option<usize>,
        patterns: Vec<String>,
    },
    Array {
        min: Option<usize>,
        max: Option<usize>,
        // TODO 3/7/2026
        // I'm quite unsure of how to model the contains keyword. It also
        // occurs to me that the constraints below don't suffice--we need
        // an array of structures.
        // As a side-note, as I recall the interaction between `contains` and
        // `unevaluatedItems` is quite baroque i.e satisfying a `contains`
        // constraint counts as evaluation. I suppose this also means that
        // there's an important distinction between `items` being absent vs.
        // having the value of `true`.
        // min_contains: Option<usize>,
        // max_contains: Option<usize>,
        // contains: (),
    },
}

impl<Id: Clone + Ord + std::fmt::Debug + std::fmt::Display> TypeNewtypeStruct<Id> {
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        default: Option<JsonValue>,
        inner: Id,
        constraints: TypeNewtypeConstraints,
    ) -> Self {
        Self {
            common: TypeCommon {
                name: name.into(),
                description,
                default,
                built: None,
            },
            inner,
            constraints,
        }
    }
    pub(crate) fn children(&self) -> Vec<Id> {
        vec![self.inner.clone()]
    }

    pub(crate) fn contained_children_mut(&mut self) -> Vec<&mut Id> {
        vec![&mut self.inner]
    }

    pub(crate) fn render(&self, typespace: &TypespaceRenderer<'_, Id>) -> proc_macro2::TokenStream {
        let Self {
            common:
                TypeCommon {
                    name,
                    description,
                    default: _,
                    built: Some(TypeCommonBuilt { traits }),
                },
            inner,
            constraints,
        } = self
        else {
            unreachable!()
        };

        let description = description.as_ref().map(|desc| quote! { #[doc = #desc ]});
        let name_ident = format_ident!("{name}");

        let inner_ident = typespace.render_ident(inner);

        let derive_attr = (!traits.is_empty()).then(|| {
            let trait_idents = traits.iter().map(|tt| tt.render(&typespace.settings));
            quote! {
                #[derive(#(#trait_idents),*)]
            }
        });

        debug!("constraints: {constraints:#?}");

        quote! {
            #description
            #derive_attr
            pub struct #name_ident(pub #inner_ident);

            impl ::std::ops::Deref for #name_ident {
                type Target = #inner_ident;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::convert::From<#name_ident> for #inner_ident {
                fn from(value: #name_ident) -> Self {
                    value.0
                }
            }

            impl ::serde::Serialize for #name_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    self.0.serialize(serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
                }
            }
        }
    }
}
