use std::collections::{BTreeMap, VecDeque};

use url::Url;

use crate::{
    bundler::Bundle,
    convert::{ConvertResult, Converter},
    normalizer::Normalizer2,
    schemalet::{to_schemalets, CanonicalSchemalet, SchemaRef, Schemalet, SchemaletDetails, State},
    typespace::{
        Type, TypeNative, Typespace, TypespaceBuilder, TypespaceSettings, TypespaceTraitSet,
    },
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
pub struct TypifyConvert {
    /// JSON Schema that is evaluated for each schema being converted. If it
    /// validates successfully, then the schema is converted into the given
    /// type.
    pattern: serde_json::Value,

    native: TypespaceNativeType,
}

// TODO 9/15/2025
// Placeholder type for non-generated types. We're going to want some mechanism
// to specify the traits we care about so that users have to specify which ones
// are implemented. I'm considering a struct of booleans so that things fail to
// compile if we start to care about some new trait.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TypespaceNativeType {
    pub name: String,
}

impl TypifySettings {
    pub fn with_convert(
        &mut self,
        pattern: serde_json::Value,
        type_name: impl ToString,
    ) -> &mut Self {
        self.convert.push(TypifyConvert {
            pattern,
            native: TypespaceNativeType {
                name: type_name.to_string(),
            },
        });
        self
    }
}

struct Normalizer {
    raw: BTreeMap<SchemaRef, Schemalet>,
    canonical: BTreeMap<SchemaRef, CanonicalSchemalet>,
}

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct TypeId(pub SchemaRef);

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
        let id_str = id.as_ref();
        let id_str = if id_str.contains('#') {
            id_str.to_string()
        } else {
            format!("{}#", id_str)
        };
        let typ_id = SchemaRef::Id(id_str);

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
            println!("work on {work_id}");

            // If we've already converted this type, we can skip it. Note that
            // this may mean we saw it in a previous iteration of this loop or
            // in a previous invocation of this method.
            if self.typespace.contains_type(&work_id) {
                continue;
            }

            // TODO 7/2/2025
            // Not sure if this is the right place to look for a name, but
            // maybe it's okay. At this point we know that the path really is
            // about to have a type at this location, and we don't know that
            // any sooner.
            //
            // Note that we need to have something more generic than $defs and
            // I'm not sure we're always going to apply this heuristic.
            //
            // TODO 7/10/2025
            // In sum: there's more thinking to do here.
            if let SchemaRef::Id(path) = &work_id {
                let url = Url::parse(path).unwrap();

                if let Some(fragment) = url.fragment() {
                    if let Some(name) = fragment.strip_prefix("/$defs/") {
                        if !name.contains('/') {
                            converter.set_name(work_id.clone(), name.to_string());
                        }
                    }
                }
            }

            // Get the original JSON that defined this type.
            // TODO 9.15.2025
            // In the future we can add this as content that the Typespace
            // may add to the doc comment for the type.
            let maybe_original_json = match &work_id {
                SchemaRef::Id(id) => Some(self.bundle.get_fully_qualified(id).unwrap()),
                _ => None,
            };

            // Compare the original JSON schema against each of the specified
            // conversions in the settings. If there's a match, we use the
            // provided type.
            if let Some(original_json) = maybe_original_json {
                for conv in &self.settings.convert {
                    if jsonschema::is_valid(&conv.pattern, original_json) {
                        // TODO 3/31/2026
                        // Need to fill in impls and type params
                        let typ = Type::Native(TypeNative {
                            name: conv.native.name.clone(),
                            impls: TypespaceTraitSet::empty(),
                            parameters: vec![],
                        });
                        self.typespace.insert(work_id.clone(), typ);
                        continue 'outer;
                    }
                }
            }

            let ConvertResult {
                primary,
                additional,
            } = converter.convert(&work_id, maybe_original_json);
            let children = primary.children();

            if !children.is_empty() {
                println!("work_id {work_id} children {children:?}");
            }

            work.extend(children);

            self.typespace.insert(work_id.clone(), primary);
            for (add_id, add_type) in additional {
                let add_children = add_type.children();

                if !add_children.is_empty() {
                    println!("add_id {add_id} children {add_children:?}");
                }

                work.extend(add_children);

                if !self.typespace.contains_type(&add_id) {
                    self.typespace.insert(add_id, add_type);
                }
            }
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
        let canonical = BTreeMap::new();

        Self {
            raw: Default::default(),
            canonical,
        }
    }
}

