use std::collections::{BTreeMap, VecDeque};

use url::Url;

use crate::{
    bundler::Bundle,
    convert::Converter,
    schemalet::{
        to_schemalets, CanonicalSchemalet, CanonicalSchemaletDetails, SchemaRef, Schemalet,
        SchemaletDetails, SchemaletValue, SchemaletValueString, State,
    },
    typespace::{Type, Typespace, TypespaceBuilder, TypespaceNativeType, TypespaceSettings},
};

pub struct Typify {
    bundle: Bundle,
    normalizer: Normalizer,
    typespace: TypespaceBuilder,
    settings: TypifySettings,
}

#[derive(Debug, serde::Deserialize, Default)]
pub struct TypifySettings {
    convert: Vec<TypifyConvert>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TypifyConvert {
    /// JSON Schema that is evaluated for each schema being converted. If it
    /// validates successfully, then the schema is converted into the given
    /// type.
    pattern: serde_json::Value,

    native: TypespaceNativeType,
}

impl TypifySettings {
    pub fn with_convert(
        &mut self,
        pattern: serde_json::Value,
        type_name: impl ToString,
    ) -> Result<Self> {
        todo!()
    }
}

struct Normalizer {
    raw: BTreeMap<SchemaRef, Schemalet>,
    canonical: BTreeMap<SchemaRef, CanonicalSchemalet>,
}
#[derive(Debug)]
pub enum Error {
    X,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct TypeId(SchemaRef);

impl Typify {
    pub fn new_with_bundle(bundle: Bundle, settings: TypifySettings) -> Self {
        Self {
            bundle,
            normalizer: Default::default(),
            typespace: Default::default(),
            settings,
        }
    }

    /// Add a new type (and any supporting types) by looking up the provided
    /// id in the associated `Bundle` of schema data. The applicable schema
    /// specification is determined by the value in the document named by the
    /// provided id; to override that value, use facilities of the `Bundle`.
    pub fn add_type_by_id(&mut self, id: impl AsRef<str>) -> Result<TypeId> {
        let typ_id = SchemaRef::Id(format!("{}#", id.as_ref()));

        // Add the schemalets reachable from `id` to the graph, and then
        // normalize those additions.
        self.normalizer.add(&self.bundle, id.as_ref())?;

        // TODO
        // This feels wrong; I think it should get a ref to the graph of types.
        // I think the whole point of this type is to isolated and provide
        // structure to the conversion methods.
        let mut converter = Converter::new(self.normalizer.canonical.clone());

        // TODO 9.15.2025
        // This root schema name should be configurable.
        converter.set_name(typ_id.clone(), "SchemaRoot".to_string());

        let mut work = VecDeque::from([typ_id.clone()]);

        'outer: while let Some(work_id) = work.pop_front() {
            if self.typespace.contains_type(&work_id) {
                continue;
            }

            // TODO 7/2/2025
            // Not sure if this is the right place to look for a name, but maybe
            // it's okay. At this point we know that the path really is about to
            // have a type at this location, and we don't know that any sooner.
            //
            // Note that we need to have something more generic than $defs and I'm
            // not sure we're always going to apply this heuristic.
            //
            // TODO 7/10/2025
            // In sum: there's more thinking to do here.
            if let SchemaRef::Id(path) = &work_id {
                let url = Url::parse(path).unwrap();

                if let Some(fragment) = url.fragment() {
                    if let Some(name) = fragment.strip_prefix("/$defs/") {
                        converter.set_name(work_id.clone(), name.to_string());
                    }
                }
            }

            // Get the original JSON that defined this type.
            // TODO 9.15.2025
            // In the future we can add this as content that the Typespace
            // may add to the doc comment for the type.
            let maybe_original_json = match &work_id {
                SchemaRef::Id(xxx) => Some(self.bundle.get_fully_qualified(xxx).unwrap()),
                _ => None,
            };

            // TODO 9.15.2025
            // This is approximately where I want to do JSON Schema validation
            // of the original JSON against any items in the
            // self.settings.convert array. If there's a match, we can skip the
            // part where we do any converting and just slap in the type
            // defined by the conversion setting.

            println!(
                "convert: {}",
                serde_json::to_string_pretty(&self.settings.convert).unwrap(),
            );

            if let Some(original_json) = maybe_original_json {
                let conv = for conv in &self.settings.convert {
                    if jsonschema::is_valid(&conv.pattern, original_json) {
                        let typ = Type::Native(conv.native.name.clone());
                        self.typespace.insert(work_id.clone(), typ);
                        continue 'outer;
                    }
                };
            }

            let typ = converter.convert(&work_id, maybe_original_json);
            let children = typ.children();

            if !children.is_empty() {
                println!("work_id {work_id} children {children:?}");
            }

            work.extend(children);

            self.typespace.insert(work_id.clone(), typ);
        }

