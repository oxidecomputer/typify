use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{Context, Result};
use schemars::schema::Schema;
use typify::{TypeSpace, TypeSpaceSettings};

/// A CLI for the `typify` crate that converts JSON Schema files to Rust code.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The input file to read from
    pub input: PathBuf,

    /// The output file to write to. If not specified, the output will be written to stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn convert(args: &Args) -> Result<String> {
    let content = std::fs::read_to_string(&args.input)
        .wrap_err_with(|| format!("Failed to open input file: {}", &args.input.display()))?;

    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)
        .wrap_err("Failed to parse input file as JSON Schema")?;

    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space
        .add_ref_types(schema.definitions)
        .wrap_err("Could not add ref types from the 'definitions' field in the JSON Schema")?;

    let base_type = &schema.schema;

    // Only convert the top-level type if it has a name
    if let Some(base_title) = &(|| base_type.metadata.as_ref()?.title.as_ref())() {
        let base_title = base_title.to_string();

        type_space
            // TODO: Fix this to use add_type_with_name
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

    use expectorate::assert_contents;

    #[test]
    fn test_simple() {
        let input = concat!(env!("CARGO_MANIFEST_DIR"), "/../example.json");
        let args = Args {
            input: input.into(),
            output: None,
        };

        let contents = convert(&args).unwrap();

        assert_contents(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/outputs/simple.rs"),
            &contents,
        );
    }
}
