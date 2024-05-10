// Copyright 2024 Oxide Computer Company

//! cargo command to generate Rust code from a JSON Schema.

#![deny(missing_docs)]

use std::{path::PathBuf, str::FromStr};

use clap::{ArgGroup, Args};
use color_eyre::eyre::{Context, Result};
use typify::{CrateVers, TypeSpace, TypeSpaceSettings};

/// A CLI for the `typify` crate that converts JSON Schema files to Rust code.
#[derive(Args)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("build")
        .args(["builder", "no_builder"]),
))]
pub struct CliArgs {
    /// The input file to read from
    pub input: PathBuf,

    /// Whether to include a builder-style interface, this is the default.
    #[arg(short, long, default_value = "false", group = "build")]
    pub builder: bool,

    /// Inverse of `--builder`. When set the builder-style interface will not
    /// be included.
    #[arg(short = 'B', long, default_value = "false", group = "build")]
    pub no_builder: bool,

    /// Add an additional derive macro to apply to all defined types.
    #[arg(short, long = "additional-derive", value_name = "derive")]
    pub additional_derives: Vec<String>,

    /// The output file to write to. If not specified, the input file name will
    /// be used with a `.rs` extension.
    ///
    /// If `-` is specified, the output will be written to stdout.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Specify each crate@version that can be assumed to be in use for types
    /// found in the schema with the x-rust-type extension.
    #[arg(long = "crate")]
    crates: Vec<CrateSpec>,
}

impl CliArgs {
    /// Output path.
    pub fn output_path(&self) -> Option<PathBuf> {
        match &self.output {
            Some(output_path) => {
                if output_path == &PathBuf::from("-") {
                    None
                } else {
                    Some(output_path.clone())
                }
            }
            None => {
                let mut output = self.input.clone();
                output.set_extension("rs");
                Some(output)
            }
        }
    }

    /// Whether builder-style interface was selected.
    pub fn use_builder(&self) -> bool {
        !self.no_builder
    }
}

#[derive(Debug, Clone)]
struct CrateSpec {
    name: String,
    version: CrateVers,
    rename: Option<String>,
}

impl FromStr for CrateSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn is_crate(s: &str) -> bool {
            !s.contains(|cc: char| !cc.is_alphabetic() && cc != '-' && cc != '_')
        }

        fn convert(s: &str) -> Option<CrateSpec> {
            let (rename, s) = if let Some(ii) = s.find('=') {
                let rename = &s[..ii];
                let rest = &s[ii + 1..];
                if !is_crate(rename) {
                    return None;
                }
                (Some(rename.to_string()), rest)
            } else {
                (None, s)
            };

            let ii = s.find('@')?;
            let crate_str = &s[..ii];
            let vers_str = &s[ii + 1..];

            if crate_str.contains(|cc: char| !cc.is_alphanumeric() && cc != '_' && cc != '-') {
                return None;
            }
            let version = CrateVers::parse(vers_str)?;

            Some(CrateSpec {
                name: crate_str.to_string(),
                version,
                rename,
            })
        }

        convert(s).ok_or("crate specifier must be of the form 'cratename@version'")
    }
}

/// Generate Rust code for the selected JSON Schema.
pub fn convert(args: &CliArgs) -> Result<String> {
    let content = std::fs::read_to_string(&args.input)
        .wrap_err_with(|| format!("Failed to open input file: {}", &args.input.display()))?;

    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)
        .wrap_err("Failed to parse input file as JSON Schema")?;

    let mut settings = &mut TypeSpaceSettings::default();
    settings = settings.with_struct_builder(args.use_builder());

    for derive in &args.additional_derives {
        settings = settings.with_derive(derive.clone());
    }

    for CrateSpec {
        name,
        version,
        rename,
    } in &args.crates
    {
        settings = settings.with_crate(name, version.clone(), rename.as_ref());
    }

    let mut type_space = TypeSpace::new(settings);
    type_space
        .add_root_schema(schema)
        .wrap_err("Schema conversion failed")?;

    let intro = "#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};
";

    let contents = format!("{intro}\n{}", type_space.to_stream());

    let contents = rustfmt_wrapper::rustfmt(contents).wrap_err("Failed to format Rust code")?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_parsing_stdout() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: Some(PathBuf::from("-")),
            no_builder: false,
            crates: vec![],
        };

        assert_eq!(args.output_path(), None);
    }

    #[test]
    fn test_output_parsing_file() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: Some(PathBuf::from("some_file.rs")),
            no_builder: false,
            crates: vec![],
        };

        assert_eq!(args.output_path(), Some(PathBuf::from("some_file.rs")));
    }

    #[test]
    fn test_output_parsing_default() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: None,
            no_builder: false,
            crates: vec![],
        };

        assert_eq!(args.output_path(), Some(PathBuf::from("input.rs")));
    }

    #[test]
    fn test_builder_as_default_style() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: None,
            no_builder: false,
            crates: vec![],
        };

        assert!(args.use_builder());
    }

    #[test]
    fn test_no_builder() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: None,
            no_builder: true,
            crates: vec![],
        };

        assert!(!args.use_builder());
    }

    #[test]
    fn test_builder_opt_in() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: true,
            additional_derives: vec![],
            output: None,
            no_builder: false,
            crates: vec![],
        };

        assert!(args.use_builder());
    }
}
