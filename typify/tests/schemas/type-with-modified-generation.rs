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
#[doc = "    \"converted_type\","]
#[doc = "    \"patched_type\","]
#[doc = "    \"replaced_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"converted_type\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        1,"]
#[doc = "        \"one\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"patched_type\": {"]
#[doc = "      \"$ref\": \"#/definitions/TypeThatNeedsMoreDerives\""]
#[doc = "    },"]
#[doc = "    \"replaced_type\": {"]
#[doc = "      \"$ref\": \"#/definitions/HandGeneratedType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate replacement, patch, and conversion settings\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestType {
    pub converted_type: serde_json::Value,
    pub patched_type: TypeThatHasMoreDerives,
    pub replaced_type: String,
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
#[doc = "`TypeThatHasMoreDerives`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct TypeThatHasMoreDerives(
    pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
);
impl ::std::ops::Deref for TypeThatHasMoreDerives {
    type Target = ::std::collections::HashMap<::std::string::String, ::std::string::String>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<TypeThatHasMoreDerives>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String>
{
    fn from(value: TypeThatHasMoreDerives) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeThatHasMoreDerives> for TypeThatHasMoreDerives {
    fn from(value: &TypeThatHasMoreDerives) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, ::std::string::String>>
    for TypeThatHasMoreDerives
{
    fn from(
        value: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self(value)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct TestType {
        converted_type: ::std::result::Result<serde_json::Value, ::std::string::String>,
        patched_type: ::std::result::Result<super::TypeThatHasMoreDerives, ::std::string::String>,
        replaced_type: ::std::result::Result<String, ::std::string::String>,
    }
    impl ::std::default::Default for TestType {
        fn default() -> Self {
            Self {
                converted_type: Err("no value supplied for converted_type".to_string()),
                patched_type: Err("no value supplied for patched_type".to_string()),
                replaced_type: Err("no value supplied for replaced_type".to_string()),
            }
        }
    }
    impl TestType {
        pub fn converted_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.converted_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for converted_type: {}", e));
            self
        }
        pub fn patched_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeThatHasMoreDerives>,
            T::Error: ::std::fmt::Display,
        {
            self.patched_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for patched_type: {}", e));
            self
        }
        pub fn replaced_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<String>,
            T::Error: ::std::fmt::Display,
        {
            self.replaced_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for replaced_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TestType> for super::TestType {
        type Error = super::error::ConversionError;
        fn try_from(value: TestType) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                converted_type: value.converted_type?,
                patched_type: value.patched_type?,
                replaced_type: value.replaced_type?,
            })
        }
    }
    impl ::std::convert::From<super::TestType> for TestType {
        fn from(value: super::TestType) -> Self {
            Self {
                converted_type: Ok(value.converted_type),
                patched_type: Ok(value.patched_type),
                replaced_type: Ok(value.replaced_type),
            }
        }
    }
}
fn main() {}
