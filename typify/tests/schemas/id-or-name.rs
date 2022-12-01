use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(uuid::Uuid),
    Name(String),
}

fn main() {}
