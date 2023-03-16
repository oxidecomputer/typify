#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, ExtraDerive, Serialize)]
pub struct Fruit(pub std::collections::HashMap<String, serde_json::Value>);
impl std::ops::Deref for Fruit {
    type Target = std::collections::HashMap<String, serde_json::Value>;
    fn deref(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.0
    }
}
impl From<Fruit> for std::collections::HashMap<String, serde_json::Value> {
    fn from(value: Fruit) -> Self {
        value.0
    }
}
impl From<&Fruit> for Fruit {
    fn from(value: &Fruit) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, serde_json::Value>> for Fruit {
    fn from(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, ExtraDerive, Serialize)]
#[serde(untagged)]
pub enum FruitOrVeg {
    Veg(Veggie),
    Fruit(Fruit),
}
impl From<&FruitOrVeg> for FruitOrVeg {
    fn from(value: &FruitOrVeg) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, ExtraDerive, Serialize)]
pub struct Veggie {
    #[doc = "Do I like this vegetable?"]
    #[serde(rename = "veggieLike")]
    pub veggie_like: bool,
    #[doc = "The name of the vegetable."]
    #[serde(rename = "veggieName")]
    pub veggie_name: String,
}
impl From<&Veggie> for Veggie {
    fn from(value: &Veggie) -> Self {
        value.clone()
    }
}
#[doc = "A representation of a person, company, organization, or place"]
#[derive(Clone, Debug, Deserialize, ExtraDerive, Serialize)]
pub struct Veggies {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fruits: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vegetables: Vec<Veggie>,
}
impl From<&Veggies> for Veggies {
    fn from(value: &Veggies) -> Self {
        value.clone()
    }
}
