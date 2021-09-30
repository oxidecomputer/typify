use typify::import_types;

use serde::{Deserialize, Serialize};

import_types!("./example.json");

fn main() {
    println!("here we are");
}
