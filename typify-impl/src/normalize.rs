// Copyright 2026 Barbacane Dev
//
// JSON Schema 2020-12 → draft-07 normalization layer.
//
// This module transforms JSON Schema 2020-12 keywords into their draft-07
// equivalents so that the existing schemars-based pipeline can process them
// without modification. The transformation is best-effort: some 2020-12
// features (like $dynamicRef) have no perfect draft-07 equivalent.

use serde_json::{Map, Value};

/// Detect the schema draft and normalize to draft-07 compatible JSON.
/// Operates in-place on the JSON value. If the schema is already draft-07
/// or has no `$schema` field, it is left unchanged.
pub(crate) fn normalize_schema(value: &mut Value) {
    if !is_2020_12(value) {
        return;
    }

    normalize_value(value);
}

/// Check if a schema declares itself as 2020-12 (or 2019-09).
fn is_2020_12(value: &Value) -> bool {
    value
        .get("$schema")
        .and_then(|s| s.as_str())
        .map(|s| s.contains("2020-12") || s.contains("2019-09") || s.contains("draft/next"))
        .unwrap_or(false)
}

/// Recursively normalize a JSON value, transforming 2020-12 keywords
/// to draft-07 equivalents.
fn normalize_value(value: &mut Value) {
    match value {
        Value::Object(map) => normalize_object(map),
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                normalize_value(item);
            }
        }
        _ => {}
    }
}

