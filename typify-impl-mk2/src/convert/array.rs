use crate::{
    convert::{ConvertResult, Converter},
    schemalet::{SchemaRef, SchemaletMetadata, SchemaletValueArray},
    typespace::{NameBuilder, NameBuilderHint, Type},
};

impl Converter {
    pub(crate) fn convert_array(
        &self,
        id: &SchemaRef,
        name: NameBuilder,
        metadata: &SchemaletMetadata,
        array: &SchemaletValueArray,
    ) -> ConvertResult {
        match array {
            // Tuple
            //
            // A vanilla, no-nonsense tuple has a fixed number of items (min
            // and max are equal). We take the first N items from `prefixItems`
            // (or `items` prior to JSON Schema 2020-12) and any additional
            // items from `items` (or `additionalItems` prior to JSON Schema
            // 2020-12). Note that our canonical form mimics the simpler,
            // modern, backward-incompatible 2020-12+ format.
            // TODO 1/10/2026
            // If we just have items and prefix_items is None/empty we should
            // produce a fixed length array type; when prefix_items is
            // non-empty, we should produce a tuple.
            SchemaletValueArray {
                items,
                prefix_items,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
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
                    .flatten()
                    .map(|item_id| self.resolve_and_get_stuff(item_id).id.clone())
                    .chain(std::iter::repeat_with(|| {
                        if let Some(items) = items {
                            self.resolve_and_get_stuff(items).id.clone()
                        } else {
                            SchemaRef::Internal("any".to_string())
                        }
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
            SchemaletValueArray {
                items,
                prefix_items: Some(prefix_items),
                max_items,
                min_items: Some(min_items),
                unique_items: None,
            } if prefix_items.len() == *min_items as usize => {
                assert!(*min_items <= prefix_items.len() as u64);

                let types = prefix_items
                    .iter()
                    .map(|item_id| self.resolve_and_get_stuff(item_id).id.clone())
                    .collect::<Vec<_>>();

                let inner_name = NameBuilder::Hints(vec![NameBuilderHint::Parent(
                    id.clone(),
                    "rest".to_string(),
                )]);
                let inner_metadata = SchemaletMetadata::default();
                let inner_array = SchemaletValueArray {
                    items: items.clone(),
                    prefix_items: None,
                    max_items: max_items.as_ref().map(|v| v - *min_items),
                    min_items: None,
                    unique_items: None,
                };

                // TODO 1.11.2026
                // I really need a way to make a generic partial, but since I don't
                // we're going to hack it up for the moment.

                let inner_id = SchemaRef::Partial(format!("{id}"), "@inner".to_string());

                let ConvertResult {
                    primary: inner_ty,
                    additional,
                } = self.convert_array(&inner_id, inner_name, &inner_metadata, &inner_array);
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

            SchemaletValueArray {
                items,
                prefix_items: None,
                max_items,
                min_items,
                unique_items,
            } => {
                let id = if let Some(items) = items {
                    self.resolve_and_get_stuff(items).id.clone()
                } else {
                    SchemaRef::Internal("any".to_string())
                };
                Type::Vec(id).into()
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
