use crate::{
    schemalet::SchemaRef,
    typespace::{TypespaceTrait, TypespaceTraitSet},
};

#[derive(Debug, Clone)]
pub struct TypeNative {
    pub name: String,

    pub impls: TypespaceTraitSet,

    // TODO from typify 1: in order to support const generics, this could be a
    // TypeOrConst enum, but note that we may some day need to disambiguate
    // char and &'static str since schemars represents a char as a string of
    // length 1.
    pub parameters: Vec<SchemaRef>,
}

impl TypeNative {
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
}
