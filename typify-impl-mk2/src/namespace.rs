use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    marker::PhantomData,
    rc::Rc,
};

use heck::ToPascalCase;

#[derive(Debug)]
pub enum Error<Id> {
    MissingHints(Vec<Id>),
}

trait NamespaceStateSealed {}
#[expect(private_bounds)]
pub trait NamespaceState: NamespaceStateSealed {}

pub enum NamespaceOpen {}
impl NamespaceStateSealed for NamespaceOpen {}
impl NamespaceState for NamespaceOpen {}

pub enum NamespaceFinalized {}
impl NamespaceStateSealed for NamespaceFinalized {}
impl NamespaceState for NamespaceFinalized {}

pub struct Namespace<Id, S: NamespaceState = NamespaceOpen> {
    names: BTreeMap<Id, Rc<RefCell<NameInner<Id>>>>,
    state: PhantomData<S>,
}

impl<Id> Default for Namespace<Id, NamespaceOpen> {
    fn default() -> Self {
        Self {
            names: Default::default(),
            state: Default::default(),
        }
    }
}

impl<Id> Namespace<Id, NamespaceOpen>
where
    Id: Ord + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn finalize(self) -> Result<Namespace<Id, NamespaceFinalized>, Error<Id>> {
        let Self { names, .. } = self;

        println!("finalizing {:#?}", names);

        // let mut resolved = BTreeMap::new();
        let mut resolved_names = BTreeSet::<String>::new();
        let mut resolved_ids = BTreeSet::<Id>::new();
        let mut id_to_children = BTreeMap::<Id, Vec<Id>>::new();

        // Make sure that all names have some sort of hint
        for (id, name) in &names {
            let xxx = name.borrow();
            let NameInner::Pending { id: _, hints } = &*xxx else {
                panic!()
            };

            let hints = hints.iter().cloned().collect::<Vec<_>>();
            drop(xxx);

            for hint in &hints {
                match hint {
                    NameInnerHint::Fixed(s) => {
                        assert!(!resolved_names.contains(s));
                        name.replace(NameInner::Resolved(s.to_pascal_case()));
                        resolved_names.insert(s.clone());
                        resolved_ids.insert(id.clone());
                        break;
                    }
                    NameInnerHint::Derive {
                        parent,
                        addition: _,
                    } => {
                        assert!(names.contains_key(parent));
                        id_to_children
                            .entry(parent.clone())
                            .or_default()
                            .push(id.clone());
                    }
                }
            }

            assert!(!hints.is_empty());
        }

        // Start with the direct dependants of the names that we've initially
        // resolved.
        let mut work = resolved_ids
            .iter()
            .inspect(|parent_id| println!("{parent_id}"))
            .filter_map(|parent_id| id_to_children.get(parent_id))
            .flatten()
            .collect::<VecDeque<_>>();

        while let Some(id) = work.pop_front() {
            if resolved_ids.contains(id) {
                continue;
            }

            println!("id {id}");

            let name = names.get(id).unwrap();
            let xxx = name.borrow();
            let NameInner::Pending { id: _, hints } = &*xxx else {
                // Cycle?
                panic!()
            };

            println!("{:#?}", xxx);

            let hints = hints.iter().cloned().collect::<Vec<_>>();
            drop(xxx);
            let mut any = false;

            for hint in hints {
                println!("{:#?}", resolved_names);
                let NameInnerHint::Derive { parent, addition } = &hint else {
                    continue;
                };

                let parent_name = names.get(parent).unwrap().borrow().as_resolved();

                let new_name = format!("{}{}", parent_name, addition);
                let new_name = new_name.to_pascal_case();

                if !resolved_names.contains(&new_name) {
                    resolved_names.insert(new_name.clone());
                    name.replace(NameInner::Resolved(new_name));

                    if let Some(children) = id_to_children.get(id) {
                        work.extend(children);
                    }
                    any = true;
                    break;
                }
            }

            assert!(any);

            resolved_ids.insert(id.clone());
        }

        for (id, name) in &names {
            let NameInner::Resolved(s) = &*name.borrow() else {
                panic!("not all names were resolved {}", id);
            };

            println!("{id} => {s}");
        }

        Ok(Namespace {
            names,
            state: PhantomData,
        })
    }

    pub fn make_name(&mut self, id: Id) -> Name<Id> {
        let inner = Rc::new(RefCell::new(NameInner::Pending {
            id: id.clone(),
            hints: Default::default(),
        }));
        self.names.insert(id.clone(), inner.clone());
        Name { id, inner }
    }
}

#[derive(Debug)]
enum NameInner<Id> {
    Pending {
        id: Id,
        hints: Vec<NameInnerHint<Id>>,
    },
    Resolved(String),
}
#[derive(Debug, Clone)]
enum NameInnerHint<Id> {
    Fixed(String),
    Derive { parent: Id, addition: String },
}

impl<Id> NameInner<Id> {
    fn as_resolved(&self) -> String {
        let NameInner::Resolved(s) = self else {
            panic!("The Namespace to which this belongs has not been finalized")
        };
        s.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Name<Id> {
    id: Id,
    inner: Rc<RefCell<NameInner<Id>>>,
}

impl<Id> Name<Id>
where
    Id: Clone,
{
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Get the name as a string; requires that the associated Namespace has
    /// been finalized, otherwise this function will panic.
    pub fn to_string(&self) -> String {
        self.inner.try_borrow().unwrap().as_resolved()
    }

    pub fn set_name(&self, s: impl AsRef<str>) {
        let NameInner::Pending { hints, .. } = &mut *self.inner.borrow_mut() else {
            panic!()
        };

        hints.push(NameInnerHint::Fixed(s.as_ref().to_string()))
    }

    pub fn suggest_name(&self, s: impl AsRef<str>) {
        todo!();
    }

    pub fn derive_name(&self, parent_id: &Id, addition: impl AsRef<str>) {
        let NameInner::Pending { hints, .. } = &mut *self.inner.borrow_mut() else {
            panic!()
        };

        hints.push(NameInnerHint::Derive {
            parent: parent_id.clone(),
            addition: addition.as_ref().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::namespace::Namespace;

    #[derive(Default)]
    struct IdMaker(usize);

    impl IdMaker {
        pub fn next(&mut self) -> usize {
            self.0 += 1;
            self.0
        }
    }

    #[test]
    fn test_naming() {
        let mut id_maker = IdMaker::default();
        let mut namespace = Namespace::default();

        let root = namespace.make_name(id_maker.next());
        let kid1 = namespace.make_name(id_maker.next());
        let kid2 = namespace.make_name(id_maker.next());
        let grandkid = namespace.make_name(id_maker.next());
        let name_holder = vec![&root, &kid1, &kid2, &grandkid];

        root.set_name("root-name");

        kid1.derive_name(root.id(), "kid1");
        kid2.derive_name(root.id(), "kid2");

        grandkid.derive_name(kid2.id(), "child");

        let namespace = namespace.finalize();

        for name in name_holder {
            println!("{}", name.to_string())
        }

        panic!()
    }
}
