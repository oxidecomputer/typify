use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TestEnum {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "success")]
    Success,
}
impl ToString for TestEnum {
    fn to_string(&self) -> String {
        match *self {
            Self::Failure => "failure".to_string(),
            Self::Skipped => "skipped".to_string(),
            Self::Success => "success".to_string(),
        }
    }
}
impl std::str::FromStr for TestEnum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "failure" => Ok(Self::Failure),
            "skipped" => Ok(Self::Skipped),
            "success" => Ok(Self::Success),
            _ => Err("invalid value"),
        }
    }
}
impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::Failure
    }
}
fn main() {}
