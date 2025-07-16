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
            SchemaletValueArray {
                items,
                prefix_items,
                max_items: Some(max_items),
                min_items: Some(min_items),
                unique_items: None,
            } if max_items == min_items && *max_items > 0 => {
                // TODO
                // This is a tuple type
                todo!()
            }

            SchemaletValueArray {
                items: Some(items),
                prefix_items: None,
                max_items,
                min_items,
                unique_items,
            } => {
                let GottenStuff { id, .. } = self.resolve_and_get_stuff(items);
                Type::Vec(id.clone())
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
