use typify::import_types;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct veggie(String);

import_types!("./example.json");

fn main() {
    println!("here we are");
}
