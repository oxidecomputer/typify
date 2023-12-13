#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Node"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"node\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"children\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#\"\n      }\n    },\n    \"value\": {\n      \"type\": \"integer\"\n    }\n  },\n  \"$comment\": \"validate references to the whole\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}
fn main() {}