        Ok(TypeId(typ_id.clone()))
    }

    pub fn get_typespace_builder(&self) -> &TypespaceBuilder {
        &self.typespace
    }

    pub fn into_typespace_builder(self) -> TypespaceBuilder {
        self.typespace
    }

    pub fn into_typespace(self, settings: TypespaceSettings) -> Typespace {
        self.typespace.finalize(settings).unwrap()
    }

    #[doc(hidden)]
    pub fn canonical_output(&self) -> String {
        self.normalizer.canonical_output()
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        let canonical = [(
            SchemaRef::Internal("string".to_string()),
            CanonicalSchemalet {
                metadata: Default::default(),
                details: CanonicalSchemaletDetails::Value(SchemaletValue::String(
                    SchemaletValueString {
                        pattern: Vec::new(),
                        format: Vec::new(),
                        min_length: None,
                        max_length: None,
                    },
                )),
            },
        )]
        .into();

        Self {
            raw: Default::default(),
            canonical,
        }
    }
}

impl Normalizer {
    pub(crate) fn add(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<()> {
        self.add_raw(bundle, id.as_ref())?;

        // TODO 7.15.2025
        // This is an extremely awful hack as I figure out this interface.
        let wip = self.raw.clone();

        self.normalize(id, wip)?;

        Ok(())
    }

    fn add_raw(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<()> {
        // TODO 7.15.2025
        // This use of "#" doesn't feel quite right
        // Test the situation where the id is not the root of the document.
        let mut references = vec![(bundle.resolve_root(id).unwrap().context, "#".to_string())];

        while let Some((context, path)) = references.pop() {
            let resolved = bundle
                .resolve(&context, &path)
                .expect("failed to resolve reference");

            if self
                .raw
                .contains_key(&SchemaRef::Id(resolved.context.location.to_string()))
            {
                continue;
            }

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
                        references.push((resolved.context.clone(), resolved_target.to_string()));
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
                        let resolved = context.dyn_resolve(&target).clone();
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

                let old = self.raw.insert(schema_ref.clone(), schemalet);
                assert!(old.is_none(), "already present: {}", schema_ref);
            }
        }
        Ok(())
    }

    fn normalize(
        &mut self,
        id: impl AsRef<str>,
        mut wip: BTreeMap<SchemaRef, Schemalet>,
    ) -> Result<()> {
        let mut pass = 0;

        loop {
            if wip.is_empty() {
                break;
            }

            pass += 1;
            println!("new pass: {pass}");
            let mut next = BTreeMap::new();
            let mut simplified = false;
            for (k, v) in wip {
                println!("simplifying {k}");
                match v.simplify(&self.canonical) {
                    State::Canonical(schemalet) => {
                        println!("canonical");
                        println!("{}", serde_json::to_string_pretty(&schemalet).unwrap());
                        simplified = true;
                        self.canonical.insert(k, schemalet);
                    }

                    State::Stuck(schemalet) => {
                        next.insert(k, schemalet);
                    }
                    State::Simplified(schemalet, items) => {
                        simplified = true;
                        next.insert(k, schemalet);
                        for (new_k, new_v) in items {
                            next.insert(new_k, new_v);
                        }
                    }
                }
            }

            wip = next;

            if !simplified {
                panic!("couldn't simplify more");
            }
        }

        for (schema_ref, schemalet) in &self.canonical {
            println!("canonical {}", schema_ref);
            println!("{}", serde_json::to_string_pretty(schemalet).unwrap());
        }

        Ok(())
    }

    fn canonical_output(&self) -> String {
        serde_json::to_string_pretty(&self.canonical.iter().collect::<Vec<_>>()).unwrap()
    }
}
