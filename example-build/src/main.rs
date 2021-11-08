// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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
