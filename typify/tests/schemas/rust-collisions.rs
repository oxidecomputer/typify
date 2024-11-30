#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
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
#[doc = "Box"]
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
impl From<&Box> for Box {
    fn from(value: &Box) -> Self {
        value.clone()
    }
}
#[doc = "Copy"]
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
impl From<&Copy> for Copy {
    fn from(value: &Copy) -> Self {
        value.clone()
    }
}
#[doc = "DoubleOptionCollision"]
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
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    pub option: ::std::option::Option<DoubleOptionCollisionOption>,
}
impl From<&DoubleOptionCollision> for DoubleOptionCollision {
    fn from(value: &DoubleOptionCollision) -> Self {
        value.clone()
    }
}
#[doc = "DoubleOptionCollisionOption"]
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
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    pub option: ::std::option::Option<::std::string::String>,
}
impl From<&DoubleOptionCollisionOption> for DoubleOptionCollisionOption {
    fn from(value: &DoubleOptionCollisionOption) -> Self {
        value.clone()
    }
}
#[doc = "Drop"]
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
impl From<&Drop> for Drop {
    fn from(value: &Drop) -> Self {
        value.clone()
    }
}
#[doc = "FlattenedKeywords"]
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
impl From<&FlattenedKeywords> for FlattenedKeywords {
    fn from(value: &FlattenedKeywords) -> Self {
        value.clone()
    }
}
#[doc = "KeywordFieldsEnum"]
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
impl From<&KeywordFieldsEnum> for KeywordFieldsEnum {
    fn from(value: &KeywordFieldsEnum) -> Self {
        value.clone()
    }
}
impl From<[::std::string::String; 2usize]> for KeywordFieldsEnum {
    fn from(value: [::std::string::String; 2usize]) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "MapOfKeywords"]
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
impl From<&MapOfKeywords> for MapOfKeywords {
    fn from(value: &MapOfKeywords) -> Self {
        value.clone()
    }
}
#[doc = "MapOfKeywordsKeywordMapValue"]
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
impl From<&MapOfKeywordsKeywordMapValue> for MapOfKeywordsKeywordMapValue {
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
impl std::str::FromStr for MapOfKeywordsKeywordMapValue {
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
impl std::convert::TryFrom<&str> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for MapOfKeywordsKeywordMapValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "NestedTypeCollisions"]
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
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    pub option_type: ::std::option::Option<NestedTypeCollisionsOptionType>,
    #[serde(rename = "type")]
    pub type_: TypeWithOptionField,
    pub types: ::std::vec::Vec<TypeWithOptionField>,
}
impl From<&NestedTypeCollisions> for NestedTypeCollisions {
    fn from(value: &NestedTypeCollisions) -> Self {
        value.clone()
    }
}
#[doc = "NestedTypeCollisionsOptionType"]
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
        skip_serializing_if = "std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl From<&NestedTypeCollisionsOptionType> for NestedTypeCollisionsOptionType {
    fn from(value: &NestedTypeCollisionsOptionType) -> Self {
        value.clone()
    }
}
#[doc = "Option"]
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
impl From<&Option> for Option {
    fn from(value: &Option) -> Self {
        value.clone()
    }
}
#[doc = "Pin"]
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
impl From<&Pin> for Pin {
    fn from(value: &Pin) -> Self {
        value.clone()
    }
}
#[doc = "RustKeywordMonster"]
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
impl From<&RustKeywordMonster> for RustKeywordMonster {
    fn from(value: &RustKeywordMonster) -> Self {
        value.clone()
    }
}
#[doc = "Send"]
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
impl From<&Send> for Send {
    fn from(value: &Send) -> Self {
        value.clone()
    }
}
#[doc = "Std"]
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
impl From<&Std> for Std {
    fn from(value: &Std) -> Self {
        value.clone()
    }
}
#[doc = "StdBoxed"]
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
impl From<&StdBoxed> for StdBoxed {
    fn from(value: &StdBoxed) -> Self {
        value.clone()
    }
}
#[doc = "StdConvert"]
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
impl From<&StdConvert> for StdConvert {
    fn from(value: &StdConvert) -> Self {
        value.clone()
    }
}
#[doc = "StdFmt"]
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
impl From<&StdFmt> for StdFmt {
    fn from(value: &StdFmt) -> Self {
        value.clone()
    }
}
#[doc = "StdOption"]
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
impl From<&StdOption> for StdOption {
    fn from(value: &StdOption) -> Self {
        value.clone()
    }
}
#[doc = "StdResult"]
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
impl From<&StdResult> for StdResult {
    fn from(value: &StdResult) -> Self {
        value.clone()
    }
}
#[doc = "StdStr"]
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
impl From<&StdStr> for StdStr {
    fn from(value: &StdStr) -> Self {
        value.clone()
    }
}
#[doc = "StdString"]
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
impl From<&StdString> for StdString {
    fn from(value: &StdString) -> Self {
        value.clone()
    }
}
#[doc = "String"]
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
impl From<&String> for String {
    fn from(value: &String) -> Self {
        value.clone()
    }
}
#[doc = "StringEnum"]
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
impl From<&StringEnum> for StringEnum {
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
impl std::str::FromStr for StringEnum {
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
impl std::convert::TryFrom<&str> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for StringEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "StringNewtype"]
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
pub struct StringNewtype(::std::string::String);
impl ::std::ops::Deref for StringNewtype {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl From<StringNewtype> for ::std::string::String {
    fn from(value: StringNewtype) -> Self {
        value.0
    }
}
impl From<&StringNewtype> for StringNewtype {
    fn from(value: &StringNewtype) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for StringNewtype {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() > 100usize {
            return Err("longer than 100 characters".into());
        }
        if value.len() < 1usize {
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
        Self::try_from(<::std::string::String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Sync"]
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
impl From<&Sync> for Sync {
    fn from(value: &Sync) -> Self {
        value.clone()
    }
}
#[doc = "TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords"]
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
pub struct TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords(
    pub ::serde_json::Value,
);
impl :: std :: ops :: Deref for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { type Target = :: serde_json :: Value ; fn deref (& self) -> & :: serde_json :: Value { & self . 0 } }
impl From < TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords > for :: serde_json :: Value { fn from (value : TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords) -> Self { value . 0 } }
impl From < & TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords > for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { fn from (value : & TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords) -> Self { value . clone () } }
impl From < :: serde_json :: Value > for TestSchemaWithVariousDefinitionsTypeNamesAndPropertiesThatLikelyConflictWithBuiltInRustTypesAndKeywords { fn from (value : :: serde_json :: Value) -> Self { Self (value) } }
#[doc = "TypeWithOptionField"]
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
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    pub optional_field: ::std::option::Option<::std::string::String>,
}
impl From<&TypeWithOptionField> for TypeWithOptionField {
    fn from(value: &TypeWithOptionField) -> Self {
        value.clone()
    }
}
#[doc = "Vec"]
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
impl From<&Vec> for Vec {
    fn from(value: &Vec) -> Self {
        value.clone()
    }
}
fn main() {}
