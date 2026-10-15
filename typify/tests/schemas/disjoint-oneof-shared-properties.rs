#![deny(warnings)]
#[doc = "`Elicitation`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Elicitation {
    Variant0 {
        #[serde(
            rename = "_meta",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        meta: ::std::option::Option<::serde_json::Value>,
        mode: ElicitationVariant0Mode,
        #[serde(rename = "requestedSchema")]
        requested_schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        #[serde(rename = "serverName")]
        server_name: ::std::string::String,
        #[serde(rename = "threadId")]
        thread_id: ::std::string::String,
    },
    Variant1 {
        #[serde(
            rename = "_meta",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        meta: ::std::option::Option<::serde_json::Value>,
        mode: ElicitationVariant1Mode,
        #[serde(rename = "requestedSchema")]
        requested_schema: ::serde_json::Value,
        #[serde(rename = "serverName")]
        server_name: ::std::string::String,
        #[serde(rename = "threadId")]
        thread_id: ::std::string::String,
    },
}
#[doc = "`ElicitationVariant0Mode`"]
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
pub enum ElicitationVariant0Mode {
    #[serde(rename = "form")]
    Form,
}
impl ::std::fmt::Display for ElicitationVariant0Mode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Form => f.write_str("form"),
        }
    }
}
impl ::std::str::FromStr for ElicitationVariant0Mode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "form" => Ok(Self::Form),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ElicitationVariant0Mode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElicitationVariant0Mode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ElicitationVariant1Mode`"]
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
pub enum ElicitationVariant1Mode {
    #[serde(rename = "openai/form")]
    OpenaiForm,
}
impl ::std::fmt::Display for ElicitationVariant1Mode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OpenaiForm => f.write_str("openai/form"),
        }
    }
}
impl ::std::str::FromStr for ElicitationVariant1Mode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "openai/form" => Ok(Self::OpenaiForm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ElicitationVariant1Mode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ElicitationVariant1Mode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = " Error types."]
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
fn main() {}