/// Normalize a JSON object's keys from 2020-12 to draft-07.
fn normalize_object(map: &mut Map<String, Value>) {
    // 1. $defs → definitions
    if let Some(defs) = map.remove("$defs") {
        // Merge with existing definitions if present
        if let Some(Value::Object(existing)) = map.get_mut("definitions") {
            if let Value::Object(new_defs) = defs {
                for (k, v) in new_defs {
                    existing.entry(k).or_insert(v);
                }
            }
        } else {
            map.insert("definitions".to_string(), defs);
        }
    }

    // 2. prefixItems → items (as array), items → additionalItems
    if let Some(prefix_items) = map.remove("prefixItems") {
        // In 2020-12, `items` with `prefixItems` means "additionalItems"
        if let Some(items) = map.remove("items") {
            map.insert("additionalItems".to_string(), items);
        }
        map.insert("items".to_string(), prefix_items);
    }

    // 3. $ref alongside other keywords → wrap in allOf
    //    In draft-07, $ref replaces the entire schema object.
    //    In 2020-12, $ref can coexist with other keywords.
    if map.contains_key("$ref") && has_non_ref_keywords(map) {
        let ref_val = map.remove("$ref").unwrap();
        let ref_schema = Value::Object({
            let mut m = Map::new();
            m.insert("$ref".to_string(), ref_val);
            m
        });

        // Extract non-meta keywords into a separate schema
        let other_schema = Value::Object({
            let mut m = Map::new();
            let non_meta_keys: Vec<String> = map
                .keys()
                .filter(|k| !is_meta_keyword(k))
                .cloned()
                .collect();
            for key in non_meta_keys {
                if let Some(v) = map.remove(&key) {
                    m.insert(key, v);
                }
            }
            m
        });

        // Wrap in allOf
        let all_of = if let Some(Value::Array(existing)) = map.remove("allOf") {
            let mut v = existing;
            v.insert(0, ref_schema);
            v.push(other_schema);
            v
        } else {
            vec![ref_schema, other_schema]
        };
        map.insert("allOf".to_string(), Value::Array(all_of));
    }

    // 4. dependentRequired → dependencies (array form)
    if let Some(Value::Object(dep_req)) = map.remove("dependentRequired") {
        let deps = map
            .entry("dependencies".to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(deps_map) = deps {
            for (k, v) in dep_req {
                deps_map.entry(k).or_insert(v);
            }
        }
    }

    // 5. dependentSchemas → dependencies (schema form)
    if let Some(Value::Object(dep_schemas)) = map.remove("dependentSchemas") {
        let deps = map
            .entry("dependencies".to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Value::Object(deps_map) = deps {
            for (k, v) in dep_schemas {
                deps_map.entry(k).or_insert(v);
            }
        }
    }

    // 6. unevaluatedProperties → additionalProperties (best-effort)
    if let Some(uneval) = map.remove("unevaluatedProperties") {
        map.entry("additionalProperties".to_string())
            .or_insert(uneval);
    }

    // 7. unevaluatedItems → additionalItems (best-effort)
    if let Some(uneval) = map.remove("unevaluatedItems") {
        map.entry("additionalItems".to_string()).or_insert(uneval);
    }

    // 8. $dynamicRef → $ref (best-effort approximation)
    //    $dynamicRef is used for recursive schemas; treating as regular $ref
    //    is often correct for non-meta-schema use cases.
    if let Some(dyn_ref) = map.remove("$dynamicRef") {
        map.entry("$ref".to_string()).or_insert(dyn_ref);
    }

    // 9. Remove $dynamicAnchor (no draft-07 equivalent, informational only)
    map.remove("$dynamicAnchor");

    // 10. $anchor → use as additional identification (informational)
    //     In draft-07, $id serves this purpose. We don't need to transform
    //     $anchor since it's used for resolution, which the bundler handles.

    // Recurse into all sub-values
    for (_, v) in map.iter_mut() {
        normalize_value(v);
    }
}

/// Check if a schema object has keywords beyond $ref and metadata.
fn has_non_ref_keywords(map: &Map<String, Value>) -> bool {
    map.keys().any(|k| k != "$ref" && !is_meta_keyword(k))
}

/// Keywords that are metadata/annotations and don't affect validation.
fn is_meta_keyword(key: &str) -> bool {
    matches!(
        key,
        "$schema"
            | "$id"
            | "$anchor"
            | "$comment"
            | "$defs"
            | "$vocabulary"
            | "title"
            | "description"
            | "default"
            | "deprecated"
            | "readOnly"
            | "writeOnly"
            | "examples"
            | "definitions"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_no_op_for_draft07() {
        let mut schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "definitions": { "Foo": { "type": "string" } }
        });
        let original = schema.clone();
        normalize_schema(&mut schema);
        assert_eq!(schema, original);
    }

    #[test]
    fn test_no_op_for_no_schema() {
        let mut schema = json!({
            "type": "object",
            "definitions": { "Foo": { "type": "string" } }
        });
        let original = schema.clone();
        normalize_schema(&mut schema);
        assert_eq!(schema, original);
    }

    #[test]
    fn test_defs_to_definitions() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "Foo": { "type": "string" }
            }
        });
        normalize_schema(&mut schema);
        assert!(schema.get("$defs").is_none());
        assert_eq!(schema["definitions"]["Foo"], json!({ "type": "string" }));
    }

    #[test]
    fn test_defs_merges_with_definitions() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "definitions": { "Bar": { "type": "integer" } },
            "$defs": { "Foo": { "type": "string" } }
        });
        normalize_schema(&mut schema);
        assert_eq!(schema["definitions"]["Bar"], json!({ "type": "integer" }));
        assert_eq!(schema["definitions"]["Foo"], json!({ "type": "string" }));
    }

    #[test]
    fn test_prefix_items_to_items() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                { "type": "string" },
                { "type": "integer" }
            ],
            "items": false
        });
        normalize_schema(&mut schema);
        assert!(schema.get("prefixItems").is_none());
        assert_eq!(
            schema["items"],
            json!([{ "type": "string" }, { "type": "integer" }])
        );
        assert_eq!(schema["additionalItems"], json!(false));
    }

    #[test]
    fn test_ref_alongside_keywords() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "field": {
                    "$ref": "#/$defs/Base",
                    "description": "A field",
                    "minimum": 5
                }
            },
            "$defs": {
                "Base": { "type": "integer" }
            }
        });
        normalize_schema(&mut schema);
        let field = &schema["properties"]["field"];
        // Should have been wrapped in allOf
        assert!(field.get("allOf").is_some());
        let all_of = field["allOf"].as_array().unwrap();
        assert!(all_of.iter().any(|s| s.get("$ref").is_some()));
        assert!(all_of.iter().any(|s| s.get("minimum").is_some()));
    }

    #[test]
    fn test_dependent_required() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "dependentRequired": {
                "creditCard": ["billingAddress"]
            }
        });
        normalize_schema(&mut schema);
        assert!(schema.get("dependentRequired").is_none());
        assert_eq!(
            schema["dependencies"]["creditCard"],
            json!(["billingAddress"])
        );
    }

    #[test]
    fn test_unevaluated_properties() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "unevaluatedProperties": false
        });
        normalize_schema(&mut schema);
        assert!(schema.get("unevaluatedProperties").is_none());
        assert_eq!(schema["additionalProperties"], json!(false));
    }

    #[test]
    fn test_dynamic_ref_to_ref() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "node": {
                    "type": "object",
                    "properties": {
                        "children": {
                            "type": "array",
                            "items": { "$dynamicRef": "#node" }
                        }
                    }
                }
            }
        });
        normalize_schema(&mut schema);
        let items = &schema["definitions"]["node"]["properties"]["children"]["items"];
        assert_eq!(items["$ref"], json!("#node"));
        assert!(items.get("$dynamicRef").is_none());
    }

    #[test]
    fn test_recursive_normalization() {
        let mut schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "Inner": {
                    "type": "array",
                    "prefixItems": [{ "type": "string" }],
                    "items": false
                }
            }
        });
        normalize_schema(&mut schema);
        let inner = &schema["definitions"]["Inner"];
        assert_eq!(inner["items"], json!([{ "type": "string" }]));
        assert_eq!(inner["additionalItems"], json!(false));
    }
}
