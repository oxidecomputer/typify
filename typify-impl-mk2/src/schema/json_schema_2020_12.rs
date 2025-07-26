use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::schema::{
    generic::{
        self, GenericItems, GenericSchema, GenericSchemaDependencies, GenericType, ToGeneric,
    },
    util::ObjectOrBool,
};

type SchemaOrBool = ObjectOrBool<Schema>;
type Type = GenericType;

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
    type_: Option<Type>,
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
    additional_properties: Option<SchemaOrBool>,
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    max_properties: Option<u64>,
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    min_properties: Option<u64>,
    #[serde(rename = "patternProperties", skip_serializing_if = "Option::is_none")]
    pattern_properties: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<BTreeMap<String, SchemaOrBool>>,
    #[serde(rename = "propertyNames", skip_serializing_if = "Option::is_none")]
    property_names: Option<SchemaOrBool>,
    #[serde(
        rename = "unevaluatedProperties",
        skip_serializing_if = "Option::is_none"
    )]
    unevaluated_properties: Option<SchemaOrBool>,
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
    items: Option<SchemaOrBool>,
    #[serde(rename = "prefixItems", skip_serializing_if = "Option::is_none")]
    prefix_items: Option<Vec<SchemaOrBool>>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    max_items: Option<u64>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    min_items: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<SchemaOrBool>,
    #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
    max_contains: Option<u64>,
    #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
    min_contains: Option<u64>,

    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    unique_items: Option<bool>,
    #[serde(rename = "unevaluatedItems", skip_serializing_if = "Option::is_none")]
    unevaluated_items: Option<SchemaOrBool>,

    // Number and integer
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<serde_json::Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<serde_json::Number>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    multiple_of: Option<serde_json::Number>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    exclusive_maximum: Option<serde_json::Number>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    exclusive_minimum: Option<serde_json::Number>,

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
    not: Option<SchemaOrBool>,
    #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
    one_of: Option<Vec<SchemaOrBool>>,

    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<SchemaOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    then: Option<SchemaOrBool>,
    #[serde(rename = "else", skip_serializing_if = "Option::is_none")]
    else_: Option<SchemaOrBool>,

    // TODO: ???
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    content_encoding: Option<String>,
    #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    content_media_type: Option<String>,
    #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    content_schema: Option<SchemaOrBool>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaDependencies {
    Schema(SchemaOrBool),
    Strings(Vec<String>),
}

impl ToGeneric<GenericSchema> for Schema {
    fn to_generic(self) -> GenericSchema {
        let Self {
            title,
            description,
            examples,
            default,
            const_,
            enum_,
            deprecated,
            type_,
            format,
            read_only: _,
            write_only: _,
            schema: _,
            anchor,
            comment: _,
            defs: _,
            dynamic_anchor,
            dynamic_ref,
            id: _,
            recursive_anchor,
            recursive_ref,
            vocabulary: _,
            definitions: _,
            ref_,
            additional_properties,
            max_properties,
            min_properties,
            pattern_properties,
            properties,
            property_names,
            unevaluated_properties,
            dependent_required,
            dependent_schemas,
            required,
            dependencies,
            items,
            prefix_items,
            max_items,
            min_items,
            contains,
            max_contains,
            min_contains,
            unique_items,
            unevaluated_items,
            minimum,
            maximum,
            multiple_of,
            exclusive_maximum,
            exclusive_minimum,
            pattern,
            max_length,
            min_length,
            all_of,
            any_of,
            not,
            one_of,
            if_,
            then,
            else_,
            content_encoding: _,
            content_media_type: _,
            content_schema: _,
        } = self;

        GenericSchema {
            title,
            description,
            examples,
            default,
            const_,
            enum_,
            deprecated,
            type_,
            format,
            anchor,
            dynamic_anchor,
            dynamic_ref,
            recursive_anchor,
            recursive_ref,
            ref_,
            additional_properties: additional_properties.to_generic(),
            max_properties,
            min_properties,
            pattern_properties: pattern_properties.to_generic(),
            properties: properties.to_generic(),
            property_names: property_names.to_generic(),
            unevaluated_properties: unevaluated_properties.to_generic(),
            dependent_required,
            dependent_schemas: dependent_schemas.to_generic(),
            required,
            dependencies: dependencies.to_generic(),
            items: (items, prefix_items).to_generic(),
            max_items,
            min_items,
            contains: contains.to_generic(),
            max_contains,
            min_contains,
            unique_items,
            unevaluated_items: unevaluated_items.to_generic(),
            minimum,
            maximum,
            multiple_of,
            exclusive_maximum,
            exclusive_minimum,
            pattern,
            max_length,
            min_length,
            all_of: all_of.to_generic(),
            any_of: any_of.to_generic(),
            not: not.to_generic(),
            one_of: one_of.to_generic(),
            if_: if_.to_generic(),
            then: then.to_generic(),
            else_: else_.to_generic(),
        }
    }
}

impl ToGeneric<GenericSchemaDependencies> for SchemaDependencies {
    fn to_generic(self) -> GenericSchemaDependencies {
        match self {
            SchemaDependencies::Schema(o) => GenericSchemaDependencies::Schema(o.to_generic()),
            SchemaDependencies::Strings(items) => GenericSchemaDependencies::Strings(items),
        }
    }
}

impl ToGeneric<Option<GenericItems>> for (Option<SchemaOrBool>, Option<Vec<SchemaOrBool>>) {
    fn to_generic(self) -> Option<GenericItems> {
        match self {
            (None, None) => None,
            (Some(items), None) => Some(GenericItems::Single {
                items: items.to_generic(),
            }),
            (items, Some(prefix_items)) => Some(GenericItems::Prefix {
                prefix_items: prefix_items.to_generic(),
                items: items.to_generic(),
            }),
        }
    }
}

pub(crate) fn to_schemalets(
    resolved: &crate::bundler::Resolved<'_>,
) -> Result<Vec<(crate::schemalet::SchemaRef, crate::schemalet::Schemalet)>, anyhow::Error> {
    let schema = SchemaOrBool::deserialize(resolved.value)?;

    let generic_schema = schema.to_generic();

    generic::to_schemalets(resolved, generic_schema)
}
