//! How this works
//!
//! 1. Read a file into a serde_json::Value
//! 2. Process anchor and dynamic anchors and the like
//!    This builds up some sort of map from names to absolute paths
//!    - do we want to look for references to other files here as well?
//! 3. Add known desired types into a queue
//!    This might include just the root schema or also any defs
//!    - Do we want to read in referenced other files before this step? I think
//!      probably because we might want to add all those defs or referenced
//!      schemas as well. On the other hand, random undiscovered refs could
//!      pull in some new file so I wouldn't be assured that I had them all.
//!    - Queue entries require a fully qualified path; they may also require
//!      some additional context? Depends on how we handle dynamic refs.

use std::collections::BTreeMap;

use crate::append_map::AppendMap;
use crate::schema::bootstrap;
use url::Url;

mod loader;
mod schemas;

pub use loader::*;

/// TODO writing the description of this in the hope that it will help me find
/// the edges of what it is.
///
/// A Bundle is a collection of documents that have been indexed for reference
/// lookups. I think it will likely involve some sort of internal mutability,
/// more or less representing itself as a cache of documents. A bundle supports
/// heterogeneous schemas meaning that we keep the documents as untyped blob
/// data (serde_json::Value).
///
/// Operations
/// ----------
///
/// Load a document into the bundle. Each document is comprised of its blob
/// of data, caches values necessary for non-path ($anchor / $dynamicAnchor)
/// lookups, and an indication of the schema for the document (concretely,
/// either a JSON Schema draft or OpenAPI spec version). This last field is
/// necessary in order to properly interpret the results of a reference lookup.
/// Adding a "root" document (or maybe any document) needs to produce a Context
/// that lets us determine where we are in order to properly evaluate reference
/// lookups.
///
/// Lookup a reference. Given a context, a reference string, and a reference
/// type (lexical or dynamic), and a type T: Deserialize, return the
/// appropriate value, deserialized as the given type T. This would assume that
/// the caller knew the appropriate type T i.e. which $schema to assume. But we
/// could allow for multi-version, cross-schema references just returning a
/// blob along with the $schema value for the containing document.
///
/// What is the interaction between progenitor and typify via the Bundle?
/// ---------------------------------------------------------------------
///
/// Progenitor is going to put its full document into the Bundle and then
/// extract an OpenAPI document from it.
///
/// We probably assume typify grows native support for OpenAPI v3.0.x schema.
///
/// When progenitor arrives at a schema... then what? There are a couple of
/// possibilities:
/// - As it does today, it could deserialize the schema into structures and
///   then pass the structure into typify. It seems non-crazy to think that
///   typify might be able to accept a materialized structure (i.e. as opposed
///   to only a serde_json::Value or bundle path).
/// - The deserialization of the OpenAPI document could stub out schemas as
///   either Values or paths.
///
/// Note that the latter option is almost certainly what we'll need to do for
/// OpenAPI v3.1 in light of the jsonSchema property that allows the document
/// to change the default interpretation of schemas.
pub struct Bundle {
    documents: AppendMap<DocumentId, Document>,
    loader: Box<dyn Loader>,
}

impl std::fmt::Debug for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bundle")
            .field("documents", &self.documents)
            .finish()
    }
}

