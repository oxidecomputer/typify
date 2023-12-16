#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "TestBed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"any\": {"]
#[doc = "      \"default\": ["]
#[doc = "        ["]
#[doc = "          8,"]
#[doc = "          6,"]
#[doc = "          7"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          5,"]
#[doc = "          3,"]
#[doc = "          0,"]
#[doc = "          9"]
#[doc = "        ]"]
#[doc = "      ],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"default\": \"abc123-is-this-a-uuid\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
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
