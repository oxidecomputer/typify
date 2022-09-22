// Copyright 2022 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[test]
fn test_main() {
    match main() {
        Ok(it) => it,
        Err(_) => (),
    };
}

fn main() -> Result<(), String> {
    let veg = Veggie::builder()
        .veggie_name("carrots")
        .veggie_like(true)
        .try_into()
        .unwrap();

    let veggies = Veggies {
        fruits: vec![String::from("apple"), String::from("mango")],
        vegetables: vec![veg],
        id_num: Some(VeggiesIdNum::try_from(25)?),
    };
    println!("{:?}", veggies);
    Ok(())
}
