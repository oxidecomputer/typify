#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = "Fruit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fruit(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for Fruit {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<Fruit> for serde_json::Map<String, serde_json::Value> {
    fn from(value: Fruit) -> Self {
        value.0
    }
}
impl From<&Fruit> for Fruit {
    fn from(value: &Fruit) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for Fruit {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "FruitOrVeg"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"oneOf\": [\n    {\n      \"title\": \"veg\",\n      \"anyOf\": [\n        {\n          \"$ref\": \"#/defs/veggie\"\n        }\n      ]\n    },\n    {\n      \"title\": \"fruit\",\n      \"anyOf\": [\n        {\n          \"$ref\": \"#/defs/fruit\"\n        }\n      ]\n    }\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl From<Veggie> for FruitOrVeg {
    fn from(value: Veggie) -> Self {
        Self::Veg(value)
    }
}
impl From<Fruit> for FruitOrVeg {
    fn from(value: Fruit) -> Self {
        Self::Fruit(value)
    }
}
#[doc = "Veggie"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"required\": [\n    \"veggieLike\",\n    \"veggieName\"\n  ],\n  \"properties\": {\n    \"veggieLike\": {\n      \"description\": \"Do I like this vegetable?\",\n      \"type\": \"boolean\"\n    },\n    \"veggieName\": {\n      \"description\": \"The name of the vegetable.\",\n      \"type\": \"string\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"$id\": \"https://example.com/arrays.schema.json\",\n  \"title\": \"veggies\",\n  \"description\": \"A representation of a person, company, organization, or place\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"fruits\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\"\n      }\n    },\n    \"vegetables\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/$defs/veggie\"\n      }\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
