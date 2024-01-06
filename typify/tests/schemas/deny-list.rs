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
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestType\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"where_not\","]
#[doc = "    \"why_not\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"where_not\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"enum\": ["]
#[doc = "          \"start\","]
#[doc = "          \"middle\","]
#[doc = "          \"end\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"why_not\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"because\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate a 'not' schema with typed- and untyped-subschemas\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestType {
    pub where_not: TestTypeWhereNot,
    pub why_not: TestTypeWhyNot,
}
impl From<&TestType> for TestType {
    fn from(value: &TestType) -> Self {
        value.clone()
    }
}
#[doc = "TestTypeWhereNot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"not\": {"]
#[doc = "    \"enum\": ["]
#[doc = "      \"start\","]
#[doc = "      \"middle\","]
#[doc = "      \"end\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TestTypeWhereNot(String);
impl std::ops::Deref for TestTypeWhereNot {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TestTypeWhereNot> for String {
    fn from(value: TestTypeWhereNot) -> Self {
        value.0
    }
}
impl From<&TestTypeWhereNot> for TestTypeWhereNot {
    fn from(value: &TestTypeWhereNot) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<String> for TestTypeWhereNot {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        if ["start".to_string(), "middle".to_string(), "end".to_string()].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for TestTypeWhereNot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "TestTypeWhyNot"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"not\": {"]
#[doc = "    \"type\": \"string\","]
#[doc = "    \"enum\": ["]
#[doc = "      \"because\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TestTypeWhyNot(String);
impl std::ops::Deref for TestTypeWhyNot {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TestTypeWhyNot> for String {
    fn from(value: TestTypeWhyNot) -> Self {
        value.0
    }
}
impl From<&TestTypeWhyNot> for TestTypeWhyNot {
    fn from(value: &TestTypeWhyNot) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<String> for TestTypeWhyNot {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        if ["because".to_string()].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for TestTypeWhyNot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
