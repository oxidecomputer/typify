//! Code generated from tests/schemas/input/json-2020-12
pub struct SchemaArray(Vec<SchemaRoot>);
impl ::serde::Serialize for SchemaArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct AnchorString(String);
impl ::serde::Serialize for AnchorString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for AnchorString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct UriReferenceString(::url::Url);
impl ::serde::Serialize for UriReferenceString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for UriReferenceString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct UriString(::url::Url);
impl ::serde::Serialize for UriString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for UriString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct NonNegativeInteger(i64);
impl ::serde::Serialize for NonNegativeInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NonNegativeInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
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
pub struct StringArray(Vec<String>);
impl ::serde::Serialize for StringArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StringArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum SchemaRootObjectType {
    String(SimpleTypes),
    Array(SchemaRootObjectTypeArray),
}
pub struct SchemaRootObjectTypeArray(Vec<SimpleTypes>);
impl ::serde::Serialize for SchemaRootObjectTypeArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaRootObjectTypeArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum SchemaRoot {
    Object {
        #[serde(
            rename = "$anchor",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        anchor: Option<AnchorString>,
        #[serde(
            rename = "$comment",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        comment: Option<String>,
        #[serde(
            rename = "$defs",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        defs: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            rename = "$dynamicAnchor",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dynamic_anchor: Option<AnchorString>,
        #[serde(
            rename = "$dynamicRef",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dynamic_ref: Option<UriReferenceString>,
        #[serde(
            rename = "$id",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        id: Option<::url::Url>,
        #[serde(
            rename = "$recursiveAnchor",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        recursive_anchor: Option<AnchorString>,
        #[serde(
            rename = "$recursiveRef",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        recursive_ref: Option<UriReferenceString>,
        #[serde(
            rename = "$ref",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        ref_: Option<UriReferenceString>,
        #[serde(
            rename = "$schema",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        schema: Option<UriString>,
        #[serde(
            rename = "$vocabulary",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        vocabulary: Option<::std::collections::BTreeMap<UriString, bool>>,
        #[serde(
            rename = "additionalProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "allOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        all_of: Option<SchemaArray>,
        #[serde(
            rename = "anyOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        any_of: Option<SchemaArray>,
        #[serde(
            rename = "const",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        const_: Option<::serde_json::Value>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        contains: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "contentEncoding",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_encoding: Option<String>,
        #[serde(
            rename = "contentMediaType",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_media_type: Option<String>,
        #[serde(
            rename = "contentSchema",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_schema: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        default: Option<::serde_json::Value>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        definitions: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependencies: Option<
            ::std::collections::BTreeMap<String, SchemaRootObjectDependenciesValue>,
        >,
        #[serde(
            rename = "dependentRequired",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependent_required: Option<::std::collections::BTreeMap<String, StringArray>>,
        #[serde(
            rename = "dependentSchemas",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependent_schemas: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        deprecated: Option<bool>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        description: Option<String>,
        #[serde(
            rename = "else",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        else_: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "enum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        enum_: Option<Vec<::serde_json::Value>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        examples: Option<Vec<::serde_json::Value>>,
        #[serde(
            rename = "exclusiveMaximum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_maximum: Option<serde_json::Number>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_minimum: Option<serde_json::Number>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        format: Option<String>,
        #[serde(
            rename = "if",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        if_: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        items: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "maxContains",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_contains: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_items: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_length: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_properties: Option<NonNegativeInteger>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        maximum: Option<serde_json::Number>,
        #[serde(
            rename = "minContains",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_contains: Option<NonNegativeInteger>,
        #[serde(
            rename = "minItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_items: Option<NonNegativeInteger>,
        #[serde(
            rename = "minLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_length: Option<NonNegativeInteger>,
        #[serde(
            rename = "minProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_properties: Option<NonNegativeInteger>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        minimum: Option<serde_json::Number>,
        #[serde(
            rename = "multipleOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        multiple_of: Option<serde_json::Number>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        not: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "oneOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        one_of: Option<SchemaArray>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pattern: Option<String>,
        #[serde(
            rename = "patternProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pattern_properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            rename = "prefixItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        prefix_items: Option<SchemaArray>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            rename = "propertyNames",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        property_names: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "readOnly",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        read_only: Option<bool>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        required: Option<StringArray>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        then: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        title: Option<String>,
        #[serde(
            rename = "type",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        type_: Option<SchemaRootObjectType>,
        #[serde(
            rename = "unevaluatedItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_items: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "unevaluatedProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_properties: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "uniqueItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unique_items: Option<bool>,
        #[serde(
            rename = "writeOnly",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        write_only: Option<bool>,
    },
    Boolean(bool),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum SchemaRootObjectDependenciesValue {
    Variant0(SchemaRoot),
    Variant1(StringArray),
}
fn main() {}
