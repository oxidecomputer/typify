// Copyright 2023 Oxide Computer Company

use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};

use crate::{
    type_entry::{
        TypeEntry, TypeEntryDetails, TypeEntryEnum, TypeEntryNewtype, TypeEntryStruct,
        VariantDetails,
    },
    TypeId, TypeSpace,
};

impl TypeSpace {
    /// We need to root out any containment cycles, breaking them by inserting
    /// a `Box` type. Our choice of *where* to break cycles is more arbitrary
    /// than optimal, but is well beyond sufficient.
    pub fn break_cycles(&mut self, range: Range<u64>) {
        enum Node {
            Start {
                type_id: TypeId,
            },
            Processing {
                type_id: TypeId,
                children_ids: Vec<TypeId>,
            },
        }

        let mut visited = BTreeSet::<TypeId>::new();

        for id in range {
            let type_id = TypeId(id);

            // This isn't strictly necessary, but we'll short-circuit some work
            // by checking this right away.
            if visited.contains(&type_id) {
                continue;
            }

            let mut active = BTreeSet::<TypeId>::new();
            let mut stack = Vec::<Node>::new();

            active.insert(type_id.clone());
            stack.push(Node::Start { type_id });

            while let Some(top) = stack.last_mut() {
                match top {
                    // Skip right to the end since we've already seen this type.
                    Node::Start { type_id } if visited.contains(type_id) => {
                        assert!(active.contains(type_id));

                        let type_id = type_id.clone();
                        *top = Node::Processing {
                            type_id,
                            children_ids: Vec::new(),
                        };
                    }

                    // Break any immediate cycles and queue up this type for
                    // descent into its child types.
                    Node::Start { type_id } => {
                        assert!(active.contains(type_id));

                        visited.insert(type_id.clone());

                        // Determine which child types form cycles--and
                        // therefore need to be snipped--and the rest--into
                        // which we should descend. We make this its own block
                        // to clarify the lifetime of the exclusive reference
                        // to the type. We don't really *need* to have an
                        // exclusive reference here, but there's no point in
                        // writing `get_child_ids` again for shared references.
                        let (snip, descend) = {
                            let type_entry = self.id_to_entry.get_mut(type_id).unwrap();

                            let child_ids = get_child_ids(type_entry)
                                .into_iter()
                                .map(|child_id| child_id.clone());

                            // If the child type is in active then we've found
                            // a cycle (otherwise we'll descend).
                            child_ids.partition::<Vec<_>, _>(|child_id| active.contains(child_id))
                        };

                        // Note that while `snip` might contain duplicates,
                        // `id_to_box` is idempotent insofar as the same input
                        // TypeId will result in the same output TypeId. Ergo
                        // the resulting pairs from which we construct the
                        // mapping would contain exact duplicates; it would not
                        // contain two values associated with the same key.
                        let replace = snip
                            .into_iter()
                            .map(|type_id| {
                                let box_id = self.id_to_box(&type_id);

                                (type_id, box_id)
                            })
                            .collect::<BTreeMap<_, _>>();

                        // Break any cycles by reassigning the child type to a box.
                        let type_entry = self.id_to_entry.get_mut(type_id).unwrap();
                        let child_ids = get_child_ids(type_entry);
                        for child_id in child_ids {
                            if let Some(replace_id) = replace.get(child_id) {
                                *child_id = replace_id.clone();
                            }
                        }

                        // Descend into child types.
                        let node = Node::Processing {
                            type_id: type_id.clone(),
                            children_ids: descend,
                        };
                        *top = node;
                    }

                    // If there are children left, push the next child onto the
                    // stack. If there are none left, pop this type.
                    Node::Processing {
                        type_id,
                        children_ids,
                    } => {
                        if let Some(type_id) = children_ids.pop() {
                            // Descend into the next child node.
                            active.insert(type_id.clone());
                            stack.push(Node::Start { type_id });
                        } else {
                            // All done; remove the item from the active list
                            // and stack.
                            active.remove(type_id);
                            let _ = stack.pop();
                        }
                    }
                }
            }
        }
    }
}

/// For types that could potentially participate in a cycle, return a list of
/// mutable references to the child types.
fn get_child_ids(type_entry: &mut TypeEntry) -> Vec<&mut TypeId> {
    match &mut type_entry.details {
        TypeEntryDetails::Enum(TypeEntryEnum { variants, .. }) => variants
            .iter_mut()
            .flat_map(|variant| match &mut variant.details {
                VariantDetails::Simple => Vec::new(),
                VariantDetails::Item(type_id) => vec![type_id],
                VariantDetails::Tuple(type_ids) => type_ids.iter_mut().collect(),
                VariantDetails::Struct(properties) => properties
                    .iter_mut()
                    .map(|prop| &mut prop.type_id)
                    .collect(),
            })
            .collect::<Vec<_>>(),

        TypeEntryDetails::Struct(TypeEntryStruct { properties, .. }) => properties
            .iter_mut()
            .map(|prop| &mut prop.type_id)
            .collect(),

        TypeEntryDetails::Newtype(TypeEntryNewtype { type_id, .. }) => {
            vec![type_id]
        }

        // Unnamed types that can participate in containment cycles.
        TypeEntryDetails::Option(type_id) => vec![type_id],
        TypeEntryDetails::Array(type_id, _) => vec![type_id],
        TypeEntryDetails::Tuple(type_ids) => type_ids.iter_mut().collect(),

        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use schema::Schema;
    use schemars::JsonSchema;

    use crate::test_util::validate_output;

    #[test]
    fn test_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A {
            a: Box<A>,
        }

        validate_output::<A>();
    }

    #[test]
    fn test_optional_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A {
            a: Option<Box<A>>,
        }

        validate_output::<A>();
    }

    #[test]
    fn test_enum_trivial_cycles() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        enum A {
            Variant0(u64),
            Variant1 {
                a: u64,
                b: Vec<A>,
                rop: Option<Box<A>>,
            },
            Variant2 {
                a: Box<A>,
            },
            Variant3(u64, Box<A>),
            Variant4(Option<Box<A>>, String),
        }

        validate_output::<A>();
    }

    #[test]
    fn test_newtype_trivial_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A(Box<A>);

        validate_output::<A>();
    }

    #[test]
    fn test_abab_cycle() {
        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct A(B);

        #[derive(JsonSchema, Schema)]
        #[allow(dead_code)]
        struct B(Box<A>);

        validate_output::<A>();
    }
}
