// Copyright 2024 Oxide Computer Company

use log::debug;
use schemars::schema::{Schema, SchemaObject};
use serde::Deserialize;

use crate::{type_entry::TypeEntry, CrateVers, Name, Result, TypeSpace};

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
        let x_rust = schema.extensions.get(RUST_TYPE_EXTENSION)?;

        // TODO warn if this fails
        let RustExtension {
            crate_name,
            version,
            path,
            parameters,
        } = serde_json::from_value(x_rust.clone()).ok()?;

        let Ok(req) = semver::VersionReq::parse(&version) else {
            debug!(
                "{} contains an invalid version",
                serde_json::to_string_pretty(&schema).unwrap(),
            );
            return None;
        };

        let crate_ident = crate_name.replace('-', "_");
        let path_sep = path.find("::")?;
        if crate_ident != path[..path_sep] {
            debug!(
                "{} path doesn't start with crate name",
                serde_json::to_string_pretty(&schema).unwrap(),
            );
            return None;
        }

        // First look for the specific crate name; failing that get the
        // wildcard crate '*'.
        let crate_spec = {
            let crate_name: &str = &crate_name;
            self.settings
                .crates
                .get(crate_name)
                .or_else(|| self.settings.crates.get("*"))
        }?;

        // The version must be non-Never and match the requirements from the
        // extension.
        match &crate_spec.version {
            CrateVers::Any => (),
            CrateVers::Version(version) if req.matches(version) => (),
            _ => return None,
        }

        // Replace the initial path component with the new crate name.
        let path = if let Some(new_crate) = &crate_spec.rename {
            format!("{}{}", new_crate.replace('-', "_"), &path[path_sep..])
        } else {
            path
        };

        // Convert and collect type parameters.
        let param_ids = parameters
            .iter()
            .map(|p_schema| {
                let (param_id, _) = self.id_for_schema(Name::Unknown, p_schema)?;
                Ok(param_id)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntry::new_native_params(path, &param_ids))
    }
}
