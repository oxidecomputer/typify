use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    bundler::{Document, DocumentId, Error, SchemaKind},
    schema::{
        generic::{
            self, GenericItems, GenericSchema, GenericSchemaDependencies, GenericType, ToGeneric,
        },
        util::ObjectOrBool,
    },
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
    max_length: Option<u64>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    min_length: Option<u64>,

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

// mod generated {
//     include!("../../tests/schemas/rust/json-2020-12.rs");
// }

// pub use generated::SchemaRoot as GeneratedSchemaOrBool;

impl SchemaKind for Schema {
    fn make_document(
        value: serde_json::Value,
    ) -> Result<crate::bundler::Document, crate::bundler::Error> {
        let doc = Schema::deserialize(&value)
            .map_err(|e| Error::deserialization_error("unknown", &e.to_string()))?;

        // TODO what to do if there's no $id?
        let id = doc
            .id
            .clone()
            .unwrap_or_else(|| "http://localhost/".to_string());
        // TODO ditto the schema value
        let schema = doc.schema.clone().unwrap_or_default();

        let dyn_anchors = doc
            .iter_schema()
            .filter_map(|(path, subschema)| {
                subschema
                    .dynamic_anchor
                    .as_ref()
                    .map(|dd| (dd.clone(), path.clone()))
            })
            .collect();

        let document = Document {
            id: DocumentId::from_str(&id),
            content: value,
            schema,
            anchors: Default::default(),
            dyn_anchors,
        };
        Ok(document)
    }
}

impl Schema {
    fn iter_schema(&self) -> impl Iterator<Item = (String, &Schema)> {
        let mut result = Vec::new();
        self.collect_children(&mut result, "");
        result.into_iter()
    }
}

trait Childish {
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str);
}

impl Childish for Schema {
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        result.push((path.to_string(), self));

        // We only need the subset of properties that can contain subschemas.
        let Self {
            defs,
            definitions,
            additional_properties,
            pattern_properties,
            properties,
            property_names,
            unevaluated_properties,
            dependent_schemas,
            dependencies,
            items,
            prefix_items,
            contains,
            unevaluated_items,
            all_of,
            any_of,
            not,
            one_of,
            if_,
            then,
            else_,
            content_schema,
            ..
        } = self;
        let properties = [
            ("$defs", defs as &dyn Childish),
            ("definitions", definitions),
            ("additionalProperties", additional_properties),
            ("patternProperties", pattern_properties),
            ("properties", properties),
            ("propertyNames", property_names),
            ("unevaluatedProperties", unevaluated_properties),
            ("dependentSchemas", dependent_schemas),
            ("dependencies", dependencies),
            ("items", items),
            ("prefixItems", prefix_items),
            ("contains", contains),
            ("unevaluatedItems", unevaluated_items),
            ("allOf", all_of),
            ("anyOf", any_of),
            ("not", not),
            ("oneOf", one_of),
            ("if", if_),
            ("then", then),
            ("else", else_),
            ("contentSchema", content_schema),
        ];

        for (prefix, property) in properties.iter() {
            property.collect_children(result, &format!("{}/{}", path, prefix));
        }
    }
}

impl<T> Childish for Option<T>
where
    T: Childish,
{
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        if let Some(inner) = self {
            inner.collect_children(result, path);
        }
    }
}

impl<T> Childish for BTreeMap<String, T>
where
    T: Childish,
{
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        for (key, value) in self {
            value.collect_children(result, &format!("{}/{}", path, key));
        }
    }
}

impl Childish for ObjectOrBool<Schema> {
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        if let ObjectOrBool::Object(schema) = self {
            schema.collect_children(result, path);
        }
    }
}

impl<T> Childish for Vec<T>
where
    T: Childish,
{
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        for (index, value) in self.iter().enumerate() {
            value.collect_children(result, &format!("{}/{}", path, index));
        }
    }
}
impl Childish for SchemaDependencies {
    fn collect_children<'a>(&'a self, result: &mut Vec<(String, &'a Schema)>, path: &str) {
        if let SchemaDependencies::Schema(schema) = self {
            schema.collect_children(result, path)
        }
    }
}
