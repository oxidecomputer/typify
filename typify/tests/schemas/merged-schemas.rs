#![deny(warnings)]
#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "`BarProp`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BarProp {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&BarProp> for BarProp {
    fn from(value: &BarProp) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BarProp {
    fn default() -> Self {
        Self {
            bar: Default::default(),
        }
    }
}
impl BarProp {
    pub fn builder() -> builder::BarProp {
        Default::default()
    }
}
#[doc = "`ButNotThat`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ButNotThat {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub this: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&ButNotThat> for ButNotThat {
    fn from(value: &ButNotThat) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ButNotThat {
    fn default() -> Self {
        Self {
            this: Default::default(),
        }
    }
}
impl ButNotThat {
    pub fn builder() -> builder::ButNotThat {
        Default::default()
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommentedTypeMerged {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub x: ::std::option::Option<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub y: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&CommentedTypeMerged> for CommentedTypeMerged {
    fn from(value: &CommentedTypeMerged) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CommentedTypeMerged {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl CommentedTypeMerged {
    pub fn builder() -> builder::CommentedTypeMerged {
        Default::default()
    }
}
#[doc = "`HereAndThere`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"foo\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"bar\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"baz\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum HereAndThere {
    Variant0 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        bar: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        foo: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        baz: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        foo: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for HereAndThere {
    fn from(value: &HereAndThere) -> Self {
        value.clone()
    }
}
#[doc = "`JsonResponseBase`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonResponseBase {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&JsonResponseBase> for JsonResponseBase {
    fn from(value: &JsonResponseBase) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for JsonResponseBase {
    fn default() -> Self {
        Self {
            result: Default::default(),
        }
    }
}
impl JsonResponseBase {
    pub fn builder() -> builder::JsonResponseBase {
        Default::default()
    }
}
#[doc = "`JsonSuccess`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonSuccess {
    pub msg: ::std::string::String,
    pub result: JsonSuccessResult,
}
impl ::std::convert::From<&JsonSuccess> for JsonSuccess {
    fn from(value: &JsonSuccess) -> Self {
        value.clone()
    }
}
impl JsonSuccess {
    pub fn builder() -> builder::JsonSuccess {
        Default::default()
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonSuccessBase {
    pub msg: ::std::string::String,
    pub result: JsonSuccessBaseResult,
}
impl ::std::convert::From<&JsonSuccessBase> for JsonSuccessBase {
    fn from(value: &JsonSuccessBase) -> Self {
        value.clone()
    }
}
impl JsonSuccessBase {
    pub fn builder() -> builder::JsonSuccessBase {
        Default::default()
    }
}
#[doc = "`JsonSuccessBaseResult`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonSuccessBaseResult {
    #[serde(rename = "success")]
    Success,
}
impl ::std::convert::From<&Self> for JsonSuccessBaseResult {
    fn from(value: &JsonSuccessBaseResult) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonSuccessBaseResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => write!(f, "success"),
        }
    }
}
impl ::std::str::FromStr for JsonSuccessBaseResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonSuccessBaseResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`JsonSuccessResult`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonSuccessResult {
    #[serde(rename = "success")]
    Success,
}
impl ::std::convert::From<&Self> for JsonSuccessResult {
    fn from(value: &JsonSuccessResult) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JsonSuccessResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Success => write!(f, "success"),
        }
    }
}
impl ::std::str::FromStr for JsonSuccessResult {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonSuccessResult {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`MergeEmpty`"]
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
#[doc = "        },"]
#[doc = "        \"token\": {"]
#[doc = "          \"type\": \"string\""]
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
#[doc = "      \"additionalProperties\": false,"]
#[doc = "      \"token\": {"]
#[doc = "        \"type\": \"integer\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"properties conflict but are not required so we end up with an empty object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MergeEmpty {}
impl ::std::convert::From<&MergeEmpty> for MergeEmpty {
    fn from(value: &MergeEmpty) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MergeEmpty {
    fn default() -> Self {
        Self {}
    }
}
impl MergeEmpty {
    pub fn builder() -> builder::MergeEmpty {
        Default::default()
    }
}
#[doc = "`NarrowNumber`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NarrowNumber(pub ::std::num::NonZeroU64);
impl ::std::ops::Deref for NarrowNumber {
    type Target = ::std::num::NonZeroU64;
    fn deref(&self) -> &::std::num::NonZeroU64 {
        &self.0
    }
}
impl ::std::convert::From<NarrowNumber> for ::std::num::NonZeroU64 {
    fn from(value: NarrowNumber) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NarrowNumber> for NarrowNumber {
    fn from(value: &NarrowNumber) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::num::NonZeroU64> for NarrowNumber {
    fn from(value: ::std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NarrowNumber {
    type Err = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NarrowNumber {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NarrowNumber {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NarrowNumber {
    type Error = <::std::num::NonZeroU64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NarrowNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`OrderDependentMerge`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OrderDependentMerge {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bar: ::std::option::Option<::serde_json::Value>,
    pub baz: bool,
}
impl ::std::convert::From<&OrderDependentMerge> for OrderDependentMerge {
    fn from(value: &OrderDependentMerge) -> Self {
        value.clone()
    }
}
impl OrderDependentMerge {
    pub fn builder() -> builder::OrderDependentMerge {
        Default::default()
    }
}
#[doc = "`Pickingone`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Pickingone {
    pub suspended_by: PickingoneSuspendedBy,
}
impl ::std::convert::From<&Pickingone> for Pickingone {
    fn from(value: &Pickingone) -> Self {
        value.clone()
    }
}
impl Pickingone {
    pub fn builder() -> builder::Pickingone {
        Default::default()
    }
}
#[doc = "`PickingoneInstallation`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PickingoneInstallation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub suspended_by: ::std::option::Option<PickingoneUser>,
}
impl ::std::convert::From<&PickingoneInstallation> for PickingoneInstallation {
    fn from(value: &PickingoneInstallation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PickingoneInstallation {
    fn default() -> Self {
        Self {
            suspended_by: Default::default(),
        }
    }
}
impl PickingoneInstallation {
    pub fn builder() -> builder::PickingoneInstallation {
        Default::default()
    }
}
#[doc = "`PickingoneSuspendedBy`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PickingoneSuspendedBy {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&PickingoneSuspendedBy> for PickingoneSuspendedBy {
    fn from(value: &PickingoneSuspendedBy) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PickingoneSuspendedBy {
    fn default() -> Self {
        Self {
            email: Default::default(),
        }
    }
}
impl PickingoneSuspendedBy {
    pub fn builder() -> builder::PickingoneSuspendedBy {
        Default::default()
    }
}
#[doc = "`PickingoneUser`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PickingoneUser {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&PickingoneUser> for PickingoneUser {
    fn from(value: &PickingoneUser) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PickingoneUser {
    fn default() -> Self {
        Self {
            email: Default::default(),
        }
    }
}
impl PickingoneUser {
    pub fn builder() -> builder::PickingoneUser {
        Default::default()
    }
}
#[doc = "`TrimFat`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TrimFat {
    pub a: ::serde_json::Value,
}
impl ::std::convert::From<&TrimFat> for TrimFat {
    fn from(value: &TrimFat) -> Self {
        value.clone()
    }
}
impl TrimFat {
    pub fn builder() -> builder::TrimFat {
        Default::default()
    }
}
#[doc = "`UnchangedByMerge`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tag\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tag\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"something\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"tag\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"tag\": {"]
#[doc = "            \"enum\": ["]
#[doc = "              \"something_else\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnchangedByMerge {
    pub tag: UnchangedByMergeTag,
}
impl ::std::convert::From<&UnchangedByMerge> for UnchangedByMerge {
    fn from(value: &UnchangedByMerge) -> Self {
        value.clone()
    }
}
impl UnchangedByMerge {
    pub fn builder() -> builder::UnchangedByMerge {
        Default::default()
    }
}
#[doc = "`UnchangedByMergeTag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"something\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum UnchangedByMergeTag {
    #[serde(rename = "something")]
    Something,
}
impl ::std::convert::From<&Self> for UnchangedByMergeTag {
    fn from(value: &UnchangedByMergeTag) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for UnchangedByMergeTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Something => write!(f, "something"),
        }
    }
}
impl ::std::str::FromStr for UnchangedByMergeTag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "something" => Ok(Self::Something),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for UnchangedByMergeTag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UnchangedByMergeTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UnchangedByMergeTag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Unresolvable`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum Unresolvable {}
impl ::std::convert::From<&Self> for Unresolvable {
    fn from(value: &Unresolvable) -> Self {
        value.clone()
    }
}
#[doc = "`Unsatisfiable1`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable1 {}
impl ::std::convert::From<&Self> for Unsatisfiable1 {
    fn from(value: &Unsatisfiable1) -> Self {
        value.clone()
    }
}
#[doc = "`Unsatisfiable2`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"action\""]
#[doc = "      ],"]
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
#[doc = "  ],"]
#[doc = "  \"$comment\": \"can't be satisfied because required properties conflict in their enum values\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable2 {}
impl ::std::convert::From<&Self> for Unsatisfiable2 {
    fn from(value: &Unsatisfiable2) -> Self {
        value.clone()
    }
}
#[doc = "`Unsatisfiable3`"]
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
#[doc = "      \"required\": ["]
#[doc = "        \"action\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"$ref\": \"#/definitions/unsatisfiable-3-b\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"tests a complex merge that can't be satisfied; it's basically the same as unsatisfiable-2, but is broken into multiple pieces\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable3 {}
impl ::std::convert::From<&Self> for Unsatisfiable3 {
    fn from(value: &Unsatisfiable3) -> Self {
        value.clone()
    }
}
#[doc = "`Unsatisfiable3A`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Unsatisfiable3A {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<Unsatisfiable3C>,
}
impl ::std::convert::From<&Unsatisfiable3A> for Unsatisfiable3A {
    fn from(value: &Unsatisfiable3A) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Unsatisfiable3A {
    fn default() -> Self {
        Self {
            action: Default::default(),
        }
    }
}
impl Unsatisfiable3A {
    pub fn builder() -> builder::Unsatisfiable3A {
        Default::default()
    }
}
#[doc = "`Unsatisfiable3B`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Unsatisfiable3B {
    #[serde(rename = "bar")]
    Bar,
}
impl ::std::convert::From<&Self> for Unsatisfiable3B {
    fn from(value: &Unsatisfiable3B) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Unsatisfiable3B {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bar => write!(f, "bar"),
        }
    }
}
impl ::std::str::FromStr for Unsatisfiable3B {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bar" => Ok(Self::Bar),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Unsatisfiable3B {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Unsatisfiable3C`"]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Unsatisfiable3C {
    #[serde(rename = "foo")]
    Foo,
}
impl ::std::convert::From<&Self> for Unsatisfiable3C {
    fn from(value: &Unsatisfiable3C) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Unsatisfiable3C {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Foo => write!(f, "foo"),
        }
    }
}
impl ::std::str::FromStr for Unsatisfiable3C {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "foo" => Ok(Self::Foo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Unsatisfiable3C {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WeirdEnum`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum WeirdEnum {
    Variant0 {
        pattern: ::std::string::String,
    },
    Variant1 {
        patterns: ::std::string::String,
    },
    Variant2 {
        #[serde(rename = "pattern-either")]
        pattern_either: ::std::string::String,
    },
    Variant3 {
        #[serde(rename = "pattern-regex")]
        pattern_regex: ::std::string::String,
    },
}
impl ::std::convert::From<&Self> for WeirdEnum {
    fn from(value: &WeirdEnum) -> Self {
        value.clone()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BarProp {
        bar: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BarProp {
        fn default() -> Self {
            Self {
                bar: Ok(Default::default()),
            }
        }
    }
    impl BarProp {
        pub fn bar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.bar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bar: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BarProp> for super::BarProp {
        type Error = super::error::ConversionError;
        fn try_from(value: BarProp) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { bar: value.bar? })
        }
    }
    impl ::std::convert::From<super::BarProp> for BarProp {
        fn from(value: super::BarProp) -> Self {
            Self { bar: Ok(value.bar) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ButNotThat {
        this: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ButNotThat {
        fn default() -> Self {
            Self {
                this: Ok(Default::default()),
            }
        }
    }
    impl ButNotThat {
        pub fn this<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.this = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for this: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ButNotThat> for super::ButNotThat {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ButNotThat,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { this: value.this? })
        }
    }
    impl ::std::convert::From<super::ButNotThat> for ButNotThat {
        fn from(value: super::ButNotThat) -> Self {
            Self {
                this: Ok(value.this),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommentedTypeMerged {
        x: ::std::result::Result<::std::option::Option<::serde_json::Value>, ::std::string::String>,
        y: ::std::result::Result<::std::option::Option<::serde_json::Value>, ::std::string::String>,
    }
    impl ::std::default::Default for CommentedTypeMerged {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl CommentedTypeMerged {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CommentedTypeMerged> for super::CommentedTypeMerged {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommentedTypeMerged,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl ::std::convert::From<super::CommentedTypeMerged> for CommentedTypeMerged {
        fn from(value: super::CommentedTypeMerged) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonResponseBase {
        result: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonResponseBase {
        fn default() -> Self {
            Self {
                result: Ok(Default::default()),
            }
        }
    }
    impl JsonResponseBase {
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonResponseBase> for super::JsonResponseBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonResponseBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonResponseBase> for JsonResponseBase {
        fn from(value: super::JsonResponseBase) -> Self {
            Self {
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonSuccess {
        msg: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::JsonSuccessResult, ::std::string::String>,
    }
    impl ::std::default::Default for JsonSuccess {
        fn default() -> Self {
            Self {
                msg: Err("no value supplied for msg".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl JsonSuccess {
        pub fn msg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.msg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for msg: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonSuccessResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonSuccess> for super::JsonSuccess {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonSuccess,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                msg: value.msg?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonSuccess> for JsonSuccess {
        fn from(value: super::JsonSuccess) -> Self {
            Self {
                msg: Ok(value.msg),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonSuccessBase {
        msg: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::JsonSuccessBaseResult, ::std::string::String>,
    }
    impl ::std::default::Default for JsonSuccessBase {
        fn default() -> Self {
            Self {
                msg: Err("no value supplied for msg".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl JsonSuccessBase {
        pub fn msg<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.msg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for msg: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonSuccessBaseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonSuccessBase> for super::JsonSuccessBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonSuccessBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                msg: value.msg?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonSuccessBase> for JsonSuccessBase {
        fn from(value: super::JsonSuccessBase) -> Self {
            Self {
                msg: Ok(value.msg),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MergeEmpty {}
    impl ::std::default::Default for MergeEmpty {
        fn default() -> Self {
            Self {}
        }
    }
    impl MergeEmpty {}
    impl ::std::convert::TryFrom<MergeEmpty> for super::MergeEmpty {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: MergeEmpty,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::MergeEmpty> for MergeEmpty {
        fn from(_value: super::MergeEmpty) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrderDependentMerge {
        bar: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        baz: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for OrderDependentMerge {
        fn default() -> Self {
            Self {
                bar: Ok(Default::default()),
                baz: Err("no value supplied for baz".to_string()),
            }
        }
    }
    impl OrderDependentMerge {
        pub fn bar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.bar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bar: {}", e));
            self
        }
        pub fn baz<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.baz = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for baz: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OrderDependentMerge> for super::OrderDependentMerge {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrderDependentMerge,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bar: value.bar?,
                baz: value.baz?,
            })
        }
    }
    impl ::std::convert::From<super::OrderDependentMerge> for OrderDependentMerge {
        fn from(value: super::OrderDependentMerge) -> Self {
            Self {
                bar: Ok(value.bar),
                baz: Ok(value.baz),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pickingone {
        suspended_by: ::std::result::Result<super::PickingoneSuspendedBy, ::std::string::String>,
    }
    impl ::std::default::Default for Pickingone {
        fn default() -> Self {
            Self {
                suspended_by: Err("no value supplied for suspended_by".to_string()),
            }
        }
    }
    impl Pickingone {
        pub fn suspended_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PickingoneSuspendedBy>,
            T::Error: ::std::fmt::Display,
        {
            self.suspended_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suspended_by: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Pickingone> for super::Pickingone {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Pickingone,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                suspended_by: value.suspended_by?,
            })
        }
    }
    impl ::std::convert::From<super::Pickingone> for Pickingone {
        fn from(value: super::Pickingone) -> Self {
            Self {
                suspended_by: Ok(value.suspended_by),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PickingoneInstallation {
        suspended_by: ::std::result::Result<
            ::std::option::Option<super::PickingoneUser>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PickingoneInstallation {
        fn default() -> Self {
            Self {
                suspended_by: Ok(Default::default()),
            }
        }
    }
    impl PickingoneInstallation {
        pub fn suspended_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PickingoneUser>>,
            T::Error: ::std::fmt::Display,
        {
            self.suspended_by = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suspended_by: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PickingoneInstallation> for super::PickingoneInstallation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PickingoneInstallation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                suspended_by: value.suspended_by?,
            })
        }
    }
    impl ::std::convert::From<super::PickingoneInstallation> for PickingoneInstallation {
        fn from(value: super::PickingoneInstallation) -> Self {
            Self {
                suspended_by: Ok(value.suspended_by),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PickingoneSuspendedBy {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PickingoneSuspendedBy {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
            }
        }
    }
    impl PickingoneSuspendedBy {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PickingoneSuspendedBy> for super::PickingoneSuspendedBy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PickingoneSuspendedBy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
            })
        }
    }
    impl ::std::convert::From<super::PickingoneSuspendedBy> for PickingoneSuspendedBy {
        fn from(value: super::PickingoneSuspendedBy) -> Self {
            Self {
                email: Ok(value.email),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PickingoneUser {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PickingoneUser {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
            }
        }
    }
    impl PickingoneUser {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PickingoneUser> for super::PickingoneUser {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PickingoneUser,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
            })
        }
    }
    impl ::std::convert::From<super::PickingoneUser> for PickingoneUser {
        fn from(value: super::PickingoneUser) -> Self {
            Self {
                email: Ok(value.email),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrimFat {
        a: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TrimFat {
        fn default() -> Self {
            Self {
                a: Err("no value supplied for a".to_string()),
            }
        }
    }
    impl TrimFat {
        pub fn a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for a: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TrimFat> for super::TrimFat {
        type Error = super::error::ConversionError;
        fn try_from(value: TrimFat) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { a: value.a? })
        }
    }
    impl ::std::convert::From<super::TrimFat> for TrimFat {
        fn from(value: super::TrimFat) -> Self {
            Self { a: Ok(value.a) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnchangedByMerge {
        tag: ::std::result::Result<super::UnchangedByMergeTag, ::std::string::String>,
    }
    impl ::std::default::Default for UnchangedByMerge {
        fn default() -> Self {
            Self {
                tag: Err("no value supplied for tag".to_string()),
            }
        }
    }
    impl UnchangedByMerge {
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UnchangedByMergeTag>,
            T::Error: ::std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UnchangedByMerge> for super::UnchangedByMerge {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnchangedByMerge,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { tag: value.tag? })
        }
    }
    impl ::std::convert::From<super::UnchangedByMerge> for UnchangedByMerge {
        fn from(value: super::UnchangedByMerge) -> Self {
            Self { tag: Ok(value.tag) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Unsatisfiable3A {
        action: ::std::result::Result<
            ::std::option::Option<super::Unsatisfiable3C>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Unsatisfiable3A {
        fn default() -> Self {
            Self {
                action: Ok(Default::default()),
            }
        }
    }
    impl Unsatisfiable3A {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Unsatisfiable3C>>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Unsatisfiable3A> for super::Unsatisfiable3A {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Unsatisfiable3A,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
            })
        }
    }
    impl ::std::convert::From<super::Unsatisfiable3A> for Unsatisfiable3A {
        fn from(value: super::Unsatisfiable3A) -> Self {
            Self {
                action: Ok(value.action),
            }
        }
    }
}
fn main() {}
