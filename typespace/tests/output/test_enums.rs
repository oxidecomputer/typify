#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum External {
    Unit,
    Item(String),
    Named { x: u32 },
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(tag = "type")]
pub enum Internal {
    Unit,
    Named { x: u32 },
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Adjacent {
    Unit,
    Item(String),
    Named { x: u32 },
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum Untagged {
    Unit,
    Item(String),
    Named { x: u32 },
}
