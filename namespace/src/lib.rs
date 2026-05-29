//! Name resolution for generated Rust types.
//!
//! [`Namespace`] collects naming hints for a set of typed ids and resolves
//! them to unique PascalCase Rust identifiers. Three hint tiers are supported,
//! resolved in priority order:
//!
//! 1. **Required** — a fixed name that must be used exactly. Duplicate
//!    required names for different ids are an error.
//! 2. **Suggested** — a preferred name. If the name is already taken, a
//!    numeric suffix (`Thing2`, `Thing3`, …) is appended. Multiple suggestions
//!    per id are tried in order before falling back to mangling.
//! 3. **Dependent** — a name derived from another id's resolved name plus a
//!    suffix (`ParentChild`). Resolved after its parent, so chains are
//!    supported.
//!
//! All input strings are converted to PascalCase before use.

use std::collections::{BTreeMap, VecDeque};

use heck::ToPascalCase;

#[derive(Debug, Clone)]
enum NameHint<Id> {
    Required(String),
    Suggested(String),
    Dependent { parent: Id, suffix: String },
}

#[derive(Debug)]
pub enum NamingError<Id> {
    /// Two types both claimed the same required name.
    DuplicateRequiredName { name: String, ids: [Id; 2] },
    /// A dependent hint referenced an id that was never registered.
    UnknownParent { id: Id, parent: Id },
    /// A type had no hints or all of its hints were unresolvable.
    Unresolvable(Id),
}

impl<Id: std::fmt::Display> std::fmt::Display for NamingError<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamingError::DuplicateRequiredName { name, ids: [a, b] } => {
                write!(f, "ids {a} and {b} both require the name \"{name}\"")
            }
            NamingError::UnknownParent { id, parent } => {
                write!(f, "id {id} depends on unregistered parent {parent}")
            }
            NamingError::Unresolvable(id) => {
                write!(f, "id {id} could not be assigned a name")
            }
        }
    }
}

impl<Id: std::fmt::Display + std::fmt::Debug> std::error::Error for NamingError<Id> {}

/// Collects naming hints for a set of ids and resolves them to unique
/// PascalCase Rust identifiers. See the [crate-level docs](crate) for the
/// resolution algorithm.
pub struct Namespace<Id> {
    hints: BTreeMap<Id, Vec<NameHint<Id>>>,
}

impl<Id> Default for Namespace<Id> {
    fn default() -> Self {
        Self {
            hints: BTreeMap::default(),
        }
    }
}

