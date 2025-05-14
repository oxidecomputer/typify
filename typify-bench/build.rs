//! Generate a type module in OUT_DIR for every schema document found in benches/*.json. Generated sources can
//! be included directly in benchmark suites.

use std::{env, fs, io, path::PathBuf};

use glob::glob;
use schemars::schema::RootSchema;
use typify::TypeSpace;

fn main() -> io::Result<()> {
    let package_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let mut type_space = TypeSpace::default();

    for entry in glob(&format!("{package_dir}/benches/*.json")).unwrap() {
        let schema_src = entry.unwrap();
        let schema_name = schema_src.file_stem().unwrap().to_string_lossy();

        let reader = io::BufReader::new(fs::File::open(&schema_src)?);
        let schema: RootSchema = serde_json::from_reader(reader)?;

        type_space.add_root_schema(schema).unwrap();
        let module_code = rustfmt_wrapper::rustfmt(type_space.to_stream()).unwrap();

        let type_module =  out_dir.join(format!("{schema_name}.rs"));
        fs::create_dir_all(type_module.parent().unwrap())?;
        fs::write(&type_module, module_code)?;

        println!("cargo::rerun-if-changed=benches/{schema_name}.json");
    }
    Ok(())
}