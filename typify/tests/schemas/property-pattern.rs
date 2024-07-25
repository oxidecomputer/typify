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
#[doc = "AliasRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"named\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"named\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^ALIAS$\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AliasRule {
    pub content: Rule,
    pub named: bool,
    #[serde(rename = "type")]
    pub type_: AliasRuleType,
    pub value: String,
}
impl From<&AliasRule> for AliasRule {
    fn from(value: &AliasRule) -> Self {
        value.clone()
    }
}
#[doc = "AliasRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^ALIAS$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AliasRuleType(String);
impl std::ops::Deref for AliasRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AliasRuleType> for String {
    fn from(value: AliasRuleType) -> Self {
        value.0
    }
}
impl From<&AliasRuleType> for AliasRuleType {
    fn from(value: &AliasRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AliasRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^ALIAS$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^ALIAS$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AliasRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AliasRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AliasRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AliasRuleType {
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
#[doc = "BlankRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^BLANK$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlankRule {
    #[serde(rename = "type")]
    pub type_: BlankRuleType,
}
impl From<&BlankRule> for BlankRule {
    fn from(value: &BlankRule) -> Self {
        value.clone()
    }
}
#[doc = "BlankRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^BLANK$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BlankRuleType(String);
impl std::ops::Deref for BlankRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BlankRuleType> for String {
    fn from(value: BlankRuleType) -> Self {
        value.0
    }
}
impl From<&BlankRuleType> for BlankRuleType {
    fn from(value: &BlankRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for BlankRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^BLANK$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^BLANK$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for BlankRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BlankRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BlankRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for BlankRuleType {
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
#[doc = "ChoiceRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"members\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/rule\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^CHOICE$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChoiceRule {
    pub members: Vec<Rule>,
    #[serde(rename = "type")]
    pub type_: ChoiceRuleType,
}
impl From<&ChoiceRule> for ChoiceRule {
    fn from(value: &ChoiceRule) -> Self {
        value.clone()
    }
}
#[doc = "ChoiceRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^CHOICE$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChoiceRuleType(String);
impl std::ops::Deref for ChoiceRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ChoiceRuleType> for String {
    fn from(value: ChoiceRuleType) -> Self {
        value.0
    }
}
impl From<&ChoiceRuleType> for ChoiceRuleType {
    fn from(value: &ChoiceRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ChoiceRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^CHOICE$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^CHOICE$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ChoiceRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ChoiceRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ChoiceRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ChoiceRuleType {
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
#[doc = "FieldRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^FIELD$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldRule {
    pub content: Box<Rule>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: FieldRuleType,
}
impl From<&FieldRule> for FieldRule {
    fn from(value: &FieldRule) -> Self {
        value.clone()
    }
}
#[doc = "FieldRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^FIELD$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FieldRuleType(String);
impl std::ops::Deref for FieldRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<FieldRuleType> for String {
    fn from(value: FieldRuleType) -> Self {
        value.0
    }
}
impl From<&FieldRuleType> for FieldRuleType {
    fn from(value: &FieldRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FieldRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^FIELD$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^FIELD$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for FieldRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FieldRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FieldRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for FieldRuleType {
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
#[doc = "PatternRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"flags\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^PATTERN$\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PatternRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(rename = "type")]
    pub type_: PatternRuleType,
    pub value: String,
}
impl From<&PatternRule> for PatternRule {
    fn from(value: &PatternRule) -> Self {
        value.clone()
    }
}
#[doc = "PatternRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^PATTERN$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PatternRuleType(String);
impl std::ops::Deref for PatternRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<PatternRuleType> for String {
    fn from(value: PatternRuleType) -> Self {
        value.0
    }
}
impl From<&PatternRuleType> for PatternRuleType {
    fn from(value: &PatternRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PatternRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^PATTERN$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^PATTERN$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PatternRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PatternRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PatternRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PatternRuleType {
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
#[doc = "PrecRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^(PREC|PREC_LEFT|PREC_RIGHT|PREC_DYNAMIC)$\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"oneof\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrecRule {
    pub content: Box<Rule>,
    #[serde(rename = "type")]
    pub type_: PrecRuleType,
    pub value: serde_json::Value,
}
impl From<&PrecRule> for PrecRule {
    fn from(value: &PrecRule) -> Self {
        value.clone()
    }
}
#[doc = "PrecRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(PREC|PREC_LEFT|PREC_RIGHT|PREC_DYNAMIC)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrecRuleType(String);
impl std::ops::Deref for PrecRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<PrecRuleType> for String {
    fn from(value: PrecRuleType) -> Self {
        value.0
    }
}
impl From<&PrecRuleType> for PrecRuleType {
    fn from(value: &PrecRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrecRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(PREC|PREC_LEFT|PREC_RIGHT|PREC_DYNAMIC)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(PREC|PREC_LEFT|PREC_RIGHT|PREC_DYNAMIC)$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for PrecRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrecRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrecRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for PrecRuleType {
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
#[doc = "Repeat1Rule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^REPEAT1$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repeat1Rule {
    pub content: Box<Rule>,
    #[serde(rename = "type")]
    pub type_: Repeat1RuleType,
}
impl From<&Repeat1Rule> for Repeat1Rule {
    fn from(value: &Repeat1Rule) -> Self {
        value.clone()
    }
}
#[doc = "Repeat1RuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^REPEAT1$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Repeat1RuleType(String);
impl std::ops::Deref for Repeat1RuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Repeat1RuleType> for String {
    fn from(value: Repeat1RuleType) -> Self {
        value.0
    }
}
impl From<&Repeat1RuleType> for Repeat1RuleType {
    fn from(value: &Repeat1RuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Repeat1RuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^REPEAT1$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^REPEAT1$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Repeat1RuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Repeat1RuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Repeat1RuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Repeat1RuleType {
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
#[doc = "RepeatRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^REPEAT$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepeatRule {
    pub content: Box<Rule>,
    #[serde(rename = "type")]
    pub type_: RepeatRuleType,
}
impl From<&RepeatRule> for RepeatRule {
    fn from(value: &RepeatRule) -> Self {
        value.clone()
    }
}
#[doc = "RepeatRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^REPEAT$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepeatRuleType(String);
impl std::ops::Deref for RepeatRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepeatRuleType> for String {
    fn from(value: RepeatRuleType) -> Self {
        value.0
    }
}
impl From<&RepeatRuleType> for RepeatRuleType {
    fn from(value: &RepeatRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepeatRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^REPEAT$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^REPEAT$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepeatRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepeatRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepeatRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepeatRuleType {
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
#[doc = "Rule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/alias-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/blank-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/string-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/pattern-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/symbol-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/seq-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/choice-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/repeat1-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/repeat-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/token-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/field-rule\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/prec-rule\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Rule {
    AliasRule(Box<AliasRule>),
    BlankRule(BlankRule),
    StringRule(StringRule),
    PatternRule(PatternRule),
    SymbolRule(SymbolRule),
    SeqRule(SeqRule),
    ChoiceRule(ChoiceRule),
    Repeat1Rule(Repeat1Rule),
    RepeatRule(RepeatRule),
    TokenRule(TokenRule),
    FieldRule(FieldRule),
    PrecRule(PrecRule),
}
impl From<&Rule> for Rule {
    fn from(value: &Rule) -> Self {
        value.clone()
    }
}
impl From<Box<AliasRule>> for Rule {
    fn from(value: Box<AliasRule>) -> Self {
        Self::AliasRule(value)
    }
}
impl From<BlankRule> for Rule {
    fn from(value: BlankRule) -> Self {
        Self::BlankRule(value)
    }
}
impl From<StringRule> for Rule {
    fn from(value: StringRule) -> Self {
        Self::StringRule(value)
    }
}
impl From<PatternRule> for Rule {
    fn from(value: PatternRule) -> Self {
        Self::PatternRule(value)
    }
}
impl From<SymbolRule> for Rule {
    fn from(value: SymbolRule) -> Self {
        Self::SymbolRule(value)
    }
}
impl From<SeqRule> for Rule {
    fn from(value: SeqRule) -> Self {
        Self::SeqRule(value)
    }
}
impl From<ChoiceRule> for Rule {
    fn from(value: ChoiceRule) -> Self {
        Self::ChoiceRule(value)
    }
}
impl From<Repeat1Rule> for Rule {
    fn from(value: Repeat1Rule) -> Self {
        Self::Repeat1Rule(value)
    }
}
impl From<RepeatRule> for Rule {
    fn from(value: RepeatRule) -> Self {
        Self::RepeatRule(value)
    }
}
impl From<TokenRule> for Rule {
    fn from(value: TokenRule) -> Self {
        Self::TokenRule(value)
    }
}
impl From<FieldRule> for Rule {
    fn from(value: FieldRule) -> Self {
        Self::FieldRule(value)
    }
}
impl From<PrecRule> for Rule {
    fn from(value: PrecRule) -> Self {
        Self::PrecRule(value)
    }
}
#[doc = "SeqRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"members\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/rule\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^SEQ$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SeqRule {
    pub members: Vec<Rule>,
    #[serde(rename = "type")]
    pub type_: SeqRuleType,
}
impl From<&SeqRule> for SeqRule {
    fn from(value: &SeqRule) -> Self {
        value.clone()
    }
}
#[doc = "SeqRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^SEQ$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SeqRuleType(String);
impl std::ops::Deref for SeqRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SeqRuleType> for String {
    fn from(value: SeqRuleType) -> Self {
        value.0
    }
}
impl From<&SeqRuleType> for SeqRuleType {
    fn from(value: &SeqRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SeqRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^SEQ$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^SEQ$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SeqRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SeqRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SeqRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SeqRuleType {
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
#[doc = "StringRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^STRING$\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringRule {
    #[serde(rename = "type")]
    pub type_: StringRuleType,
    pub value: String,
}
impl From<&StringRule> for StringRule {
    fn from(value: &StringRule) -> Self {
        value.clone()
    }
}
#[doc = "StringRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^STRING$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StringRuleType(String);
impl std::ops::Deref for StringRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<StringRuleType> for String {
    fn from(value: StringRuleType) -> Self {
        value.0
    }
}
impl From<&StringRuleType> for StringRuleType {
    fn from(value: &StringRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for StringRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^STRING$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^STRING$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for StringRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StringRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StringRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for StringRuleType {
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
#[doc = "SymbolRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^SYMBOL$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SymbolRule {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SymbolRuleType,
}
impl From<&SymbolRule> for SymbolRule {
    fn from(value: &SymbolRule) -> Self {
        value.clone()
    }
}
#[doc = "SymbolRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^SYMBOL$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SymbolRuleType(String);
impl std::ops::Deref for SymbolRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SymbolRuleType> for String {
    fn from(value: SymbolRuleType) -> Self {
        value.0
    }
}
impl From<&SymbolRuleType> for SymbolRuleType {
    fn from(value: &SymbolRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SymbolRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^SYMBOL$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^SYMBOL$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SymbolRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SymbolRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SymbolRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SymbolRuleType {
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
#[doc = "TokenRule"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"$ref\": \"#/definitions/rule\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^(TOKEN|IMMEDIATE_TOKEN)$\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenRule {
    pub content: Box<Rule>,
    #[serde(rename = "type")]
    pub type_: TokenRuleType,
}
impl From<&TokenRule> for TokenRule {
    fn from(value: &TokenRule) -> Self {
        value.clone()
    }
}
#[doc = "TokenRuleType"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^(TOKEN|IMMEDIATE_TOKEN)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TokenRuleType(String);
impl std::ops::Deref for TokenRuleType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TokenRuleType> for String {
    fn from(value: TokenRuleType) -> Self {
        value.0
    }
}
impl From<&TokenRuleType> for TokenRuleType {
    fn from(value: &TokenRuleType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TokenRuleType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(TOKEN|IMMEDIATE_TOKEN)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(TOKEN|IMMEDIATE_TOKEN)$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TokenRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TokenRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TokenRuleType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TokenRuleType {
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
#[doc = "TreeSitterGrammarSpecification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"tree-sitter grammar specification\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"rules\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"conflicts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[a-zA-Z_]\\\\w*$\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"externals\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/rule\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"extras\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/rule\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"inherits\": {"]
#[doc = "      \"description\": \"the name of the parent grammar\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "    },"]
#[doc = "    \"inline\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"pattern\": \"^[a-zA-Z_]\\\\w*$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"the name of the grammar\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "    },"]
#[doc = "    \"precedences\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"$ref\": \"#/definitions/rule\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"rules\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-zA-Z_]\\\\w*$\": {"]
#[doc = "          \"$ref\": \"#/definitions/rule\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"supertypes\": {"]
#[doc = "      \"description\": \"A list of hidden rule names that should be considered supertypes in the generated node types file. See https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"the name of a rule in `rules` or `extras`\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"word\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TreeSitterGrammarSpecification {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<Vec<TreeSitterGrammarSpecificationConflictsItemItem>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub externals: Vec<Rule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extras: Vec<Rule>,
    #[doc = "the name of the parent grammar"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<TreeSitterGrammarSpecificationInherits>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inline: Vec<TreeSitterGrammarSpecificationInlineItem>,
    #[doc = "the name of the grammar"]
    pub name: TreeSitterGrammarSpecificationName,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precedences: Vec<Vec<Rule>>,
    pub rules: TreeSitterGrammarSpecificationRules,
    #[doc = "A list of hidden rule names that should be considered supertypes in the generated node types file. See https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word: Option<TreeSitterGrammarSpecificationWord>,
}
impl From<&TreeSitterGrammarSpecification> for TreeSitterGrammarSpecification {
    fn from(value: &TreeSitterGrammarSpecification) -> Self {
        value.clone()
    }
}
#[doc = "TreeSitterGrammarSpecificationConflictsItemItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TreeSitterGrammarSpecificationConflictsItemItem(String);
impl std::ops::Deref for TreeSitterGrammarSpecificationConflictsItemItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationConflictsItemItem> for String {
    fn from(value: TreeSitterGrammarSpecificationConflictsItemItem) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationConflictsItemItem>
    for TreeSitterGrammarSpecificationConflictsItemItem
{
    fn from(value: &TreeSitterGrammarSpecificationConflictsItemItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TreeSitterGrammarSpecificationConflictsItemItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z_]\\w*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TreeSitterGrammarSpecificationConflictsItemItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TreeSitterGrammarSpecificationConflictsItemItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TreeSitterGrammarSpecificationConflictsItemItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationConflictsItemItem {
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
#[doc = "the name of the parent grammar"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"the name of the parent grammar\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TreeSitterGrammarSpecificationInherits(String);
impl std::ops::Deref for TreeSitterGrammarSpecificationInherits {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationInherits> for String {
    fn from(value: TreeSitterGrammarSpecificationInherits) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationInherits> for TreeSitterGrammarSpecificationInherits {
    fn from(value: &TreeSitterGrammarSpecificationInherits) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TreeSitterGrammarSpecificationInherits {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z_]\\w*")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TreeSitterGrammarSpecificationInherits {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TreeSitterGrammarSpecificationInherits {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TreeSitterGrammarSpecificationInherits {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationInherits {
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
#[doc = "TreeSitterGrammarSpecificationInlineItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TreeSitterGrammarSpecificationInlineItem(String);
impl std::ops::Deref for TreeSitterGrammarSpecificationInlineItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationInlineItem> for String {
    fn from(value: TreeSitterGrammarSpecificationInlineItem) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationInlineItem> for TreeSitterGrammarSpecificationInlineItem {
    fn from(value: &TreeSitterGrammarSpecificationInlineItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TreeSitterGrammarSpecificationInlineItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z_]\\w*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TreeSitterGrammarSpecificationInlineItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TreeSitterGrammarSpecificationInlineItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TreeSitterGrammarSpecificationInlineItem {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationInlineItem {
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
#[doc = "the name of the grammar"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"the name of the grammar\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TreeSitterGrammarSpecificationName(String);
impl std::ops::Deref for TreeSitterGrammarSpecificationName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationName> for String {
    fn from(value: TreeSitterGrammarSpecificationName) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationName> for TreeSitterGrammarSpecificationName {
    fn from(value: &TreeSitterGrammarSpecificationName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TreeSitterGrammarSpecificationName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z_]\\w*")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TreeSitterGrammarSpecificationName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TreeSitterGrammarSpecificationName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TreeSitterGrammarSpecificationName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationName {
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
#[doc = "TreeSitterGrammarSpecificationRules"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$ref\": \"#/definitions/rule\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Serialize)]
pub struct TreeSitterGrammarSpecificationRules(std::collections::HashMap<String, Rule>);
impl std::ops::Deref for TreeSitterGrammarSpecificationRules {
    type Target = std::collections::HashMap<String, Rule>;
    fn deref(&self) -> &std::collections::HashMap<String, Rule> {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationRules> for std::collections::HashMap<String, Rule> {
    fn from(value: TreeSitterGrammarSpecificationRules) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationRules> for TreeSitterGrammarSpecificationRules {
    fn from(value: &TreeSitterGrammarSpecificationRules) -> Self {
        value.clone()
    }
}
struct TreeSitterGrammarSpecificationRulesVisitor {
    marker: std::marker::PhantomData<fn() -> TreeSitterGrammarSpecificationRules>,
}
impl TreeSitterGrammarSpecificationRulesVisitor {
    fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}
impl<'de> serde::de::Visitor<'de> for TreeSitterGrammarSpecificationRulesVisitor {
    type Value = TreeSitterGrammarSpecificationRules;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with keys matching the pattern '^[a-zA-Z_]\\w*$'")
    }
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut map = std::collections::HashMap::new();
        while let Some((key, value)) = access.next_entry()? {
            if regress::Regex::new("^[a-zA-Z_]\\w*$")
                .unwrap()
                .find(key)
                .is_none()
            {
                return Err(serde::de::Error::custom(format!(
                    "key '{}' doesn't match pattern '{}'",
                    key, "^[a-zA-Z_]\\w*$"
                )));
            }
            map.insert(key.to_string(), value);
        }
        Ok(TreeSitterGrammarSpecificationRules(map))
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TreeSitterGrammarSpecificationRulesVisitor::new())
    }
}
#[doc = "TreeSitterGrammarSpecificationWord"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TreeSitterGrammarSpecificationWord(String);
impl std::ops::Deref for TreeSitterGrammarSpecificationWord {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TreeSitterGrammarSpecificationWord> for String {
    fn from(value: TreeSitterGrammarSpecificationWord) -> Self {
        value.0
    }
}
impl From<&TreeSitterGrammarSpecificationWord> for TreeSitterGrammarSpecificationWord {
    fn from(value: &TreeSitterGrammarSpecificationWord) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TreeSitterGrammarSpecificationWord {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^[a-zA-Z_]\\w*")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TreeSitterGrammarSpecificationWord {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TreeSitterGrammarSpecificationWord {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TreeSitterGrammarSpecificationWord {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TreeSitterGrammarSpecificationWord {
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
