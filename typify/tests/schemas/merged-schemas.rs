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
#[doc = "BarProp"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bar\": {"]
#[doc = "      \"bar\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BarProp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar: Option<serde_json::Value>,
}
impl From<&BarProp> for BarProp {
    fn from(value: &BarProp) -> Self {
        value.clone()
    }
}
#[doc = "ButNotThat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"not\": {"]
#[doc = "    \"required\": ["]
#[doc = "      \"that\""]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"that\": {},"]
#[doc = "    \"this\": {}"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButNotThat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub this: Option<serde_json::Value>,
}
impl From<&ButNotThat> for ButNotThat {
    fn from(value: &ButNotThat) -> Self {
        value.clone()
    }
}
#[doc = "if we don't see this, we dropped the metadata"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"if we don't see this, we dropped the metadata\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"y\": true"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"x\": true"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommentedTypeMerged {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<serde_json::Value>,
}
impl From<&CommentedTypeMerged> for CommentedTypeMerged {
    fn from(value: &CommentedTypeMerged) -> Self {
        value.clone()
    }
}
#[doc = "JsonResponseBase"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"result\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonResponseBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl From<&JsonResponseBase> for JsonResponseBase {
    fn from(value: &JsonResponseBase) -> Self {
        value.clone()
    }
}
#[doc = "JsonSuccess"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/components/schemas/JsonSuccessBase\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"properties\": {"]
#[doc = "        \"msg\": {},"]
#[doc = "        \"result\": {}"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonSuccess {
    pub msg: String,
    pub result: JsonSuccessResult,
}
impl From<&JsonSuccess> for JsonSuccess {
    fn from(value: &JsonSuccess) -> Self {
        value.clone()
    }
}
#[doc = "x"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"x\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/components/schemas/JsonResponseBase\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"msg\","]
#[doc = "        \"result\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"msg\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"result\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"success\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonSuccessBase {
    pub msg: String,
    pub result: JsonSuccessBaseResult,
}
impl From<&JsonSuccessBase> for JsonSuccessBase {
    fn from(value: &JsonSuccessBase) -> Self {
        value.clone()
    }
}
#[doc = "JsonSuccessBaseResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JsonSuccessBaseResult {
    #[serde(rename = "success")]
    Success,
}
impl From<&JsonSuccessBaseResult> for JsonSuccessBaseResult {
    fn from(value: &JsonSuccessBaseResult) -> Self {
        value.clone()
    }
}
impl ToString for JsonSuccessBaseResult {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
        }
    }
}
impl std::str::FromStr for JsonSuccessBaseResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "JsonSuccessResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JsonSuccessResult {
    #[serde(rename = "success")]
    Success,
}
impl From<&JsonSuccessResult> for JsonSuccessResult {
    fn from(value: &JsonSuccessResult) -> Self {
        value.clone()
    }
}
impl ToString for JsonSuccessResult {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
        }
    }
}
impl std::str::FromStr for JsonSuccessResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "NarrowNumber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NarrowNumber(pub std::num::NonZeroU64);
impl std::ops::Deref for NarrowNumber {
    type Target = std::num::NonZeroU64;
    fn deref(&self) -> &std::num::NonZeroU64 {
        &self.0
    }
}
impl From<NarrowNumber> for std::num::NonZeroU64 {
    fn from(value: NarrowNumber) -> Self {
        value.0
    }
}
impl From<&NarrowNumber> for NarrowNumber {
    fn from(value: &NarrowNumber) -> Self {
        value.clone()
    }
}
impl From<std::num::NonZeroU64> for NarrowNumber {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for NarrowNumber {
    type Err = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for NarrowNumber {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "OrderDependentMerge"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/components/schemas/BarProp\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"properties\": {"]
#[doc = "        \"baz\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"baz\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderDependentMerge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bar: Option<serde_json::Value>,
    pub baz: bool,
}
impl From<&OrderDependentMerge> for OrderDependentMerge {
    fn from(value: &OrderDependentMerge) -> Self {
        value.clone()
    }
}
#[doc = "Pickingone"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/pickingone-installation\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"suspended_by\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"suspended_by\": {"]
#[doc = "          \"$ref\": \"#/definitions/pickingone-user\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"TODO this generates an extra type for the pickingone-user dependency\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pickingone {
    pub suspended_by: PickingoneSuspendedBy,
}
impl From<&Pickingone> for Pickingone {
    fn from(value: &Pickingone) -> Self {
        value.clone()
    }
}
#[doc = "PickingoneInstallation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"suspended_by\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/pickingone-user\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PickingoneInstallation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_by: Option<PickingoneUser>,
}
impl From<&PickingoneInstallation> for PickingoneInstallation {
    fn from(value: &PickingoneInstallation) -> Self {
        value.clone()
    }
}
#[doc = "PickingoneSuspendedBy"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"email\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/pickingone-user\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"type\": \"null\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PickingoneSuspendedBy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl From<&PickingoneSuspendedBy> for PickingoneSuspendedBy {
    fn from(value: &PickingoneSuspendedBy) -> Self {
        value.clone()
    }
}
#[doc = "PickingoneUser"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"email\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PickingoneUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl From<&PickingoneUser> for PickingoneUser {
    fn from(value: &PickingoneUser) -> Self {
        value.clone()
    }
}
#[doc = "TrimFat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"not\": {"]
#[doc = "    \"anyOf\": ["]
#[doc = "      {"]
#[doc = "        \"required\": ["]
#[doc = "          \"b\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"required\": ["]
#[doc = "          \"c\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"a\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"a\": {},"]
#[doc = "    \"b\": {},"]
#[doc = "    \"c\": {}"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrimFat {
    pub a: serde_json::Value,
}
impl From<&TrimFat> for TrimFat {
    fn from(value: &TrimFat) -> Self {
        value.clone()
    }
}
#[doc = "Unresolvable"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"x\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"x\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"a\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"x\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"x\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"b\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"required\": ["]
#[doc = "    \"x\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"x\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"c\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"subschemas all end up unresolvable\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Unresolvable {}
impl From<&Unresolvable> for Unresolvable {
    fn from(value: &Unresolvable) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"foo\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"bar\": {}"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable1 {}
impl From<&Unsatisfiable1> for Unsatisfiable1 {
    fn from(value: &Unsatisfiable1) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable2"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"foo\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"bar\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Unsatisfiable2 {}
impl From<&Unsatisfiable2> for Unsatisfiable2 {
    fn from(value: &Unsatisfiable2) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/unsatisfiable-3-a\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"$ref\": \"#/definitions/unsatisfiable-3-b\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsatisfiable3 {}
impl From<&Unsatisfiable3> for Unsatisfiable3 {
    fn from(value: &Unsatisfiable3) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3A"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"action\": {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/unsatisfiable-3-c\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsatisfiable3A {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Unsatisfiable3C>,
}
impl From<&Unsatisfiable3A> for Unsatisfiable3A {
    fn from(value: &Unsatisfiable3A) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3B"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bar\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Unsatisfiable3B {
    #[serde(rename = "bar")]
    Bar,
}
impl From<&Unsatisfiable3B> for Unsatisfiable3B {
    fn from(value: &Unsatisfiable3B) -> Self {
        value.clone()
    }
}
impl ToString for Unsatisfiable3B {
    fn to_string(&self) -> String {
        match *self {
            Self::Bar => "bar".to_string(),
        }
    }
}
impl std::str::FromStr for Unsatisfiable3B {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bar" => Ok(Self::Bar),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Unsatisfiable3C"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"foo\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Unsatisfiable3C {
    #[serde(rename = "foo")]
    Foo,
}
impl From<&Unsatisfiable3C> for Unsatisfiable3C {
    fn from(value: &Unsatisfiable3C) -> Self {
        value.clone()
    }
}
impl ToString for Unsatisfiable3C {
    fn to_string(&self) -> String {
        match *self {
            Self::Foo => "foo".to_string(),
        }
    }
}
impl std::str::FromStr for Unsatisfiable3C {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "foo" => Ok(Self::Foo),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "WeirdEnum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"patterns\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern-either\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern-regex\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pattern-either\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pattern-regex\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"patterns\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WeirdEnum {
    Variant0 {
        pattern: String,
    },
    Variant1 {
        patterns: String,
    },
    Variant2 {
        #[serde(rename = "pattern-either")]
        pattern_either: String,
    },
    Variant3 {
        #[serde(rename = "pattern-regex")]
        pattern_regex: String,
    },
}
impl From<&WeirdEnum> for WeirdEnum {
    fn from(value: &WeirdEnum) -> Self {
        value.clone()
    }
}
fn main() {}
