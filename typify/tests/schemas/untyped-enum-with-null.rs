#[allow(unused_imports)]
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
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestType\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        null,"]
#[doc = "        \"start\","]
#[doc = "        \"middle\","]
#[doc = "        \"end\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate a type with no type and enum values that include a null\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestType {
    pub value: Option<TestTypeValue>,
}
impl From<&TestType> for TestType {
    fn from(value: &TestType) -> Self {
        value.clone()
    }
}
#[doc = "TestTypeValue"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    null,"]
#[doc = "    \"start\","]
#[doc = "    \"middle\","]
#[doc = "    \"end\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TestTypeValue {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
}
impl From<&TestTypeValue> for TestTypeValue {
    fn from(value: &TestTypeValue) -> Self {
        value.clone()
    }
}
impl ToString for TestTypeValue {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::Middle => "middle".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
impl std::str::FromStr for TestTypeValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TestTypeValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TestTypeValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TestTypeValue {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
fn main() {}
