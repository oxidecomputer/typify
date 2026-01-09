// Copyright 2025 Oxide Computer Company

#![allow(clippy::derivable_impls)]

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[test]
fn test_main() {
    main()
}

fn main() {
    // let veg = Veggie::builder()
    //     .veggie_name("carrots")
    //     .veggie_like(true)
    //     .try_into()
    //     .unwrap();

    // let veggies = Veggies {
    //     fruits: vec![String::from("apple"), String::from("mango")],
    //     vegetables: vec![veg],
    // };
    // println!("{:?}", veggies);
}
