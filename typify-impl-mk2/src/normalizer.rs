use std::collections::{BTreeMap, BTreeSet};

use anyhow::bail;
use log::{debug, trace};

use crate::{
    bundler::{Bundle, Context},
    schemalet::{to_schemalets, CanonicalSchemalet, SchemaRef, Schemalet, SchemaletDetails, State},
    typify::Result,
};

#[derive(Debug, Default)]
pub(crate) struct Normalizer2 {
    pub raw: BTreeMap<SchemaRef, Schemalet>,
    pub canonical: BTreeMap<SchemaRef, CanonicalSchemalet>,
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

                let old = self.raw.insert(schema_ref.clone(), schemalet);
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
                canonical: false,
            },
        );

        normalizer.normalize_from_id("string").unwrap();

        let node = &normalizer.raw[&id];
        assert!(matches!(
            node.details,
            SchemaletDetails::Value(SchemaletValue::String(_))
        ));
        assert!(node.canonical);
    }
}
