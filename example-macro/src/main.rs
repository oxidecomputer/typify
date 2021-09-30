use typify::import_types;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Veggie(String);

import_types!("./example.json");

fn main() {
    println!("here we are");
}
