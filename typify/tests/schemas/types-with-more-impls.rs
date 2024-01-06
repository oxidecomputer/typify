#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
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
#[doc = "PatternString"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"xx\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PatternString(String);
impl std::ops::Deref for PatternString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<PatternString> for String {
    fn from(value: PatternString) -> Self {
        value.0
    }
}
impl From<&PatternString> for PatternString {
    fn from(value: &PatternString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PatternString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("xx").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"xx\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PatternString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PatternString {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PatternString {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PatternString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Sub10Primes"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint\","]
#[doc = "  \"enum\": ["]
#[doc = "    2,"]
#[doc = "    3,"]
#[doc = "    5,"]
#[doc = "    7"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Serialize)]
pub struct Sub10Primes(u32);
impl std::ops::Deref for Sub10Primes {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
impl From<Sub10Primes> for u32 {
    fn from(value: Sub10Primes) -> Self {
        value.0
    }
}
impl From<&Sub10Primes> for Sub10Primes {
    fn from(value: &Sub10Primes) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<u32> for Sub10Primes {
    type Error = self::error::ConversionError;
    fn try_from(value: u32) -> Result<Self, self::error::ConversionError> {
        if ![2_u32, 3_u32, 5_u32, 7_u32].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for Sub10Primes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<u32>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
