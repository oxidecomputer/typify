// Copyright 2026 Barbacane Dev
//
// Schema reference bundler. Resolves non-standard internal references
// (e.g., #/properties/foo) and external file references (e.g.,
// other.json#/definitions/Foo) by inlining them into the definitions
// map and rewriting $ref pointers.

use std::collections::{BTreeMap, BTreeSet};

use schemars::schema::RootSchema;
use serde_json::Value;

use crate::util::sanitize;
use crate::Case;

/// Resolve internal references that point to non-$defs locations.
///
/// Walks the schema tree, finds any `$ref` that doesn't match
/// `#/definitions/X` or `#/$defs/X`, resolves it by walking the JSON
/// Pointer into the schema, and registers the sub-schema as a definition.
pub(crate) fn resolve_internal_refs(schema: &mut RootSchema) {
    // Convert to Value for JSON Pointer resolution
    let schema_value = serde_json::to_value(&*schema).unwrap();

    // Collect all $ref values from the schema
    let refs = collect_refs(&serde_json::to_value(&*schema).unwrap());

    // Find refs that need resolution (not #/definitions/X, not #/$defs/X, not #)
    let exotic_refs: Vec<String> = refs
        .into_iter()
        .filter(|r| {
            r.starts_with('#')
                && r != "#"
                && !r.starts_with("#/definitions/")
                && !r.starts_with("#/$defs/")
        })
        .collect();

    if exotic_refs.is_empty() {
        return;
    }

    // Build rewrite map
    let mut rewrites: BTreeMap<String, String> = BTreeMap::new();
    let mut used_names: BTreeSet<String> = schema.definitions.keys().cloned().collect();

    for ref_str in &exotic_refs {
        if rewrites.contains_key(ref_str) {
            continue;
        }

        // Convert the JSON Pointer fragment to a serde_json pointer path
        let pointer = &ref_str[1..]; // strip leading #

        if let Some(sub_schema) = schema_value.pointer(pointer) {
            let name = generate_name_from_pointer(pointer, &used_names);
            used_names.insert(name.clone());

            // Parse sub-schema back into schemars Schema
            if let Ok(parsed) =
                serde_json::from_value::<schemars::schema::Schema>(sub_schema.clone())
            {
                schema.definitions.insert(name.clone(), parsed);
            }

            rewrites.insert(ref_str.clone(), format!("#/definitions/{}", name));
        }
    }

    // Apply rewrites to the schema
    if !rewrites.is_empty() {
        let mut schema_value = serde_json::to_value(&*schema).unwrap();
        rewrite_refs(&mut schema_value, &rewrites);
        if let Ok(rewritten) = serde_json::from_value::<RootSchema>(schema_value) {
            *schema = rewritten;
        }
    }
}

/// Bundle external schema references into the root schema's definitions.
///
/// Takes a map of external schemas keyed by their filename/URI as they
/// appear in `$ref` values. Resolves each external reference by extracting
/// the referenced sub-schema and registering it as a local definition.
pub(crate) fn bundle_external_refs(
    schema: &mut RootSchema,
    external_schemas: &BTreeMap<String, Value>,
) {
    let schema_value = serde_json::to_value(&*schema).unwrap();
    let refs = collect_refs(&schema_value);

    // Find external refs (don't start with #)
    let external_refs: Vec<String> = refs
        .into_iter()
        .filter(|r| !r.starts_with('#') && !r.is_empty())
        .collect();

    if external_refs.is_empty() {
        return;
    }

    let mut rewrites: BTreeMap<String, String> = BTreeMap::new();
    let mut used_names: BTreeSet<String> = schema.definitions.keys().cloned().collect();
    let mut visited: BTreeSet<String> = BTreeSet::new();

    for ref_str in &external_refs {
        if rewrites.contains_key(ref_str) {
            continue;
        }

        resolve_external_ref(
            ref_str,
            external_schemas,
            &mut schema.definitions,
            &mut rewrites,
            &mut used_names,
            &mut visited,
        );
    }

    // Apply rewrites
    if !rewrites.is_empty() {
        let mut schema_value = serde_json::to_value(&*schema).unwrap();
        rewrite_refs(&mut schema_value, &rewrites);
        if let Ok(rewritten) = serde_json::from_value::<RootSchema>(schema_value) {
            *schema = rewritten;
        }
    }
}

