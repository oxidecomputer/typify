#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "`Fruit`"]
#[extra_attr]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Fruit(pub ::std::collections::HashMap<::std::string::String, ::std::string::String>);
impl ::std::ops::Deref for Fruit {
    type Target = ::std::collections::HashMap<::std::string::String, ::std::string::String>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<Fruit>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String>
{
    fn from(value: Fruit) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, ::std::string::String>>
    for Fruit
{
    fn from(
        value: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`FruitOrVeg`"]
#[extra_attr]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FruitOrVeg {
    Veg(Veggie),
    Fruit(Fruit),
}
impl ::std::convert::From<Veggie> for FruitOrVeg {
    fn from(value: Veggie) -> Self {
        Self::Veg(value)
    }
}
impl ::std::convert::From<Fruit> for FruitOrVeg {
    fn from(value: Fruit) -> Self {
        Self::Fruit(value)
    }
}
#[doc = "`Veggie`"]
#[extra_attr]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Veggie {
    #[doc = "Do I like this vegetable?"]
    #[serde(rename = "veggieLike")]
    pub veggie_like: bool,
    #[doc = "The name of the vegetable."]
    #[serde(rename = "veggieName")]
    pub veggie_name: ::std::string::String,
}
#[doc = "A representation of a person, company, organization, or place"]
#[extra_attr]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct Veggies {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fruits: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vegetables: ::std::vec::Vec<Veggie>,
}
