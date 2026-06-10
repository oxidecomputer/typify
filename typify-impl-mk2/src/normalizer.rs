use std::{collections::BTreeMap, marker::PhantomData};

use log::{debug, trace};

use crate::{
    bundler::{Bundle, Context},
    schemalet::{to_schemalets, CanonicalSchemalet, SchemaRef, Schemalet, SchemaletDetails, State},
    typify::Result,
};

// TODO 6/9/2026
// This is really just a re-do on the CanonicalSchemalet, but I'm renaming to
// draw greater distinction. I'd like this to be the type that we feed into the
// converter.
#[derive(Debug, serde::Serialize)]
pub struct NormalizedSchema {
    pub metadata: NormalizedMetadata,
    pub details: NormalizedSchemaDetails,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct NormalizedMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    pub examples: Vec<serde_json::Value>,
}

#[derive(Debug, serde::Serialize)]
pub enum NormalizedSchemaDetails {
    ExclusiveOneOf {
        /// The JSON type that all subschemas share, if known. Used for naming
        /// untagged enum variants (e.g. a string-typed enum gets "String").
        typ: Option<crate::schemalet::SchemaletType>,
        subschemas: Vec<SchemaRef>,
    },
    /// Carries metadata and delegates type identity to the target.
    Wrapper(SchemaRef),
    Anything,
    Nothing,
    Constant(serde_json::Value),
    Boolean,
    Null,
    Integer(NormalizedInteger),
    Number(NormalizedNumber),
    String(NormalizedString),
    Array(NormalizedSchemaArray),
    Object(NormalizedObject),
}

#[derive(Debug, serde::Serialize)]
pub struct NormalizedInteger {
    pub minimum: Option<serde_json::Number>,
    pub exclusive_minimum: Option<serde_json::Number>,
    pub maximum: Option<serde_json::Number>,
    pub exclusive_maximum: Option<serde_json::Number>,
    pub multiple_of: Vec<serde_json::Number>,
}

