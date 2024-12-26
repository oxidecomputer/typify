// Copyright 2024 Oxide Computer Company

use log::warn;
use schemars::schema::{Schema, SchemaObject};
use serde::Deserialize;

use crate::{type_entry::TypeEntry, CrateVers, Name, Result, TypeSpace};

const RUST_TYPE_EXTENSION: &str = "x-rust-type";

/// Definition of the value of the x-rust-type extension. This structure
/// must not change incompatibly (and probably shouldn't change at all).
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

        let Ok(RustExtension {
            crate_name,
            version,
            path,
            parameters,
        }) = serde_json::from_value(x_rust.clone())
        else {
            warn!(
                "{} contains an invalid value for {}",
                serde_json::to_string_pretty(&schema).unwrap(),
                RUST_TYPE_EXTENSION,
            );
            return None;
        };

        let Ok(req) = semver::VersionReq::parse(&version) else {
            warn!(
                "{} contains an invalid version",
                serde_json::to_string_pretty(&schema).unwrap(),
            );
            return None;
        };

        let crate_ident = crate_name.replace('-', "_");
        let path_sep = path.find("::")?;
        if crate_ident != path[..path_sep] {
            warn!(
                "{} path doesn't start with crate name",
                serde_json::to_string_pretty(&schema).unwrap(),
            );
            return None;
        }

        let path = {
            if let Some(crate_spec) = self.settings.crates.get(crate_name.as_str()) {
                // The version must be non-Never and match the requirements
                // from the extension.
                match &crate_spec.version {
                    CrateVers::Any => (),
                    CrateVers::Version(version) if req.matches(version) => (),
                    _ => return None,
                }

                // Replace the initial path component with the new crate name.
                if let Some(new_crate) = &crate_spec.rename {
                    format!("{}{}", new_crate.replace('-', "_"), &path[path_sep..])
                } else {
                    path
                }
            } else {
                match self.settings.unknown_crates {
                    crate::UnknownPolicy::Generate => return None,
                    crate::UnknownPolicy::Allow => path,

                    // TODO need to bubble up a coherent compiler error via the
                    // generated code.
                    crate::UnknownPolicy::Deny => return None,
                }
            }
        };

        // Convert and collect type parameters.
        let param_ids = parameters
            .iter()
            .map(|p_schema| {
                // TODO could we have some reasonable type name? Do we need to?
                let (param_id, _) = self.id_for_schema(Name::Unknown, p_schema)?;
                Ok(param_id)
            })
            .collect::<Result<Vec<_>>>()
            .ok()?;

        Some(TypeEntry::new_native_params(
            format!("::{path}"),
            &param_ids,
        ))
    }
}
