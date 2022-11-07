use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LetterBox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub letter: Option<LetterBoxLetter>,
}
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, educe :: Educe,
)]
#[educe(Default)]
pub enum LetterBoxLetter {
    #[educe(Default)]
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
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LetterBoxLetter {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LetterBoxLetter {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
fn main() {}
