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
pub struct NormalizedSchema {
    pub metadata: NormalizedMetadata,
    pub details: NormalizedSchemaDetails,
}

pub struct NormalizedMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    pub examples: Vec<serde_json::Value>,
}

pub enum NormalizedSchemaDetails {
    ExclusiveOneOf { subschemas: Vec<SchemaRef> },
    Concrete(NormalizedSchemaConcreteDetails),
}
pub enum NormalizedSchemaConcreteDetails {
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

pub struct NormalizedInteger {
    pub minimum: Option<serde_json::Number>,
    pub exclusive_minimum: Option<serde_json::Number>,
    pub maximum: Option<serde_json::Number>,
    pub exclusive_maximum: Option<serde_json::Number>,
    pub multiple_of: Vec<serde_json::Number>,
}

pub struct NormalizedNumber {
    pub minimum: Option<f64>,
    pub exclusive_minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub multiple_of: Option<f64>,
}

pub struct NormalizedString {
    pub pattern: Vec<String>,
    pub format: Vec<String>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}

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

pub struct NormalizedObject {
    pub fields: BTreeMap<String, NormalizedObjectField>,
    pub additional_properties: Option<SchemaRef>,
    pub min_properties: Option<u64>,
    pub max_properties: Option<u64>,
    pub property_names: Option<SchemaRef>,
    pub pattern_properties: Vec<(String, SchemaRef)>,
}

pub struct NormalizedObjectField {
    pub schema: SchemaRef,
    pub required: bool,
}

pub struct NormalizedSchemaGraph {
    nodes: BTreeMap<SchemaRef, NormalizedSchema>,
}

pub struct NormalizedSchemaHandle<'a, Kind: NormalizedSchemaHandleKind> {
    schema: &'a NormalizedSchema,
    _phantom: PhantomData<Kind>,
}

impl<'a, Kind: NormalizedSchemaHandleKind> NormalizedSchemaHandle<'a, Kind> {
    pub fn metadata(&self) -> &'a NormalizedMetadata {
        &self.schema.metadata
    }
}

impl<'a> NormalizedSchemaHandle<'a, NormalizedSchemaHandleKindAny> {
    pub fn get_details_any(&self) -> &'a NormalizedSchemaDetails {
        &self.schema.details
    }
}

impl<'a> NormalizedSchemaHandle<'a, NormalizedSchemaHandleKindConcrete> {
    pub fn get_details_concrete(&self) -> &'a NormalizedSchemaConcreteDetails {
        match &self.schema.details {
            NormalizedSchemaDetails::ExclusiveOneOf { .. } => unreachable!(),
            NormalizedSchemaDetails::Concrete(details) => details,
        }
    }
}

pub trait NormalizedSchemaHandleKind {}
pub enum NormalizedSchemaHandleKindAny {}
impl NormalizedSchemaHandleKind for NormalizedSchemaHandleKindAny {}
pub enum NormalizedSchemaHandleKindConcrete {}
impl NormalizedSchemaHandleKind for NormalizedSchemaHandleKindConcrete {}

impl NormalizedSchemaGraph {
    pub fn get_schema<'a>(
        &'a self,
        id: &SchemaRef,
    ) -> NormalizedSchemaHandle<'a, NormalizedSchemaHandleKindAny> {
        NormalizedSchemaHandle {
            schema: self.nodes.get(id).unwrap(),
            _phantom: PhantomData,
        }
    }

    pub fn get_concrete_schema<'a>(
        &'a self,
        id: &SchemaRef,
    ) -> NormalizedSchemaHandle<'a, NormalizedSchemaHandleKindConcrete> {
        let schema = self.nodes.get(id).unwrap();
        if let NormalizedSchemaDetails::ExclusiveOneOf { .. } = &schema.details {
            panic!("expected concrete schema")
        }
        NormalizedSchemaHandle {
            schema: self.nodes.get(id).unwrap(),
            _phantom: PhantomData,
        }
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
