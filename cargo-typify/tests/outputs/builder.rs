#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Fruit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
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
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"veg\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/defs/veggie\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"fruit\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/defs/fruit\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
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
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"veggieLike\","]
#[doc = "    \"veggieName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"veggieLike\": {"]
#[doc = "      \"description\": \"Do I like this vegetable?\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"veggieName\": {"]
#[doc = "      \"description\": \"The name of the vegetable.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
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
impl Veggie {
    pub fn builder() -> builder::Veggie {
        Default::default()
    }
}
#[doc = "A representation of a person, company, organization, or place"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://example.com/arrays.schema.json\","]
#[doc = "  \"title\": \"veggies\","]
#[doc = "  \"description\": \"A representation of a person, company, organization, or place\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fruits\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vegetables\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/veggie\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
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
impl Veggies {
    pub fn builder() -> builder::Veggies {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Veggie {
        veggie_like: Result<bool, String>,
        veggie_name: Result<String, String>,
    }
    impl Default for Veggie {
        fn default() -> Self {
            Self {
                veggie_like: Err("no value supplied for veggie_like".to_string()),
                veggie_name: Err("no value supplied for veggie_name".to_string()),
            }
        }
    }
    impl Veggie {
        pub fn veggie_like<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.veggie_like = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for veggie_like: {}", e));
            self
        }
        pub fn veggie_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.veggie_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for veggie_name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Veggie> for super::Veggie {
        type Error = super::error::ConversionError;
        fn try_from(value: Veggie) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                veggie_like: value.veggie_like?,
                veggie_name: value.veggie_name?,
            })
        }
    }
    impl From<super::Veggie> for Veggie {
        fn from(value: super::Veggie) -> Self {
            Self {
                veggie_like: Ok(value.veggie_like),
                veggie_name: Ok(value.veggie_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Veggies {
        fruits: Result<Vec<String>, String>,
        vegetables: Result<Vec<super::Veggie>, String>,
    }
    impl Default for Veggies {
        fn default() -> Self {
            Self {
                fruits: Ok(Default::default()),
                vegetables: Ok(Default::default()),
            }
        }
    }
    impl Veggies {
        pub fn fruits<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.fruits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fruits: {}", e));
            self
        }
        pub fn vegetables<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Veggie>>,
            T::Error: std::fmt::Display,
        {
            self.vegetables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vegetables: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Veggies> for super::Veggies {
        type Error = super::error::ConversionError;
        fn try_from(value: Veggies) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                fruits: value.fruits?,
                vegetables: value.vegetables?,
            })
        }
    }
    impl From<super::Veggies> for Veggies {
        fn from(value: super::Veggies) -> Self {
            Self {
                fruits: Ok(value.fruits),
                vegetables: Ok(value.vegetables),
            }
        }
    }
}
