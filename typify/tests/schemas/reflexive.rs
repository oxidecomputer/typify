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
#[doc = "`Node`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"node\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"children\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"validate references to the whole\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Node {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub children: ::std::vec::Vec<Node>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<i64>,
}
impl ::std::convert::From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Node {
    fn default() -> Self {
        Self {
            children: Default::default(),
            value: Default::default(),
        }
    }
}
impl Node {
    pub fn builder() -> builder::Node {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Node {
        children: ::std::result::Result<::std::vec::Vec<super::Node>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
    }
    impl ::std::default::Default for Node {
        fn default() -> Self {
            Self {
                children: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Node {
        pub fn children<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Node>>,
            T::Error: ::std::fmt::Display,
        {
            self.children = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for children: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Node> for super::Node {
        type Error = super::error::ConversionError;
        fn try_from(value: Node) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                children: value.children?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Node> for Node {
        fn from(value: super::Node) -> Self {
            Self {
                children: Ok(value.children),
                value: Ok(value.value),
            }
        }
    }
}
fn main() {}
