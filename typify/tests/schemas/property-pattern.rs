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
#[doc = "`TestGrammarForPatternProperties`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"test grammar for pattern properties\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"rules\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rules\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-zA-Z_]\\\\w*$\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TestGrammarForPatternProperties {
    pub rules:
        ::std::collections::HashMap<TestGrammarForPatternPropertiesRulesKey, ::std::string::String>,
}
impl ::std::convert::From<&TestGrammarForPatternProperties> for TestGrammarForPatternProperties {
    fn from(value: &TestGrammarForPatternProperties) -> Self {
        value.clone()
    }
}
impl TestGrammarForPatternProperties {
    pub fn builder() -> builder::TestGrammarForPatternProperties {
        Default::default()
    }
}
#[doc = "`TestGrammarForPatternPropertiesRulesKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_]\\\\w*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TestGrammarForPatternPropertiesRulesKey(::std::string::String);
impl ::std::ops::Deref for TestGrammarForPatternPropertiesRulesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TestGrammarForPatternPropertiesRulesKey> for ::std::string::String {
    fn from(value: TestGrammarForPatternPropertiesRulesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TestGrammarForPatternPropertiesRulesKey>
    for TestGrammarForPatternPropertiesRulesKey
{
    fn from(value: &TestGrammarForPatternPropertiesRulesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TestGrammarForPatternPropertiesRulesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-zA-Z_]\\w*$").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z_]\\w*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TestGrammarForPatternPropertiesRulesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TestGrammarForPatternPropertiesRulesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TestGrammarForPatternPropertiesRulesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TestGrammarForPatternPropertiesRulesKey {
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct TestGrammarForPatternProperties {
        rules: ::std::result::Result<
            ::std::collections::HashMap<
                super::TestGrammarForPatternPropertiesRulesKey,
                ::std::string::String,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TestGrammarForPatternProperties {
        fn default() -> Self {
            Self {
                rules: Err("no value supplied for rules".to_string()),
            }
        }
    }
    impl TestGrammarForPatternProperties {
        pub fn rules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::TestGrammarForPatternPropertiesRulesKey,
                    ::std::string::String,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.rules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rules: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TestGrammarForPatternProperties>
        for super::TestGrammarForPatternProperties
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TestGrammarForPatternProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rules: value.rules?,
            })
        }
    }
    impl ::std::convert::From<super::TestGrammarForPatternProperties>
        for TestGrammarForPatternProperties
    {
        fn from(value: super::TestGrammarForPatternProperties) -> Self {
            Self {
                rules: Ok(value.rules),
            }
        }
    }
}
fn main() {}
