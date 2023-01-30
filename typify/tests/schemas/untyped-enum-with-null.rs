use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TestType {
    pub value: Option<TestTypeValue>,
}
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, educe :: Educe,
)]
#[educe(Default)]
pub enum TestTypeValue {
    #[educe(Default)]
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
}
impl ToString for TestTypeValue {
    fn to_string(&self) -> String {
        match *self {
            Self::Start => "start".to_string(),
            Self::Middle => "middle".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
impl std::str::FromStr for TestTypeValue {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TestTypeValue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TestTypeValue {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
fn main() {}
