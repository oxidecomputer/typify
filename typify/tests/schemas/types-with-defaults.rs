use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestBed {
    #[serde(default = "defaults::test_bed_any")]
    pub any: Vec<serde_json::Value>,
    #[serde(default = "defaults::test_bed_id")]
    pub id: uuid::Uuid,
}
mod defaults {
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
