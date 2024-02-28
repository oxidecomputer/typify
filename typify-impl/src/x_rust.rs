// Copyright 2024 Oxide Computer Company

use schemars::schema::{Metadata, Schema, SchemaObject};
use serde::Deserialize;

use crate::{type_entry::TypeEntry, Name, Result, TypeSpace};

// TODO crate renames?
#[derive(Deserialize)]
struct RustExtension {
    #[serde(rename = "crate")]
    crate_name: String,
    version: String,
    path: String,
    #[serde(default)]
    parameters: Vec<Schema>,
}

impl TypeSpace {
    pub(crate) fn convert_rust_extension(&mut self, schema: &SchemaObject) -> Option<TypeEntry> {
        let xxx = schema.extensions.get("x-rust")?;

        // TODO warn if this fails
        let RustExtension {
            crate_name,
            version,
            path,
            parameters,
        } = serde_json::from_value(xxx.clone()).ok()?;

        // Do the crate and version check
        // TODO

        let crate_name = crate_name.replace("-", "_");

        assert!(path.starts_with("::"));

        let type_name = format!("{}{}", crate_name, path);

        let zzz = parameters
            .iter()
            .map(|p_schema| {
                let lll = self.id_for_schema(Name::Unknown, p_schema)?;
                Ok(lll.0)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntry::new_native_params(type_name, &zzz))
    }
}
