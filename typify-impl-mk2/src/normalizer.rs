use std::collections::{BTreeMap, BTreeSet};

use anyhow::bail;
use log::{debug, trace};

use crate::{
    bundler::{Bundle, Context},
    schemalet::{to_schemalets, SchemaRef, Schemalet, SchemaletDetails},
    typify::Result,
};

#[derive(Debug, Default)]
pub(crate) struct Normalizer2 {
    pub nodes: BTreeMap<SchemaRef, Schemalet>,
}

impl Normalizer2 {
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
                        canonical,
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
                            canonical,
                        }
                    }

                    // When we hit a dynamic reference, we resolve it right here and
                    // now. This is imperfect in some ways, but suffices for the
                    // singular use of $dynamicRef that we know of and/or care about.
                    Schemalet {
                        details: SchemaletDetails::RawDynamicRef(target),
                        metadata,
                        canonical,
                    } => {
                        let resolved = resolved.context.dyn_resolve(&target).clone();
                        debug!("$dynReference => {target} {resolved}");
                        Schemalet {
                            details: SchemaletDetails::ResolvedDynamicRef(SchemaRef::Id(
                                resolved.to_string(),
                            )),
                            metadata,
                            canonical,
                        }
                    }

                    schemalet => schemalet,
                };

                let old = self.nodes.insert(schema_ref.clone(), schemalet);
                // Note that we really should not hit this; we've checked for
                // duplicate IDs when processing the WIP queue.
                assert!(old.is_none(), "already present: {}", schema_ref);
            }

            let Some((context, path)) = self.next_wip(&mut wip) else {
                break;
            };

            resolved = bundle
                .resolve(&context, &path)
                .expect("failed to resolve reference");
        }

        Ok(root_ref)
    }

    fn next_wip(&self, wip: &mut Vec<(Context, String)>) -> Option<(Context, String)> {
        loop {
            let Some((context, path)) = wip.pop() else {
                return None;
            };

            if self
                .nodes
                .contains_key(&SchemaRef::Id(context.location.to_string()))
            {
                continue;
            }

            return Some((context, path));
        }
    }

    fn normalize_from_id(&mut self, id: &str) -> Result<()> {
        let mut pass = 0;

        // TODO 4/6/2026
        // Where can I get this SchemaRef from rather that consing it up?
        let mut wip = vec![SchemaRef::Id(id.to_string())];

        while !wip.is_empty() {
            pass += 1;
            debug!("pass {pass}");

            let mut simplified = false;

            for schema_ref in wip.drain(..) {
                let schemalet = self.nodes.get(&schema_ref).unwrap();
                debug!("normalizing {schema_ref}");
                trace!("  {schemalet:#?}");

                let xxx = schemalet.simplify2(&self.nodes);

                if let crate::schemalet::State2::Simplified(new_schemalet, items) = xxx {
                    simplified = true;

                    self.nodes.insert(schema_ref.clone(), new_schemalet);
                    self.nodes.extend(items);
                }
            }

            if !simplified {
                debug!("no simplifications on pass {pass}, stopping");
                break;
            }
        }

        self.check_canonical(id)
    }

    fn check_canonical(&self, id: &str) -> Result<()> {
        let schema_ref = SchemaRef::Id(id.to_string());
        let mut seen = BTreeSet::new();
        let mut wip = vec![schema_ref];

        while let Some(schema_ref) = wip.pop() {
            if seen.contains(&schema_ref) {
                continue;
            }
            seen.insert(schema_ref.clone());

            let schemalet = self.nodes.get(&schema_ref).unwrap();
            if !schemalet.canonical {
                bail!(
                    "schemalet {} is not marked canonical after normalization: {:#?}",
                    schema_ref,
                    schemalet
                );
            }

            wip.extend(schemalet.children());
        }

        Ok(())
    }

    pub(crate) fn canonical_output(&self) -> String {
        serde_json::to_string_pretty(&self.nodes.iter().collect::<Vec<_>>()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::schemalet::{
        SchemaRef, Schemalet, SchemaletDetails, SchemaletMetadata, SchemaletValue,
        SchemaletValueString,
    };

    use super::Normalizer2;

    #[test]
    fn test_normalize_plain_string() {
        env_logger::init();
        let mut normalizer = Normalizer2::default();

        let id = SchemaRef::Id("string".to_string());
        normalizer.nodes.insert(
            id.clone(),
            Schemalet {
                metadata: SchemaletMetadata::default(),
                details: SchemaletDetails::Value(SchemaletValue::String(SchemaletValueString {
                    pattern: vec![],
                    format: vec![],
                    min_length: None,
                    max_length: None,
                })),
                canonical: false,
            },
        );

        normalizer.normalize_from_id("string").unwrap();

        let node = &normalizer.nodes[&id];
        assert!(matches!(
            node.details,
            SchemaletDetails::Value(SchemaletValue::String(_))
        ));
        assert!(node.canonical);
    }
}
