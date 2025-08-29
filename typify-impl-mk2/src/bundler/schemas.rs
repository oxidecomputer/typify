use std::{collections::BTreeMap, marker::PhantomData};

use crate::bundler::{Document, Error, SchemaKind};

pub struct Schemas {
    schemas: BTreeMap<String, Box<dyn SchemaDispatch>>,
}

impl Schemas {
    pub fn new() -> Self {
        let schemas = Default::default();
        Self { schemas }
    }

    pub fn get(&self, id: &str) -> Option<&Box<dyn SchemaDispatch>> {
        self.schemas.get(id)
    }
}

pub trait SchemaDispatch {
    fn make_document(&self, value: serde_json::Value) -> Result<Document, Error>;
    fn populate_document(&self, document: &mut Document);
}

pub struct SchemaDispatchImpl<T> {
    _marker: PhantomData<T>,
}

// impl<T: SchemaKind> SchemaDispatch for SchemaDispatchImpl<T> {}
