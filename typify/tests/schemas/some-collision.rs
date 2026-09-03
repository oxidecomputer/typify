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
    impl From<::std::string::String> for ConversionError {
        fn from(value: ::std::string::String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`DenyValueWithSomeCollision`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"not\": {"]
#[doc = "    \"enum\": ["]
#[doc = "      1"]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct DenyValueWithSomeCollision(f64);
impl ::std::ops::Deref for DenyValueWithSomeCollision {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<DenyValueWithSomeCollision> for f64 {
    fn from(value: DenyValueWithSomeCollision) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<f64> for DenyValueWithSomeCollision {
    type Error = self::error::ConversionError;
    fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if [1_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for DenyValueWithSomeCollision {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`Some`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Some {
    pub value: ::std::string::String,
}
impl Some {
    pub fn builder() -> builder::Some {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Some {
        value: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Some {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl Some {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Some> for super::Some {
        type Error = super::error::ConversionError;
        fn try_from(value: Some) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Some> for Some {
        fn from(value: super::Some) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
}
fn main() {}
