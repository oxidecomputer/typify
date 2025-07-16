use quote::format_ident;
use unicode_ident::is_xid_continue;

use crate::{
    convert::{Converter, GottenStuff},
    schemalet::{SchemaRef, SchemaletMetadata, SchemaletValueObject},
    typespace::{
        NameBuilder, StructProperty, StructPropertySerde, StructPropertyState, Type, TypeStruct,
    },
};

impl Converter {
    pub(crate) fn convert_object(
        &self,
        name: NameBuilder,
        metadata: &SchemaletMetadata,
        object: &SchemaletValueObject,
    ) -> Type {
        // TODO 6/30/2025
        // Increasingly I'm of the opinion I need to do the conversion from the
        // JSON Schema style object into my new, "structural" encoding.
        match object {
            SchemaletValueObject {
                properties,
                required,
                additional_properties: None,
                property_names: None,
                pattern_properties: None,
            } => {
                let prop_names = properties
                    .keys()
                    .map(|prop_name| tmp_sanitize(prop_name))
                    .collect::<Vec<_>>();

                let properties = properties
                    .iter()
                    .zip(prop_names)
                    .map(|((prop_name, prop_id), new_prop_name)| {
                        let GottenStuff {
                            id,
                            schemalet: _,
                            description,
                            title: _,
                        } = self.resolve_and_get_stuff(prop_id);

                        let rust_name = format_ident!("{new_prop_name}");
                        let json_name = if *prop_name == new_prop_name {
                            StructPropertySerde::None
                        } else {
                            StructPropertySerde::Rename(prop_name.clone())
                        };
                        StructProperty::new(
                            rust_name,
                            json_name,
                            // TODO need to figure this out
                            StructPropertyState::Optional,
                            // TODO maybe a helper to pull out descriptions for property meta?
                            description,
                            id.clone(),
                        )
                    })
                    .collect();

                Type::Struct(TypeStruct::new(
                    name,
                    metadata.description.clone(),
                    None,
                    properties,
                    false,
                ))
            }

            SchemaletValueObject {
                properties,
                required,
                additional_properties: Some(additional_properties),
                property_names: None,
                pattern_properties: None,
            } if properties.is_empty() && required.is_empty() => {
                // TODO not sure what to do here...
                let key_id = SchemaRef::Internal("string".to_string());
                let GottenStuff {
                    id,
                    schemalet: _,
                    description: _,
                    title: _,
                } = self.resolve_and_get_stuff(additional_properties);

                Type::Map(key_id.clone(), id.clone())
            }

            _ => todo!(
                "unhandled object {}",
                serde_json::to_string_pretty(object).unwrap()
            ),
        }
    }
}

fn tmp_sanitize(prop_name: &str) -> String {
    use heck::ToSnakeCase;

    let x = prop_name.replace(|ch| !is_xid_continue(ch), "-");

    let mut out = x.to_snake_case();

    if syn::parse_str::<syn::Ident>(&out).is_err() {
        out.push('_');
    }

    out
}
