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
#[doc = "`TestType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestType\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        null,"]
#[doc = "        \"start\","]
#[doc = "        \"middle\","]
#[doc = "        \"end\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate a type with no type and enum values that include a null\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestType {
    pub value: ::std::option::Option<TestTypeValue>,
}
impl ::std::convert::From<&TestType> for TestType {
    fn from(value: &TestType) -> Self {
        value.clone()
    }
}
impl TestType {
    pub fn builder() -> builder::TestType {
        Default::default()
    }
}
#[doc = "`TestTypeValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    null,"]
#[doc = "    \"start\","]
#[doc = "    \"middle\","]
#[doc = "    \"end\""]
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
pub enum TestTypeValue {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "end")]
    End,
}
impl ::std::convert::From<&Self> for TestTypeValue {
    fn from(value: &TestTypeValue) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TestTypeValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Start => write!(f, "start"),
            Self::Middle => write!(f, "middle"),
            Self::End => write!(f, "end"),
        }
    }
}
impl ::std::str::FromStr for TestTypeValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "start" => Ok(Self::Start),
            "middle" => Ok(Self::Middle),
            "end" => Ok(Self::End),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TestTypeValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TestTypeValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestTypeValue {
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
    pub struct TestType {
        value: ::std::result::Result<
            ::std::option::Option<super::TestTypeValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TestType {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl TestType {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TestTypeValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TestType> for super::TestType {
        type Error = super::error::ConversionError;
        fn try_from(value: TestType) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::TestType> for TestType {
        fn from(value: super::TestType) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
}
fn main() {}
