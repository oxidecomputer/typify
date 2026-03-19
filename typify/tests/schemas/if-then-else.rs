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
#[doc = "Basic if/then/else with string discrimination"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Basic if/then/else with string discrimination\","]
#[doc = "  \"else\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"value\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"if\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"kind\": {"]
#[doc = "        \"const\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {}"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"then\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"value\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IfThenElseBasic {
    Variant0 {
        kind: IfThenElseBasicVariant0Kind,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        value: ::std::option::Option<f64>,
    },
    Variant1(IfThenElseBasicVariant1),
}
impl ::std::convert::From<IfThenElseBasicVariant1> for IfThenElseBasic {
    fn from(value: IfThenElseBasicVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`IfThenElseBasicVariant0Kind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\""]
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
pub enum IfThenElseBasicVariant0Kind {
    #[serde(rename = "number")]
    Number,
}
impl ::std::fmt::Display for IfThenElseBasicVariant0Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Number => f.write_str("number"),
        }
    }
}
impl ::std::str::FromStr for IfThenElseBasicVariant0Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IfThenElseBasicVariant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IfThenElseBasicVariant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IfThenElseBasicVariant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`IfThenElseBasicVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"kind\": {"]
#[doc = "                  \"const\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"not\": {"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"kind\": {"]
#[doc = "                    \"const\": \"number\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"value\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"kind\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"value\": {}"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"kind\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"not\": {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"kind\": {"]
#[doc = "                \"const\": \"number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"allOf\": ["]
#[doc = "          {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"kind\": {"]
#[doc = "                \"const\": \"number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"value\": {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IfThenElseBasicVariant1 {
    Variant0 {
        kind: IfThenElseBasicVariant1Variant0Kind,
    },
    Variant1 {
        kind: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        value: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`IfThenElseBasicVariant1Variant0Kind`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"number\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\""]
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
pub enum IfThenElseBasicVariant1Variant0Kind {
    #[serde(rename = "number")]
    Number,
}
impl ::std::fmt::Display for IfThenElseBasicVariant1Variant0Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Number => f.write_str("number"),
        }
    }
}
impl ::std::str::FromStr for IfThenElseBasicVariant1Variant0Kind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IfThenElseBasicVariant1Variant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IfThenElseBasicVariant1Variant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IfThenElseBasicVariant1Variant0Kind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "if/then without else"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"if/then without else\","]
#[doc = "  \"if\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"tag\": {"]
#[doc = "        \"const\": \"special\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"extra\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"tag\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"then\": {"]
#[doc = "    \"required\": ["]
#[doc = "      \"extra\""]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IfThenNoElse {
    Variant0(IfThenNoElseVariant0),
    Variant1(IfThenNoElseVariant1),
}
impl ::std::convert::From<IfThenNoElseVariant0> for IfThenNoElse {
    fn from(value: IfThenNoElseVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<IfThenNoElseVariant1> for IfThenNoElse {
    fn from(value: IfThenNoElseVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`IfThenNoElseVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"tag\": {"]
#[doc = "                  \"const\": \"special\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"required\": ["]
#[doc = "                \"extra\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"not\": {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"tag\": {"]
#[doc = "                \"const\": \"special\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"extra\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"tag\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"tag\": {"]
#[doc = "              \"const\": \"special\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"required\": ["]
#[doc = "            \"extra\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"not\": {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"tag\": {"]
#[doc = "              \"const\": \"special\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IfThenNoElseVariant0 {
    Variant0 {
        extra: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tag: ::std::option::Option<IfThenNoElseVariant0Variant0Tag>,
    },
    Variant1 {
        extra: i64,
    },
}
#[doc = "`IfThenNoElseVariant0Variant0Tag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"special\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\""]
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
pub enum IfThenNoElseVariant0Variant0Tag {
    #[serde(rename = "special")]
    Special,
}
impl ::std::fmt::Display for IfThenNoElseVariant0Variant0Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Special => f.write_str("special"),
        }
    }
}
impl ::std::str::FromStr for IfThenNoElseVariant0Variant0Tag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "special" => Ok(Self::Special),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IfThenNoElseVariant0Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IfThenNoElseVariant0Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IfThenNoElseVariant0Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`IfThenNoElseVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"allOf\": ["]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"tag\": {"]
#[doc = "                  \"const\": \"special\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"required\": ["]
#[doc = "                \"extra\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"not\": {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"tag\": {"]
#[doc = "                \"const\": \"special\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"extra\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"tag\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"properties\": {"]
#[doc = "          \"tag\": {"]
#[doc = "            \"const\": \"special\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"allOf\": ["]
#[doc = "          {"]
#[doc = "            \"properties\": {"]
#[doc = "              \"tag\": {"]
#[doc = "                \"const\": \"special\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"extra\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum IfThenNoElseVariant1 {
    Variant0 {
        extra: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tag: ::std::option::Option<IfThenNoElseVariant1Variant0Tag>,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        extra: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        tag: ::std::option::Option<::std::string::String>,
    },
}
#[doc = "`IfThenNoElseVariant1Variant0Tag`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"special\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\""]
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
pub enum IfThenNoElseVariant1Variant0Tag {
    #[serde(rename = "special")]
    Special,
}
impl ::std::fmt::Display for IfThenNoElseVariant1Variant0Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Special => f.write_str("special"),
        }
    }
}
impl ::std::str::FromStr for IfThenNoElseVariant1Variant0Tag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "special" => Ok(Self::Special),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IfThenNoElseVariant1Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for IfThenNoElseVariant1Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IfThenNoElseVariant1Variant0Tag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
fn main() {}
