use std::collections::BTreeMap;

use crate::{
    convert::{ConvertResult, Converter},
    normalizer::{NormalizedMetadata, NormalizedSchemaArray},
    schemalet::SchemaRef,
    typespace::{NameBuilder, NameBuilderHint, Type, TypeNewtypeConstraints, TypeNewtypeStruct},
};

impl Converter {
    pub(crate) fn convert_array(
        &self,
        id: &SchemaRef,
        name: NameBuilder,
        metadata: &NormalizedMetadata,
        array: &NormalizedSchemaArray,
    ) -> ConvertResult {
        match array {
            // Tuple
            //
            // A vanilla, no-nonsense tuple has a fixed number of items (min
            // and max are equal). We take the first N items from `prefixItems`
            // (nee `items` prior to JSON Schema 2020-12) and any additional
            // items from `items` (or `additionalItems` prior to JSON Schema
            // 2020-12). Note that our canonical form mimics the simpler,
            // modern, backward-incompatible 2020-12+ format.
            // TODO 1/10/2026
            // If we just have items and prefix_items is None/empty we should
            // produce a fixed length array type; when prefix_items is
            // non-empty, we should produce a tuple.
            NormalizedSchemaArray {
                items,
                prefix_items,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: false,
            } if max_items == min_items && *max_items > 0 => {
                // TODO 11/14/2025
                // One thing I'm not sure about is tuple-like structs i.e.
                // named tuple types.
                // TODO 1/10/2026
                // I'm not sure what I was unsure about. Certainly we need to
                // make a decision somewhere to turn tuples we really care
                // about into named, struct tuples. That could be here (if we
                // could infer that this is an important type) or it could be
                // in the caller who would convert a tuple into a tuple struct.
                // In other cases we **need** to use a tuple struct because we
                // need a custom serialization (e.g. flattened sequences).

                let types = prefix_items
                    .iter()
                    .map(|item_id| self.resolve_and_get_stuff(item_id).id.clone())
                    .chain(std::iter::repeat_with(|| {
                        self.resolve_and_get_stuff(items).id.clone()
                    }))
                    .take(*max_items as usize)
                    .collect::<Vec<_>>();

                Type::Tuple(types).into()
            }

            // Tuple
            //
            // A tuple with a variable-length "rest" component.
            //
            // TODO 1/10/2026 note that we're pretty restrictive here at the
            // moment, but we can get fancier in the future.
            NormalizedSchemaArray {
                items,
                prefix_items,
                max_items,
                min_items: Some(min_items),
                unique_items: false,
            } if !prefix_items.is_empty() && prefix_items.len() == *min_items as usize => {
                assert!(*min_items <= prefix_items.len() as u64);

                let types = prefix_items
                    .iter()
                    .map(|item_id| self.resolve_and_get_stuff(item_id).id.clone())
                    .collect::<Vec<_>>();

                let inner_name = NameBuilder::Hints(vec![NameBuilderHint::Parent(
                    id.clone(),
                    "rest".to_string(),
                )]);
                let inner_metadata = NormalizedMetadata::default();
                let inner_array = NormalizedSchemaArray {
                    items: items.clone(),
                    prefix_items: vec![],
                    max_items: max_items.as_ref().map(|v| v - *min_items),
                    min_items: None,
                    unique_items: false,
                };

                let inner_id = SchemaRef::Child(Box::new(id.clone()), "@inner".to_string());

                let ConvertResult {
                    primary: inner_ty,
                    additional,
                } = self.convert_array(&inner_id, inner_name, &inner_metadata, &inner_array);

                // TODO 3/7/2026
                // This assertion appears to be invalid. For example, if the
                // "rest" field has length constraints (or contains
                // constraints), we would expect there to be more than merely
                // the primary type.
                assert!(additional.is_empty());

                let primary = Type::TupleStruct(crate::typespace::TypeTupleStruct::new(
                    name,
                    metadata.description.clone(),
                    types,
                    Some(inner_id.clone()),
                ));

                ConvertResult {
                    primary,
                    additional: vec![(inner_id, inner_ty)].into_iter().collect(),
                }
            }

            // An unbounded array. Depending on the value of unique_items this
            // is modeled as a Vec (false or absent) or a Set (true).
            // TODO 3/7/2026
            // Note that the proper rendering of a Set requires that the
            // referenced type implement certain traits that we didn't do
            // properly in typify 1
            NormalizedSchemaArray {
                items,
                prefix_items,
                max_items: None,
                min_items: None,
                unique_items,
            } if prefix_items.is_empty() => {
                let item_id = self.resolve_and_get_stuff(items).id.clone();

                let primary = if *unique_items {
                    Type::Set(item_id)
                } else {
                    Type::Vec(item_id)
                };

                primary.into()
            }

            // A constrained array.
            NormalizedSchemaArray {
                items,
                prefix_items,
                max_items,
                min_items,
                unique_items,
            } if prefix_items.is_empty() => {
                // First construct the unconstrained type; we could probably
                // do this more directly, but we'll rely on our implementation
                // above for now.
                let inner_metadata = NormalizedMetadata::default();
                let inner_array = NormalizedSchemaArray {
                    items: items.clone(),
                    prefix_items: vec![],
                    max_items: None,
                    min_items: None,
                    unique_items: *unique_items,
                };

                let inner_id = SchemaRef::Child(Box::new(id.clone()), "@inner".to_string());

                let ConvertResult {
                    primary: inner_ty,
                    mut additional,
                } = self.convert_array(
                    &inner_id,
                    NameBuilder::Fixed("xxx_busted".to_string()),
                    &inner_metadata,
                    &inner_array,
                );

                // Otherwise why are we here?
                assert!(min_items.is_some() || max_items.is_some());

                let constraints = TypeNewtypeConstraints::Array {
                    min: min_items.map(|min| min as usize),
                    max: max_items.map(|max| max as usize),
                };

                let ty = Type::NewtypeStruct(TypeNewtypeStruct::new(
                    name,
                    metadata.description.clone(),
                    None,
                    inner_id.clone(),
                    constraints,
                ));

                additional.insert(inner_id, inner_ty);

                ConvertResult {
                    primary: ty,
                    additional,
                }
            }

            _ => {
                todo!(
                    "unhandled array {}",
                    serde_json::to_string_pretty(array).unwrap(),
                )
            }
        }
    }
}
