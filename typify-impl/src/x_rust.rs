// Copyright 2024 Oxide Computer Company

use log::debug;
use schemars::schema::{Schema, SchemaObject};
use serde::Deserialize;

use crate::{type_entry::TypeEntry, Name, Result, TypeSpace};

const RUST_TYPE_EXTENSION: &str = "x-rust-type";

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
        let xxx = schema.extensions.get(RUST_TYPE_EXTENSION)?;

        // TODO warn if this fails
        let RustExtension {
            crate_name,
            version,
            path,
            parameters,
        } = serde_json::from_value(xxx.clone()).ok()?;

        let Ok(req) = semver::VersionReq::parse(&version) else {
            debug!(
                "{} contains an invalid version",
                serde_json::to_string_pretty(&schema).unwrap(),
            );
            return None;
        };

        let crate_vers = self.settings.crates.get(&crate_name)?;
        if !req.matches(crate_vers) {
            return None;
        }

        // Do the crate and version check
        // TODO

        let zzz = parameters
            .iter()
            .map(|p_schema| {
                let lll = self.id_for_schema(Name::Unknown, p_schema)?;
                Ok(lll.0)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntry::new_native_params(path, &zzz))
    }
}
