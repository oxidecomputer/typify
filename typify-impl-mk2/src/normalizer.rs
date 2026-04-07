use std::collections::BTreeMap;

use log::trace;

use crate::{
    bundler::{Bundle, Context},
    schemalet::{to_schemalets, SchemaRef, Schemalet, SchemaletDetails, State2},
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
                    } => {
                        let resolved_target = bundle
                            .resolve(&resolved.context, &target)
                            .expect("failed to resolved reference")
                            .context
                            .location;
                        println!("$ref => {target} {resolved_target}");
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
                        println!("$dynReference => {target} {resolved}");
                        Schemalet {
                            details: SchemaletDetails::ResolvedDynamicRef(SchemaRef::Id(
                                resolved.to_string(),
                            )),
                            metadata,
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
        // let mut wip = vec![SchemaRef::Id(id.to_string())];
        let mut wip = self.nodes.keys().cloned().collect::<Vec<_>>();

        while !wip.is_empty() {
            pass += 1;
            trace!("");
            trace!("new normalization pass: {pass}");
            trace!("");

            let mut modified = Vec::new();
            let mut next_pass_wip = Vec::new();
            let mut simplified = false;
            let mut all_canonical = true;

            // TODO 4/6/2026
            // This is super-jank. Rather than doing anything even nominally
            // clever wrt identifying potentially modified nodes that we want
            // to consider for simplification, we just yeet everything we know
            // about into that list.
            next_pass_wip.extend(self.nodes.keys().cloned());

            for schema_ref in wip.drain(..) {
                let schemalet = self.nodes.get(&schema_ref).unwrap();

                // TODO 4/6/2026
                // We really shouldn't be putting canonical schemalets into
                // the work queue... something to sort out later.
                if let SchemaletDetails::Canonical(_) = &schemalet.details {
                    // If this is already canonical, we don't need to do anything
                    // with it.
                    continue;
                }

                all_canonical = false;

                trace!("simplifying {schema_ref} with details {schemalet:#?}");

                if let State2::Simplified(schemalet, items) = schemalet.simplify2(&self.nodes) {
                    simplified = true;

                    // We've modified this node so add it to the list of
                    // modified nodes so that we can examine any incoming
                    // edges.
                    modified.push(schema_ref.clone());
                    // Add new nodes to the list of nodes to process in the
                    // next pass.
                    next_pass_wip
                        .extend(items.iter().map(|(new_schemaref, _)| new_schemaref.clone()));
                    // Replace the node with the simplified version.
                    self.nodes.insert(schema_ref.clone(), schemalet);
                    // Add any new nodes to the graph.
                    self.nodes.extend(items);
                }
            }

            // We may be able to simplify anything that references a node that
            // has been modified.
            // TODO 4/6/2026
            // We should do that here...

            wip = next_pass_wip;

            // TODO 4/6/2026
            // This can also be rethought once we more tightly manage the
            // work queue for each pass.
            if !simplified {
                if all_canonical {
                    trace!("all canonical, done!");
                    break;
                } else {
                    panic!("no simplifications possible, done!");
                }
            }
        }

        // TODO 4/6/2026
        // It would be cool to validate that everything is normalized as we
        // descend from the root_id

        Ok(())
    }

    pub(crate) fn canonical_output(&self) -> String {
        serde_json::to_string_pretty(&self.nodes.iter().collect::<Vec<_>>()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::schemalet::{
        CanonicalSchemaletDetails, SchemaRef, Schemalet, SchemaletDetails, SchemaletMetadata,
        SchemaletValue, SchemaletValueString,
    };

    use super::Normalizer2;

    #[test]
    fn test_normalize_plain_string() {
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
            },
        );

        normalizer.normalize_from_id("string").unwrap();

        let node = &normalizer.nodes[&id];
        assert!(
            matches!(
                &node.details,
                SchemaletDetails::Canonical(CanonicalSchemaletDetails::Value(
                    SchemaletValue::String(_)
                ))
            ),
            "expected canonical string, got {:#?}",
            node.details
        );
    }
}
