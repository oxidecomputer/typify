//! Code generated from tests/schemas/input/json-2020-12
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
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum SchemaRootObjectType {
    String(SimpleTypes),
    Array(Vec<SimpleTypes>),
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
        anchor: Option<String>,
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
        dynamic_anchor: Option<String>,
        #[serde(
            rename = "$dynamicRef",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dynamic_ref: Option<::url::Url>,
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
        recursive_anchor: Option<String>,
        #[serde(
            rename = "$recursiveRef",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        recursive_ref: Option<::url::Url>,
        #[serde(
            rename = "$ref",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        ref_: Option<::url::Url>,
        #[serde(
            rename = "$schema",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        schema: Option<::url::Url>,
        #[serde(
            rename = "$vocabulary",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        vocabulary: Option<::std::collections::BTreeMap<::url::Url, bool>>,
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
        all_of: Option<Vec<SchemaRoot>>,
        #[serde(
            rename = "anyOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        any_of: Option<Vec<SchemaRoot>>,
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
        dependent_required: Option<::std::collections::BTreeMap<String, Vec<String>>>,
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
        exclusive_maximum: Option<f64>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_minimum: Option<f64>,
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
        max_contains: Option<i64>,
        #[serde(
            rename = "maxItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_items: Option<i64>,
        #[serde(
            rename = "maxLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_length: Option<i64>,
        #[serde(
            rename = "maxProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_properties: Option<i64>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        maximum: Option<f64>,
        #[serde(
            rename = "minContains",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_contains: Option<i64>,
        #[serde(
            rename = "minItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_items: Option<i64>,
        #[serde(
            rename = "minLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_length: Option<i64>,
        #[serde(
            rename = "minProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_properties: Option<i64>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        minimum: Option<f64>,
        #[serde(
            rename = "multipleOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        multiple_of: Option<f64>,
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
        one_of: Option<Vec<SchemaRoot>>,
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
        prefix_items: Option<Vec<SchemaRoot>>,
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
        required: Option<Vec<String>>,
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
    Variant1(Vec<String>),
}
fn main() {}