impl Normalizer {
    pub(crate) fn add(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<()> {
        // We add the raw schemalets from the bundle...
        self.add_raw(bundle, id.as_ref())?;

        // TODO 7.15.2025
        // This is an extremely awful hack as I figure out this interface.
        let wip = self.raw.clone();

        // ... and then normalize everything.
        self.normalize(id, wip)?;

        Ok(())
    }

    fn add_raw(&mut self, bundle: &Bundle, id: impl AsRef<str>) -> Result<()> {
        let fragment = if let Some(ii) = id.as_ref().find('#') {
            &id.as_ref()[ii..]
        } else {
            "#"
        }
        .to_string();

        // TODO 7.15.2025
        // This use of "#" doesn't feel quite right
        // Test the situation where the id is not the root of the document.
        // TODO 11/14/2025
        // We're no longer just using "#" and instead we're grabbing the
        // fragment. But this still feels pretty jank.
        let mut references = vec![(bundle.resolve_root(id).unwrap().context, fragment)];

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
                        canonical: _,
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
                            canonical: false,
                        }
                    }

                    // When we hit a dynamic reference, we resolve it right here and
                    // now. This is imperfect in some ways, but suffices for the
                    // singular use of $dynamicRef that we know of and/or care about.
                    Schemalet {
                        details: SchemaletDetails::RawDynamicRef(target),
                        metadata,
                        canonical: _,
                    } => {
                        let resolved = context.dyn_resolve(&target).clone();
                        println!("$dynReference => {target} {resolved}");
                        Schemalet {
                            details: SchemaletDetails::ResolvedDynamicRef(SchemaRef::Id(
                                resolved.to_string(),
                            )),
                            metadata,
                            canonical: false,
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
        // TODO 1/12/2026
        // I think this is here so I have somewhere to start from?
        _id: impl AsRef<str>,
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

// TODO 4/6/2026
// I'm going to rewrite the Typify interface here before replacing the existing
// version.
/// Object for accumulating a collection of processed schemas. Each schema is
/// processed one-at-a-time and added to that canonical representation. Once
/// all desired schemas have been added, the `typify` method converts these
/// into a `Typespace` that can be used for code generation.
pub struct Typify2 {
    // TODO 4/6/2026
    // Making this an owned object for now, but I think it should be either an
    // owned object or a ref (but Cow requires Clone, which Bundle doesn't
    // currently implement).
    bundle: Bundle,
    normalizer: Normalizer2,

    roots: Vec<(SchemaRef, Typify2NameHint)>,
}

pub enum Typify2NameHint {
    Mandatory(String),
    Suggested(String),
}

impl Typify2 {
    /// Start a new collection of processed schemas within the provided
    /// `Bundle`.
    pub fn new_with_bundle(bundle: Bundle) -> Self {
        Self {
            bundle,
            normalizer: Default::default(),
            roots: Default::default(),
        }
    }

    /// Add a new type (and any supporting types) by looking up the provided
    /// id in the associated `Bundle` of schema data. The applicable schema
    /// specification is determined by the value in the document named by the
    /// provided id; to override that value, use facilities of the `Bundle`.
    /// The provided `name_hint` influences the name of the generated type.
    pub fn add_type(&mut self, id: impl AsRef<str>, name_hint: Typify2NameHint) -> Result<TypeId> {
        let id = if id.as_ref().contains('#') {
            id.as_ref().to_string()
        } else {
            format!("{}#", id.as_ref())
        };
        let root_ref = self.normalizer.add(&self.bundle, &id)?;

        self.roots.push((root_ref.clone(), name_hint));

        Ok(TypeId(root_ref))
    }

    // TODO 4/6/2026
    // Typespace settings should only be needed when rendering to an
    // Codespace... which isn't implemented yet.
    pub fn typify(
        self,
        _settings: TypifySettings,
        _typespace_settings: TypespaceSettings,
    ) -> Result<Typespace> {
        todo!()
        // let Self {
        //     bundle: _,
        //     normalizer: Normalizer2 { nodes },
        //     roots,
        // } = self;

        // let nodes = nodes
        //     .into_iter()
        //     .map(|(k, v)| {
        //         let Schemalet { details, metadata } = v;

        //         let SchemaletDetails::Canonical(canonical) = details else {
        //             panic!()
        //         };

        //         (
        //             k,
        //             CanonicalSchemalet {
        //                 metadata,
        //                 details: canonical,
        //             },
        //         )
        //     })
        //     .collect();

        // let mut typespace_builder = TypespaceBuilder::default();
        // let mut converter = Converter::new(nodes);
        // let mut work = VecDeque::new();
        // roots.into_iter().for_each(|(schema_ref, name)| {
        //     work.push_front(schema_ref.clone());

        //     // TODO 4/6/2026
        //     // Jank alert. I need a way to specify the different kinds of
        //     // hints.
        //     let xxx = match name {
        //         Typify2NameHint::Mandatory(name) => name,
        //         Typify2NameHint::Suggested(name) => name,
        //     };
        //     converter.set_name(schema_ref, xxx);
        // });

        // 'outer: while let Some(work_id) = work.pop_front() {
        //     println!("work on {work_id}");

        //     // If we've already converted this type, we can skip it. Note that
        //     // this may mean we saw it in a previous iteration of this loop or
        //     // in a previous invocation of this method.
        //     if typespace_builder.contains_type(&work_id) {
        //         continue;
        //     }

        //     // Get the original JSON that defined this type.
        //     // TODO 9.15.2025
        //     // In the future we can add this as content that the Typespace
        //     // may add to the doc comment for the type.
        //     let maybe_original_json = match &work_id {
        //         SchemaRef::Id(id) => Some(self.bundle.get_fully_qualified(id).unwrap()),
        //         _ => None,
        //     };

        //     // Compare the original JSON schema against each of the specified
        //     // conversions in the settings. If there's a match, we use the
        //     // provided type.
        //     if let Some(original_json) = maybe_original_json {
        //         for conv in &settings.convert {
        //             if jsonschema::is_valid(&conv.pattern, original_json) {
        //                 // TODO 3/31/2026
        //                 // Need to fill in impls and type params
        //                 let typ = Type::Native(TypeNative {
        //                     name: conv.native.name.clone(),
        //                     impls: TypespaceTraitSet::empty(),
        //                     parameters: vec![],
        //                 });
        //                 typespace_builder.insert(work_id.clone(), typ);
        //                 continue 'outer;
        //             }
        //         }
        //     }

        //     let ConvertResult {
        //         primary,
        //         additional,
        //     } = converter.convert(&work_id, maybe_original_json);
        //     let children = primary.children();

        //     if !children.is_empty() {
        //         println!("work_id {work_id} children {children:?}");
        //     }

        //     work.extend(children);

        //     typespace_builder.insert(work_id.clone(), primary);
        //     for (add_id, add_type) in additional {
        //         let add_children = add_type.children();

        //         if !add_children.is_empty() {
        //             println!("add_id {add_id} children {add_children:?}");
        //         }

        //         work.extend(add_children);

        //         if !typespace_builder.contains_type(&add_id) {
        //             typespace_builder.insert(add_id, add_type);
        //         }
        //     }
        // }

        // Ok(typespace_builder.finalize(typespace_settings).unwrap())
    }

    #[doc(hidden)]
    pub fn canonical_output(&self) -> String {
        self.normalizer.canonical_output()
    }
}
