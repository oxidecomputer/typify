#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
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
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for IntOrStr {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntOrStr {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntOrStr {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOfSeveral {
    Null,
    Boolean(bool),
    Object(std::collections::HashMap<String, serde_json::Value>),
    Array(Vec<serde_json::Value>),
    String(String),
    Integer(i64),
}
impl From<&OneOfSeveral> for OneOfSeveral {
    fn from(value: &OneOfSeveral) -> Self {
        value.clone()
    }
}
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
fn main() {}
