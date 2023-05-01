#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(uuid::Uuid),
    Name(Name),
}
impl From<&IdOrName> for IdOrName {
    fn from(value: &IdOrName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdOrName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Name(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for IdOrName {
    fn to_string(&self) -> String {
        match self {
            Self::Id(x) => x.to_string(),
            Self::Name(x) => x.to_string(),
        }
    }
}
impl From<uuid::Uuid> for IdOrName {
    fn from(value: uuid::Uuid) -> Self {
        Self::Id(value)
    }
}
impl From<Name> for IdOrName {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrNameRedundant {
    Variant0(uuid::Uuid),
    Variant1(Name),
}
impl From<&IdOrNameRedundant> for IdOrNameRedundant {
    fn from(value: &IdOrNameRedundant) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdOrNameRedundant {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrNameRedundant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrNameRedundant {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrNameRedundant {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for IdOrNameRedundant {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<uuid::Uuid> for IdOrNameRedundant {
    fn from(value: uuid::Uuid) -> Self {
        Self::Variant0(value)
    }
}
impl From<Name> for IdOrNameRedundant {
    fn from(value: Name) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdOrYolo {
    Id(uuid::Uuid),
    Yolo(IdOrYoloYolo),
}
impl From<&IdOrYolo> for IdOrYolo {
    fn from(value: &IdOrYolo) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdOrYolo {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Yolo(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrYolo {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrYolo {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrYolo {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for IdOrYolo {
    fn to_string(&self) -> String {
        match self {
            Self::Id(x) => x.to_string(),
            Self::Yolo(x) => x.to_string(),
        }
    }
}
impl From<uuid::Uuid> for IdOrYolo {
    fn from(value: uuid::Uuid) -> Self {
        Self::Id(value)
    }
}
impl From<IdOrYoloYolo> for IdOrYolo {
    fn from(value: IdOrYoloYolo) -> Self {
        Self::Yolo(value)
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct IdOrYoloYolo(String);
impl std::ops::Deref for IdOrYoloYolo {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IdOrYoloYolo> for String {
    fn from(value: IdOrYoloYolo) -> Self {
        value.0
    }
}
impl From<&IdOrYoloYolo> for IdOrYoloYolo {
    fn from(value: &IdOrYoloYolo) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IdOrYoloYolo {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new(".*").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \".*\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IdOrYoloYolo {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrYoloYolo {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrYoloYolo {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IdOrYoloYolo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
#[derive(Clone, Debug, Serialize)]
pub struct Name(String);
impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl From<&Name> for Name {
    fn from(value: &Name) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 63usize {
            return Err("longer than 63 characters");
        }
        if regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"") ; }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