/// Resolve a single external reference and register it as a local definition.
fn resolve_external_ref(
    ref_str: &str,
    external_schemas: &BTreeMap<String, Value>,
    definitions: &mut BTreeMap<String, schemars::schema::Schema>,
    rewrites: &mut BTreeMap<String, String>,
    used_names: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
) {
    if visited.contains(ref_str) || rewrites.contains_key(ref_str) {
        return;
    }
    visited.insert(ref_str.to_string());

    // Parse the ref into (document, fragment)
    let (doc_uri, fragment) = match ref_str.split_once('#') {
        Some((doc, frag)) => (doc.to_string(), format!("#{}", frag)),
        None => (ref_str.to_string(), "#".to_string()),
    };

    // Look up the external document
    let ext_schema = match external_schemas.get(&doc_uri) {
        Some(s) => s,
        None => {
            // Try with/without extension
            let alt = if doc_uri.ends_with(".json") {
                doc_uri.trim_end_matches(".json").to_string()
            } else {
                format!("{}.json", doc_uri)
            };
            match external_schemas.get(&alt) {
                Some(s) => s,
                None => return, // Can't resolve — leave unchanged
            }
        }
    };

    // Extract the sub-schema from the external document
    let sub_schema = if fragment == "#" || fragment.is_empty() {
        // Root of external document
        ext_schema.clone()
    } else {
        // JSON Pointer into external document
        let pointer = &fragment[1..]; // strip #
        match ext_schema.pointer(pointer) {
            Some(v) => v.clone(),
            None => return,
        }
    };

    // Generate a unique name
    let base_name = generate_external_name(&doc_uri, &fragment);
    let name = make_unique_name(&base_name, used_names);
    used_names.insert(name.clone());

    // Register as a definition
    if let Ok(parsed) = serde_json::from_value::<schemars::schema::Schema>(sub_schema.clone()) {
        definitions.insert(name.clone(), parsed);
    }

    rewrites.insert(ref_str.to_string(), format!("#/definitions/{}", name));

    // Recursively resolve refs within the external schema
    let nested_refs = collect_refs(&sub_schema);
    for nested_ref in nested_refs {
        if !nested_ref.starts_with('#') {
            // Resolve relative to the same document's directory
            resolve_external_ref(
                &nested_ref,
                external_schemas,
                definitions,
                rewrites,
                used_names,
                visited,
            );
        }
    }
}

/// Recursively collect all `$ref` string values from a JSON value.
fn collect_refs(value: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    collect_refs_inner(value, &mut refs);
    refs
}

fn collect_refs_inner(value: &Value, refs: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_str)) = map.get("$ref") {
                refs.push(ref_str.clone());
            }
            for (_, v) in map {
                collect_refs_inner(v, refs);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_refs_inner(item, refs);
            }
        }
        _ => {}
    }
}

/// Recursively rewrite `$ref` values according to the rewrites map.
fn rewrite_refs(value: &mut Value, rewrites: &BTreeMap<String, String>) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(ref_str)) = map.get("$ref") {
                if let Some(new_ref) = rewrites.get(ref_str.as_str()) {
                    map.insert("$ref".to_string(), Value::String(new_ref.clone()));
                }
            }
            for (_, v) in map.iter_mut() {
                rewrite_refs(v, rewrites);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                rewrite_refs(item, rewrites);
            }
        }
        _ => {}
    }
}

/// Generate a definition name from a JSON Pointer path.
/// E.g., "/properties/foo/items" → "FooItems"
fn generate_name_from_pointer(pointer: &str, used_names: &BTreeSet<String>) -> String {
    let segments: Vec<&str> = pointer
        .split('/')
        .filter(|s| !s.is_empty())
        .filter(|s| {
            // Skip structural keywords
            !matches!(
                *s,
                "properties"
                    | "items"
                    | "additionalProperties"
                    | "definitions"
                    | "$defs"
                    | "allOf"
                    | "anyOf"
                    | "oneOf"
                    | "not"
                    | "if"
                    | "then"
                    | "else"
                    | "patternProperties"
            )
        })
        .collect();

    let base = if segments.is_empty() {
        "AnonymousRef".to_string()
    } else {
        segments
            .iter()
            .map(|s| sanitize(s, Case::Pascal))
            .collect::<Vec<_>>()
            .join("")
    };

    make_unique_name(&base, used_names)
}

/// Generate a definition name for an external reference.
/// E.g., "other-file.json" + "#/definitions/Foo" → "OtherFileFoo"
fn generate_external_name(doc_uri: &str, fragment: &str) -> String {
    // Extract the file stem
    let stem = doc_uri
        .rsplit('/')
        .next()
        .unwrap_or(doc_uri)
        .trim_end_matches(".json")
        .trim_end_matches(".schema");

    let stem_pascal = sanitize(stem, Case::Pascal);

    // Extract the definition name from the fragment
    let def_name = if fragment == "#" || fragment.is_empty() {
        String::new()
    } else {
        let parts: Vec<&str> = fragment
            .split('/')
            .filter(|s| !s.is_empty() && *s != "#" && *s != "definitions" && *s != "$defs")
            .collect();
        parts
            .iter()
            .map(|s| sanitize(s, Case::Pascal))
            .collect::<Vec<_>>()
            .join("")
    };

    if def_name.is_empty() {
        stem_pascal
    } else {
        format!("{}{}", stem_pascal, def_name)
    }
}

