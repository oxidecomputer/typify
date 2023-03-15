use std::path::PathBuf;

use clap::{ArgGroup, Args};
use color_eyre::eyre::{Context, Result};
use schemars::schema::Schema;
use typify::{TypeSpace, TypeSpaceSettings};

/// A CLI for the `typify` crate that converts JSON Schema files to Rust code.
#[derive(Args)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("build")
        .args(["builder", "positional"]),
))]
pub struct CliArgs {
    /// The input file to read from
    pub input: PathBuf,

    /// Whether to include a builder-style interface, this is the default.
    #[arg(short, long, default_value = "false", group = "build")]
    pub builder: bool,

    /// Inverse of `--builder`. When set the builder-style interface will not be included.
    #[arg(short, long, default_value = "false", group = "build")]
    pub positional: bool,

    /// Add an additional derive macro to apply to all defined types.
    #[arg(short, long)]
    pub additional_derives: Vec<String>,

    /// The output file to write to. If not specified, the input file name will be used with a
    /// `.rs` extension.
    ///
    /// If `-` is specified, the output will be written to stdout.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

impl CliArgs {
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

    pub fn use_builder(&self) -> bool {
        !self.positional
    }
}

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

    let mut type_space = TypeSpace::new(settings);
    type_space
        .add_ref_types(schema.definitions)
        .wrap_err("Could not add ref types from the 'definitions' field in the JSON Schema")?;

    let base_type = &schema.schema;

    // Only convert the top-level type if it has a name
    if let Some(base_title) = &(|| base_type.metadata.as_ref()?.title.as_ref())() {
        let base_title = base_title.to_string();

        type_space
            .add_type(&Schema::Object(schema.schema))
            .wrap_err_with(|| {
                format!("Could not add the top level type `{base_title}` to the type space")
            })?;
    }

    let intro = "#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};
";

    let contents = format!("{intro}\n{}", type_space.to_string());

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
            positional: false,
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
            positional: false,
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
            positional: false,
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
            positional: false,
        };

        assert!(args.use_builder());
    }

    #[test]
    fn test_positional_builder() {
        let args = CliArgs {
            input: PathBuf::from("input.json"),
            builder: false,
            additional_derives: vec![],
            output: None,
            positional: true,
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
            positional: false,
        };

        assert!(args.use_builder());
    }
}
