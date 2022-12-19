// Copyright 2022 Oxide Computer Company

use schemars::schema::SchemaObject;

use crate::type_entry::{TypeEntry, TypeEntryDetails};

// TODO Everything about this is inefficient.

#[derive(Debug, Default)]
pub(crate) struct SchemaCache {
    schemas: Vec<(SchemaObject, TypeEntry)>,
}

impl SchemaCache {
    pub fn insert(&mut self, schema: &SchemaObject, type_name: &String, impls: &[String]) {
        let type_entry = TypeEntry {
            details: TypeEntryDetails::BuiltIn(type_name.into()),
            derives: Default::default(),
            impls: impls.iter().map(ToString::to_string).collect(),
        };
        self.schemas.push((
            SchemaObject {
                metadata: None,
                ..schema.clone()
            },
            type_entry,
        ));
    }

    pub fn lookup(&self, search_schema: &SchemaObject) -> Option<TypeEntry> {
        let search_schema = SchemaObject {
            metadata: None,
            ..search_schema.clone()
        };
        self.schemas
            .iter()
            .filter_map(|(schema, type_entry)| {
                (&search_schema == schema).then(|| type_entry.clone())
            })
            .next()
    }
}
