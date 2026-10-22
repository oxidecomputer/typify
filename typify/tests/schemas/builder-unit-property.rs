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
#[doc = "`UnitProperty`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UnitProperty\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"unit\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"unit\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UnitProperty {
    pub unit: (),
}
impl UnitProperty {
    pub fn builder() -> builder::UnitProperty {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct UnitProperty {
        unit: ::std::result::Result<(), ::std::string::String>,
    }
    impl ::std::default::Default for UnitProperty {
        fn default() -> Self {
            Self {
                unit: Err("no value supplied for unit".to_string()),
            }
        }
    }
    impl UnitProperty {
        pub fn unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<()>,
            T::Error: ::std::fmt::Display,
        {
            self.unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<UnitProperty> for super::UnitProperty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnitProperty,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { unit: value.unit? })
        }
    }
    impl ::std::convert::From<super::UnitProperty> for UnitProperty {
        fn from(_value: super::UnitProperty) -> Self {
            Self { unit: Ok(()) }
        }
    }
}
fn main() {}
