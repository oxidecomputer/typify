#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FloatsArentTerribleImTold {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_timeout: Option<f64>,
}
impl From<&FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
    fn from(value: &FloatsArentTerribleImTold) -> Self {
        value.clone()
    }
}
fn main() {}
