use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IpNet {
    V4(Ipv4Net),
    V6(Ipv4Net),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ipv4Net(pub String);
impl std::ops::Deref for Ipv4Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ipv6Net(pub String);
impl std::ops::Deref for Ipv6Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullStringEnumWithUnknownFormat(pub Option<NullStringEnumWithUnknownFormatInner>);
impl std::ops::Deref for NullStringEnumWithUnknownFormat {
    type Target = Option<NullStringEnumWithUnknownFormatInner>;
    fn deref(&self) -> &Option<NullStringEnumWithUnknownFormatInner> {
        &self.0
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NullStringEnumWithUnknownFormatInner {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
}
impl ToString for NullStringEnumWithUnknownFormatInner {
    fn to_string(&self) -> String {
        match *self {
            Self::A => "a".to_string(),
            Self::B => "b".to_string(),
            Self::C => "c".to_string(),
        }
    }
}
impl std::str::FromStr for NullStringEnumWithUnknownFormatInner {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for NullStringEnumWithUnknownFormatInner {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NullStringEnumWithUnknownFormatInner {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NullStringEnumWithUnknownFormatInner {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOfTypes {
    Variant0 { bar: i64 },
    Variant1 { foo: String },
}
fn main() {}
