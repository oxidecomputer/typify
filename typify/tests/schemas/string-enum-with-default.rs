#[allow(unused_imports)]
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
impl From<&TestEnum> for TestEnum {
    fn from(value: &TestEnum) -> Self {
        value.clone()
    }
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
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "failure" => Ok(Self::Failure),
            "skipped" => Ok(Self::Skipped),
            "success" => Ok(Self::Success),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TestEnum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TestEnum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TestEnum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::Failure
    }
}
fn main() {}
