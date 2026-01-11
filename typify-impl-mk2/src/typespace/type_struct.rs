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
}

#[derive(Debug, Clone)]
pub struct TypeTupleStruct {
    pub name: NameBuilder,
    pub description: Option<String>,
    pub fields: Vec<SchemaRef>,

    /// Optional type, which must be represented as an array, the stores
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
