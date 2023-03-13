use cargo_typify::{convert, Args};
use clap::Parser;

use color_eyre::eyre::{Context, Result};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let contents = convert(&args).wrap_err("Failed to convert JSON Schema to Rust code")?;

    if let Some(output_path) = &args.output {
        std::fs::write(output_path, contents).wrap_err_with(|| {
            format!("Failed to write output to file: {}", output_path.display())
        })?;
    } else {
        print!("{}", contents);
    }

    Ok(())
}
