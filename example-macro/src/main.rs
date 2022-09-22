// Copyright 2022 Oxide Computer Company

use typify::import_types;

use serde::{Deserialize, Serialize};

import_types!("../example.json");

#[test]
fn test_main() {
    match main() {
        Ok(it) => it,
        Err(_) => (),
    };
}

fn main() -> Result<(), String> {
    let veg = Veggie {
        veggie_name: String::from("carrots"),
        veggie_like: true,
    };
    let veggies = Veggies {
        fruits: vec![String::from("apple"), String::from("mango")],
        vegetables: vec![veg],
        id_num: Some(VeggiesIdNum::try_from(25)?),
    };
    println!("{:?}", veggies);
    Ok(())
}
