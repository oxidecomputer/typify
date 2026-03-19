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
#[doc = "`ArrayBs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"boolean\""]
#[doc = "  },"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"multipleOf\": 100,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {}"]
#[doc = "  },"]
#[doc = "  \"type\": \"array\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ArrayBs(pub ::std::vec::Vec<bool>);
impl ::std::ops::Deref for ArrayBs {
    type Target = ::std::vec::Vec<bool>;
    fn deref(&self) -> &::std::vec::Vec<bool> {
        &self.0
    }
}
impl ::std::convert::From<ArrayBs> for ::std::vec::Vec<bool> {
    fn from(value: ArrayBs) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<bool>> for ArrayBs {
    fn from(value: ::std::vec::Vec<bool>) -> Self {
        Self(value)
    }
}
#[doc = "`IntegerBs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"minimum\": 0,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {}"]
#[doc = "  },"]
#[doc = "  \"type\": \"integer\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct IntegerBs(u64);
impl ::std::ops::Deref for IntegerBs {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<IntegerBs> for u64 {
    fn from(value: IntegerBs) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u64> for IntegerBs {
    type Error = self::error::ConversionError;
    fn try_from(value: u64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 0i64 as u64 {
            return Err("value must be >= 0".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for IntegerBs {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
impl ::std::fmt::Display for IntegerBs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for IntegerBs {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value
            .parse::<u64>()
            .map_err(|e| e.to_string().into())
            .and_then(|v| Self::try_from(v))
    }
}
impl ::std::convert::TryFrom<&str> for IntegerBs {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for IntegerBs {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ObjectBs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"maxItems\": 100,"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"multipleOf\": 100,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ObjectBs {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ok: ::std::option::Option<bool>,
}
impl ::std::default::Default for ObjectBs {
    fn default() -> Self {
        Self {
            ok: Default::default(),
        }
    }
}
impl ObjectBs {
    pub fn builder() -> builder::ObjectBs {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ObjectBs {
        ok: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for ObjectBs {
        fn default() -> Self {
            Self {
                ok: Ok(Default::default()),
            }
        }
    }
    impl ObjectBs {
        pub fn ok<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.ok = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ok: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ObjectBs> for super::ObjectBs {
        type Error = super::error::ConversionError;
        fn try_from(value: ObjectBs) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { ok: value.ok? })
        }
    }
    impl ::std::convert::From<super::ObjectBs> for ObjectBs {
        fn from(value: super::ObjectBs) -> Self {
            Self { ok: Ok(value.ok) }
        }
    }
}
fn main() {}