/// Make a name unique by appending a numeric suffix if needed.
fn make_unique_name(base: &str, used_names: &BTreeSet<String>) -> String {
    if !used_names.contains(base) {
        return base.to_string();
    }
    let mut i = 2;
    loop {
        let candidate = format!("{}{}", base, i);
        if !used_names.contains(&candidate) {
            return candidate;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn root_schema_from_json(value: Value) -> RootSchema {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn test_resolve_internal_property_ref() {
        let mut schema = root_schema_from_json(json!({
            "type": "object",
            "properties": {
                "foo": { "type": "string", "minLength": 1 },
                "bar": { "$ref": "#/properties/foo" }
            }
        }));
        resolve_internal_refs(&mut schema);
        // The ref should now point to a definition
        let bar = &schema.schema.object.as_ref().unwrap().properties["bar"];
        if let schemars::schema::Schema::Object(obj) = bar {
            let ref_str = obj.reference.as_ref().unwrap();
            assert!(ref_str.starts_with("#/definitions/"), "got: {}", ref_str);
        } else {
            panic!("expected schema object");
        }
        // The definition should exist
        assert!(!schema.definitions.is_empty());
    }

    #[test]
    fn test_resolve_internal_ref_no_change_for_standard() {
        let mut schema = root_schema_from_json(json!({
            "definitions": {
                "Foo": { "type": "string" }
            },
            "type": "object",
            "properties": {
                "bar": { "$ref": "#/definitions/Foo" }
            }
        }));
        let original_defs_count = schema.definitions.len();
        resolve_internal_refs(&mut schema);
        // No new definitions added
        assert_eq!(schema.definitions.len(), original_defs_count);
    }

    #[test]
    fn test_bundle_external_simple() {
        let mut schema = root_schema_from_json(json!({
            "type": "object",
            "properties": {
                "item": { "$ref": "types.json#/definitions/Item" }
            }
        }));

        let mut externals = BTreeMap::new();
        externals.insert(
            "types.json".to_string(),
            json!({
                "definitions": {
                    "Item": { "type": "string" }
                }
            }),
        );

        bundle_external_refs(&mut schema, &externals);

        // The ref should be rewritten to a local definition
        let item = &schema.schema.object.as_ref().unwrap().properties["item"];
        if let schemars::schema::Schema::Object(obj) = item {
            let ref_str = obj.reference.as_ref().unwrap();
            assert!(ref_str.starts_with("#/definitions/"), "got: {}", ref_str);
        }
        // The definition should exist
        assert!(schema.definitions.values().any(|s| {
            if let schemars::schema::Schema::Object(obj) = s {
                obj.instance_type.is_some()
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_bundle_external_bare_filename() {
        let mut schema = root_schema_from_json(json!({
            "type": "object",
            "properties": {
                "thing": { "$ref": "thing.schema.json" }
            }
        }));

        let mut externals = BTreeMap::new();
        externals.insert(
            "thing.schema.json".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" }
                }
            }),
        );

        bundle_external_refs(&mut schema, &externals);

        let thing = &schema.schema.object.as_ref().unwrap().properties["thing"];
        if let schemars::schema::Schema::Object(obj) = thing {
            let ref_str = obj.reference.as_ref().unwrap();
            assert!(ref_str.starts_with("#/definitions/"), "got: {}", ref_str);
        }
    }

    #[test]
    fn test_generate_name_from_pointer() {
        let used = BTreeSet::new();
        assert_eq!(generate_name_from_pointer("/properties/foo", &used), "Foo");
        assert_eq!(
            generate_name_from_pointer("/properties/foo/items", &used),
            "Foo"
        );
        assert_eq!(
            generate_name_from_pointer("/properties/foo/properties/bar", &used),
            "FooBar"
        );
    }

    #[test]
    fn test_generate_external_name() {
        assert_eq!(
            generate_external_name("types.json", "#/definitions/Item"),
            "TypesItem"
        );
        assert_eq!(generate_external_name("thing.schema.json", "#"), "Thing");
        assert_eq!(
            generate_external_name("my-types.json", "#/definitions/Foo"),
            "MyTypesFoo"
        );
    }

    #[test]
    fn test_make_unique_name() {
        let mut used = BTreeSet::new();
        assert_eq!(make_unique_name("Foo", &used), "Foo");
        used.insert("Foo".to_string());
        assert_eq!(make_unique_name("Foo", &used), "Foo2");
        used.insert("Foo2".to_string());
        assert_eq!(make_unique_name("Foo", &used), "Foo3");
    }

    #[test]
    fn test_collect_refs() {
        let schema = json!({
            "properties": {
                "a": { "$ref": "#/definitions/A" },
                "b": { "$ref": "external.json#/definitions/B" }
            },
            "allOf": [
                { "$ref": "#/properties/a" }
            ]
        });
        let refs = collect_refs(&schema);
        assert_eq!(refs.len(), 3);
        assert!(refs.contains(&"#/definitions/A".to_string()));
        assert!(refs.contains(&"external.json#/definitions/B".to_string()));
        assert!(refs.contains(&"#/properties/a".to_string()));
    }
}
