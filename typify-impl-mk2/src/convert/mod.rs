mod array;
mod object;
mod one_of;

use std::collections::BTreeMap;

use crate::{
    schemalet::{CanonicalSchemalet, CanonicalSchemaletDetails, SchemaRef, SchemaletValue},
    typespace::{NameBuilder, Type},
};

// TODO naming?
pub struct Converter {
    graph: BTreeMap<SchemaRef, CanonicalSchemalet>,
    known_names: BTreeMap<SchemaRef, String>,
}

impl Converter {
    pub fn new(graph: BTreeMap<SchemaRef, CanonicalSchemalet>) -> Self {
        Self {
            graph,
            known_names: Default::default(),
        }
    }

    pub fn set_name(&mut self, id: SchemaRef, name: String) {
        self.known_names.insert(id, name);
    }

    fn get<'a>(&'a self, id: &SchemaRef) -> &'a CanonicalSchemalet {
        self.graph
            .get(id)
            .unwrap_or_else(|| panic!("failed to lookup {id}"))
    }

    fn resolve<'a>(&'a self, mut id: &'a SchemaRef) -> &'a CanonicalSchemalet {
        loop {
            let schemalet = self.get(id);
            if let CanonicalSchemaletDetails::Reference(schema_ref) = &schemalet.details {
                id = schema_ref;
            } else {
                break schemalet;
            }
        }
    }

    pub fn resolve_and_get_stuff<'a>(&'a self, mut id: &'a SchemaRef) -> GottenStuff<'a> {
        let mut title = None;
        let mut description = None;
        loop {
            let schemalet = self.get(id);

            let CanonicalSchemaletDetails::Reference(next_id) = &schemalet.details else {
                return GottenStuff {
                    id,
                    schemalet,
                    description,
                    title,
                };
            };

            if let (None, Some(new_title)) = (&title, &schemalet.metadata.title) {
                title = Some(new_title.clone());
            }
            if let (None, Some(new_description)) = (&description, &schemalet.metadata.description) {
                description = Some(new_description.clone());
            }

            id = next_id;
        }
    }

    pub fn convert(&self, id: &SchemaRef) -> Type {
        let name = match self.known_names.get(id) {
            Some(s) => NameBuilder::Fixed(s.clone()),
            None => NameBuilder::Unset,
        };

        let schemalet = self.get(id);
        println!(
            "converting {}",
            serde_json::to_string_pretty(schemalet).unwrap(),
        );
        let CanonicalSchemalet { metadata, details } = schemalet;

        let typ = match details {
            CanonicalSchemaletDetails::Anything => Type::JsonValue,
            CanonicalSchemaletDetails::Nothing => todo!(),
            CanonicalSchemaletDetails::Constant(_) => todo!(),
            CanonicalSchemaletDetails::Reference(schema_ref) => todo!(),
            CanonicalSchemaletDetails::Note(schema_ref) => todo!(),
            CanonicalSchemaletDetails::ExclusiveOneOf { subschemas, .. } => {
                self.convert_one_of(name, metadata, subschemas)
            }

            CanonicalSchemaletDetails::Value(SchemaletValue::Boolean) => Type::Boolean,
            CanonicalSchemaletDetails::Value(SchemaletValue::Array(array)) => {
                self.convert_array(name, metadata, array)
            }
            CanonicalSchemaletDetails::Value(SchemaletValue::Object(object)) => {
                self.convert_object(name, metadata, object)
            }
            CanonicalSchemaletDetails::Value(SchemaletValue::String { pattern, format }) => {
                self.convert_string(name, metadata, pattern.as_ref(), format.as_ref())
            }
            CanonicalSchemaletDetails::Value(SchemaletValue::Integer {
                minimum,
                exclusive_minimum,
            }) => {
                // TODO not handling this well ...
                Type::Float("i64".to_string())
            }
            CanonicalSchemaletDetails::Value(SchemaletValue::Number {
                minimum,
                exclusive_minimum,
            }) => {
                // TODO not handling this well ...
                Type::Float("f64".to_string())
            }
            CanonicalSchemaletDetails::Value(SchemaletValue::Null) => todo!(),
        };

        typ
    }

    fn convert_string(
        &self,
        name: NameBuilder,
        metadata: &crate::schemalet::SchemaletMetadata,
        pattern: Option<&String>,
        format: Option<&String>,
    ) -> Type {
        match (pattern, format) {
            (_, _) => Type::String,
            // _ => panic!("{:?} {:?}", pattern, format),
        }
    }
}

pub struct GottenStuff<'a> {
    id: &'a SchemaRef,
    schemalet: &'a CanonicalSchemalet,
    description: Option<String>,
    title: Option<String>,
}
