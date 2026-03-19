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
#[doc = "`EmailAddress`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"format\": \"email\","]
#[doc = "  \"minLength\": 5,"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct EmailAddress(pub ::std::string::String);
impl ::std::ops::Deref for EmailAddress {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EmailAddress> for ::std::string::String {
    fn from(value: EmailAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for EmailAddress {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for EmailAddress {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`Person`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"dependencies\": {"]
#[doc = "    \"age\": ["]
#[doc = "      \"name\""]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"age\": {"]
#[doc = "      \"minimum\": 0,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"email\": {"]
#[doc = "      \"$ref\": \"#/$defs/email-address\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"email\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Person {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub age: ::std::option::Option<u64>,
    pub email: EmailAddress,
    pub name: ::std::string::String,
}
impl Person {
    pub fn builder() -> builder::Person {
        Default::default()
    }
}
#[doc = "2020-12: $ref alongside other keywords (wrapped in allOf)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"2020-12: $ref alongside other keywords (wrapped in allOf)\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"tag\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"$ref\": \"#/$defs/person\","]
#[doc = "      \"description\": \"A person with additional context\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaggedValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    #[doc = "A person with additional context"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<Person>,
}
impl ::std::default::Default for TaggedValue {
    fn default() -> Self {
        Self {
            tag: Default::default(),
            value: Default::default(),
        }
    }
}
impl TaggedValue {
    pub fn builder() -> builder::TaggedValue {
        Default::default()
    }
}
#[doc = "`Team`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/person\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"type\": \"array\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {},"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"members\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Team {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub members: ::std::vec::Vec<Person>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::serde_json::Value>,
    pub name: ::std::string::String,
}
impl Team {
    pub fn builder() -> builder::Team {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Person {
        age: ::std::result::Result<::std::option::Option<u64>, ::std::string::String>,
        email: ::std::result::Result<super::EmailAddress, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Person {
        fn default() -> Self {
            Self {
                age: Ok(Default::default()),
                email: Err("no value supplied for email".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Person {
        pub fn age<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u64>>,
            T::Error: ::std::fmt::Display,
        {
            self.age = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for age: {e}"));
            self
        }
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EmailAddress>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Person> for super::Person {
        type Error = super::error::ConversionError;
        fn try_from(value: Person) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                age: value.age?,
                email: value.email?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Person> for Person {
        fn from(value: super::Person) -> Self {
            Self {
                age: Ok(value.age),
                email: Ok(value.email),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaggedValue {
        tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<super::Person>, ::std::string::String>,
    }
    impl ::std::default::Default for TaggedValue {
        fn default() -> Self {
            Self {
                tag: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl TaggedValue {
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {e}"));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Person>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TaggedValue> for super::TaggedValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaggedValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tag: value.tag?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::TaggedValue> for TaggedValue {
        fn from(value: super::TaggedValue) -> Self {
            Self {
                tag: Ok(value.tag),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Team {
        members: ::std::result::Result<::std::vec::Vec<super::Person>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Team {
        fn default() -> Self {
            Self {
                members: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Team {
        pub fn members<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Person>>,
            T::Error: ::std::fmt::Display,
        {
            self.members = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for members: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Team> for super::Team {
        type Error = super::error::ConversionError;
        fn try_from(value: Team) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                members: value.members?,
                metadata: value.metadata?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Team> for Team {
        fn from(value: super::Team) -> Self {
            Self {
                members: Ok(value.members),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
            }
        }
    }
}
fn main() {}
