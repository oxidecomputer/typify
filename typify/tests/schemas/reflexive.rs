#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchema {
    Boolean(bool),
    Object {
        #[serde(
            rename = "additionalItems",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        additional_items: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "additionalProperties",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(rename = "allOf", default, skip_serializing_if = "Option::is_none")]
        all_of: Option<SchemaArray>,
        #[serde(rename = "anyOf", default, skip_serializing_if = "Option::is_none")]
        any_of: Option<SchemaArray>,
        #[serde(rename = "$comment", default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(rename = "const", default, skip_serializing_if = "Option::is_none")]
        const_: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        contains: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "contentEncoding",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        content_encoding: Option<String>,
        #[serde(
            rename = "contentMediaType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        content_media_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        definitions: std::collections::HashMap<String, CoreSchemaMetaSchema>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        dependencies:
            std::collections::HashMap<String, CoreSchemaMetaSchemaObjectDependenciesValue>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(rename = "else", default, skip_serializing_if = "Option::is_none")]
        else_: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
        enum_: Option<Vec<serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        examples: Vec<serde_json::Value>,
        #[serde(
            rename = "exclusiveMaximum",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_maximum: Option<f64>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_minimum: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(rename = "$id", default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
        if_: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(default = "defaults::core_schema_meta_schema_object_items")]
        items: CoreSchemaMetaSchemaObjectItems,
        #[serde(rename = "maxItems", default, skip_serializing_if = "Option::is_none")]
        max_items: Option<NonNegativeInteger>,
        #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
        max_length: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxProperties",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        max_properties: Option<NonNegativeInteger>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(rename = "minItems", default, skip_serializing_if = "Option::is_none")]
        min_items: Option<NonNegativeIntegerDefault0>,
        #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
        min_length: Option<NonNegativeIntegerDefault0>,
        #[serde(
            rename = "minProperties",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        min_properties: Option<NonNegativeIntegerDefault0>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(
            rename = "multipleOf",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        multiple_of: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        not: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(rename = "oneOf", default, skip_serializing_if = "Option::is_none")]
        one_of: Option<SchemaArray>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(
            rename = "patternProperties",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pattern_properties: std::collections::HashMap<String, CoreSchemaMetaSchema>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        properties: std::collections::HashMap<String, CoreSchemaMetaSchema>,
        #[serde(
            rename = "propertyNames",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        property_names: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(rename = "readOnly", default)]
        read_only: bool,
        #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
        ref_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        required: Option<StringArray>,
        #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
        schema: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        then: Option<Box<CoreSchemaMetaSchema>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<CoreSchemaMetaSchemaObjectType>,
        #[serde(rename = "uniqueItems", default)]
        unique_items: bool,
    },
}
impl From<&CoreSchemaMetaSchema> for CoreSchemaMetaSchema {
    fn from(value: &CoreSchemaMetaSchema) -> Self {
        value.clone()
    }
}
impl Default for CoreSchemaMetaSchema {
    fn default() -> Self {
        CoreSchemaMetaSchema::Boolean(true)
    }
}
impl From<bool> for CoreSchemaMetaSchema {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectDependenciesValue {
    Variant0(CoreSchemaMetaSchema),
    Variant1(StringArray),
}
impl From<&CoreSchemaMetaSchemaObjectDependenciesValue>
    for CoreSchemaMetaSchemaObjectDependenciesValue
{
    fn from(value: &CoreSchemaMetaSchemaObjectDependenciesValue) -> Self {
        value.clone()
    }
}
impl From<CoreSchemaMetaSchema> for CoreSchemaMetaSchemaObjectDependenciesValue {
    fn from(value: CoreSchemaMetaSchema) -> Self {
        Self::Variant0(value)
    }
}
impl From<StringArray> for CoreSchemaMetaSchemaObjectDependenciesValue {
    fn from(value: StringArray) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectItems {
    Variant0(Box<CoreSchemaMetaSchema>),
    Variant1(SchemaArray),
}
impl From<&CoreSchemaMetaSchemaObjectItems> for CoreSchemaMetaSchemaObjectItems {
    fn from(value: &CoreSchemaMetaSchemaObjectItems) -> Self {
        value.clone()
    }
}
impl Default for CoreSchemaMetaSchemaObjectItems {
    fn default() -> Self {
        CoreSchemaMetaSchemaObjectItems::Variant0(Box::new(CoreSchemaMetaSchema::Boolean(true)))
    }
}
impl From<Box<CoreSchemaMetaSchema>> for CoreSchemaMetaSchemaObjectItems {
    fn from(value: Box<CoreSchemaMetaSchema>) -> Self {
        Self::Variant0(value)
    }
}
impl From<SchemaArray> for CoreSchemaMetaSchemaObjectItems {
    fn from(value: SchemaArray) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectType {
    Variant0(SimpleTypes),
    Variant1(Vec<SimpleTypes>),
}
impl From<&CoreSchemaMetaSchemaObjectType> for CoreSchemaMetaSchemaObjectType {
    fn from(value: &CoreSchemaMetaSchemaObjectType) -> Self {
        value.clone()
    }
}
impl From<SimpleTypes> for CoreSchemaMetaSchemaObjectType {
    fn from(value: SimpleTypes) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<SimpleTypes>> for CoreSchemaMetaSchemaObjectType {
    fn from(value: Vec<SimpleTypes>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonNegativeInteger(pub u64);
impl std::ops::Deref for NonNegativeInteger {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl From<NonNegativeInteger> for u64 {
    fn from(value: NonNegativeInteger) -> Self {
        value.0
    }
}
impl From<&NonNegativeInteger> for NonNegativeInteger {
    fn from(value: &NonNegativeInteger) -> Self {
        value.clone()
    }
}
impl From<u64> for NonNegativeInteger {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for NonNegativeInteger {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for NonNegativeInteger {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NonNegativeInteger {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NonNegativeInteger {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for NonNegativeInteger {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonNegativeIntegerDefault0 {
    #[serde(flatten)]
    pub subtype_0: NonNegativeInteger,
    #[serde(flatten)]
    pub subtype_1: serde_json::Value,
}
impl From<&NonNegativeIntegerDefault0> for NonNegativeIntegerDefault0 {
    fn from(value: &NonNegativeIntegerDefault0) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchemaArray(pub Vec<CoreSchemaMetaSchema>);
impl std::ops::Deref for SchemaArray {
    type Target = Vec<CoreSchemaMetaSchema>;
    fn deref(&self) -> &Vec<CoreSchemaMetaSchema> {
        &self.0
    }
}
impl From<SchemaArray> for Vec<CoreSchemaMetaSchema> {
    fn from(value: SchemaArray) -> Self {
        value.0
    }
}
impl From<&SchemaArray> for SchemaArray {
    fn from(value: &SchemaArray) -> Self {
        value.clone()
    }
}
impl From<Vec<CoreSchemaMetaSchema>> for SchemaArray {
    fn from(value: Vec<CoreSchemaMetaSchema>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SimpleTypes {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}
impl From<&SimpleTypes> for SimpleTypes {
    fn from(value: &SimpleTypes) -> Self {
        value.clone()
    }
}
impl ToString for SimpleTypes {
    fn to_string(&self) -> String {
        match *self {
            Self::Array => "array".to_string(),
            Self::Boolean => "boolean".to_string(),
            Self::Integer => "integer".to_string(),
            Self::Null => "null".to_string(),
            Self::Number => "number".to_string(),
            Self::Object => "object".to_string(),
            Self::String => "string".to_string(),
        }
    }
}
impl std::str::FromStr for SimpleTypes {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "array" => Ok(Self::Array),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "null" => Ok(Self::Null),
            "number" => Ok(Self::Number),
            "object" => Ok(Self::Object),
            "string" => Ok(Self::String),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SimpleTypes {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SimpleTypes {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SimpleTypes {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringArray(pub Vec<String>);
impl std::ops::Deref for StringArray {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl From<StringArray> for Vec<String> {
    fn from(value: StringArray) -> Self {
        value.0
    }
}
impl From<&StringArray> for StringArray {
    fn from(value: &StringArray) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for StringArray {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn core_schema_meta_schema_object_items() -> super::CoreSchemaMetaSchemaObjectItems {
        super::CoreSchemaMetaSchemaObjectItems::Variant0(Box::new(
            super::CoreSchemaMetaSchema::Boolean(true),
        ))
    }
}
fn main() {}
