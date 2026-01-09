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
#[doc = "`TestNotPattern`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestNotPattern\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"key\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"key\": {"]
#[doc = "      \"description\": \"A unique identifier that must not be a UUID\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"not\": {"]
#[doc = "        \"pattern\": \"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A name that cannot be 'test'\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"not\": {"]
#[doc = "        \"pattern\": \"^test$\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestNotPattern {
    #[doc = "A unique identifier that must not be a UUID"]
    pub key: TestNotPatternKey,
    #[doc = "A name that cannot be 'test'"]
    pub name: TestNotPatternName,
}
impl TestNotPattern {
    pub fn builder() -> builder::TestNotPattern {
        Default::default()
    }
}
#[doc = "A unique identifier that must not be a UUID"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A unique identifier that must not be a UUID\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"not\": {"]
#[doc = "    \"pattern\": \"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TestNotPatternKey(::std::string::String);
impl ::std::ops::Deref for TestNotPatternKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TestNotPatternKey> for ::std::string::String {
    fn from(value: TestNotPatternKey) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestNotPatternKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        let pattern = regress::Regex::new(
            "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;
        if pattern.find(&value).is_some() {
            Err("value matches excluded pattern".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for TestNotPatternKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<::std::string::String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A name that cannot be 'test'"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A name that cannot be 'test'\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"not\": {"]
#[doc = "    \"pattern\": \"^test$\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TestNotPatternName(::std::string::String);
impl ::std::ops::Deref for TestNotPatternName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TestNotPatternName> for ::std::string::String {
    fn from(value: TestNotPatternName) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestNotPatternName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        let pattern =
            regress::Regex::new("^test$").map_err(|e| format!("Invalid regex pattern: {}", e))?;
        if pattern.find(&value).is_some() {
            Err("value matches excluded pattern".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for TestNotPatternName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<::std::string::String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct TestNotPattern {
        key: ::std::result::Result<super::TestNotPatternKey, ::std::string::String>,
        name: ::std::result::Result<super::TestNotPatternName, ::std::string::String>,
    }
    impl ::std::default::Default for TestNotPattern {
        fn default() -> Self {
            Self {
                key: Err("no value supplied for key".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl TestNotPattern {
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TestNotPatternKey>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TestNotPatternName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TestNotPattern> for super::TestNotPattern {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TestNotPattern,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key: value.key?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::TestNotPattern> for TestNotPattern {
        fn from(value: super::TestNotPattern) -> Self {
            Self {
                key: Ok(value.key),
                name: Ok(value.name),
            }
        }
    }
}
fn main() {}
