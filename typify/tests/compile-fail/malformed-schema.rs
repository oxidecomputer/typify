use typify::import_types;

// Cargo.toml exists in trybuild's generated crate but is not valid JSON.
import_types!("Cargo.toml");

fn main() {}
