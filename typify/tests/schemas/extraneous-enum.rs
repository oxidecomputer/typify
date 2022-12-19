use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LetterBox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub letter: Option<LetterBoxLetter>,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LetterBoxLetter {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
}
impl ToString for LetterBoxLetter {
    fn to_string(&self) -> String {
        match *self {
            Self::A => "a".to_string(),
            Self::B => "b".to_string(),
        }
    }
}
impl std::str::FromStr for LetterBoxLetter {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err("invalid value"),
        }
    }
}

fn main() {}
