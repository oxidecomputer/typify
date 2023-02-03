#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestType {
    pub converted_type: serde_json::Value,
    pub patched_type: TypeThatHasMoreDerives,
    pub replaced_type: String,
}
impl From<&TestType> for TestType {
    fn from(value: &TestType) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypeThatHasMoreDerives(pub std::collections::HashMap<String, String>);
impl std::ops::Deref for TypeThatHasMoreDerives {
    type Target = std::collections::HashMap<String, String>;
    fn deref(&self) -> &std::collections::HashMap<String, String> {
        &self.0
    }
}
impl From<TypeThatHasMoreDerives> for std::collections::HashMap<String, String> {
    fn from(value: TypeThatHasMoreDerives) -> Self {
        value.0
    }
}
impl From<&TypeThatHasMoreDerives> for TypeThatHasMoreDerives {
    fn from(value: &TypeThatHasMoreDerives) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for TypeThatHasMoreDerives {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self(value)
    }
}
fn main() {}
