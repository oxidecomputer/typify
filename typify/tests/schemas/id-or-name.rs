use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(uuid::Uuid),
    Name(Name),
}
impl ToString for IdOrName {
    fn to_string(&self) -> String {
        match self {
            Self::Id(x) => x.to_string(),
            Self::Name(x) => x.to_string(),
        }
    }
}
#[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
#[derive(Clone, Debug, Serialize)]
pub struct Name(String);
impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 63usize {
            return Err("longer than 63 characters");
        }
        if regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"") ; }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
