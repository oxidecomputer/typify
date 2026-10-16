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
#[doc = "`ComparisonOperator`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"=\","]
#[doc = "    \">\","]
#[doc = "    \"<\","]
#[doc = "    \"≥\","]
#[doc = "    \">=\","]
#[doc = "    \"≤\","]
#[doc = "    \"<=\","]
#[doc = "    \"≠\","]
#[doc = "    \"!=\""]
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
pub enum ComparisonOperator {
    #[serde(rename = "=")]
    X,
    #[serde(rename = ">")]
    X2,
    #[serde(rename = "<")]
    X3,
    #[serde(rename = "≥")]
    X4,
    #[serde(rename = ">=")]
    Xx,
    #[serde(rename = "≤")]
    X5,
    #[serde(rename = "<=")]
    Xx2,
    #[serde(rename = "≠")]
    X6,
    #[serde(rename = "!=")]
    Xx3,
}
impl ::std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str("="),
            Self::X2 => f.write_str(">"),
            Self::X3 => f.write_str("<"),
            Self::X4 => f.write_str("≥"),
            Self::Xx => f.write_str(">="),
            Self::X5 => f.write_str("≤"),
            Self::Xx2 => f.write_str("<="),
            Self::X6 => f.write_str("≠"),
            Self::Xx3 => f.write_str("!="),
        }
    }
}
impl ::std::str::FromStr for ComparisonOperator {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "=" => Ok(Self::X),
            ">" => Ok(Self::X2),
            "<" => Ok(Self::X3),
            "≥" => Ok(Self::X4),
            ">=" => Ok(Self::Xx),
            "≤" => Ok(Self::X5),
            "<=" => Ok(Self::Xx2),
            "≠" => Ok(Self::X6),
            "!=" => Ok(Self::Xx3),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ComparisonOperator {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ComparisonOperator {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
fn main() {}
