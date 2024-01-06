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
#[doc = "IntOrStr"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntOrStr {
    String(String),
    Integer(i64),
}
impl From<&IntOrStr> for IntOrStr {
    fn from(value: &IntOrStr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IntOrStr {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IntOrStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntOrStr {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntOrStr {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for IntOrStr {
    fn to_string(&self) -> String {
        match self {
            Self::String(x) => x.to_string(),
            Self::Integer(x) => x.to_string(),
        }
    }
}
impl From<i64> for IntOrStr {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "OneOfSeveral"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"null\","]
#[doc = "    \"boolean\","]
#[doc = "    \"object\","]
#[doc = "    \"array\","]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOfSeveral {
    Null,
    Boolean(bool),
    Object(serde_json::Map<String, serde_json::Value>),
    Array(Vec<serde_json::Value>),
    String(String),
    Integer(i64),
}
impl From<&OneOfSeveral> for OneOfSeveral {
    fn from(value: &OneOfSeveral) -> Self {
        value.clone()
    }
}
impl From<bool> for OneOfSeveral {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for OneOfSeveral {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self::Object(value)
    }
}
impl From<Vec<serde_json::Value>> for OneOfSeveral {
    fn from(value: Vec<serde_json::Value>) -> Self {
        Self::Array(value)
    }
}
impl From<i64> for OneOfSeveral {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "ReallyJustNull"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"enum\": ["]
#[doc = "    null"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReallyJustNull(pub ());
impl std::ops::Deref for ReallyJustNull {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl From<ReallyJustNull> for () {
    fn from(value: ReallyJustNull) -> Self {
        value.0
    }
}
impl From<&ReallyJustNull> for ReallyJustNull {
    fn from(value: &ReallyJustNull) -> Self {
        value.clone()
    }
}
impl From<()> for ReallyJustNull {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
#[doc = "SeriouslyAnything"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"null\","]
#[doc = "    \"boolean\","]
#[doc = "    \"object\","]
#[doc = "    \"array\","]
#[doc = "    \"number\","]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SeriouslyAnything(pub serde_json::Value);
impl std::ops::Deref for SeriouslyAnything {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<SeriouslyAnything> for serde_json::Value {
    fn from(value: SeriouslyAnything) -> Self {
        value.0
    }
}
impl From<&SeriouslyAnything> for SeriouslyAnything {
    fn from(value: &SeriouslyAnything) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for SeriouslyAnything {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "YesNoMaybe"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"boolean\","]
#[doc = "    \"object\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum YesNoMaybe {
    Boolean(bool),
    Object {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
}
impl From<&YesNoMaybe> for YesNoMaybe {
    fn from(value: &YesNoMaybe) -> Self {
        value.clone()
    }
}
impl From<bool> for YesNoMaybe {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
fn main() {}
