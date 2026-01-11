use crate::{
    convert::Converter,
    schemalet::{SchemaRef, SchemaletValueArray},
    typespace::{NameBuilder, Type},
};

impl Converter {
    pub(crate) fn convert_array(
        &self,
        name: NameBuilder,
        metadata: &crate::schemalet::SchemaletMetadata,
        array: &SchemaletValueArray,
    ) -> Type {
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

                Type::Tuple(types)
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
                max_items: None,
                min_items: Some(min_items),
                unique_items: None,
            } if prefix_items.len() == *min_items as usize => {
                let types = prefix_items
                    .iter()
                    .map(|item_id| self.resolve_and_get_stuff(item_id).id.clone())
                    .collect::<Vec<_>>();
                todo!()
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
                Type::Vec(id)
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
