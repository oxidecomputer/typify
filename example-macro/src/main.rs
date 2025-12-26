// Copyright 2023 Oxide Computer Company

use typify::import_types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MyFruit {
    seeds: (),
}

import_types!(
    schema = "../example.json",
    patch = {
        Veggie = {
            rename = "Vegetable",
        },
    },
    replace = {
        Fruit = MyFruit: ?Display,
    }
);

#[test]
fn test_main() {
    main();
}

fn main() {
    let veg = Vegetable {
        veggie_name: String::from("carrots"),
        veggie_like: true,
    };
    let veggies = Veggies {
        fruits: vec![String::from("apple"), String::from("mango")],
        vegetables: vec![veg],
    };
    println!("{veggies:?}");
    let fov = FruitOrVeg::Fruit(MyFruit { seeds: () });
    println!("{fov:?}");
}
