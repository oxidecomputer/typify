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
        #[serde(rename = "$anchor", skip_serializing_if = "Option::is_none")]
        anchor: Option<String>,
        #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
        defs: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(rename = "$dynamicAnchor", skip_serializing_if = "Option::is_none")]
        dynamic_anchor: Option<String>,
        #[serde(rename = "$dynamicRef", skip_serializing_if = "Option::is_none")]
        dynamic_ref: Option<::url::Url>,
        #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
        id: Option<::url::Url>,
        #[serde(rename = "$recursiveAnchor", skip_serializing_if = "Option::is_none")]
        recursive_anchor: Option<String>,
        #[serde(rename = "$recursiveRef", skip_serializing_if = "Option::is_none")]
        recursive_ref: Option<::url::Url>,
        #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
        ref_: Option<::url::Url>,
        #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
        schema: Option<::url::Url>,
        #[serde(rename = "$vocabulary", skip_serializing_if = "Option::is_none")]
        vocabulary: Option<::std::collections::BTreeMap<::url::Url, bool>>,
        #[serde(
            rename = "additionalProperties",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<Box<SchemaRoot>>,
        #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
        all_of: Option<Vec<SchemaRoot>>,
        #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
        any_of: Option<Vec<SchemaRoot>>,
        #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
        const_: Option<::serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contains: Option<Box<SchemaRoot>>,
        #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
        content_encoding: Option<String>,
        #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
        content_media_type: Option<String>,
        #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
        content_schema: Option<Box<SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<::serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        definitions: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dependencies: Option<
            ::std::collections::BTreeMap<String, SchemaRootObjectDependenciesValue>,
        >,
        #[serde(rename = "dependentRequired", skip_serializing_if = "Option::is_none")]
        dependent_required: Option<::std::collections::BTreeMap<String, Vec<String>>>,
        #[serde(rename = "dependentSchemas", skip_serializing_if = "Option::is_none")]
        dependent_schemas: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        deprecated: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
        else_: Option<Box<SchemaRoot>>,
        #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
        enum_: Option<Vec<::serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        examples: Option<Vec<::serde_json::Value>>,
        #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
        exclusive_maximum: Option<f64>,
        #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
        exclusive_minimum: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
        if_: Option<Box<SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<Box<SchemaRoot>>,
        #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
        max_contains: Option<i64>,
        #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
        max_items: Option<i64>,
        #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
        max_length: Option<i64>,
        #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
        max_properties: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<f64>,
        #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
        min_contains: Option<i64>,
        #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
        min_items: Option<i64>,
        #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
        min_length: Option<i64>,
        #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
        min_properties: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<f64>,
        #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
        multiple_of: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        not: Option<Box<SchemaRoot>>,
        #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
        one_of: Option<Vec<SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pattern: Option<String>,
        #[serde(rename = "patternProperties", skip_serializing_if = "Option::is_none")]
        pattern_properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(rename = "prefixItems", skip_serializing_if = "Option::is_none")]
        prefix_items: Option<Vec<SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(rename = "propertyNames", skip_serializing_if = "Option::is_none")]
        property_names: Option<Box<SchemaRoot>>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        then: Option<Box<SchemaRoot>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<SchemaRootObjectType>,
        #[serde(rename = "unevaluatedItems", skip_serializing_if = "Option::is_none")]
        unevaluated_items: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "unevaluatedProperties",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_properties: Option<Box<SchemaRoot>>,
        #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
        unique_items: Option<bool>,
        #[serde(rename = "writeOnly", skip_serializing_if = "Option::is_none")]
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
