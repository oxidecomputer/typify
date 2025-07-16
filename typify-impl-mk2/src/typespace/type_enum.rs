use crate::{
    namespace::Name,
    schemalet::SchemaRef,
    typespace::{JsonValue, NameBuilder, StructProperty},
};

#[derive(Debug, Clone)]
pub struct TypeEnum {
    pub name: NameBuilder,
    pub description: Option<String>,
    pub default: Option<JsonValue>,
    pub tag_type: EnumTagType,
    pub variants: Vec<EnumVariant>,
    pub deny_unknown_fields: bool,

    pub built: Option<TypeEnumBuilt>,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeEnumBuilt {
    pub name: Name<SchemaRef>,
}

impl TypeEnum {
    pub fn new(
        name: NameBuilder,
        description: Option<String>,
        default: Option<JsonValue>,
        tag_type: EnumTagType,
        variants: Vec<EnumVariant>,
        deny_unknown_fields: bool,
    ) -> Self {
        let name = name.into();
        Self {
            name,
            description,
            default,
            tag_type,
            variants,
            deny_unknown_fields,
            built: None,
        }
    }

    pub(crate) fn children(&self) -> Vec<SchemaRef> {
        self.variants
            .iter()
            .flat_map(|variant| variant.children())
            .collect()
    }

    pub(crate) fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        self.variants
            .iter()
            .flat_map(|variant| variant.children_with_context())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnumTagType {
    /// serde external tagging (serde's default)
    External,
    /// serde internal tagging
    Internal { tag: String },
    /// serde adjacent tagging
    Adjacent { tag: String, content: String },
    /// serde untagged
    Untagged,
}

// TODO 6/24/2025
// Do I want the variants to have tagging? I mean we could support the variant
// tagging for untagged if we wanted. Also how would we support more custom
// enums ala typify#811
// 6/28/2025
// Answer: No. Recall that the untagged variant markers need to be at the end
// of the type which makes it kind of a pain in the neck.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumVariant {
    pub rust_name: String,
    pub rename: Option<String>,
    // TODO need a name for serialization?
    // pub json_name: String,
    pub description: Option<String>,
    pub details: VariantDetails,
}
impl EnumVariant {
    fn children(&self) -> Vec<SchemaRef> {
        match &self.details {
            VariantDetails::Simple => Vec::new(),
            VariantDetails::Item(id) => vec![id.clone()],
            VariantDetails::Tuple(items) => items.clone(),
            VariantDetails::Struct(items) => {
                items.iter().map(|prop| prop.type_id.clone()).collect()
            }
        }
    }

    fn children_with_context(&self) -> Vec<(SchemaRef, String)> {
        match &self.details {
            VariantDetails::Simple => Vec::new(),
            VariantDetails::Item(id) => vec![(id.clone(), self.rust_name.clone())],
            VariantDetails::Tuple(items) => items
                .iter()
                .enumerate()
                .map(|(ii, id)| (id.clone(), format!("{}.{}", &self.rust_name, ii)))
                .collect(),
            VariantDetails::Struct(items) => items
                .iter()
                .map(|prop| {
                    (
                        prop.type_id.clone(),
                        format!("{}.{}", &self.rust_name, &prop.rust_name),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariantDetails {
    Simple,
    Item(SchemaRef),
    Tuple(Vec<SchemaRef>),
    Struct(Vec<StructProperty>),
}
