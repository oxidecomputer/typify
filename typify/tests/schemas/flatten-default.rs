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
#[doc = "`Container`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Container\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"thing\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"extra_key\": \"extra_value\","]
#[doc = "        \"id\": \"abc\""]
#[doc = "      },"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"regression test: a default value for a struct type with a flattened property must produce a valid Rust identifier for the flattened field\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Container {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thing: ::std::option::Option<ContainerThing>,
}
impl ::std::default::Default for Container {
    fn default() -> Self {
        Self {
            thing: Default::default(),
        }
    }
}
impl Container {
    pub fn builder() -> builder::Container {
        Default::default()
    }
}
#[doc = "`ContainerThing`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": {"]
#[doc = "    \"extra_key\": \"extra_value\","]
#[doc = "    \"id\": \"abc\""]
#[doc = "  },"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ContainerThing {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::default::Default for ContainerThing {
    fn default() -> Self {
        ContainerThing {
            id: ::std::option::Option::Some("abc".to_string()),
            extra: [("extra_key".to_string(), "extra_value".to_string())]
                .into_iter()
                .collect(),
        }
    }
}
impl ContainerThing {
    pub fn builder() -> builder::ContainerThing {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Container {
        thing: ::std::result::Result<
            ::std::option::Option<super::ContainerThing>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Container {
        fn default() -> Self {
            Self {
                thing: Ok(Default::default()),
            }
        }
    }
    impl Container {
        pub fn thing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ContainerThing>>,
            T::Error: ::std::fmt::Display,
        {
            self.thing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thing: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Container> for super::Container {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Container,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thing: value.thing?,
            })
        }
    }
    impl ::std::convert::From<super::Container> for Container {
        fn from(value: super::Container) -> Self {
            Self {
                thing: Ok(value.thing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerThing {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extra: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ContainerThing {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ContainerThing {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ContainerThing> for super::ContainerThing {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContainerThing,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ContainerThing> for ContainerThing {
        fn from(value: super::ContainerThing) -> Self {
            Self {
                id: Ok(value.id),
                extra: Ok(value.extra),
            }
        }
    }
}
fn main() {}
