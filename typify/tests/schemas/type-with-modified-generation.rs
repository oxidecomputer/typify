#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"TestType\",\n  \"type\": \"object\",\n  \"required\": [\n    \"converted_type\",\n    \"patched_type\",\n    \"replaced_type\"\n  ],\n  \"properties\": {\n    \"converted_type\": {\n      \"enum\": [\n        1,\n        \"one\"\n      ]\n    },\n    \"patched_type\": {\n      \"$ref\": \"#/definitions/TypeThatNeedsMoreDerives\"\n    },\n    \"replaced_type\": {\n      \"$ref\": \"#/definitions/HandGeneratedType\"\n    }\n  },\n  \"$comment\": \"validate replacement, patch, and conversion settings\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
#[doc = "TypeThatHasMoreDerives"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"additionalProperties\": {\n    \"type\": \"string\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