impl Default for Bundle {
    fn default() -> Self {
        Self {
            documents: Default::default(),
            loader: Box::new(loader::NullLoader),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentId(Url);

impl DocumentId {
    fn from_url(mut url: Url) -> (Self, String) {
        let fragment = url.fragment().unwrap_or_default().to_string();
        url.set_fragment(None);
        (Self(url), fragment)
    }

    pub fn from_str(s: &str) -> Self {
        let url: Url = s.parse().unwrap();
        assert!(url.fragment().is_none());
        Self(url)
    }
    fn url(&self) -> &Url {
        &self.0
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn with_fragment(&self, fragment: &str) -> Url {
        let mut url = self.0.clone();
        url.set_fragment(Some(fragment));
        url
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct LoadError(pub String);

pub trait Loader {
    /// Return the canonical contents for the given URL.
    fn load(&self, url: Url) -> Result<String, LoadError>;
}

#[derive(Clone, Debug)]
pub struct Document {
    pub id: DocumentId,
    pub content: serde_json::Value,
    pub schema: String,
    pub anchors: BTreeMap<String, String>,
    pub dyn_anchors: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Error(pub String);

impl Error {
    fn missing_schema(id: &str) -> Self {
        Self(format!("Document {id} is missing a schema."))
    }

    fn unknown_schema(id: &str, schema: &str) -> Self {
        Self(format!("Document {id} has an unknown schema: {schema}."))
    }

    pub fn deserialization_error(id: &str, message: &str) -> Self {
        Self(format!("Document {id} failed to deserialize: {message}"))
    }
}

#[derive(Debug)]
pub struct Resolved<'a> {
    pub context: Context,
    pub value: &'a serde_json::Value,
    pub schema: &'a str,
}

pub trait SchemaKind {
    fn make_document(value: serde_json::Value) -> Result<Document, Error>;
}

impl Bundle {
    // TODO playing with the interface

    pub fn new(loader: impl Loader + 'static) -> Self {
        Self {
            documents: Default::default(),
            loader: Box::new(loader),
        }
    }

    /// Add explicit content (i.e. with no file lookup or web download).
    pub fn add_content(&mut self, content: impl AsRef<str>) -> Result<Context, Error> {
        // Turn the text into a JSON blob
        let value: serde_json::Value = serde_json::from_str(content.as_ref())
            .map_err(|e| Error::deserialization_error("unknown", &e.to_string()))?;

        // Figure out the schema
        let schema = value.get("$schema");

        let document = match schema.and_then(serde_json::Value::as_str) {
            // TODO If there's no schema defined, we'll need to figure out some
            // sort of fallback position. We'll want some settings that let us
            // say things like "ignore $schema and use this" or "if there's no
            // schema, try whatever" or "if there's no schema only use this"
            None => Err(Error::missing_schema("unknown"))?,

            Some("https://json-schema.org/draft/2020-12/schema") => {
                bootstrap::Schema::make_document(value)
            }
            other => Err(Error::unknown_schema("unknown", other.unwrap_or("unknown")))?,
        }?;

        let context = Context {
            location: document.id.url().clone(),
            dyn_anchors: document
                .dyn_anchors
                .iter()
                .map(|(anchor, path)| (anchor.clone(), document.id.with_fragment(path)))
                .collect(),
        };

        println!("adding {}", &document.id);
        self.documents.insert(document.id.clone(), document);

        Ok(context)
    }

    pub fn canonicalize_ref(base: &str, reference: &str) -> String {
        let base_url = url::Url::parse(base).unwrap();
        let mut ref_url = base_url.join(reference).unwrap();
        if ref_url.fragment().is_none() {
            ref_url.set_fragment(Some(""));
        }
        ref_url.to_string()
    }

    pub fn resolve_root(&self, id: impl AsRef<str>) -> Result<Resolved, Error> {
        let context = Context {
            location: url::Url::parse(id.as_ref()).unwrap(),
            dyn_anchors: Default::default(),
        };

        self.resolve(&context, "")
    }

    fn xxx_url(base: &Url, reference: &str) -> (DocumentId, String) {
        let ref_url = base.join(reference).unwrap();
        DocumentId::from_url(ref_url)
    }

    pub fn get_fully_qualified(&self, id: impl AsRef<str>) -> Result<&serde_json::Value, Error> {
        let location = Url::parse(id.as_ref()).map_err(|e| Error(e.to_string()))?;

        let (doc_id, fragment) = DocumentId::from_url(location);

        assert!(fragment.starts_with('/') || fragment.is_empty());

        let Some(doc) = self.documents.get(&doc_id) else {
            return Err(Error(format!("document {doc_id} not found")));
        };

        doc.content
            .pointer(&fragment)
            .ok_or_else(|| Error(format!("{fragment} could not be resolved within {doc_id}")))
    }

    /// Resolve a reference within the scope of the given context.
    pub fn resolve(
        &self,
        context: &Context,
        reference: impl AsRef<str>,
    ) -> Result<Resolved, Error> {
        let (id, fragment) = Self::xxx_url(&context.location, reference.as_ref());

        println!("resolving {} as {} {}", reference.as_ref(), id, fragment);

        let doc = if let Some(doc) = self.documents.get(&id) {
            doc
        } else {
            // TODO this is the interesting case. We need to somehow load up
            // the document and get it properly indexed.
            //
            // I think I want to have some kind of plug-in architecture where
            // we can choose between options such as mapping $id -> local files
            // and actually fetching files over the web (maybe with some
            // allow-list or cache or something?).
            //
            // We also need something pluggable dependent on the schema. ...
            // although, we're basically talking about two kinds of thing: JSON
            // Schema (in all its flavors) and OpenAPI (in its several
            // flavors). Could we just ... do those two things? Let's start
            // with that and not worry about plug-ins. At *least* let's start
            // with JSON Schema stuff built-in and then think about how to
            // handle OpenAPI.

            let contents = self.loader.load(id.url().clone()).unwrap();

            let doc = self.load_document(id.as_str(), &contents);

            doc
        };

        let value = &doc.content;

        let value = if fragment.starts_with('/') || fragment.is_empty() {
            value.pointer(&fragment)
        } else {
            let path = doc.anchors.get(&fragment).unwrap();
            // .ok_or(Error)?;
            value.pointer(path)
        }
        .unwrap();
        // .ok_or(Error)?;

        // The dynamic anchors of the incoming context *intentionally*
        // overwrite those of the document.
        let mut dyn_anchors = doc
            .dyn_anchors
            .iter()
            .map(|(anchor, path)| (anchor.clone(), doc.id.with_fragment(path)))
            .collect::<BTreeMap<_, _>>();

        for (k, v) in &context.dyn_anchors {
            dyn_anchors.insert(k.clone(), v.clone());
        }

        let new_context = Context {
            location: id.with_fragment(&fragment),
            dyn_anchors,
        };

        let resolved = Resolved {
            context: new_context,
            value,
            schema: doc.schema.as_str(),
        };

        Ok(resolved)
    }

    pub fn load_document<'a>(&'a self, id: &str, contents: &str) -> &'a Document {
        let content: serde_json::Value =
            serde_json::from_str(contents).expect("couldn't parse into a Value");

        // We need to deduce the schema type from the document. In the case of
        // JSON Schema it might be easy if we see a `$schema` property. For
        // OpenAPI we could look at the `openapi` field. And if we don't find
        // those... I guess we'll just have to figure out something...
        // TODO later

        // TODO If there's no schema defined, we'll need to figure out some
        // sort of fallback position. We'll want some settings that let us
        // say things like "ignore $schema and use this" or "if there's no
        // schema, try whatever" or "if there's no schema only use this"

        let schema = if let Some(schema) = content.get("$schema") {
            schema
                .as_str()
                .expect("we should handle a non-string better")
                .to_string()
        } else {
            todo!("not sure of the schema type");
        };

        let mut document = Document {
            id: DocumentId::from_str(id),
            content,
            anchors: Default::default(),
            dyn_anchors: Default::default(),
            schema: schema.clone(),
        };

        match schema.as_ref() {
            "https://json-schema.org/draft/2020-12/schema" => {
                bootstrap::Schema::populate_document(&mut document);
            }
            _ => todo!(),
        }

        self.documents.insert(document.id.clone(), document)
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub location: Url,
    dyn_anchors: BTreeMap<String, Url>,
}

impl Context {
    pub fn dyn_resolve(&self, target: &'_ str) -> &Url {
        println!("dyn resolve id {} {}", self.location, target);
        println!("{:#?}", self.dyn_anchors);
        self.dyn_anchors.get(target).unwrap()
    }
}

// pub fn to_generic(bundle: &Bundle, context: Context, value: &serde_json::Value, schema: &str) {
//     match schema {
//         "https://json-schema.org/draft/2020-12/schema" => {
//             bootstrap::Schema::to_generic(bundle, context, value);
//         }
//         _ => todo!(),
//     }
// }

// TODO should this be fallible? Probably! What if it's a $schema I don't know?
// What if the serde fails?
// pub fn to_ir(value: &serde_json::Value, schema: &str) -> ir::Schema {
//     match schema {
//         "https://json-schema.org/draft/2020-12/schema" => bootstrap::Schema::to_ir(value),
//         _ => todo!(),
//     }
// }

// pub fn xxx_to_ir(xxx: &Resolved<'_>) -> anyhow::Result<Vec<(ir::SchemaRef, ir::Schema)>> {
//     match xxx.schema {
//         "https://json-schema.org/draft/2020-12/schema" => bootstrap::Schema::xxx_to_ir(xxx),
//         _ => todo!(),
//     }
// }

// pub fn xxx_to_ir2(resolved: &Resolved<'_>) -> anyhow::Result<Vec<(ir2::SchemaRef, ir2::Schema)>> {
//     match resolved.schema {
//         "https://json-schema.org/draft/2020-12/schema" => bootstrap::xxx_to_ir2(resolved),
//         _ => todo!(),
//     }
// }

#[cfg(test)]
mod tests {
    use crate::bundler::Bundle;

    ///
    /// ideas
    /// 1. read in the top-level schema as RAW
    ///     RAW is schema draft specific
    /// 2. convert from RAW -> GENERIC
    ///     GENERIC get from all the various raw forms into some common form
    ///
    /// 3. GENERIC -> INTERNAL
    ///     INTERNAL, simpler and we manipulate it
    ///
    /// 4. Successive passes over INTERNAL to make CANONICAL
    ///     should be in a simple form
    ///     only constructions we like
    ///
    ///
    /// questions
    /// 1. when do we want to deal with dyn refs? I think **after** the
    ///    conversion into GENERIC
    #[test]
    fn xxx() {
        let id = "https://json-schema.org/draft/2020-12/schema";
        let contents = include_str!("../../tests/schemas/input/json-2020-12/schema");

        let bundle = Bundle::default();

        let _doc = bundle.load_document(id, contents);

        panic!();
    }
}
