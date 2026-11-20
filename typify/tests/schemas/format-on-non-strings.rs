#![deny(warnings)]
#[doc = "`AnythingList`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AnythingList(pub ::std::vec::Vec<::serde_json::Value>);
impl ::std::ops::Deref for AnythingList {
    type Target = ::std::vec::Vec<::serde_json::Value>;
    fn deref(&self) -> &::std::vec::Vec<::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<AnythingList> for ::std::vec::Vec<::serde_json::Value> {
    fn from(value: AnythingList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for AnythingList {
    fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "an object that carries a string format"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DateOfBirth {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}
impl DateOfBirth {
    pub fn builder() -> builder::DateOfBirth {
        Default::default()
    }
}
#[doc = "`Flag`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Flag(pub bool);
impl ::std::ops::Deref for Flag {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<Flag> for bool {
    fn from(value: Flag) -> Self {
        value.0
    }
}
impl ::std::convert::From<bool> for Flag {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for Flag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for Flag {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
#[doc = "`MaybeDateOfBirth`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MaybeDateOfBirth(pub ::std::option::Option<MaybeDateOfBirthInner>);
impl ::std::ops::Deref for MaybeDateOfBirth {
    type Target = ::std::option::Option<MaybeDateOfBirthInner>;
    fn deref(&self) -> &::std::option::Option<MaybeDateOfBirthInner> {
        &self.0
    }
}
impl ::std::convert::From<MaybeDateOfBirth> for ::std::option::Option<MaybeDateOfBirthInner> {
    fn from(value: MaybeDateOfBirth) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<MaybeDateOfBirthInner>> for MaybeDateOfBirth {
    fn from(value: ::std::option::Option<MaybeDateOfBirthInner>) -> Self {
        Self(value)
    }
}
#[doc = "`MaybeDateOfBirthInner`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct MaybeDateOfBirthInner {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub year: ::std::option::Option<u16>,
}
impl MaybeDateOfBirthInner {
    pub fn builder() -> builder::MaybeDateOfBirthInner {
        Default::default()
    }
}
#[doc = "`Tags`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Tags(pub ::std::vec::Vec<::std::string::String>);
impl ::std::ops::Deref for Tags {
    type Target = ::std::vec::Vec<::std::string::String>;
    fn deref(&self) -> &::std::vec::Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<Tags> for ::std::vec::Vec<::std::string::String> {
    fn from(value: Tags) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for Tags {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DateOfBirth {
        day: ::std::result::Result<u8, ::std::string::String>,
        month: ::std::result::Result<u8, ::std::string::String>,
        year: ::std::result::Result<u16, ::std::string::String>,
    }
    impl ::std::default::Default for DateOfBirth {
        fn default() -> Self {
            Self {
                day: Err("no value supplied for day".to_string()),
                month: Err("no value supplied for month".to_string()),
                year: Err("no value supplied for year".to_string()),
            }
        }
    }
    impl DateOfBirth {
        pub fn day<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u8>,
            T::Error: ::std::fmt::Display,
        {
            self.day = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for day: {e}"));
            self
        }
        pub fn month<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u8>,
            T::Error: ::std::fmt::Display,
        {
            self.month = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for month: {e}"));
            self
        }
        pub fn year<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u16>,
            T::Error: ::std::fmt::Display,
        {
            self.year = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for year: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DateOfBirth> for super::DateOfBirth {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DateOfBirth,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                day: value.day?,
                month: value.month?,
                year: value.year?,
            })
        }
    }
    impl ::std::convert::From<super::DateOfBirth> for DateOfBirth {
        fn from(value: super::DateOfBirth) -> Self {
            Self {
                day: Ok(value.day),
                month: Ok(value.month),
                year: Ok(value.year),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MaybeDateOfBirthInner {
        year: ::std::result::Result<::std::option::Option<u16>, ::std::string::String>,
    }
    impl ::std::default::Default for MaybeDateOfBirthInner {
        fn default() -> Self {
            Self {
                year: Ok(Default::default()),
            }
        }
    }
    impl MaybeDateOfBirthInner {
        pub fn year<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<u16>>,
            T::Error: ::std::fmt::Display,
        {
            self.year = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for year: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MaybeDateOfBirthInner> for super::MaybeDateOfBirthInner {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MaybeDateOfBirthInner,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { year: value.year? })
        }
    }
    impl ::std::convert::From<super::MaybeDateOfBirthInner> for MaybeDateOfBirthInner {
        fn from(value: super::MaybeDateOfBirthInner) -> Self {
            Self {
                year: Ok(value.year),
            }
        }
    }
}
#[doc = " Error types."]
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
fn main() {}
