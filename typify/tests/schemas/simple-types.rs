#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
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
#[doc = "AnythingWorks"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AnythingWorks {
    pub value: ::serde_json::Value,
}
impl From<&AnythingWorks> for AnythingWorks {
    fn from(value: &AnythingWorks) -> Self {
        value.clone()
    }
}
#[doc = "FloatsArentTerribleImTold"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"flush_timeout\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FloatsArentTerribleImTold {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_timeout: Option<f32>,
}
impl From<&FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
    fn from(value: &FloatsArentTerribleImTold) -> Self {
        value.clone()
    }
}
#[doc = "JustOne"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct JustOne(pub String);
impl ::std::ops::Deref for JustOne {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<JustOne> for String {
    fn from(value: JustOne) -> Self {
        value.0
    }
}
impl From<&JustOne> for JustOne {
    fn from(value: &JustOne) -> Self {
        value.clone()
    }
}
impl From<String> for JustOne {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for JustOne {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for JustOne {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
fn main() {}
