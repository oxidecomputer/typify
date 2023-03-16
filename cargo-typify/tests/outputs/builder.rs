#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
        builder::Veggie::default()
    }
}
#[doc = "A representation of a person, company, organization, or place"]
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
        builder::Veggies::default()
    }
}
mod builder {
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
        type Error = String;
        fn try_from(value: Veggie) -> Result<Self, String> {
            Ok(Self {
                veggie_like: value.veggie_like?,
                veggie_name: value.veggie_name?,
            })
        }
    }
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
        type Error = String;
        fn try_from(value: Veggies) -> Result<Self, String> {
            Ok(Self {
                fruits: value.fruits?,
                vegetables: value.vegetables?,
            })
        }
    }
}
