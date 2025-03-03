#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "`Fruit`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, ExtraDerive)]
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
impl ::std::convert::From<&Fruit> for Fruit {
    fn from(value: &Fruit) -> Self {
        value.clone()
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, ExtraDerive)]
#[serde(untagged)]
pub enum FruitOrVeg {
    Veg(Veggie),
    Fruit(Fruit),
}
impl ::std::convert::From<&Self> for FruitOrVeg {
    fn from(value: &FruitOrVeg) -> Self {
        value.clone()
    }
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, ExtraDerive)]
pub struct Veggie {
    #[doc = "Do I like this vegetable?"]
    #[serde(rename = "veggieLike")]
    pub veggie_like: bool,
    #[doc = "The name of the vegetable."]
    #[serde(rename = "veggieName")]
    pub veggie_name: ::std::string::String,
}
impl ::std::convert::From<&Veggie> for Veggie {
    fn from(value: &Veggie) -> Self {
        value.clone()
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, ExtraDerive)]
pub struct Veggies {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub fruits: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vegetables: ::std::vec::Vec<Veggie>,
}
impl ::std::convert::From<&Veggies> for Veggies {
    fn from(value: &Veggies) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Veggies {
    fn default() -> Self {
        Self {
            fruits: Default::default(),
            vegetables: Default::default(),
        }
    }
}
