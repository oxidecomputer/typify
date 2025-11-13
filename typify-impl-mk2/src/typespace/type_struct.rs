use syn::Ident;

use crate::{
    namespace::Name,
    schemalet::SchemaRef,
    typespace::{JsonValue, NameBuilder},
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