#[derive(Debug, serde::Serialize)]
pub struct NormalizedNumber {
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub multiple_of: Option<f64>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct NormalizedString {
    pub pattern: Vec<String>,
    pub format: Vec<String>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
pub struct NormalizedSchemaArray {
    pub items: SchemaRef,
    pub prefix_items: Vec<SchemaRef>,
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
    pub unique_items: bool,
    // TODO 6/9/2026
    // contains/minContains/maxContains are not represented. We'll probably
    // need them at some point and would turn them into a newtype wrapper
    // constraint.
}

#[derive(Debug, serde::Serialize)]
pub struct NormalizedObject {
    pub fields: BTreeMap<String, NormalizedObjectField>,
    pub additional_properties: Option<SchemaRef>,
    pub min_properties: Option<u64>,
    pub max_properties: Option<u64>,
    pub property_names: Option<SchemaRef>,
    pub pattern_properties: Vec<(String, SchemaRef)>,
}

#[derive(Debug, serde::Serialize)]
pub struct NormalizedObjectField {
    pub schema: SchemaRef,
    pub required: bool,
}

pub struct NormalizedSchemaGraph {
    nodes: BTreeMap<SchemaRef, NormalizedSchema>,
}

impl NormalizedSchemaGraph {
    pub(crate) fn get_schema_inner(&self, id: &SchemaRef) -> &NormalizedSchema {
        self.nodes
            .get(id)
            .unwrap_or_else(|| panic!("failed to lookup {id}"))
    }
}

#[derive(Debug, Default)]
pub(crate) struct Normalizer {
    pub raw: BTreeMap<SchemaRef, Schemalet>,
    pub canonical: BTreeMap<SchemaRef, CanonicalSchemalet>,
}

impl Normalizer {
    pub(crate) fn add(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<SchemaRef> {
        let id = id.as_ref();

        // Add the schemalets from the bundle...
        let root_ref = self.add_nodes(bundle, id)?;

        // ... and then normalize descending from the given id.
        self.normalize_from_id(id)?;

        Ok(root_ref)
    }

    fn add_nodes(&mut self, bundle: &Bundle, root_id: &str) -> Result<SchemaRef> {
        let mut resolved = bundle.resolve_root(root_id).unwrap();
        let mut wip = Vec::new();

        let root_ref = SchemaRef::Id(resolved.context.location.to_string());

        loop {
            let schemalets = to_schemalets(&resolved).unwrap();

            for (schema_ref, schemalet) in schemalets {
                let schemalet = match schemalet {
                    // I've decided that the final "raw" form should have relative
                    // references resolved. This makes some of the logic ... into
                    // an opportunity for greater consistency!
                    Schemalet {
                        details: SchemaletDetails::RawRef(target),
                        metadata,
                    } => {
                        let resolved_target = bundle
                            .resolve(&resolved.context, &target)
                            .expect("failed to resolved reference")
                            .context
                            .location;
                        debug!("$ref => {target} {resolved_target}");
                        wip.push((resolved.context.clone(), resolved_target.to_string()));
                        Schemalet {
                            details: SchemaletDetails::ResolvedRef(SchemaRef::Id(
                                resolved_target.to_string(),
                            )),
                            metadata,
                        }
                    }

                    // When we hit a dynamic reference, we resolve it right here and
                    // now. This is imperfect in some ways, but suffices for the
                    // singular use of $dynamicRef that we know of and/or care about.
                    Schemalet {
                        details: SchemaletDetails::RawDynamicRef(target),
                        metadata,
                    } => {
                        let resolved = resolved.context.dyn_resolve(&target).clone();
                        debug!("$dynReference => {target} {resolved}");
                        Schemalet {
                            details: SchemaletDetails::ResolvedDynamicRef(SchemaRef::Id(
                                resolved.to_string(),
                            )),
                            metadata,
                        }
                    }

                    schemalet => schemalet,
                };

                let old = self.raw.insert(schema_ref.clone(), schemalet.clone());
                // Note that we really should not hit this; we've checked for
                // duplicate IDs when processing the WIP queue.
                if let Some(old) = old {
                    panic!("duplicate schema reference: {schema_ref}\n  old: {old:#?}\n  new: {schemalet:#?}");
                }
            }

            let Some((context, path)) = self.next_wip(&mut wip) else {
                break;
            };

            resolved = bundle
                .resolve(&context, &path)
                .expect("failed to resolve reference");
        }

        for (schema_ref, schemalet) in &self.raw {
            let xxx = schemalet.children();
            for yyy in xxx {
                assert!(self.raw.contains_key(&yyy), "{schema_ref} {schemalet:#?}");
            }
        }

        Ok(root_ref)
    }

    fn next_wip(&self, wip: &mut Vec<(Context, String)>) -> Option<(Context, String)> {
        loop {
            let Some((context, path)) = wip.pop() else {
                return None;
            };

            if self.raw.contains_key(&SchemaRef::Id(path.clone())) {
                continue;
            }

            return Some((context, path));
        }
    }

    fn normalize_from_id(&mut self, id: &str) -> Result<()> {
        // First, we're going to descend from the given Id and do simple
        // conversions into the "canonical" form--which is really just a
        // simpler IR that we'll continue to manipulate.
        let mut pass = 0;

        // TODO 4/6/2026
        // Where can I get this SchemaRef from rather that consing it up?
        // let mut wip = vec![SchemaRef::Id(id.to_string())];

        loop {
            pass += 1;
            debug!("\npass {pass}\n");

            let mut simplified = false;
            let mut all_canonical = true;

            // TODO 4/7/2026
            // Very inefficient, but let's just scrub the whole list each time.
            let mut wip = self.raw.keys().cloned().collect::<Vec<_>>();

            for schema_ref in wip.drain(..) {
                // We can skip any schemalet that we've already converted to
                // their canonical form.
                if self.canonical.contains_key(&schema_ref) {
                    trace!("already canonical: {schema_ref}");
                    continue;
                }

                all_canonical = false;

                // TODO 4/7/2026 clean up this clone()
                let schemalet = self.raw.get(&schema_ref).unwrap().clone();
                debug!("normalizing {schema_ref}");
                trace!("  {schemalet:#?}");

                match schemalet.simplify(&self.canonical) {
                    State::Stuck(schemalet) => {
                        let _ = schemalet;
                    }
                    State::Simplified(schemalet, items) => {
                        simplified = true;
                        self.raw.insert(schema_ref.clone(), schemalet);
                        self.raw.extend(items);
                    }
                    State::Canonical(canonical_schemalet) => {
                        simplified = true;
                        self.canonical
                            .insert(schema_ref.clone(), canonical_schemalet);
                        debug!("  canonical {schema_ref}");
                    }
                }
            }

            if all_canonical {
                break;
            }

            if !simplified {
                debug!("couldn't simplify further on pass {pass}");
                for (schema_ref, schemalet) in &self.raw {
                    if !self.canonical.contains_key(schema_ref) {
                        debug!("stuck: {schema_ref}: {schemalet:#?}");
                        // } else {
                        //     debug!("done: {schema_ref}: {schemalet:#?}");
                    }
                }

                panic!("no simplifications on pass {pass}, stopping");
            }
        }

        // TODO 4/7/2026
        // DO I want to have something in here to make it... *more* canonical??

        Ok(())
    }

    pub(crate) fn canonical_output(&self) -> String {
        serde_json::to_string_pretty(&self.canonical.iter().collect::<Vec<_>>()).unwrap()
    }

    // Follow transparent (no-metadata) Reference/Note chains to find the first
    // "significant" node — one that has metadata or is not a bare indirection.
    fn resolve_transparent<'a>(&'a self, start: &'a SchemaRef) -> &'a SchemaRef {
        use crate::schemalet::CanonicalSchemaletDetails;
        let mut current = start;
        loop {
            let canonical = &self.canonical[current];
            let has_metadata = canonical.metadata.title.is_some()
                || canonical.metadata.description.is_some()
                || !canonical.metadata.examples.is_empty();
            match &canonical.details {
                CanonicalSchemaletDetails::Reference(r) | CanonicalSchemaletDetails::Note(r)
                    if !has_metadata =>
                {
                    current = r;
                }
                _ => return current,
            }
        }
    }

    pub(crate) fn to_normalized_graph(&self) -> NormalizedSchemaGraph {
        use crate::schemalet::{CanonicalSchemaletDetails, SchemaletValue, SchemaletValueObject};

        // A well-known ref used when a schema has no item/property constraint
        // (i.e. the constraint is implicitly "Anything").
        const ANYTHING_REF_ID: &str = "urn:typify:well-known/anything";
        let anything_ref = SchemaRef::Id(ANYTHING_REF_ID.to_string());

        let mut nodes: BTreeMap<SchemaRef, NormalizedSchema> = BTreeMap::new();

        // Insert the well-known Anything node.
        nodes.insert(
            anything_ref.clone(),
            NormalizedSchema {
                metadata: NormalizedMetadata {
                    title: None,
                    description: None,
                    default: None,
                    examples: vec![],
                },
                details: NormalizedSchemaDetails::Anything,
            },
        );

        let translate_object = |obj: &SchemaletValueObject| -> NormalizedObject {
            let required: std::collections::BTreeSet<_> = obj.required.iter().collect();
            let fields = obj
                .properties
                .iter()
                .map(|(name, schema_ref)| {
                    (
                        name.clone(),
                        NormalizedObjectField {
                            schema: self.resolve_transparent(schema_ref).clone(),
                            required: required.contains(name),
                        },
                    )
                })
                .collect();
            let pattern_properties = obj
                .pattern_properties
                .iter()
                .flat_map(|m| m.iter())
                .map(|(pat, sr)| (pat.clone(), self.resolve_transparent(sr).clone()))
                .collect();
            NormalizedObject {
                fields,
                pattern_properties,
                additional_properties: obj
                    .additional_properties
                    .as_ref()
                    .map(|r| self.resolve_transparent(r).clone()),
                property_names: obj
                    .property_names
                    .as_ref()
                    .map(|r| self.resolve_transparent(r).clone()),
                min_properties: None,
                max_properties: None,
            }
        };

        // Translate a non-indirection CanonicalSchemaletDetails to NormalizedSchemaDetails.
        let translate_details = |v: &CanonicalSchemaletDetails| -> NormalizedSchemaDetails {
            match v {
                CanonicalSchemaletDetails::Anything => NormalizedSchemaDetails::Anything,
                CanonicalSchemaletDetails::Nothing => NormalizedSchemaDetails::Nothing,
                CanonicalSchemaletDetails::Constant(v) => {
                    NormalizedSchemaDetails::Constant(v.clone())
                }
                CanonicalSchemaletDetails::ExclusiveOneOf { typ, subschemas } => {
                    NormalizedSchemaDetails::ExclusiveOneOf {
                        typ: typ.clone(),
                        subschemas: subschemas
                            .iter()
                            .map(|s| self.resolve_transparent(s).clone())
                            .collect(),
                    }
                }
                CanonicalSchemaletDetails::Value(v) => match v {
                    SchemaletValue::Boolean => NormalizedSchemaDetails::Boolean,
                    SchemaletValue::Null => NormalizedSchemaDetails::Null,
                    SchemaletValue::Integer(i) => {
                        NormalizedSchemaDetails::Integer(NormalizedInteger {
                            minimum: i.minimum.clone(),
                            exclusive_minimum: i.exclusive_minimum.clone(),
                            maximum: None,
                            exclusive_maximum: None,
                            multiple_of: vec![],
                        })
                    }
                    SchemaletValue::Number(n) => NormalizedSchemaDetails::Number(NormalizedNumber {
                        minimum: n.minimum,
                        exclusive_minimum: n.exclusive_minimum,
                        maximum: n.maximum,
                        exclusive_maximum: n.exclusive_maximum,
                        multiple_of: n.multiple_of,
                    }),
                    SchemaletValue::String(s) => NormalizedSchemaDetails::String(NormalizedString {
                        pattern: s.pattern.clone(),
                        format: s.format.clone(),
                        min_length: s.min_length,
                        max_length: s.max_length,
                    }),
                    SchemaletValue::Array(a) => NormalizedSchemaDetails::Array(NormalizedSchemaArray {
                        items: a
                            .items
                            .as_ref()
                            .map(|r| self.resolve_transparent(r).clone())
                            .unwrap_or_else(|| anything_ref.clone()),
                        prefix_items: a
                            .prefix_items
                            .iter()
                            .flatten()
                            .map(|r| self.resolve_transparent(r).clone())
                            .collect(),
                        min_items: a.min_items,
                        max_items: a.max_items,
                        unique_items: a.unique_items.unwrap_or(false),
                    }),
                    SchemaletValue::Object(obj) => {
                        NormalizedSchemaDetails::Object(translate_object(obj))
                    }
                },
                // Callers must have resolved through chains before calling this.
                CanonicalSchemaletDetails::Reference(_) | CanonicalSchemaletDetails::Note(_) => {
                    unreachable!("indirection should have been resolved before translate_details")
                }
            }
        };

        for (schema_ref, canonical) in &self.canonical {
            let metadata = NormalizedMetadata {
                title: canonical.metadata.title.clone(),
                description: canonical.metadata.description.clone(),
                default: None,
                examples: canonical.metadata.examples.clone(),
            };

            let details = match &canonical.details {
                CanonicalSchemaletDetails::Anything => NormalizedSchemaDetails::Anything,
                CanonicalSchemaletDetails::Nothing => NormalizedSchemaDetails::Nothing,
                CanonicalSchemaletDetails::Constant(v) => {
                    NormalizedSchemaDetails::Constant(v.clone())
                }
                // References/Notes with metadata become Wrapper nodes so the
                // converter can collect that metadata while following chains.
                // Those without metadata are inlined directly to avoid
                // pointless indirection.
                CanonicalSchemaletDetails::Reference(r) | CanonicalSchemaletDetails::Note(r) => {
                    let has_metadata = metadata.title.is_some()
                        || metadata.description.is_some()
                        || !metadata.examples.is_empty();
                    if has_metadata {
                        NormalizedSchemaDetails::Wrapper(r.clone())
                    } else {
                        translate_details(&self.canonical[self.resolve_transparent(r)].details)
                    }
                }
                other => translate_details(other),
            };

            nodes.insert(schema_ref.clone(), NormalizedSchema { metadata, details });
        }

        NormalizedSchemaGraph { nodes }
    }
}

#[cfg(test)]
mod tests {
    use crate::schemalet::{
        SchemaRef, Schemalet, SchemaletDetails, SchemaletMetadata, SchemaletValue,
        SchemaletValueString,
    };

    use super::Normalizer;

    #[test]
    fn test_normalize_plain_string() {
        env_logger::init();
        let mut normalizer = Normalizer::default();

        let id = SchemaRef::Id("string".to_string());
        normalizer.raw.insert(
            id.clone(),
            Schemalet {
                metadata: SchemaletMetadata::default(),
                details: SchemaletDetails::Value(SchemaletValue::String(SchemaletValueString {
                    pattern: vec![],
                    format: vec![],
                    min_length: None,
                    max_length: None,
                })),
            },
        );

        normalizer.normalize_from_id("string").unwrap();

        let node = &normalizer.raw[&id];
        assert!(matches!(
            node.details,
            SchemaletDetails::Value(SchemaletValue::String(_))
        ));
    }
}
