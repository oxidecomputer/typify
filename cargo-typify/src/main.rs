use std::path::PathBuf;

use cargo_typify::{convert, Args};
use clap::Parser;

use color_eyre::eyre::{Context, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

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
            dbg!(&output);
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
