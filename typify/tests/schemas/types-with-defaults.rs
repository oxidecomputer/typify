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
#[doc = "OuterThing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"thing\": {"]
#[doc = "      \"title\": \"ThingWithDefaults\","]
#[doc = "      \"default\": {"]
#[doc = "        \"type\": \"bee\""]
#[doc = "      },"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"a\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OuterThing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thing: Option<ThingWithDefaults>,
}
impl From<&OuterThing> for OuterThing {
    fn from(value: &OuterThing) -> Self {
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
#[doc = "ThingWithDefaults"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThingWithDefaults\","]
#[doc = "  \"default\": {"]
#[doc = "    \"type\": \"bee\""]
#[doc = "  },"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"a\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThingWithDefaults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&ThingWithDefaults> for ThingWithDefaults {
    fn from(value: &ThingWithDefaults) -> Self {
        value.clone()
    }
}
impl Default for ThingWithDefaults {
    fn default() -> Self {
        ThingWithDefaults {
            a: Default::default(),
            type_: Some("bee".to_string()),
        }
    }
}
#[doc = r" Generation of default values for serde."]
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
