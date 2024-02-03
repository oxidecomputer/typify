#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = r" Error types."]
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
#[doc = "AlternativeEnum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"Choice2\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Choice1\","]
#[doc = "    \"Choice2\","]
#[doc = "    \"Choice3\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AlternativeEnum {
    Choice1,
    Choice2,
    Choice3,
}
impl From<&AlternativeEnum> for AlternativeEnum {
    fn from(value: &AlternativeEnum) -> Self {
        value.clone()
    }
}
impl ToString for AlternativeEnum {
    fn to_string(&self) -> String {
        match *self {
            Self::Choice1 => "Choice1".to_string(),
            Self::Choice2 => "Choice2".to_string(),
            Self::Choice3 => "Choice3".to_string(),
        }
    }
}
impl std::str::FromStr for AlternativeEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Choice1" => Ok(Self::Choice1),
            "Choice2" => Ok(Self::Choice2),
            "Choice3" => Ok(Self::Choice3),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for AlternativeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlternativeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlternativeEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for AlternativeEnum {
    fn default() -> Self {
        AlternativeEnum::Choice2
    }
}
#[doc = "DiskAttachment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alternate\","]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternate\": {"]
#[doc = "      \"$ref\": \"#/components/schemas/AlternativeEnum\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"default\": \"Detached\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Detached\","]
#[doc = "        \"Destroyed\","]
#[doc = "        \"Faulted\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiskAttachment {
    pub alternate: AlternativeEnum,
    pub state: DiskAttachmentState,
}
impl From<&DiskAttachment> for DiskAttachment {
    fn from(value: &DiskAttachment) -> Self {
        value.clone()
    }
}
#[doc = "DiskAttachmentState"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"Detached\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Detached\","]
#[doc = "    \"Destroyed\","]
#[doc = "    \"Faulted\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DiskAttachmentState {
    Detached,
    Destroyed,
    Faulted,
}
impl From<&DiskAttachmentState> for DiskAttachmentState {
    fn from(value: &DiskAttachmentState) -> Self {
        value.clone()
    }
}
impl ToString for DiskAttachmentState {
    fn to_string(&self) -> String {
        match *self {
            Self::Detached => "Detached".to_string(),
            Self::Destroyed => "Destroyed".to_string(),
            Self::Faulted => "Faulted".to_string(),
        }
    }
}
impl std::str::FromStr for DiskAttachmentState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Detached" => Ok(Self::Detached),
            "Destroyed" => Ok(Self::Destroyed),
            "Faulted" => Ok(Self::Faulted),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DiskAttachmentState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiskAttachmentState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiskAttachmentState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for DiskAttachmentState {
    fn default() -> Self {
        DiskAttachmentState::Detached
    }
}
#[doc = "EmptyObject"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"prop\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"enum\": ["]
#[doc = "        {}"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmptyObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<EmptyObjectProp>,
}
impl From<&EmptyObject> for EmptyObject {
    fn from(value: &EmptyObject) -> Self {
        value.clone()
    }
}
#[doc = "EmptyObjectProp"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"enum\": ["]
#[doc = "    {}"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Serialize)]
pub struct EmptyObjectProp(serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for EmptyObjectProp {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<EmptyObjectProp> for serde_json::Map<String, serde_json::Value> {
    fn from(value: EmptyObjectProp) -> Self {
        value.0
    }
}
impl From<&EmptyObjectProp> for EmptyObjectProp {
    fn from(value: &EmptyObjectProp) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<serde_json::Map<String, serde_json::Value>> for EmptyObjectProp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, self::error::ConversionError> {
        if ![[].into_iter().collect()].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for EmptyObjectProp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<serde_json::Map<String, serde_json::Value>>::deserialize(
            deserializer,
        )?)
        .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "EnumAndConstant"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"bark\","]
#[doc = "        \"petType\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"bark\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"petType\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"dog\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"petType\","]
#[doc = "        \"purr\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"petType\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"cat\""]
#[doc = "        },"]
#[doc = "        \"purr\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"help\","]
#[doc = "        \"petType\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"help\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"petType\": {"]
#[doc = "          \"const\": \"monkey\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"float\","]
#[doc = "        \"petType\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"float\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"petType\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"fish\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "petType")]
pub enum EnumAndConstant {
    #[serde(rename = "dog")]
    Dog { bark: String },
    #[serde(rename = "cat")]
    Cat { purr: String },
    #[serde(rename = "monkey")]
    Monkey { help: String },
    #[serde(rename = "fish")]
    Fish { float: String },
}
impl From<&EnumAndConstant> for EnumAndConstant {
    fn from(value: &EnumAndConstant) -> Self {
        value.clone()
    }
}
#[doc = "IpNet"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"V4\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/components/schemas/Ipv4Net\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"V6\","]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/components/schemas/Ipv6Net\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"we want to see *nice* variant names in the output\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IpNet {
    V4(Ipv4Net),
    V6(Ipv6Net),
}
impl From<&IpNet> for IpNet {
    fn from(value: &IpNet) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IpNet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::V4(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::V6(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IpNet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IpNet {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IpNet {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for IpNet {
    fn to_string(&self) -> String {
        match self {
            Self::V4(x) => x.to_string(),
            Self::V6(x) => x.to_string(),
        }
    }
}
impl From<Ipv4Net> for IpNet {
    fn from(value: Ipv4Net) -> Self {
        Self::V4(value)
    }
}
impl From<Ipv6Net> for IpNet {
    fn from(value: Ipv6Net) -> Self {
        Self::V6(value)
    }
}
#[doc = "Ipv4Net"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Ipv4Net(pub String);
impl std::ops::Deref for Ipv4Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Ipv4Net> for String {
    fn from(value: Ipv4Net) -> Self {
        value.0
    }
}
impl From<&Ipv4Net> for Ipv4Net {
    fn from(value: &Ipv4Net) -> Self {
        value.clone()
    }
}
impl From<String> for Ipv4Net {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Ipv4Net {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Ipv4Net {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Ipv6Net"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Ipv6Net(pub String);
impl std::ops::Deref for Ipv6Net {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Ipv6Net> for String {
    fn from(value: Ipv6Net) -> Self {
        value.0
    }
}
impl From<&Ipv6Net> for Ipv6Net {
    fn from(value: &Ipv6Net) -> Self {
        value.clone()
    }
}
impl From<String> for Ipv6Net {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Ipv6Net {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Ipv6Net {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "JankNames"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Animation Specification\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Animation Specification\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 1,"]
#[doc = "      \"minProperties\": 1,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"maxProperties\": 2,"]
#[doc = "      \"minProperties\": 2,"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JankNames {
    Variant0(String),
    Variant1(std::collections::HashMap<String, String>),
    Variant2(std::collections::HashMap<String, i64>),
}
impl From<&JankNames> for JankNames {
    fn from(value: &JankNames) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for JankNames {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self::Variant1(value)
    }
}
impl From<std::collections::HashMap<String, i64>> for JankNames {
    fn from(value: std::collections::HashMap<String, i64>) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "Never"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "false"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Never {}
impl From<&Never> for Never {
    fn from(value: &Never) -> Self {
        value.clone()
    }
}
#[doc = "NeverEver"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "false"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum NeverEver {}
impl From<&NeverEver> for NeverEver {
    fn from(value: &NeverEver) -> Self {
        value.clone()
    }
}
#[doc = "NullStringEnumWithUnknownFormat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"format\": \"?\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"a\","]
#[doc = "    \"b\","]
#[doc = "    \"c\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NullStringEnumWithUnknownFormat(pub Option<NullStringEnumWithUnknownFormatInner>);
impl std::ops::Deref for NullStringEnumWithUnknownFormat {
    type Target = Option<NullStringEnumWithUnknownFormatInner>;
    fn deref(&self) -> &Option<NullStringEnumWithUnknownFormatInner> {
        &self.0
    }
}
impl From<NullStringEnumWithUnknownFormat> for Option<NullStringEnumWithUnknownFormatInner> {
    fn from(value: NullStringEnumWithUnknownFormat) -> Self {
        value.0
    }
}
impl From<&NullStringEnumWithUnknownFormat> for NullStringEnumWithUnknownFormat {
    fn from(value: &NullStringEnumWithUnknownFormat) -> Self {
        value.clone()
    }
}
impl From<Option<NullStringEnumWithUnknownFormatInner>> for NullStringEnumWithUnknownFormat {
    fn from(value: Option<NullStringEnumWithUnknownFormatInner>) -> Self {
        Self(value)
    }
}
#[doc = "NullStringEnumWithUnknownFormatInner"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"?\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"a\","]
#[doc = "    \"b\","]
#[doc = "    \"c\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NullStringEnumWithUnknownFormatInner {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
}
impl From<&NullStringEnumWithUnknownFormatInner> for NullStringEnumWithUnknownFormatInner {
    fn from(value: &NullStringEnumWithUnknownFormatInner) -> Self {
        value.clone()
    }
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
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for NullStringEnumWithUnknownFormatInner {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NullStringEnumWithUnknownFormatInner {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NullStringEnumWithUnknownFormatInner {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "OneOfTypes"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"bar\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"bar\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"foo\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"foo\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OneOfTypes {
    #[serde(rename = "bar")]
    Bar(i64),
    #[serde(rename = "foo")]
    Foo(String),
}
impl From<&OneOfTypes> for OneOfTypes {
    fn from(value: &OneOfTypes) -> Self {
        value.clone()
    }
}
impl From<i64> for OneOfTypes {
    fn from(value: i64) -> Self {
        Self::Bar(value)
    }
}
#[doc = "ReferenceDef"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReferenceDef(pub String);
impl std::ops::Deref for ReferenceDef {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ReferenceDef> for String {
    fn from(value: ReferenceDef) -> Self {
        value.0
    }
}
impl From<&ReferenceDef> for ReferenceDef {
    fn from(value: &ReferenceDef) -> Self {
        value.clone()
    }
}
impl From<String> for ReferenceDef {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ReferenceDef {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for ReferenceDef {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "issue 280"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"issue 280\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/StringVersion\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ReferenceDef\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"$comment\": \"Mapping of mod name to the desired version\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum References {
    Variant0(Vec<String>),
    Variant1(std::collections::HashMap<String, ReferencesVariant1Value>),
}
impl From<&References> for References {
    fn from(value: &References) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for References {
    fn from(value: Vec<String>) -> Self {
        Self::Variant0(value)
    }
}
impl From<std::collections::HashMap<String, ReferencesVariant1Value>> for References {
    fn from(value: std::collections::HashMap<String, ReferencesVariant1Value>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "ReferencesVariant1Value"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/StringVersion\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ReferenceDef\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReferencesVariant1Value {
    StringVersion(StringVersion),
    ReferenceDef(ReferenceDef),
}
impl From<&ReferencesVariant1Value> for ReferencesVariant1Value {
    fn from(value: &ReferencesVariant1Value) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ReferencesVariant1Value {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::StringVersion(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::ReferenceDef(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for ReferencesVariant1Value {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReferencesVariant1Value {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReferencesVariant1Value {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for ReferencesVariant1Value {
    fn to_string(&self) -> String {
        match self {
            Self::StringVersion(x) => x.to_string(),
            Self::ReferenceDef(x) => x.to_string(),
        }
    }
}
impl From<StringVersion> for ReferencesVariant1Value {
    fn from(value: StringVersion) -> Self {
        Self::StringVersion(value)
    }
}
impl From<ReferenceDef> for ReferencesVariant1Value {
    fn from(value: ReferenceDef) -> Self {
        Self::ReferenceDef(value)
    }
}
#[doc = "ShouldBeExclusive"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"id\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"reference\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reference\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ShouldBeExclusive {
    Variant0 { id: String },
    Variant1 { reference: String },
}
impl From<&ShouldBeExclusive> for ShouldBeExclusive {
    fn from(value: &ShouldBeExclusive) -> Self {
        value.clone()
    }
}
#[doc = "StringVersion"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StringVersion(pub String);
impl std::ops::Deref for StringVersion {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<StringVersion> for String {
    fn from(value: StringVersion) -> Self {
        value.0
    }
}
impl From<&StringVersion> for StringVersion {
    fn from(value: &StringVersion) -> Self {
        value.clone()
    }
}
impl From<String> for StringVersion {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for StringVersion {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for StringVersion {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
fn main() {}