impl<Id> Namespace<Id>
where
    Id: Ord + Clone + std::fmt::Debug,
{
    /// Register a required name for `id`. The name is converted to PascalCase.
    /// Registering two different ids with the same required name is an error
    /// at [`finalize`](Self::finalize) time.
    ///
    /// # Panics
    ///
    /// Panics if `require` has already been called for this `id`.
    pub fn require(&mut self, id: &Id, name: impl Into<String>) {
        let hints = self.hints.entry(id.clone()).or_default();
        assert!(
            !hints.iter().any(|h| matches!(h, NameHint::Required(_))),
            "require called twice for the same id",
        );
        hints.push(NameHint::Required(name.into()));
    }

    /// Register a suggested name for `id`. Multiple suggestions may be added;
    /// they are tried in registration order before falling back to a numeric
    /// suffix on the first suggestion.
    pub fn suggest(&mut self, id: &Id, name: impl Into<String>) {
        self.hints
            .entry(id.clone())
            .or_default()
            .push(NameHint::Suggested(name.into()));
    }

    /// Register a dependent name for `id`: once `parent` resolves, `id` gets
    /// the name `{parent_name}{Suffix}`. The suffix is converted to PascalCase
    /// and appended directly to the already-cased parent name.
    pub fn depend(&mut self, id: &Id, parent: &Id, suffix: impl Into<String>) {
        self.hints
            .entry(id.clone())
            .or_default()
            .push(NameHint::Dependent {
                parent: parent.clone(),
                suffix: suffix.into(),
            });
    }

    /// Resolve all hints to unique PascalCase names. Returns a map from every
    /// registered id to its final name, or the first error encountered.
    pub fn finalize(self) -> Result<BTreeMap<Id, String>, NamingError<Id>> {
        let Self { hints } = self;
        let mut resolved: BTreeMap<Id, String> = BTreeMap::new();
        // Maps every taken name to the id that claimed it. Used for O(log n)
        // conflict checks in all three passes and for identifying the
        // conflicting id in DuplicateRequiredName errors.
        let mut taken: BTreeMap<String, Id> = BTreeMap::new();

        // Validate that all parent ids referenced by Dependent hints are
        // registered.
        for (id, id_hints) in &hints {
            for hint in id_hints {
                if let NameHint::Dependent { parent, .. } = hint {
                    if !hints.contains_key(parent) {
                        return Err(NamingError::UnknownParent {
                            id: id.clone(),
                            parent: parent.clone(),
                        });
                    }
                }
            }
        }

        // Pass 1: resolve all Required hints.
        for (id, id_hints) in &hints {
            let Some(name) = id_hints.iter().find_map(|h| {
                if let NameHint::Required(n) = h { Some(n) } else { None }
            }) else {
                continue;
            };
            let name = name.to_pascal_case();
            if let Some(other_id) = taken.get(&name) {
                return Err(NamingError::DuplicateRequiredName {
                    name,
                    ids: [other_id.clone(), id.clone()],
                });
            }
            resolved.insert(id.clone(), name.clone());
            taken.insert(name, id.clone());
        }

        // Pass 2: resolve Suggested hints for types not yet resolved. For
        // each type, try its suggestions in order and use the first
        // conflict-free one. Only mangle with a numeric suffix if every
        // suggestion is already taken.
        let unresolved: Vec<Id> = hints
            .keys()
            .filter(|id| !resolved.contains_key(*id))
            .cloned()
            .collect();

        for id in unresolved {
            let id_hints = hints.get(&id).expect("id came from hints.keys()");
            let candidates: Vec<String> = id_hints
                .iter()
                .filter_map(|h| {
                    if let NameHint::Suggested(s) = h {
                        Some(s.to_pascal_case())
                    } else {
                        None
                    }
                })
                .collect();

            if candidates.is_empty() {
                continue;
            }

            // Prefer any conflict-free candidate; fall back to mangling
            // the first one.
            let chosen = candidates
                .iter()
                .find(|s| !taken.contains_key(*s))
                .cloned()
                .unwrap_or_else(|| unique_name(&candidates[0], &taken));

            taken.insert(chosen.clone(), id.clone());
            resolved.insert(id, chosen);
        }

        // Pass 3: resolve Dependent hints via a topological walk. As parent
        // names resolve, their dependents become available.
        let mut parent_to_dependents: BTreeMap<Id, Vec<Id>> = BTreeMap::new();
        for (id, id_hints) in &hints {
            if resolved.contains_key(id) {
                continue;
            }
            for hint in id_hints {
                if let NameHint::Dependent { parent, .. } = hint {
                    parent_to_dependents
                        .entry(parent.clone())
                        .or_default()
                        .push(id.clone());
                }
            }
        }

        // Seed the queue with dependents whose parents are already resolved.
        let mut queue: VecDeque<Id> = resolved
            .keys()
            .filter_map(|id| parent_to_dependents.get(id))
            .flatten()
            .cloned()
            .collect();

        while let Some(id) = queue.pop_front() {
            if resolved.contains_key(&id) {
                continue;
            }

            let id_hints = hints.get(&id).expect("id came from hints.keys()");

            // Find the first Dependent hint whose parent is already resolved.
            // The parent name is already correctly cased; only the suffix
            // needs conversion. This must succeed: the id is in the queue
            // only because one of its parents just resolved.
            let name = id_hints
                .iter()
                .find_map(|h| {
                    if let NameHint::Dependent { parent, suffix } = h {
                        let parent_name = resolved.get(parent)?;
                        Some(format!("{}{}", parent_name, suffix.to_pascal_case()))
                    } else {
                        None
                    }
                })
                .expect("id in queue must have a resolved parent");

            let unique = unique_name(&name, &taken);
            taken.insert(unique.clone(), id.clone());
            resolved.insert(id.clone(), unique);

            // Unblock any dependents of this id.
            if let Some(children) = parent_to_dependents.get(&id) {
                queue.extend(children.iter().cloned());
            }
        }

        // Any id still unresolved is an error.
        for id in hints.keys() {
            if !resolved.contains_key(id) {
                return Err(NamingError::Unresolvable(id.clone()));
            }
        }

        Ok(resolved)
    }
}

