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
#[doc = "`Box`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Box {
    pub data: ::std::string::String,
}
impl ::std::convert::From<&Box> for Box {
    fn from(value: &Box) -> Self {
        value.clone()
    }
}
impl Box {
    pub fn builder() -> builder::Box {
        Default::default()
    }
}
#[doc = "`Copy`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Copy {
    pub value: i64,
}
impl ::std::convert::From<&Copy> for Copy {
    fn from(value: &Copy) -> Self {
        value.clone()
    }
}
impl Copy {
    pub fn builder() -> builder::Copy {
        Default::default()
    }
}
#[doc = "`DoubleOptionCollision`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"option\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"null\","]
#[doc = "        \"object\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"option\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"null\","]
#[doc = "            \"string\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DoubleOptionCollision {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option: ::std::option::Option<DoubleOptionCollisionOption>,
}
impl ::std::convert::From<&DoubleOptionCollision> for DoubleOptionCollision {
    fn from(value: &DoubleOptionCollision) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DoubleOptionCollision {
    fn default() -> Self {
        Self {
            option: Default::default(),
        }
    }
}
impl DoubleOptionCollision {
    pub fn builder() -> builder::DoubleOptionCollision {
        Default::default()
    }
}
#[doc = "`DoubleOptionCollisionOption`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"option\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"null\","]
#[doc = "        \"string\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DoubleOptionCollisionOption {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&DoubleOptionCollisionOption> for DoubleOptionCollisionOption {
    fn from(value: &DoubleOptionCollisionOption) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DoubleOptionCollisionOption {
    fn default() -> Self {
        Self {
            option: Default::default(),
        }
    }
}
impl DoubleOptionCollisionOption {
    pub fn builder() -> builder::DoubleOptionCollisionOption {
        Default::default()
    }
}
#[doc = "`Drop`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cleanup\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cleanup\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Drop {
    pub cleanup: bool,
}
impl ::std::convert::From<&Drop> for Drop {
    fn from(value: &Drop) -> Self {
        value.clone()
    }
}
impl Drop {
    pub fn builder() -> builder::Drop {
        Default::default()
    }
}
#[doc = "`FlattenedKeywords`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"normal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"normal\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FlattenedKeywords {
    pub normal: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&FlattenedKeywords> for FlattenedKeywords {
    fn from(value: &FlattenedKeywords) -> Self {
        value.clone()
    }
}
impl FlattenedKeywords {
    pub fn builder() -> builder::FlattenedKeywords {
        Default::default()
    }
}
#[doc = "`KeywordFieldsEnum`"]
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
#[doc = "        \"impl\","]
#[doc = "        \"match\","]
#[doc = "        \"ref\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"impl\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"match\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"ref\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 2,"]
#[doc = "      \"minItems\": 2"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum KeywordFieldsEnum {
    Variant0 {
        #[serde(rename = "impl")]
        impl_: ::std::string::String,
        #[serde(rename = "match")]
        match_: i64,
        #[serde(rename = "ref")]
        ref_: bool,
        #[serde(rename = "type")]
        type_: ::std::string::String,
    },
    Variant1([::std::string::String; 2usize]),
}
impl ::std::convert::From<&Self> for KeywordFieldsEnum {
    fn from(value: &KeywordFieldsEnum) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<[::std::string::String; 2usize]> for KeywordFieldsEnum {
    fn from(value: [::std::string::String; 2usize]) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`MapOfKeywords`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"keyword_map\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"keyword_map\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"type\","]
#[doc = "          \"impl\","]
#[doc = "          \"fn\","]
#[doc = "          \"let\","]
#[doc = "          \"match\","]
#[doc = "          \"mod\","]
#[doc = "          \"move\","]
#[doc = "          \"pub\","]
#[doc = "          \"ref\","]
#[doc = "          \"self\","]
#[doc = "          \"super\","]
#[doc = "          \"trait\","]
#[doc = "          \"use\","]
#[doc = "          \"where\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapOfKeywords {
    pub keyword_map:
        ::std::collections::HashMap<::std::string::String, MapOfKeywordsKeywordMapValue>,
}
impl ::std::convert::From<&MapOfKeywords> for MapOfKeywords {
    fn from(value: &MapOfKeywords) -> Self {
        value.clone()
    }
}
impl MapOfKeywords {
    pub fn builder() -> builder::MapOfKeywords {
        Default::default()
    }
}
#[doc = "`MapOfKeywordsKeywordMapValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"type\","]
#[doc = "    \"impl\","]
#[doc = "    \"fn\","]
#[doc = "    \"let\","]
#[doc = "    \"match\","]
#[doc = "    \"mod\","]
#[doc = "    \"move\","]
#[doc = "    \"pub\","]
#[doc = "    \"ref\","]
#[doc = "    \"self\","]
#[doc = "    \"super\","]
#[doc = "    \"trait\","]
#[doc = "    \"use\","]
#[doc = "    \"where\""]
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
pub enum MapOfKeywordsKeywordMapValue {
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "impl")]
    Impl,
    #[serde(rename = "fn")]
    Fn,
    #[serde(rename = "let")]
    Let,
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "mod")]
    Mod,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "pub")]
    Pub,
    #[serde(rename = "ref")]
    Ref,
    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "super")]
    Super,
    #[serde(rename = "trait")]
    Trait,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "where")]
    Where,
}
impl ::std::convert::From<&Self> for MapOfKeywordsKeywordMapValue {
    fn from(value: &MapOfKeywordsKeywordMapValue) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MapOfKeywordsKeywordMapValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Type => write!(f, "type"),
            Self::Impl => write!(f, "impl"),
            Self::Fn => write!(f, "fn"),
            Self::Let => write!(f, "let"),
            Self::Match => write!(f, "match"),
            Self::Mod => write!(f, "mod"),
            Self::Move => write!(f, "move"),
            Self::Pub => write!(f, "pub"),
            Self::Ref => write!(f, "ref"),
            Self::Self_ => write!(f, "self"),
            Self::Super => write!(f, "super"),
            Self::Trait => write!(f, "trait"),
            Self::Use => write!(f, "use"),
            Self::Where => write!(f, "where"),
        }
    }
}
impl ::std::str::FromStr for MapOfKeywordsKeywordMapValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "type" => Ok(Self::Type),
            "impl" => Ok(Self::Impl),
            "fn" => Ok(Self::Fn),
            "let" => Ok(Self::Let),
            "match" => Ok(Self::Match),
            "mod" => Ok(Self::Mod),
            "move" => Ok(Self::Move),
            "pub" => Ok(Self::Pub),
            "ref" => Ok(Self::Ref),
            "self" => Ok(Self::Self_),
            "super" => Ok(Self::Super),
            "trait" => Ok(Self::Trait),
            "use" => Ok(Self::Use),
            "where" => Ok(Self::Where),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`NestedTypeCollisions`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"types\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"option_type\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"null\","]
