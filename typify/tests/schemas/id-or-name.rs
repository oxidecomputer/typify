#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "IdOrName"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Id\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uuid\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Name\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Name\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Name(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
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
#[doc = "IdOrNameRedundant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"$ref\": \"#/definitions/Name\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"tests references that include a redundant type field\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrNameRedundant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrNameRedundant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrNameRedundant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
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
#[doc = "IdOrYolo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Id\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uuid\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Yolo\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \".*\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Id(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Yolo(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IdOrYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
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
#[doc = "IdOrYoloYolo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \".*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new(".*").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \".*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IdOrYoloYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IdOrYoloYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IdOrYoloYolo {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
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
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"A name unique within the parent collection\","]
#[doc = "  \"description\": \"Names must begin with a lower case ASCII letter, be composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end with a '-'. Names cannot be a UUID though they may contain a UUID.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 63,"]
#[doc = "  \"pattern\": \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 63usize {
            return Err("longer than 63 characters".into());
        }
        if regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"" . into ()) ; }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Name {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Name {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
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
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
fn main() {}