fn unique_name<Id>(base: &str, taken: &BTreeMap<String, Id>) -> String {
    if !taken.contains_key(base) {
        return base.to_string();
    }
    (2..)
        .find_map(|n| {
            let candidate = format!("{base}{n}");
            if !taken.contains_key(&candidate) {
                Some(candidate)
            } else {
                None
            }
        })
        .expect("infinite iterator always finds a free name")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── basic ────────────────────────────────────────────────────────────────

    #[test]
    fn required_names_resolve() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "foo-bar");
        ns.require(&2u32, "baz");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "FooBar");
        assert_eq!(map[&2], "Baz");
    }

    #[test]
    fn suggested_names_resolve() {
        let mut ns = Namespace::default();
        ns.suggest(&1u32, "thing");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Thing");
    }

    #[test]
    fn suggested_collision_gets_suffix() {
        let mut ns = Namespace::default();
        ns.suggest(&1u32, "thing");
        ns.suggest(&2u32, "thing");
        let map = ns.finalize().unwrap();
        let names: std::collections::BTreeSet<_> = map.values().cloned().collect();
        assert!(names.contains("Thing"));
        assert!(names.contains("Thing2"));
    }

    #[test]
    fn dependent_resolves_after_parent() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "parent");
        ns.depend(&2u32, &1u32, "child");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Parent");
        assert_eq!(map[&2], "ParentChild");
    }

    #[test]
    fn dependent_chain() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "root");
        ns.depend(&2u32, &1u32, "mid");
        ns.depend(&3u32, &2u32, "leaf");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Root");
        assert_eq!(map[&2], "RootMid");
        assert_eq!(map[&3], "RootMidLeaf");
    }

    #[test]
    fn required_beats_suggested() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "explicit");
        ns.suggest(&1u32, "ignored");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Explicit");
    }

    #[test]
    fn duplicate_required_is_error() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "clash");
        ns.require(&2u32, "clash");
        assert!(matches!(
            ns.finalize(),
            Err(NamingError::DuplicateRequiredName { .. })
        ));
    }

    #[test]
    #[should_panic(expected = "require called twice for the same id")]
    fn require_twice_same_id_panics() {
        let mut ns = Namespace::<u32>::default();
        ns.require(&1u32, "foo");
        ns.require(&1u32, "bar");
    }

    #[test]
    fn unknown_parent_is_error() {
        let mut ns = Namespace::default();
        ns.depend(&1u32, &99u32, "x");
        assert!(matches!(
            ns.finalize(),
            Err(NamingError::UnknownParent { .. })
        ));
    }

    // ── harder ───────────────────────────────────────────────────────────────

    #[test]
    fn three_way_suggested_collision() {
        let mut ns = Namespace::default();
        ns.suggest(&1u32, "widget");
        ns.suggest(&2u32, "widget");
        ns.suggest(&3u32, "widget");
        let map = ns.finalize().unwrap();
        let names: std::collections::BTreeSet<_> = map.values().cloned().collect();
        assert_eq!(names.len(), 3);
        assert!(names.contains("Widget"));
        assert!(names.contains("Widget2"));
        assert!(names.contains("Widget3"));
    }

    #[test]
    fn suggested_collision_lower_id_wins_unmangled() {
        // BTreeMap ordering means lower ids are processed first, so id=1
        // should win the clean name.
        let mut ns = Namespace::default();
        ns.suggest(&1u32, "thing");
        ns.suggest(&2u32, "thing");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Thing");
        assert_eq!(map[&2], "Thing2");
    }

    #[test]
    fn suggested_blocked_by_required_gets_suffix() {
        // "Widget" is taken by a Required; the Suggested for id=2 should get
        // a numeric suffix.
        let mut ns = Namespace::default();
        ns.require(&1u32, "widget");
        ns.suggest(&2u32, "widget");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Widget");
        assert_eq!(map[&2], "Widget2");
    }

    #[test]
    fn multiple_suggestions_tries_next_before_mangling() {
        // id=1 wants "Widget" first, then "Gadget". "Widget" is taken.
        // The second suggestion "Gadget" is available and should be preferred
        // over "Widget2".
        let mut ns = Namespace::default();
        ns.require(&99u32, "widget");
        ns.suggest(&1u32, "widget");
        ns.suggest(&1u32, "gadget");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Gadget");
    }

    #[test]
    fn required_blocks_dependents_natural_name() {
        // "ParentChild" is claimed by a Required. The dependent that would
        // naturally derive that name must get a suffix instead.
        let mut ns = Namespace::default();
        ns.require(&1u32, "parent");
        ns.require(&3u32, "parent-child");
        ns.depend(&2u32, &1u32, "child");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Parent");
        assert_eq!(map[&3], "ParentChild");
        assert_eq!(map[&2], "ParentChild2");
    }

    #[test]
    fn two_dependents_same_parent_same_suffix() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "base");
        ns.depend(&2u32, &1u32, "item");
        ns.depend(&3u32, &1u32, "item");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Base");
        let names: std::collections::BTreeSet<_> =
            [&map[&2], &map[&3]].into_iter().cloned().collect();
        assert!(names.contains("BaseItem"));
        assert!(names.contains("BaseItem2"));
    }

    #[test]
    fn fan_out_from_one_parent() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "event");
        for (id, suffix) in [(2u32, "created"), (3, "updated"), (4, "deleted")] {
            ns.depend(&id, &1u32, suffix);
        }
        let map = ns.finalize().unwrap();
        assert_eq!(map[&2], "EventCreated");
        assert_eq!(map[&3], "EventUpdated");
        assert_eq!(map[&4], "EventDeleted");
    }

    #[test]
    fn deep_dependent_chain() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "a");
        ns.depend(&2u32, &1u32, "b");
        ns.depend(&3u32, &2u32, "c");
        ns.depend(&4u32, &3u32, "d");
        ns.depend(&5u32, &4u32, "e");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "A");
        assert_eq!(map[&2], "AB");
        assert_eq!(map[&3], "ABC");
        assert_eq!(map[&4], "ABCD");
        assert_eq!(map[&5], "ABCDE");
    }

    #[test]
    fn dependent_on_suggested_parent() {
        // The parent has no Required name, only a Suggested one. The
        // dependent must wait for the suggested name to be assigned.
        let mut ns = Namespace::default();
        ns.suggest(&1u32, "base");
        ns.depend(&2u32, &1u32, "part");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Base");
        assert_eq!(map[&2], "BasePart");
    }

    #[test]
    fn dependent_on_suggested_parent_with_collision() {
        // Parent suggestion collides; the dependent derives from whatever the
        // parent actually resolved to (the mangled name).
        let mut ns = Namespace::default();
        ns.require(&99u32, "base");
        ns.suggest(&1u32, "base");
        ns.depend(&2u32, &1u32, "part");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Base2");
        assert_eq!(map[&2], "Base2Part");
    }

    #[test]
    fn circular_dependency_is_unresolvable() {
        // A depends on B and B depends on A; neither can resolve.
        let mut ns = Namespace::default();
        ns.depend(&1u32, &2u32, "x");
        ns.depend(&2u32, &1u32, "y");
        assert!(matches!(ns.finalize(), Err(NamingError::Unresolvable(_))));
    }

    #[test]
    fn mixed_all_tiers() {
        // Required wins for id=1. Suggested wins for id=2 (no conflict).
        // Suggested is blocked for id=3 (conflict with Required), gets suffix.
        // Dependent resolves off id=1 for id=4.
        let mut ns = Namespace::default();
        ns.require(&1u32, "alpha");
        ns.suggest(&2u32, "beta");
        ns.require(&5u32, "gamma");
        ns.suggest(&3u32, "gamma");
        ns.depend(&4u32, &1u32, "prime");
        let map = ns.finalize().unwrap();
        assert_eq!(map[&1], "Alpha");
        assert_eq!(map[&2], "Beta");
        assert_eq!(map[&3], "Gamma2");
        assert_eq!(map[&4], "AlphaPrime");
        assert_eq!(map[&5], "Gamma");
    }

    #[test]
    fn naming_error_display() {
        let mut ns = Namespace::default();
        ns.require(&1u32, "clash");
        ns.require(&2u32, "clash");
        let err = ns.finalize().unwrap_err();
        let s = err.to_string();
        assert!(s.contains("Clash"), "{s}");
        assert!(s.contains('1') && s.contains('2'), "{s}");
    }
}
