use crate::{
    convert::{Converter, GottenStuff},
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
            // A vanilla, no-nonsense tuple has a fixed number of items (min
            // and max are equal). We take the first N items from `prefixItems`
            // (or `items` prior to JSON Schema 2020-12) and any additional
            // items from `items` (or `additionalItems` prior to JSON Schema
            // 2020-12). Note that our canonical form mimics the simpler,
            // modern, backward-incompatible 2020-12+ format.
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
