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
#[doc = "`AllTheThings`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"option_marker\": {"]
#[doc = "      \"$ref\": \"#/$defs/OptionMarker\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"$ref\": \"#/$defs/PathBuf\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AllTheThings {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_marker: ::std::option::Option<::std::option::Option<Marker>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::path::PathBuf>,
}
impl ::std::convert::From<&AllTheThings> for AllTheThings {
    fn from(value: &AllTheThings) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AllTheThings {
    fn default() -> Self {
        Self {
            option_marker: Default::default(),
            path: Default::default(),
        }
    }
}
impl AllTheThings {
    pub fn builder() -> builder::AllTheThings {
        Default::default()
    }
}
#[doc = "`Marker`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "false"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum Marker {}
impl ::std::convert::From<&Self> for Marker {
    fn from(value: &Marker) -> Self {
        value.clone()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AllTheThings {
        option_marker: ::std::result::Result<
            ::std::option::Option<::std::option::Option<super::Marker>>,
            ::std::string::String,
        >,
        path: ::std::result::Result<
            ::std::option::Option<::std::path::PathBuf>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AllTheThings {
        fn default() -> Self {
            Self {
                option_marker: Ok(Default::default()),
                path: Ok(Default::default()),
            }
        }
    }
    impl AllTheThings {
        pub fn option_marker<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::option::Option<super::Marker>>>,
            T::Error: ::std::fmt::Display,
        {
            self.option_marker = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option_marker: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::path::PathBuf>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AllTheThings> for super::AllTheThings {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AllTheThings,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                option_marker: value.option_marker?,
                path: value.path?,
            })
        }
    }
    impl ::std::convert::From<super::AllTheThings> for AllTheThings {
        fn from(value: super::AllTheThings) -> Self {
            Self {
                option_marker: Ok(value.option_marker),
                path: Ok(value.path),
            }
        }
    }
}
fn main() {}
