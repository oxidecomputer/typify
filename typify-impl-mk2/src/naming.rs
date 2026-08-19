//! Pre-insertion naming pass.
//!
//! The standalone `typespace` crate requires that every named type carry its
//! final name at insertion time; it has no naming machinery by design. This
//! module resolves names for the converter's output--a map of not-yet-named
//! types--before anything is inserted into the `TypespaceBuilder`.
//!
//! Names come from, in order of precedence:
//!
//! 1. Fixed names: explicit caller hints (`Typify2NameHint`) and names
//!    derived from `$defs` path segments, both recorded by the caller as the
//!    converter's known names.
//! 2. Hints emitted by the converter itself (e.g. the "rest" component of a
//!    tuple struct), which name a type in terms of its parent.
//! 3. Hints derived from the structure of the type graph: a named type
//!    reachable from another named type is named after that parent and the
//!    context (property, variant, item, ...) through which it is reached.
//!
//! Collision handling lives in [`Namespace`]: fixed names win outright (a
//! duplicate fixed name is a caller error), and derived names are tried in
//! hint order, skipping any candidate that is already taken.
//!
//! Future sources called out by the design but not yet wired up: schema
//! titles and generated (e.g. counter-based) fallback names. The internal
//! typespace module never produced either, so adding them here would change
//! output without an oracle to check against.

use std::collections::{BTreeMap, VecDeque};

use log::debug;
use typespace::Type;

use crate::{
    namespace::Namespace,
    schemalet::SchemaRef,
    typespace_ext::children_with_context,
};

/// The name for a named type that has not yet been through the naming pass.
/// The standalone typespace rejects empty names at insertion, so a
/// placeholder that leaks through is caught rather than rendered.
pub(crate) const UNRESOLVED_NAME: &str = "";

/// A converter-supplied naming hint for a single type.
#[derive(Debug, Clone)]
pub(crate) enum NameHint {
    /// Derive the name from the name of the given (named) type plus the
    /// given contextual addition.
    Parent(SchemaRef, String),
}

/// Resolve a name for every named type in `types`.
///
/// `fixed_names` maps type IDs to names that must be used (modulo case
/// normalization); `extra_hints` carries converter-supplied hints, which
/// take precedence over hints derived from the structure of the graph.
///
/// Panics if a named type ends up with no name source at all, or if the
/// hints cannot be resolved without collision; both indicate a converter
/// bug rather than a user error.
pub(crate) fn resolve_names(
    types: &BTreeMap<SchemaRef, Type<SchemaRef>>,
    fixed_names: &BTreeMap<SchemaRef, String>,
    extra_hints: &BTreeMap<SchemaRef, Vec<NameHint>>,
) -> BTreeMap<SchemaRef, String> {
    // Figure out names for the types that need names.
    let mut work = VecDeque::new();

    for (id, typ) in types {
        // If it's not a named type, continue
        if !typ.is_named() {
            continue;
        }

        for (child_id, child_sigil) in children_with_context(typ) {
            work.push_back((id.clone(), child_id, child_sigil))
        }
    }

    // Walk from each named type through unnamed intermediate types to the
    // named types reachable from it, accumulating the context along the way.
    // Each such path becomes a parent-derived hint for the named child.
    let mut structural_hints = BTreeMap::<_, Vec<NameHint>>::new();

    while let Some((parent_id, child_id, child_sigil)) = work.pop_front() {
        debug!("naming walk parent: {parent_id} child: {child_id} child_sigil: {child_sigil}");
        let child_typ = types.get(&child_id).unwrap();

        if child_typ.is_named() {
            structural_hints
                .entry(child_id)
                .or_default()
                .push(NameHint::Parent(parent_id.clone(), child_sigil));
        } else {
            for (grandchild_id, grandchild_sigil) in children_with_context(child_typ) {
                work.push_back((
                    parent_id.clone(),
                    grandchild_id,
                    format!("{child_sigil}-=-{grandchild_sigil}"),
                ))
            }
        }
    }

    let mut namespace = Namespace::<SchemaRef>::default();
    let mut names = BTreeMap::new();

    for (id, typ) in types {
        if !typ.is_named() {
            continue;
        }

        debug!("naming id {id}");
        let nn = namespace.make_name(id.clone());

        if let Some(fixed) = fixed_names.get(id) {
            nn.set_name(fixed);
        } else {
            // Converter-supplied hints take precedence over hints derived
            // from the structure of the type graph.
            let hints = extra_hints
                .get(id)
                .into_iter()
                .flatten()
                .chain(structural_hints.get(id).into_iter().flatten());

            for hint in hints {
                match hint {
                    NameHint::Parent(parent_id, s) => {
                        nn.derive_name(parent_id, s);
                    }
                }
            }
        }

        names.insert(id.clone(), nn);
    }

    // Resolving the namespace turns each accumulated set of hints into a
    // unique, final name (or panics if it can't; see above).
    let _finalized = namespace.finalize().unwrap();

    names
        .into_iter()
        .map(|(id, nn)| (id, nn.to_string()))
        .collect::<BTreeMap<_, _>>()
}
