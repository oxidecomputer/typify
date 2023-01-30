use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestType {
    pub where_not: TestTypeWhereNot,
    pub why_not: TestTypeWhyNot,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestTypeWhereNot(String);
impl std::ops::Deref for TestTypeWhereNot {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestTypeWhyNot(String);
impl std::ops::Deref for TestTypeWhyNot {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
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
fn main() {}
