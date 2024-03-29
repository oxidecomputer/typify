#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = r" Error types."]
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
#[doc = "Doodad"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"when\": {"]
#[doc = "      \"default\": \"1970-01-01T00:00:00Z\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Doodad {
    #[serde(default = "defaults::doodad_when")]
    pub when: chrono::DateTime<chrono::offset::Utc>,
}
impl From<&Doodad> for Doodad {
    fn from(value: &Doodad) -> Self {
        value.clone()
    }
}
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
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn doodad_when() -> chrono::DateTime<chrono::offset::Utc> {
        serde_json::from_str::<chrono::DateTime<chrono::offset::Utc>>("\"1970-01-01T00:00:00Z\"")
            .unwrap()
    }
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
