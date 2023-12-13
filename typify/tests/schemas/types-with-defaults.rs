#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestBed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"properties\": {\n    \"any\": {\n      \"default\": [\n        [\n          8,\n          6,\n          7\n        ],\n        [\n          5,\n          3,\n          0,\n          9\n        ]\n      ],\n      \"type\": \"array\",\n      \"items\": {}\n    },\n    \"id\": {\n      \"default\": \"abc123-is-this-a-uuid\",\n      \"type\": \"string\",\n      \"format\": \"uuid\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestBed {
    #[serde(default = "defaults::test_bed_any")]
    pub any: Vec<serde_json::Value>,
    #[serde(default = "defaults::test_bed_id")]
    pub id: uuid::Uuid,
}
impl From<&TestBed> for TestBed {
    fn from(value: &TestBed) -> Self {
        value.clone()
    }
}
pub mod defaults {
    pub(super) fn test_bed_any() -> Vec<serde_json::Value> {
        vec![
            serde_json::from_str::<serde_json::Value>("[8,6,7]").unwrap(),
            serde_json::from_str::<serde_json::Value>("[5,3,0,9]").unwrap(),
        ]
    }
    pub(super) fn test_bed_id() -> uuid::Uuid {
        serde_json::from_str::<uuid::Uuid>("\"abc123-is-this-a-uuid\"").unwrap()
    }
}
fn main() {}
