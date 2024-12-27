// Copyright 2022 Oxide Computer Company

use schemars::schema::SchemaObject;

use crate::{type_entry::TypeEntry, TypeSpaceImpl};

// TODO Everything about this is inefficient.

#[derive(Debug, Default)]
pub(crate) struct SchemaCache {
    schemas: Vec<(SchemaObject, TypeEntry)>,
}

impl SchemaCache {
    pub fn insert(&mut self, schema: &SchemaObject, type_name: &String, impls: &[TypeSpaceImpl]) {
        let type_entry = TypeEntry::new_native(type_name, impls);
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
            .filter(|(schema, _)| &search_schema == schema)
            .map(|(_, type_entry)| type_entry.clone())
            .next()
    }
}
