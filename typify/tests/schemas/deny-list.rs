#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"TestType\",\n  \"type\": \"object\",\n  \"required\": [\n    \"where_not\",\n    \"why_not\"\n  ],\n  \"properties\": {\n    \"where_not\": {\n      \"not\": {\n        \"enum\": [\n          \"start\",\n          \"middle\",\n          \"end\"\n        ]\n      }\n    },\n    \"why_not\": {\n      \"not\": {\n        \"type\": \"string\",\n        \"enum\": [\n          \"because\"\n        ]\n      }\n    }\n  },\n  \"$comment\": \"validate a 'not' schema with typed- and untyped-subschemas\"\n}"]
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
#[doc = "{\n  \"not\": {\n    \"enum\": [\n      \"start\",\n      \"middle\",\n      \"end\"\n    ]\n  }\n}"]
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
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        if ["start".to_string(), "middle".to_string(), "end".to_string()].contains(&value) {
            Err("invalid value")
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
#[doc = "{\n  \"not\": {\n    \"type\": \"string\",\n    \"enum\": [\n      \"because\"\n    ]\n  }\n}"]
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
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        if ["because".to_string()].contains(&value) {
            Err("invalid value")
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
