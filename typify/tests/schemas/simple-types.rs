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
impl AnythingWorks {
    pub fn builder() -> builder::AnythingWorks {
        Default::default()
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
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub flush_timeout: ::std::option::Option<f32>,
}
impl From<&FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
    fn from(value: &FloatsArentTerribleImTold) -> Self {
        value.clone()
    }
}
impl FloatsArentTerribleImTold {
    pub fn builder() -> builder::FloatsArentTerribleImTold {
        Default::default()
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
pub struct JustOne(pub ::std::string::String);
impl ::std::ops::Deref for JustOne {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl From<JustOne> for ::std::string::String {
    fn from(value: JustOne) -> Self {
        value.0
    }
}
impl From<&JustOne> for JustOne {
    fn from(value: &JustOne) -> Self {
        value.clone()
    }
}
impl From<::std::string::String> for JustOne {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for JustOne {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for JustOne {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AnythingWorks {
        value: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl Default for AnythingWorks {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AnythingWorks {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AnythingWorks> for super::AnythingWorks {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnythingWorks,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl From<super::AnythingWorks> for AnythingWorks {
        fn from(value: super::AnythingWorks) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FloatsArentTerribleImTold {
        flush_timeout: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl Default for FloatsArentTerribleImTold {
        fn default() -> Self {
            Self {
                flush_timeout: Ok(Default::default()),
            }
        }
    }
    impl FloatsArentTerribleImTold {
        pub fn flush_timeout<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: std::fmt::Display,
        {
            self.flush_timeout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flush_timeout: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FloatsArentTerribleImTold> for super::FloatsArentTerribleImTold {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FloatsArentTerribleImTold,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flush_timeout: value.flush_timeout?,
            })
        }
    }
    impl From<super::FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
        fn from(value: super::FloatsArentTerribleImTold) -> Self {
            Self {
                flush_timeout: Ok(value.flush_timeout),
            }
        }
    }
}
fn main() {}
