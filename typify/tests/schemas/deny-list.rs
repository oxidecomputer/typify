#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize)]
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
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
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
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
