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
#[doc = "    \"where_not\","]
#[doc = "    \"why_not\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"where_not\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"enum\": ["]
#[doc = "          \"start\","]
#[doc = "          \"middle\","]
#[doc = "          \"end\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"why_not\": {"]
#[doc = "      \"not\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"because\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate a 'not' schema with typed- and untyped-subschemas\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestType {
    pub where_not: TestTypeWhereNot,
    pub why_not: TestTypeWhyNot,
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
#[doc = "`TestTypeWhereNot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"not\": {"]
#[doc = "    \"enum\": ["]
#[doc = "      \"start\","]
#[doc = "      \"middle\","]
#[doc = "      \"end\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TestTypeWhereNot(::std::string::String);
impl ::std::ops::Deref for TestTypeWhereNot {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TestTypeWhereNot> for ::std::string::String {
    fn from(value: TestTypeWhereNot) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TestTypeWhereNot> for TestTypeWhereNot {
    fn from(value: &TestTypeWhereNot) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestTypeWhereNot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ["start".to_string(), "middle".to_string(), "end".to_string()].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for TestTypeWhereNot {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<::std::string::String>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`TestTypeWhyNot`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"not\": {"]
#[doc = "    \"type\": \"string\","]
#[doc = "    \"enum\": ["]
#[doc = "      \"because\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TestTypeWhyNot(::std::string::String);
impl ::std::ops::Deref for TestTypeWhyNot {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TestTypeWhyNot> for ::std::string::String {
    fn from(value: TestTypeWhyNot) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TestTypeWhyNot> for TestTypeWhyNot {
    fn from(value: &TestTypeWhyNot) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestTypeWhyNot {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ["because".to_string()].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for TestTypeWhyNot {
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
    pub struct TestType {
        where_not: ::std::result::Result<super::TestTypeWhereNot, ::std::string::String>,
        why_not: ::std::result::Result<super::TestTypeWhyNot, ::std::string::String>,
    }
    impl ::std::default::Default for TestType {
        fn default() -> Self {
            Self {
                where_not: Err("no value supplied for where_not".to_string()),
                why_not: Err("no value supplied for why_not".to_string()),
            }
        }
    }
    impl TestType {
        pub fn where_not<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TestTypeWhereNot>,
            T::Error: ::std::fmt::Display,
        {
            self.where_not = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for where_not: {}", e));
            self
        }
        pub fn why_not<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TestTypeWhyNot>,
            T::Error: ::std::fmt::Display,
        {
            self.why_not = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for why_not: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TestType> for super::TestType {
        type Error = super::error::ConversionError;
        fn try_from(value: TestType) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                where_not: value.where_not?,
                why_not: value.why_not?,
            })
        }
    }
    impl ::std::convert::From<super::TestType> for TestType {
        fn from(value: super::TestType) -> Self {
            Self {
                where_not: Ok(value.where_not),
                why_not: Ok(value.why_not),
            }
        }
    }
}
fn main() {}