#[doc = "        \"object\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeWithOptionField\""]
#[doc = "    },"]
#[doc = "    \"types\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TypeWithOptionField\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NestedTypeCollisions {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_type: ::std::option::Option<NestedTypeCollisionsOptionType>,
    #[serde(rename = "type")]
    pub type_: TypeWithOptionField,
    pub types: ::std::vec::Vec<TypeWithOptionField>,
}
impl ::std::convert::From<&NestedTypeCollisions> for NestedTypeCollisions {
    fn from(value: &NestedTypeCollisions) -> Self {
        value.clone()
    }
}
impl NestedTypeCollisions {
    pub fn builder() -> builder::NestedTypeCollisions {
        Default::default()
    }
}
#[doc = "`NestedTypeCollisionsOptionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NestedTypeCollisionsOptionType {
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&NestedTypeCollisionsOptionType> for NestedTypeCollisionsOptionType {
    fn from(value: &NestedTypeCollisionsOptionType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for NestedTypeCollisionsOptionType {
    fn default() -> Self {
        Self {
            type_: Default::default(),
        }
    }
}
impl NestedTypeCollisionsOptionType {
    pub fn builder() -> builder::NestedTypeCollisionsOptionType {
        Default::default()
    }
}
#[doc = "`Option`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"maybe\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"maybe\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Option {
    pub maybe: ::std::string::String,
}
impl ::std::convert::From<&Option> for Option {
    fn from(value: &Option) -> Self {
        value.clone()
    }
}
impl Option {
    pub fn builder() -> builder::Option {
        Default::default()
    }
}
#[doc = "`Pin`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pointer\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pointer\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Pin {
    pub pointer: ::std::string::String,
}
impl ::std::convert::From<&Pin> for Pin {
    fn from(value: &Pin) -> Self {
        value.clone()
    }
}
impl Pin {
    pub fn builder() -> builder::Pin {
        Default::default()
    }
}
#[doc = "`RustKeywordMonster`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"abstract\","]
#[doc = "    \"as\","]
#[doc = "    \"async\","]
#[doc = "    \"await\","]
#[doc = "    \"become\","]
#[doc = "    \"box\","]
#[doc = "    \"break\","]
#[doc = "    \"const\","]
#[doc = "    \"continue\","]
#[doc = "    \"crate\","]
#[doc = "    \"do\","]
#[doc = "    \"dyn\","]
#[doc = "    \"else\","]
#[doc = "    \"enum\","]
#[doc = "    \"extern\","]
#[doc = "    \"false\","]
#[doc = "    \"final\","]
#[doc = "    \"fn\","]
#[doc = "    \"for\","]
#[doc = "    \"if\","]
#[doc = "    \"impl\","]
#[doc = "    \"in\","]
#[doc = "    \"let\","]
#[doc = "    \"loop\","]
#[doc = "    \"macro\","]
#[doc = "    \"match\","]
#[doc = "    \"mod\","]
#[doc = "    \"move\","]
#[doc = "    \"mut\","]
#[doc = "    \"override\","]
#[doc = "    \"priv\","]
#[doc = "    \"pub\","]
#[doc = "    \"ref\","]
#[doc = "    \"return\","]
#[doc = "    \"self\","]
#[doc = "    \"static\","]
#[doc = "    \"struct\","]
#[doc = "    \"super\","]
#[doc = "    \"trait\","]
#[doc = "    \"true\","]
#[doc = "    \"try\","]
#[doc = "    \"type\","]
#[doc = "    \"typeof\","]
#[doc = "    \"unsafe\","]
#[doc = "    \"unsized\","]
#[doc = "    \"use\","]
#[doc = "    \"virtual\","]
#[doc = "    \"where\","]
#[doc = "    \"while\","]
#[doc = "    \"yield\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"abstract\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"as\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"async\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"await\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"become\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"box\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"break\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"const\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"continue\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"crate\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"do\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"dyn\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"else\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"false\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"final\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fn\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"for\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"if\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"impl\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"in\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"let\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"loop\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"macro\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"match\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mod\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"move\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mut\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"override\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"priv\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pub\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"ref\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"return\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"self\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"static\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"struct\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"super\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trait\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"true\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"try\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"typeof\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"unsafe\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"unsized\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"virtual\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"where\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"while\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"yield\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RustKeywordMonster {
    #[serde(rename = "abstract")]
    pub abstract_: ::std::string::String,
    #[serde(rename = "as")]
    pub as_: ::std::string::String,
    #[serde(rename = "async")]
    pub async_: ::std::string::String,
    #[serde(rename = "await")]
    pub await_: ::std::string::String,
    #[serde(rename = "become")]
    pub become_: ::std::string::String,
    #[serde(rename = "box")]
    pub box_: ::std::string::String,
    #[serde(rename = "break")]
    pub break_: ::std::string::String,
    #[serde(rename = "const")]
    pub const_: ::std::string::String,
    #[serde(rename = "continue")]
    pub continue_: ::std::string::String,
    #[serde(rename = "crate")]
    pub crate_: ::std::string::String,
    #[serde(rename = "do")]
    pub do_: ::std::string::String,
    #[serde(rename = "dyn")]
    pub dyn_: ::std::string::String,
    #[serde(rename = "else")]
    pub else_: ::std::string::String,
    #[serde(rename = "enum")]
    pub enum_: ::std::string::String,
    #[serde(rename = "extern")]
    pub extern_: ::std::string::String,
    #[serde(rename = "false")]
    pub false_: ::std::string::String,
    #[serde(rename = "final")]
    pub final_: ::std::string::String,
    #[serde(rename = "fn")]
    pub fn_: ::std::string::String,
    #[serde(rename = "for")]
    pub for_: ::std::string::String,
    #[serde(rename = "if")]
    pub if_: ::std::string::String,
    #[serde(rename = "impl")]
    pub impl_: ::std::string::String,
    #[serde(rename = "in")]
    pub in_: ::std::string::String,
    #[serde(rename = "let")]
    pub let_: ::std::string::String,
    #[serde(rename = "loop")]
    pub loop_: ::std::string::String,
    #[serde(rename = "macro")]
    pub macro_: ::std::string::String,
    #[serde(rename = "match")]
    pub match_: ::std::string::String,
    #[serde(rename = "mod")]
    pub mod_: ::std::string::String,
    #[serde(rename = "move")]
    pub move_: ::std::string::String,
    #[serde(rename = "mut")]
    pub mut_: ::std::string::String,
    #[serde(rename = "override")]
    pub override_: ::std::string::String,
    #[serde(rename = "priv")]
    pub priv_: ::std::string::String,
    #[serde(rename = "pub")]
    pub pub_: ::std::string::String,
    #[serde(rename = "ref")]
    pub ref_: ::std::string::String,
    #[serde(rename = "return")]
    pub return_: ::std::string::String,
    #[serde(rename = "self")]
    pub self_: ::std::string::String,
    #[serde(rename = "static")]
    pub static_: ::std::string::String,
    #[serde(rename = "struct")]
    pub struct_: ::std::string::String,
    #[serde(rename = "super")]
    pub super_: ::std::string::String,
    #[serde(rename = "trait")]
    pub trait_: ::std::string::String,
    #[serde(rename = "true")]
    pub true_: ::std::string::String,
    #[serde(rename = "try")]
    pub try_: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[serde(rename = "typeof")]
    pub typeof_: ::std::string::String,
    #[serde(rename = "unsafe")]
    pub unsafe_: ::std::string::String,
    #[serde(rename = "unsized")]
    pub unsized_: ::std::string::String,
    #[serde(rename = "use")]
    pub use_: ::std::string::String,
    #[serde(rename = "virtual")]
    pub virtual_: ::std::string::String,
    #[serde(rename = "where")]
    pub where_: ::std::string::String,
    #[serde(rename = "while")]
    pub while_: ::std::string::String,
    #[serde(rename = "yield")]
    pub yield_: ::std::string::String,
}
impl ::std::convert::From<&RustKeywordMonster> for RustKeywordMonster {
    fn from(value: &RustKeywordMonster) -> Self {
        value.clone()
    }
}
impl RustKeywordMonster {
    pub fn builder() -> builder::RustKeywordMonster {
        Default::default()
    }
}
#[doc = "`Send`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Send {
    pub message: ::std::string::String,
}
impl ::std::convert::From<&Send> for Send {
    fn from(value: &Send) -> Self {
        value.clone()
    }
}
impl Send {
    pub fn builder() -> builder::Send {
        Default::default()
    }
}
#[doc = "`Std`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"boxed\","]
#[doc = "    \"convert\","]
#[doc = "    \"fmt\","]
#[doc = "    \"option\","]
#[doc = "    \"result\","]
#[doc = "    \"str\","]
#[doc = "    \"string\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"boxed\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"convert\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"fmt\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"option\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"str\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"string\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Std {
    pub boxed: StdBoxed,
    pub convert: StdConvert,
    pub fmt: StdFmt,
    pub option: StdOption,
    pub result: StdResult,
    pub str: StdStr,
    pub string: StdString,
}
impl ::std::convert::From<&Std> for Std {
    fn from(value: &Std) -> Self {
        value.clone()
    }
}
impl Std {
    pub fn builder() -> builder::Std {
        Default::default()
    }
}
#[doc = "`StdBoxed`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdBoxed {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdBoxed> for StdBoxed {
    fn from(value: &StdBoxed) -> Self {
        value.clone()
    }
}
impl StdBoxed {
    pub fn builder() -> builder::StdBoxed {
        Default::default()
    }
}
#[doc = "`StdConvert`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdConvert {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdConvert> for StdConvert {
    fn from(value: &StdConvert) -> Self {
        value.clone()
    }
}
impl StdConvert {
    pub fn builder() -> builder::StdConvert {
        Default::default()
    }
}
#[doc = "`StdFmt`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdFmt {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdFmt> for StdFmt {
    fn from(value: &StdFmt) -> Self {
        value.clone()
    }
}
impl StdFmt {
    pub fn builder() -> builder::StdFmt {
        Default::default()
    }
}
#[doc = "`StdOption`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdOption {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdOption> for StdOption {
    fn from(value: &StdOption) -> Self {
        value.clone()
    }
}
impl StdOption {
    pub fn builder() -> builder::StdOption {
        Default::default()
    }
}
#[doc = "`StdResult`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdResult {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdResult> for StdResult {
    fn from(value: &StdResult) -> Self {
        value.clone()
    }
}
impl StdResult {
    pub fn builder() -> builder::StdResult {
        Default::default()
    }
}
#[doc = "`StdStr`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdStr {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdStr> for StdStr {
    fn from(value: &StdStr) -> Self {
        value.clone()
    }
}
impl StdStr {
    pub fn builder() -> builder::StdStr {
        Default::default()
    }
}
#[doc = "`StdString`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StdString {
    pub value: ::std::string::String,
}
impl ::std::convert::From<&StdString> for StdString {
    fn from(value: &StdString) -> Self {
        value.clone()
    }
}
impl StdString {
    pub fn builder() -> builder::StdString {
        Default::default()
    }
}
#[doc = "`String`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct String {
    pub text: ::std::string::String,
}
impl ::std::convert::From<&String> for String {
    fn from(value: &String) -> Self {
        value.clone()
    }
}
impl String {
    pub fn builder() -> builder::String {
        Default::default()
    }
}
#[doc = "`StringEnum`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"one\","]
#[doc = "    \"two\","]
#[doc = "    \"three\""]
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
pub enum StringEnum {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "two")]
    Two,
    #[serde(rename = "three")]
    Three,
}
impl ::std::convert::From<&Self> for StringEnum {
    fn from(value: &StringEnum) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StringEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::One => write!(f, "one"),
            Self::Two => write!(f, "two"),
            Self::Three => write!(f, "three"),
        }
    }
}
impl ::std::str::FromStr for StringEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "one" => Ok(Self::One),
            "two" => Ok(Self::Two),
            "three" => Ok(Self::Three),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StringNewtype`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StringNewtype(::std::string::String);
impl ::std::ops::Deref for StringNewtype {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StringNewtype> for ::std::string::String {
    fn from(value: StringNewtype) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StringNewtype> for StringNewtype {
    fn from(value: &StringNewtype) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StringNewtype {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for StringNewtype {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StringNewtype {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StringNewtype {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for StringNewtype {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`Sync`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Sync {
    pub data: ::std::string::String,
}
impl ::std::convert::From<&Sync> for Sync {
    fn from(value: &Sync) -> Self {
        value.clone()
    }
}
impl Sync {
    pub fn builder() -> builder::Sync {
        Default::default()
    }
}
#[doc = "`TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Test schema with various definitions, type names, and properties that likely conflict with built-in Rust types and keywords\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords(
    pub ::serde_json::Value,
);
impl :: std :: ops :: Deref for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { type Target = :: serde_json :: Value ; fn deref (& self) -> & :: serde_json :: Value { & self . 0 } }
impl :: std :: convert :: From < TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords > for :: serde_json :: Value { fn from (value : TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords) -> Self { value . 0 } }
impl :: std :: convert :: From < & TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords > for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { fn from (value : & TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords) -> Self { value . clone () } }
impl :: std :: convert :: From < :: serde_json :: Value > for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { fn from (value : :: serde_json :: Value) -> Self { Self (value) } }
#[doc = "`TypeWithOptionField`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"boxed_field\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"boxed_field\": {"]
#[doc = "      \"$ref\": \"#/definitions/Box\""]
#[doc = "    },"]
#[doc = "    \"optional_field\": {"]
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
pub struct TypeWithOptionField {
    pub boxed_field: Box,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub optional_field: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TypeWithOptionField> for TypeWithOptionField {
    fn from(value: &TypeWithOptionField) -> Self {
        value.clone()
    }
}
impl TypeWithOptionField {
    pub fn builder() -> builder::TypeWithOptionField {
        Default::default()
    }
}
#[doc = "`Vec`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Vec {
    pub items: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&Vec> for Vec {
    fn from(value: &Vec) -> Self {
        value.clone()
    }
}
impl Vec {
    pub fn builder() -> builder::Vec {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Box {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Box {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl Box {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Box> for super::Box {
        type Error = super::error::ConversionError;
        fn try_from(value: Box) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::Box> for Box {
        fn from(value: super::Box) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Copy {
        value: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for Copy {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Copy {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Copy> for super::Copy {
        type Error = super::error::ConversionError;
        fn try_from(value: Copy) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Copy> for Copy {
        fn from(value: super::Copy) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DoubleOptionCollision {
        option: ::std::result::Result<
            ::std::option::Option<super::DoubleOptionCollisionOption>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DoubleOptionCollision {
        fn default() -> Self {
            Self {
                option: Ok(Default::default()),
            }
        }
    }
    impl DoubleOptionCollision {
        pub fn option<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DoubleOptionCollisionOption>>,
            T::Error: ::std::fmt::Display,
        {
            self.option = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DoubleOptionCollision> for super::DoubleOptionCollision {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DoubleOptionCollision,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                option: value.option?,
            })
        }
    }
    impl ::std::convert::From<super::DoubleOptionCollision> for DoubleOptionCollision {
        fn from(value: super::DoubleOptionCollision) -> Self {
            Self {
                option: Ok(value.option),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DoubleOptionCollisionOption {
        option: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DoubleOptionCollisionOption {
        fn default() -> Self {
            Self {
                option: Ok(Default::default()),
            }
        }
    }
    impl DoubleOptionCollisionOption {
        pub fn option<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.option = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DoubleOptionCollisionOption> for super::DoubleOptionCollisionOption {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DoubleOptionCollisionOption,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                option: value.option?,
            })
        }
    }
    impl ::std::convert::From<super::DoubleOptionCollisionOption> for DoubleOptionCollisionOption {
        fn from(value: super::DoubleOptionCollisionOption) -> Self {
            Self {
                option: Ok(value.option),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Drop {
        cleanup: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for Drop {
        fn default() -> Self {
            Self {
                cleanup: Err("no value supplied for cleanup".to_string()),
            }
        }
    }
    impl Drop {
        pub fn cleanup<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.cleanup = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cleanup: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Drop> for super::Drop {
        type Error = super::error::ConversionError;
        fn try_from(value: Drop) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cleanup: value.cleanup?,
            })
        }
    }
    impl ::std::convert::From<super::Drop> for Drop {
        fn from(value: super::Drop) -> Self {
            Self {
                cleanup: Ok(value.cleanup),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FlattenedKeywords {
        normal: ::std::result::Result<::std::string::String, ::std::string::String>,
        extra: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FlattenedKeywords {
        fn default() -> Self {
            Self {
                normal: Err("no value supplied for normal".to_string()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl FlattenedKeywords {
        pub fn normal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.normal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for normal: {}", e));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FlattenedKeywords> for super::FlattenedKeywords {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FlattenedKeywords,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                normal: value.normal?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::FlattenedKeywords> for FlattenedKeywords {
        fn from(value: super::FlattenedKeywords) -> Self {
            Self {
                normal: Ok(value.normal),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapOfKeywords {
        keyword_map: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::MapOfKeywordsKeywordMapValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MapOfKeywords {
        fn default() -> Self {
            Self {
                keyword_map: Err("no value supplied for keyword_map".to_string()),
            }
        }
    }
    impl MapOfKeywords {
        pub fn keyword_map<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::MapOfKeywordsKeywordMapValue,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.keyword_map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keyword_map: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MapOfKeywords> for super::MapOfKeywords {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapOfKeywords,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                keyword_map: value.keyword_map?,
            })
        }
    }
    impl ::std::convert::From<super::MapOfKeywords> for MapOfKeywords {
        fn from(value: super::MapOfKeywords) -> Self {
            Self {
                keyword_map: Ok(value.keyword_map),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NestedTypeCollisions {
        option_type: ::std::result::Result<
            ::std::option::Option<super::NestedTypeCollisionsOptionType>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::TypeWithOptionField, ::std::string::String>,
        types: ::std::result::Result<
            ::std::vec::Vec<super::TypeWithOptionField>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NestedTypeCollisions {
        fn default() -> Self {
            Self {
                option_type: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                types: Err("no value supplied for types".to_string()),
            }
        }
    }
    impl NestedTypeCollisions {
        pub fn option_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NestedTypeCollisionsOptionType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.option_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option_type: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeWithOptionField>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TypeWithOptionField>>,
            T::Error: ::std::fmt::Display,
        {
            self.types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for types: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NestedTypeCollisions> for super::NestedTypeCollisions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NestedTypeCollisions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                option_type: value.option_type?,
                type_: value.type_?,
                types: value.types?,
            })
        }
    }
    impl ::std::convert::From<super::NestedTypeCollisions> for NestedTypeCollisions {
        fn from(value: super::NestedTypeCollisions) -> Self {
            Self {
                option_type: Ok(value.option_type),
                type_: Ok(value.type_),
                types: Ok(value.types),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NestedTypeCollisionsOptionType {
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NestedTypeCollisionsOptionType {
        fn default() -> Self {
            Self {
                type_: Ok(Default::default()),
            }
        }
    }
    impl NestedTypeCollisionsOptionType {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<NestedTypeCollisionsOptionType>
        for super::NestedTypeCollisionsOptionType
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NestedTypeCollisionsOptionType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::NestedTypeCollisionsOptionType>
        for NestedTypeCollisionsOptionType
    {
        fn from(value: super::NestedTypeCollisionsOptionType) -> Self {
            Self {
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Option {
        maybe: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Option {
        fn default() -> Self {
            Self {
                maybe: Err("no value supplied for maybe".to_string()),
            }
        }
    }
    impl Option {
        pub fn maybe<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.maybe = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maybe: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Option> for super::Option {
        type Error = super::error::ConversionError;
        fn try_from(value: Option) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                maybe: value.maybe?,
            })
        }
    }
    impl ::std::convert::From<super::Option> for Option {
        fn from(value: super::Option) -> Self {
            Self {
                maybe: Ok(value.maybe),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Pin {
        pointer: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Pin {
        fn default() -> Self {
            Self {
                pointer: Err("no value supplied for pointer".to_string()),
            }
        }
    }
    impl Pin {
        pub fn pointer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pointer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pointer: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Pin> for super::Pin {
        type Error = super::error::ConversionError;
        fn try_from(value: Pin) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                pointer: value.pointer?,
            })
        }
    }
    impl ::std::convert::From<super::Pin> for Pin {
        fn from(value: super::Pin) -> Self {
            Self {
                pointer: Ok(value.pointer),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RustKeywordMonster {
        abstract_: ::std::result::Result<::std::string::String, ::std::string::String>,
        as_: ::std::result::Result<::std::string::String, ::std::string::String>,
        async_: ::std::result::Result<::std::string::String, ::std::string::String>,
        await_: ::std::result::Result<::std::string::String, ::std::string::String>,
        become_: ::std::result::Result<::std::string::String, ::std::string::String>,
        box_: ::std::result::Result<::std::string::String, ::std::string::String>,
        break_: ::std::result::Result<::std::string::String, ::std::string::String>,
        const_: ::std::result::Result<::std::string::String, ::std::string::String>,
        continue_: ::std::result::Result<::std::string::String, ::std::string::String>,
        crate_: ::std::result::Result<::std::string::String, ::std::string::String>,
        do_: ::std::result::Result<::std::string::String, ::std::string::String>,
        dyn_: ::std::result::Result<::std::string::String, ::std::string::String>,
        else_: ::std::result::Result<::std::string::String, ::std::string::String>,
        enum_: ::std::result::Result<::std::string::String, ::std::string::String>,
        extern_: ::std::result::Result<::std::string::String, ::std::string::String>,
        false_: ::std::result::Result<::std::string::String, ::std::string::String>,
        final_: ::std::result::Result<::std::string::String, ::std::string::String>,
        fn_: ::std::result::Result<::std::string::String, ::std::string::String>,
        for_: ::std::result::Result<::std::string::String, ::std::string::String>,
        if_: ::std::result::Result<::std::string::String, ::std::string::String>,
        impl_: ::std::result::Result<::std::string::String, ::std::string::String>,
        in_: ::std::result::Result<::std::string::String, ::std::string::String>,
        let_: ::std::result::Result<::std::string::String, ::std::string::String>,
        loop_: ::std::result::Result<::std::string::String, ::std::string::String>,
        macro_: ::std::result::Result<::std::string::String, ::std::string::String>,
        match_: ::std::result::Result<::std::string::String, ::std::string::String>,
        mod_: ::std::result::Result<::std::string::String, ::std::string::String>,
        move_: ::std::result::Result<::std::string::String, ::std::string::String>,
        mut_: ::std::result::Result<::std::string::String, ::std::string::String>,
        override_: ::std::result::Result<::std::string::String, ::std::string::String>,
        priv_: ::std::result::Result<::std::string::String, ::std::string::String>,
        pub_: ::std::result::Result<::std::string::String, ::std::string::String>,
        ref_: ::std::result::Result<::std::string::String, ::std::string::String>,
        return_: ::std::result::Result<::std::string::String, ::std::string::String>,
        self_: ::std::result::Result<::std::string::String, ::std::string::String>,
        static_: ::std::result::Result<::std::string::String, ::std::string::String>,
        struct_: ::std::result::Result<::std::string::String, ::std::string::String>,
        super_: ::std::result::Result<::std::string::String, ::std::string::String>,
        trait_: ::std::result::Result<::std::string::String, ::std::string::String>,
        true_: ::std::result::Result<::std::string::String, ::std::string::String>,
        try_: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        typeof_: ::std::result::Result<::std::string::String, ::std::string::String>,
        unsafe_: ::std::result::Result<::std::string::String, ::std::string::String>,
        unsized_: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        virtual_: ::std::result::Result<::std::string::String, ::std::string::String>,
        where_: ::std::result::Result<::std::string::String, ::std::string::String>,
        while_: ::std::result::Result<::std::string::String, ::std::string::String>,
        yield_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RustKeywordMonster {
        fn default() -> Self {
            Self {
                abstract_: Err("no value supplied for abstract_".to_string()),
                as_: Err("no value supplied for as_".to_string()),
                async_: Err("no value supplied for async_".to_string()),
                await_: Err("no value supplied for await_".to_string()),
                become_: Err("no value supplied for become_".to_string()),
                box_: Err("no value supplied for box_".to_string()),
                break_: Err("no value supplied for break_".to_string()),
                const_: Err("no value supplied for const_".to_string()),
                continue_: Err("no value supplied for continue_".to_string()),
                crate_: Err("no value supplied for crate_".to_string()),
                do_: Err("no value supplied for do_".to_string()),
                dyn_: Err("no value supplied for dyn_".to_string()),
                else_: Err("no value supplied for else_".to_string()),
                enum_: Err("no value supplied for enum_".to_string()),
                extern_: Err("no value supplied for extern_".to_string()),
                false_: Err("no value supplied for false_".to_string()),
                final_: Err("no value supplied for final_".to_string()),
                fn_: Err("no value supplied for fn_".to_string()),
                for_: Err("no value supplied for for_".to_string()),
                if_: Err("no value supplied for if_".to_string()),
                impl_: Err("no value supplied for impl_".to_string()),
                in_: Err("no value supplied for in_".to_string()),
                let_: Err("no value supplied for let_".to_string()),
                loop_: Err("no value supplied for loop_".to_string()),
                macro_: Err("no value supplied for macro_".to_string()),
                match_: Err("no value supplied for match_".to_string()),
                mod_: Err("no value supplied for mod_".to_string()),
                move_: Err("no value supplied for move_".to_string()),
                mut_: Err("no value supplied for mut_".to_string()),
                override_: Err("no value supplied for override_".to_string()),
                priv_: Err("no value supplied for priv_".to_string()),
                pub_: Err("no value supplied for pub_".to_string()),
                ref_: Err("no value supplied for ref_".to_string()),
                return_: Err("no value supplied for return_".to_string()),
                self_: Err("no value supplied for self_".to_string()),
                static_: Err("no value supplied for static_".to_string()),
                struct_: Err("no value supplied for struct_".to_string()),
                super_: Err("no value supplied for super_".to_string()),
                trait_: Err("no value supplied for trait_".to_string()),
                true_: Err("no value supplied for true_".to_string()),
                try_: Err("no value supplied for try_".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                typeof_: Err("no value supplied for typeof_".to_string()),
                unsafe_: Err("no value supplied for unsafe_".to_string()),
                unsized_: Err("no value supplied for unsized_".to_string()),
                use_: Err("no value supplied for use_".to_string()),
                virtual_: Err("no value supplied for virtual_".to_string()),
                where_: Err("no value supplied for where_".to_string()),
                while_: Err("no value supplied for while_".to_string()),
                yield_: Err("no value supplied for yield_".to_string()),
            }
        }
    }
    impl RustKeywordMonster {
        pub fn abstract_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.abstract_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for abstract_: {}", e));
            self
        }
        pub fn as_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.as_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for as_: {}", e));
            self
        }
        pub fn async_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.async_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for async_: {}", e));
            self
        }
        pub fn await_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.await_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for await_: {}", e));
            self
        }
        pub fn become_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.become_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for become_: {}", e));
            self
        }
        pub fn box_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.box_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for box_: {}", e));
            self
        }
        pub fn break_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.break_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for break_: {}", e));
            self
        }
        pub fn const_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.const_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for const_: {}", e));
            self
        }
        pub fn continue_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.continue_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for continue_: {}", e));
            self
        }
        pub fn crate_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.crate_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for crate_: {}", e));
            self
        }
        pub fn do_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.do_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for do_: {}", e));
            self
        }
        pub fn dyn_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.dyn_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dyn_: {}", e));
            self
        }
        pub fn else_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.else_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for else_: {}", e));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn extern_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.extern_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extern_: {}", e));
            self
        }
        pub fn false_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.false_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for false_: {}", e));
            self
        }
        pub fn final_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.final_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for final_: {}", e));
            self
        }
        pub fn fn_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.fn_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fn_: {}", e));
            self
        }
        pub fn for_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.for_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for for_: {}", e));
            self
        }
        pub fn if_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.if_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for if_: {}", e));
            self
        }
        pub fn impl_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.impl_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for impl_: {}", e));
            self
        }
        pub fn in_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.in_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_: {}", e));
            self
        }
        pub fn let_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.let_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for let_: {}", e));
            self
        }
        pub fn loop_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.loop_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for loop_: {}", e));
            self
        }
        pub fn macro_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.macro_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for macro_: {}", e));
            self
        }
        pub fn match_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.match_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_: {}", e));
            self
        }
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
            self
        }
        pub fn move_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.move_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for move_: {}", e));
            self
        }
        pub fn mut_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.mut_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mut_: {}", e));
            self
        }
        pub fn override_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.override_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for override_: {}", e));
            self
        }
        pub fn priv_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.priv_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priv_: {}", e));
            self
        }
        pub fn pub_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.pub_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pub_: {}", e));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {}", e));
            self
        }
        pub fn return_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.return_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for return_: {}", e));
            self
        }
        pub fn self_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.self_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for self_: {}", e));
            self
        }
        pub fn static_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.static_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for static_: {}", e));
            self
        }
        pub fn struct_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.struct_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for struct_: {}", e));
            self
        }
        pub fn super_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.super_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for super_: {}", e));
            self
        }
        pub fn trait_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.trait_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trait_: {}", e));
            self
        }
        pub fn true_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.true_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for true_: {}", e));
            self
        }
        pub fn try_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.try_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for try_: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn typeof_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.typeof_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for typeof_: {}", e));
            self
        }
        pub fn unsafe_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.unsafe_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unsafe_: {}", e));
            self
        }
        pub fn unsized_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.unsized_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unsized_: {}", e));
            self
        }
        pub fn use_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.use_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for use_: {}", e));
            self
        }
        pub fn virtual_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.virtual_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for virtual_: {}", e));
            self
        }
        pub fn where_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.where_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for where_: {}", e));
            self
        }
        pub fn while_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.while_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for while_: {}", e));
            self
        }
        pub fn yield_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.yield_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for yield_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RustKeywordMonster> for super::RustKeywordMonster {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RustKeywordMonster,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abstract_: value.abstract_?,
                as_: value.as_?,
                async_: value.async_?,
                await_: value.await_?,
                become_: value.become_?,
                box_: value.box_?,
                break_: value.break_?,
                const_: value.const_?,
                continue_: value.continue_?,
                crate_: value.crate_?,
                do_: value.do_?,
                dyn_: value.dyn_?,
                else_: value.else_?,
                enum_: value.enum_?,
                extern_: value.extern_?,
                false_: value.false_?,
                final_: value.final_?,
                fn_: value.fn_?,
                for_: value.for_?,
                if_: value.if_?,
                impl_: value.impl_?,
                in_: value.in_?,
                let_: value.let_?,
                loop_: value.loop_?,
                macro_: value.macro_?,
                match_: value.match_?,
                mod_: value.mod_?,
                move_: value.move_?,
                mut_: value.mut_?,
                override_: value.override_?,
                priv_: value.priv_?,
                pub_: value.pub_?,
                ref_: value.ref_?,
                return_: value.return_?,
                self_: value.self_?,
                static_: value.static_?,
                struct_: value.struct_?,
                super_: value.super_?,
                trait_: value.trait_?,
                true_: value.true_?,
                try_: value.try_?,
                type_: value.type_?,
                typeof_: value.typeof_?,
                unsafe_: value.unsafe_?,
                unsized_: value.unsized_?,
                use_: value.use_?,
                virtual_: value.virtual_?,
                where_: value.where_?,
                while_: value.while_?,
                yield_: value.yield_?,
            })
        }
    }
    impl ::std::convert::From<super::RustKeywordMonster> for RustKeywordMonster {
        fn from(value: super::RustKeywordMonster) -> Self {
            Self {
                abstract_: Ok(value.abstract_),
                as_: Ok(value.as_),
                async_: Ok(value.async_),
                await_: Ok(value.await_),
                become_: Ok(value.become_),
                box_: Ok(value.box_),
                break_: Ok(value.break_),
                const_: Ok(value.const_),
                continue_: Ok(value.continue_),
                crate_: Ok(value.crate_),
                do_: Ok(value.do_),
                dyn_: Ok(value.dyn_),
                else_: Ok(value.else_),
                enum_: Ok(value.enum_),
                extern_: Ok(value.extern_),
                false_: Ok(value.false_),
                final_: Ok(value.final_),
                fn_: Ok(value.fn_),
                for_: Ok(value.for_),
                if_: Ok(value.if_),
                impl_: Ok(value.impl_),
                in_: Ok(value.in_),
                let_: Ok(value.let_),
                loop_: Ok(value.loop_),
                macro_: Ok(value.macro_),
                match_: Ok(value.match_),
                mod_: Ok(value.mod_),
                move_: Ok(value.move_),
                mut_: Ok(value.mut_),
                override_: Ok(value.override_),
                priv_: Ok(value.priv_),
                pub_: Ok(value.pub_),
                ref_: Ok(value.ref_),
                return_: Ok(value.return_),
                self_: Ok(value.self_),
                static_: Ok(value.static_),
                struct_: Ok(value.struct_),
                super_: Ok(value.super_),
                trait_: Ok(value.trait_),
                true_: Ok(value.true_),
                try_: Ok(value.try_),
                type_: Ok(value.type_),
                typeof_: Ok(value.typeof_),
                unsafe_: Ok(value.unsafe_),
                unsized_: Ok(value.unsized_),
                use_: Ok(value.use_),
                virtual_: Ok(value.virtual_),
                where_: Ok(value.where_),
                while_: Ok(value.while_),
                yield_: Ok(value.yield_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Send {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Send {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl Send {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Send> for super::Send {
        type Error = super::error::ConversionError;
        fn try_from(value: Send) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::Send> for Send {
        fn from(value: super::Send) -> Self {
            Self {
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Std {
        boxed: ::std::result::Result<super::StdBoxed, ::std::string::String>,
        convert: ::std::result::Result<super::StdConvert, ::std::string::String>,
        fmt: ::std::result::Result<super::StdFmt, ::std::string::String>,
        option: ::std::result::Result<super::StdOption, ::std::string::String>,
        result: ::std::result::Result<super::StdResult, ::std::string::String>,
        str: ::std::result::Result<super::StdStr, ::std::string::String>,
        string: ::std::result::Result<super::StdString, ::std::string::String>,
    }
    impl ::std::default::Default for Std {
        fn default() -> Self {
            Self {
                boxed: Err("no value supplied for boxed".to_string()),
                convert: Err("no value supplied for convert".to_string()),
                fmt: Err("no value supplied for fmt".to_string()),
                option: Err("no value supplied for option".to_string()),
                result: Err("no value supplied for result".to_string()),
                str: Err("no value supplied for str".to_string()),
                string: Err("no value supplied for string".to_string()),
            }
        }
    }
    impl Std {
        pub fn boxed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdBoxed>,
            T::Error: ::std::fmt::Display,
        {
            self.boxed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boxed: {}", e));
            self
        }
        pub fn convert<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdConvert>,
            T::Error: ::std::fmt::Display,
        {
            self.convert = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for convert: {}", e));
            self
        }
        pub fn fmt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdFmt>,
            T::Error: ::std::fmt::Display,
        {
            self.fmt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fmt: {}", e));
            self
        }
        pub fn option<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdOption>,
            T::Error: ::std::fmt::Display,
        {
            self.option = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
        pub fn str<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdStr>,
            T::Error: ::std::fmt::Display,
        {
            self.str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for str: {}", e));
            self
        }
        pub fn string<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StdString>,
            T::Error: ::std::fmt::Display,
        {
            self.string = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for string: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Std> for super::Std {
        type Error = super::error::ConversionError;
        fn try_from(value: Std) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                boxed: value.boxed?,
                convert: value.convert?,
                fmt: value.fmt?,
                option: value.option?,
                result: value.result?,
                str: value.str?,
                string: value.string?,
            })
        }
    }
    impl ::std::convert::From<super::Std> for Std {
        fn from(value: super::Std) -> Self {
            Self {
                boxed: Ok(value.boxed),
                convert: Ok(value.convert),
                fmt: Ok(value.fmt),
                option: Ok(value.option),
                result: Ok(value.result),
                str: Ok(value.str),
                string: Ok(value.string),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdBoxed {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdBoxed {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdBoxed {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdBoxed> for super::StdBoxed {
        type Error = super::error::ConversionError;
        fn try_from(value: StdBoxed) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdBoxed> for StdBoxed {
        fn from(value: super::StdBoxed) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdConvert {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdConvert {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdConvert {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdConvert> for super::StdConvert {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StdConvert,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdConvert> for StdConvert {
        fn from(value: super::StdConvert) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdFmt {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdFmt {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdFmt {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdFmt> for super::StdFmt {
        type Error = super::error::ConversionError;
        fn try_from(value: StdFmt) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdFmt> for StdFmt {
        fn from(value: super::StdFmt) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdOption {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdOption {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdOption {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdOption> for super::StdOption {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StdOption,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdOption> for StdOption {
        fn from(value: super::StdOption) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdResult {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdResult {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdResult {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdResult> for super::StdResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StdResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdResult> for StdResult {
        fn from(value: super::StdResult) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdStr {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdStr {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdStr {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdStr> for super::StdStr {
        type Error = super::error::ConversionError;
        fn try_from(value: StdStr) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdStr> for StdStr {
        fn from(value: super::StdStr) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StdString {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StdString {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl StdString {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StdString> for super::StdString {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StdString,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::StdString> for StdString {
        fn from(value: super::StdString) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct String {
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for String {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl String {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<String> for super::String {
        type Error = super::error::ConversionError;
        fn try_from(value: String) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { text: value.text? })
        }
    }
    impl ::std::convert::From<super::String> for String {
        fn from(value: super::String) -> Self {
            Self {
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Sync {
        data: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Sync {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl Sync {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Sync> for super::Sync {
        type Error = super::error::ConversionError;
        fn try_from(value: Sync) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::Sync> for Sync {
        fn from(value: super::Sync) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeWithOptionField {
        boxed_field: ::std::result::Result<super::Box, ::std::string::String>,
        optional_field: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TypeWithOptionField {
        fn default() -> Self {
            Self {
                boxed_field: Err("no value supplied for boxed_field".to_string()),
                optional_field: Ok(Default::default()),
            }
        }
    }
    impl TypeWithOptionField {
        pub fn boxed_field<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Box>,
            T::Error: ::std::fmt::Display,
        {
            self.boxed_field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for boxed_field: {}", e));
            self
        }
        pub fn optional_field<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.optional_field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for optional_field: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TypeWithOptionField> for super::TypeWithOptionField {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeWithOptionField,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                boxed_field: value.boxed_field?,
                optional_field: value.optional_field?,
            })
        }
    }
    impl ::std::convert::From<super::TypeWithOptionField> for TypeWithOptionField {
        fn from(value: super::TypeWithOptionField) -> Self {
            Self {
                boxed_field: Ok(value.boxed_field),
                optional_field: Ok(value.optional_field),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Vec {
        items: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for Vec {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl Vec {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Vec> for super::Vec {
        type Error = super::error::ConversionError;
        fn try_from(value: Vec) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::Vec> for Vec {
        fn from(value: super::Vec) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
}
fn main() {}
