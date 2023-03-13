use std::path::PathBuf;

use cargo_typify::{convert, CliArgs};
use clap::Parser;

use color_eyre::eyre::{Context, Result};

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Typify(CliArgs),
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = CargoCli::parse();
    let CargoCli::Typify(args) = cli;

    let contents = convert(&args).wrap_err("Failed to convert JSON Schema to Rust code")?;

    let output_path = match &args.output {
        Some(output_path) => {
            if output_path == &PathBuf::from("-") {
                None
            } else {
                Some(output_path.clone())
            }
        }
        None => {
            let mut output = args.input.clone();
            output.set_extension("rs");
            Some(output)
        }
    };

    if let Some(output_path) = &output_path {
        std::fs::write(output_path, contents).wrap_err_with(|| {
            format!("Failed to write output to file: {}", output_path.display())
        })?;
    } else {
        print!("{}", contents);
    }

    Ok(())
}
