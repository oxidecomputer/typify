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
#[doc = "`LetterBox`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"LetterBox\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"letter\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"a\","]
#[doc = "        \"b\","]
#[doc = "        \"cee\""]
#[doc = "      ],"]
#[doc = "      \"maxLength\": 2"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LetterBox {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub letter: ::std::option::Option<LetterBoxLetter>,
}
impl ::std::convert::From<&LetterBox> for LetterBox {
    fn from(value: &LetterBox) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LetterBox {
    fn default() -> Self {
        Self {
            letter: Default::default(),
        }
    }
}
impl LetterBox {
    pub fn builder() -> builder::LetterBox {
        Default::default()
    }
}
#[doc = "`LetterBoxLetter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"a\","]
#[doc = "    \"b\","]
#[doc = "    \"cee\""]
#[doc = "  ],"]
#[doc = "  \"maxLength\": 2"]
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
pub enum LetterBoxLetter {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
}
impl ::std::convert::From<&Self> for LetterBoxLetter {
    fn from(value: &LetterBoxLetter) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LetterBoxLetter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
        }
    }
}
impl ::std::str::FromStr for LetterBoxLetter {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LetterBoxLetter {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LetterBoxLetter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LetterBoxLetter {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct LetterBox {
        letter: ::std::result::Result<
            ::std::option::Option<super::LetterBoxLetter>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LetterBox {
        fn default() -> Self {
            Self {
                letter: Ok(Default::default()),
            }
        }
    }
    impl LetterBox {
        pub fn letter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LetterBoxLetter>>,
            T::Error: ::std::fmt::Display,
        {
            self.letter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for letter: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LetterBox> for super::LetterBox {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LetterBox,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                letter: value.letter?,
            })
        }
    }
    impl ::std::convert::From<super::LetterBox> for LetterBox {
        fn from(value: super::LetterBox) -> Self {
            Self {
                letter: Ok(value.letter),
            }
        }
    }
}
fn main() {}
