#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "FloatsArentTerribleImTold"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"flush_timeout\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FloatsArentTerribleImTold {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_timeout: Option<f32>,
}
impl From<&FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
    fn from(value: &FloatsArentTerribleImTold) -> Self {
        value.clone()
    }
}
fn main() {}
