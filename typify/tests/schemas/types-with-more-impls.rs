use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize)]
pub struct PatternString(String);
impl std::ops::Deref for PatternString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::str::FromStr for PatternString {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if regress::Regex::new("xx").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"xx\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PatternString {
    type Error = <Self as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PatternString {
    type Error = <Self as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PatternString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: <Self as std::str::FromStr>::Err| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sub10Primes(u32);
impl std::ops::Deref for Sub10Primes {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<u32> for Sub10Primes {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if ![2_u32, 3_u32, 5_u32, 7_u32].contains(&value) {
            Err("invalid value")
        } else {
            Ok(Self(value))
        }
    }
}
fn main() {}
