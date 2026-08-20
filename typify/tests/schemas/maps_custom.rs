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
#[doc = "`DeadSimple`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct DeadSimple(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for DeadSimple {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<DeadSimple>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: DeadSimple) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for DeadSimple
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`Eh`"]
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
pub struct Eh(pub ::std::string::String);
impl ::std::ops::Deref for Eh {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Eh> for ::std::string::String {
    fn from(value: Eh) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Eh {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Eh {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Eh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`MapWithDateKeys`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithDateKeys(pub std::collections::BTreeMap<::chrono::naive::NaiveDate, Value>);
impl ::std::ops::Deref for MapWithDateKeys {
    type Target = std::collections::BTreeMap<::chrono::naive::NaiveDate, Value>;
    fn deref(&self) -> &std::collections::BTreeMap<::chrono::naive::NaiveDate, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithDateKeys>
    for std::collections::BTreeMap<::chrono::naive::NaiveDate, Value>
{
    fn from(value: MapWithDateKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<std::collections::BTreeMap<::chrono::naive::NaiveDate, Value>>
    for MapWithDateKeys
{
    fn from(value: std::collections::BTreeMap<::chrono::naive::NaiveDate, Value>) -> Self {
        Self(value)
    }
}
#[doc = "`MapWithDateTimeKeys`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithDateTimeKeys(
    pub std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
);
impl ::std::ops::Deref for MapWithDateTimeKeys {
    type Target = std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value>;
    fn deref(
        &self,
    ) -> &std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithDateTimeKeys>
    for std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value>
{
    fn from(value: MapWithDateTimeKeys) -> Self {
        value.0
    }
}
impl
    ::std::convert::From<
        std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
    > for MapWithDateTimeKeys
{
    fn from(
        value: std::collections::BTreeMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`MapWithKeys`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithKeys(pub std::collections::BTreeMap<Eh, Value>);
impl ::std::ops::Deref for MapWithKeys {
    type Target = std::collections::BTreeMap<Eh, Value>;
    fn deref(&self) -> &std::collections::BTreeMap<Eh, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithKeys> for std::collections::BTreeMap<Eh, Value> {
    fn from(value: MapWithKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<std::collections::BTreeMap<Eh, Value>> for MapWithKeys {
    fn from(value: std::collections::BTreeMap<Eh, Value>) -> Self {
        Self(value)
    }
}
#[doc = "`Value`"]
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
pub struct Value(pub ::std::string::String);
impl ::std::ops::Deref for Value {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Value> for ::std::string::String {
    fn from(value: Value) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for Value {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Value {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
fn main() {}
