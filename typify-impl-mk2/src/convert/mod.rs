mod array;
mod object;
mod one_of;

use std::collections::BTreeMap;

use crate::{
    schemalet::{
        CanonicalSchemalet, CanonicalSchemaletDetails, SchemaRef, SchemaletValue,
        SchemaletValueString,
    },
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
            "converting {id} {}",
            serde_json::to_string_pretty(schemalet).unwrap(),
        );
        let CanonicalSchemalet { metadata, details } = schemalet;

        let typ = match details {
            CanonicalSchemaletDetails::Anything => Type::JsonValue,
            CanonicalSchemaletDetails::Nothing => todo!(),
            CanonicalSchemaletDetails::Constant(_) => todo!(),
            CanonicalSchemaletDetails::Reference(schema_ref) => self.convert(schema_ref),

            // TODO 7/28/2025
            // This is probably wrong, but I'm not sure exactly how we're going
            // to use this Note thing.
            CanonicalSchemaletDetails::Note(schema_ref) => self.convert(schema_ref),

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
            CanonicalSchemaletDetails::Value(SchemaletValue::String(string_value)) => {
                self.convert_string(name, metadata, string_value)
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
                maximum,
                exclusive_maximum,
                multiple_of,
            }) => {
                // TODO not handling this well ...

                // TODO 7/21/2025
                // The plan here needs to be to generate a wrapper type that
                // applies any numerical constraints.
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
        string_value: &SchemaletValueString,
    ) -> Type {
        let SchemaletValueString {
            pattern,
            format,
            min_length,
            max_length,
        } = string_value;

        // The format of a string may let us infer a useful type to use for
        // this schema. Multiple formats may be present e.g. if multiple
        // schemas with formats were merged as a result of an `allOf`, however
        // multiple formats are not generally something that's satisfiable.
        let ty = if !format.is_empty() {
            assert_eq!(format.len(), 1);

            let format_str = format.iter().next().unwrap().as_str();
            match format_str {
                "uri" => Type::Native("::url::Url".to_string()),
                "uri-reference" => Type::Native("::url::Url".to_string()),
                _ => Type::String,
            }
        } else {
            Type::String
        };

        ty

        // If we recognize the format, we can use a specific Rust type to model
        // the value.
        // TODO 7/27/2025
        // There are two interesting ways we might make this configurable.
        // First, we can use the idea of the `crates` configuration that we use
        // for `x-rust-type` to determine which crates we're willing to rely
        // on. I'd imagine that--for crates whose types are used below--we
        // would include some by default.
        // We could also provide a way to inject format translations...
        // although we might just provide some more generic conversion or
        // schema pattern matching facility.
        // Both of these feel like settings on conversion rather than, say,
        // settings on the Typespace. But the Typespace should be able to tell
        // us what crates actually get used.
        // let rust_type = match format.map(String::as_str) {
        //     Some("date-time") => Some("::chrono::DateTime<::chrono::offset::Utc>"),
        //     Some("date") => Some("::chrono::naive::NaiveDate"),
        //     Some("time") => Some("::chrono::naive::NaiveTime"),
        //     Some("duration") => Some("::std::time::Duration"),
        //     Some("uuid") => Some("::uuid::Uuid"),
        //     Some("uri") => Some("::url::Url"),
        //     Some("ip") => Some("::std::net::IpAddr"),
        //     Some("ipv4") => Some("::std::net::Ipv4Addr"),
        //     Some("ipv6") => Some("::std::net::Ipv6Addr"),
        //     _ => None,
        // };

        // if let Some(rust_type) = rust_type {
        //     Type::Native(rust_type.to_string())
        // } else {
        //     Type::String
        // }
        // Type::String
    }
}

pub struct GottenStuff<'a> {
    id: &'a SchemaRef,
    schemalet: &'a CanonicalSchemalet,
    description: Option<String>,
    title: Option<String>,
}
