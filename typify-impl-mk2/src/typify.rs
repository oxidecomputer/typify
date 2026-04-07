use std::collections::VecDeque;

use log::{debug, trace};
use url::Url;

use crate::{
    bundler::Bundle,
    convert::{ConvertResult, Converter},
    normalizer::Normalizer,
    schemalet::SchemaRef,
    typespace::{
        Type, TypeNative, Typespace, TypespaceBuilder, TypespaceSettings, TypespaceTraitSet,
    },
};

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

    /// The external, native type that replaces the schema when the pattern
    /// matches.
    native: TypespaceNativeType,
}

// TODO 9/15/2025
// Placeholder type for non-generated types. We're going to want some mechanism
// to specify the traits we care about so that users have to specify which ones
// are implemented. I'm considering a struct of booleans so that things fail to
// compile if we start to care about some new trait.
/// Used to specify external, replacement types whose use augments how types
/// are constructed.
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

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct TypeId(pub SchemaRef);

/// Object for accumulating a collection of processed schemas. Each schema is
/// processed one-at-a-time and added to that canonical representation. Once
/// all desired schemas have been added, the `typify` method converts these
/// into a `Typespace` that can be used for code generation.
pub struct Typify {
    // TODO 4/6/2026
    // Making this an owned object for now, but I think it should be either an
    // owned object or a ref (but Cow requires Clone, which Bundle doesn't
    // currently implement).
    bundle: Bundle,
    normalizer: Normalizer,

    roots: Vec<(SchemaRef, Typify2NameHint)>,
}

pub enum Typify2NameHint {
    Mandatory(String),
    Suggested(String),
}

impl Typify {
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
        settings: TypifySettings,
        typespace_settings: TypespaceSettings,
    ) -> Result<Typespace> {
        let Self {
            bundle,
            normalizer: Normalizer { raw: _, canonical },
            roots,
        } = self;

        let mut typespace_builder = TypespaceBuilder::default();
        let mut converter = Converter::new(canonical);
        let mut work = VecDeque::new();
        roots.into_iter().for_each(|(schema_ref, name)| {
            work.push_front(schema_ref.clone());

            // TODO 4/6/2026
            // Jank alert. I need a way to specify the different kinds of
            // hints.
            let xxx = match name {
                Typify2NameHint::Mandatory(name) => name,
                Typify2NameHint::Suggested(name) => name,
            };
            trace!("typify name {schema_ref} {xxx}");
            converter.set_name(schema_ref, xxx);
        });

        'outer: while let Some(work_id) = work.pop_front() {
            debug!("work on {work_id}");

            // If we've already converted this type, we can skip it. Note that
            // this may mean we saw it in a previous iteration of this loop or
            // in a previous invocation of this method.
            if typespace_builder.contains_type(&work_id) {
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
                SchemaRef::Id(id) => Some(bundle.get_fully_qualified(id).unwrap()),
                _ => None,
            };

            // Compare the original JSON schema against each of the specified
            // conversions in the settings. If there's a match, we use the
            // provided type.
            if let Some(original_json) = maybe_original_json {
                for conv in &settings.convert {
                    if jsonschema::is_valid(&conv.pattern, original_json) {
                        // TODO 3/31/2026
                        // Need to fill in impls and type params
                        let typ = Type::Native(TypeNative {
                            name: conv.native.name.clone(),
                            impls: TypespaceTraitSet::empty(),
                            parameters: vec![],
                        });
                        typespace_builder.insert(work_id.clone(), typ);
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
                trace!("work_id {work_id} children {children:?}");
            }

            work.extend(children);

            typespace_builder.insert(work_id.clone(), primary);
            for (add_id, add_type) in additional {
                let add_children = add_type.children();

                if !add_children.is_empty() {
                    trace!("add_id {add_id} children {add_children:?}");
                }

                work.extend(add_children);

                if !typespace_builder.contains_type(&add_id) {
                    typespace_builder.insert(add_id, add_type);
                }
            }
        }

        Ok(typespace_builder.finalize(typespace_settings).unwrap())
    }

    #[doc(hidden)]
    pub fn canonical_output(&self) -> String {
        self.normalizer.canonical_output()
    }
}
