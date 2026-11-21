#![deny(warnings)]
#[doc = "`CountedBase`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CountedBase {
    pub count: i64,
}
impl CountedBase {
    pub fn builder() -> builder::CountedBase {
        Default::default()
    }
}
#[doc = "count matches the pattern, so it takes the pattern's minimum"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CountedWithPattern {
    pub count: u64,
}
impl CountedWithPattern {
    pub fn builder() -> builder::CountedWithPattern {
        Default::default()
    }
}
#[doc = "`ExtensionMap`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ExtensionMap(pub ::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue>);
impl ::std::ops::Deref for ExtensionMap {
    type Target = ::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue>;
    fn deref(&self) -> &::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue> {
        &self.0
    }
}
impl ::std::convert::From<ExtensionMap>
    for ::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue>
{
    fn from(value: ExtensionMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue>>
    for ExtensionMap
{
    fn from(value: ::std::collections::HashMap<ExtensionMapKey, ExtensionMapValue>) -> Self {
        Self(value)
    }
}
#[doc = "`ExtensionMapKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExtensionMapKey(::std::string::String);
impl ::std::ops::Deref for ExtensionMapKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExtensionMapKey> for ::std::string::String {
    fn from(value: ExtensionMapKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExtensionMapKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}
impl ::std::convert::TryFrom<&str> for ExtensionMapKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::with_flags("^x-", "u").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^x-\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExtensionMapKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for ExtensionMapKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ExtensionMapValue`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExtensionMapValue(::std::string::String);
impl ::std::ops::Deref for ExtensionMapValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ExtensionMapValue> for ::std::string::String {
    fn from(value: ExtensionMapValue) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ExtensionMapValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}
impl ::std::convert::TryFrom<&str> for ExtensionMapValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExtensionMapValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for ExtensionMapValue {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`NamesBase`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
pub struct NamesBase {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub names: ::std::vec::Vec<::std::string::String>,
}
impl NamesBase {
    pub fn builder() -> builder::NamesBase {
        Default::default()
    }
}
#[doc = "the reduced schema from typify#882"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StringMapOverBase(
    pub ::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String>,
);
impl ::std::ops::Deref for StringMapOverBase {
    type Target = ::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String>;
    fn deref(&self) -> &::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<StringMapOverBase>
    for ::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String>
{
    fn from(value: StringMapOverBase) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String>>
    for StringMapOverBase
{
    fn from(
        value: ::std::collections::HashMap<StringMapOverBaseKey, ::std::string::String>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`StringMapOverBaseKey`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StringMapOverBaseKey(::std::string::String);
impl ::std::ops::Deref for StringMapOverBaseKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StringMapOverBaseKey> for ::std::string::String {
    fn from(value: StringMapOverBaseKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for StringMapOverBaseKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value)
    }
}
impl ::std::convert::TryFrom<&str> for StringMapOverBaseKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::with_flags("^([^.]+)$", "u").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^([^.]+)$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StringMapOverBaseKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for StringMapOverBaseKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CountedBase {
        count: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for CountedBase {
        fn default() -> Self {
            Self {
                count: Err("no value supplied for count".to_string()),
            }
        }
    }
    impl CountedBase {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CountedBase> for super::CountedBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CountedBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
            })
        }
    }
    impl ::std::convert::From<super::CountedBase> for CountedBase {
        fn from(value: super::CountedBase) -> Self {
            Self {
                count: Ok(value.count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CountedWithPattern {
        count: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for CountedWithPattern {
        fn default() -> Self {
            Self {
                count: Err("no value supplied for count".to_string()),
            }
        }
    }
    impl CountedWithPattern {
        pub fn count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CountedWithPattern> for super::CountedWithPattern {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CountedWithPattern,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                count: value.count?,
            })
        }
    }
    impl ::std::convert::From<super::CountedWithPattern> for CountedWithPattern {
        fn from(value: super::CountedWithPattern) -> Self {
            Self {
                count: Ok(value.count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NamesBase {
        names: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for NamesBase {
        fn default() -> Self {
            Self {
                names: Ok(Default::default()),
            }
        }
    }
    impl NamesBase {
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NamesBase> for super::NamesBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NamesBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                names: value.names?,
            })
        }
    }
    impl ::std::convert::From<super::NamesBase> for NamesBase {
        fn from(value: super::NamesBase) -> Self {
            Self {
                names: Ok(value.names),
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
