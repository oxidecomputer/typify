use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::schema::util::ObjectOrBool;

type SchemaOrBool = ObjectOrBool<Schema>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SimpleTypes {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaRootObjectType {
    Single(SimpleTypes),
    Array(Vec<SimpleTypes>),
}

#[derive(Deserialize, Serialize)]
pub struct Schema {
    // Schema metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    examples: Option<Vec<::serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<::serde_json::Value>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    const_: Option<::serde_json::Value>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<::serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deprecated: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<SchemaRootObjectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
    #[serde(rename = "writeOnly", skip_serializing_if = "Option::is_none")]
    write_only: Option<bool>,

    // Schema document metadata
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    schema: Option<String>,
    #[serde(rename = "$anchor", skip_serializing_if = "Option::is_none")]
    anchor: Option<String>,
    #[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
    defs: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(rename = "$dynamicAnchor", skip_serializing_if = "Option::is_none")]
    dynamic_anchor: Option<String>,
    #[serde(rename = "$dynamicRef", skip_serializing_if = "Option::is_none")]
    dynamic_ref: Option<String>,
    #[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "$recursiveAnchor", skip_serializing_if = "Option::is_none")]
    recursive_anchor: Option<String>,
    #[serde(rename = "$recursiveRef", skip_serializing_if = "Option::is_none")]
    recursive_ref: Option<String>,
    #[serde(rename = "$vocabulary", skip_serializing_if = "Option::is_none")]
    vocabulary: Option<BTreeMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definitions: Option<BTreeMap<String, SchemaOrBool>>,

    // Reference
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<String>,

    // Object
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    additional_properties: Option<Box<SchemaOrBool>>,
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    max_properties: Option<i64>,
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    min_properties: Option<i64>,
    #[serde(rename = "patternProperties", skip_serializing_if = "Option::is_none")]
    pattern_properties: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(rename = "propertyNames", skip_serializing_if = "Option::is_none")]
    property_names: Option<Box<SchemaOrBool>>,
    #[serde(
        rename = "unevaluatedProperties",
        skip_serializing_if = "Option::is_none"
    )]
    unevaluated_properties: Option<Box<SchemaOrBool>>,
    #[serde(rename = "dependentRequired", skip_serializing_if = "Option::is_none")]
    dependent_required: Option<BTreeMap<String, Vec<String>>>,
    #[serde(rename = "dependentSchemas", skip_serializing_if = "Option::is_none")]
    dependent_schemas: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<BTreeMap<String, SchemaDependencies>>,

    // Array
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<SchemaOrBool>>,
    #[serde(rename = "prefixItems", skip_serializing_if = "Option::is_none")]
    prefix_items: Option<Vec<SchemaOrBool>>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    max_items: Option<i64>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    min_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<Box<SchemaOrBool>>,
    #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
    max_contains: Option<i64>,
    #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
    min_contains: Option<i64>,

    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    unique_items: Option<bool>,
    #[serde(rename = "unevaluatedItems", skip_serializing_if = "Option::is_none")]
    unevaluated_items: Option<Box<SchemaOrBool>>,

    // Number and integer
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    multiple_of: Option<f64>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    exclusive_maximum: Option<f64>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    exclusive_minimum: Option<f64>,

    // String
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    max_length: Option<i64>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    min_length: Option<i64>,

    // Subschemas
    #[serde(rename = "allOf", skip_serializing_if = "Option::is_none")]
    all_of: Option<Vec<SchemaOrBool>>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    any_of: Option<Vec<SchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<Box<SchemaOrBool>>,
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    one_of: Option<Vec<SchemaOrBool>>,

    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<Box<SchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    then: Option<Box<SchemaOrBool>>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    else_: Option<Box<SchemaOrBool>>,

    // TODO: ???
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    content_encoding: Option<String>,
    #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    content_media_type: Option<String>,
    #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    content_schema: Option<Box<SchemaOrBool>>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaDependencies {
    Schema(SchemaOrBool),
    Strings(Vec<String>),
}
