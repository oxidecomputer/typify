#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"TestType\",\n  \"type\": \"object\",\n  \"required\": [\n    \"value\"\n  ],\n  \"properties\": {\n    \"value\": {\n      \"enum\": [\n        null,\n        \"start\",\n        \"middle\",\n        \"end\"\n      ]\n    }\n  },\n  \"$comment\": \"validate a type with no type and enum values that include a null\"\n}"]
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
#[doc = "{\n  \"enum\": [\n    null,\n    \"start\",\n    \"middle\",\n    \"end\"\n  ]\n}"]
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
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TestTypeValue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TestTypeValue {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TestTypeValue {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
fn main() {}
