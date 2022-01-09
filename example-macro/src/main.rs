// Copyright 2021 Oxide Computer Company

use typify::import_types;

use serde::{Deserialize, Serialize};

import_types!("../example.json");

fn main() {
    let veg = Veggie {
        veggie_name: String::from("carrots"),
        veggie_like: true,
    };
    let veggies = Veggies {
        fruits: vec![String::from("apple"), String::from("mango")],
        vegetables: vec![veg],
    };
    println!("{:?}", veggies);
}
