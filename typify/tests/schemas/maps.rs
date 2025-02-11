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
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"DeadSimple\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$comment\": \"usual case of a map whose name must come from its title\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
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
impl ::std::convert::From<&DeadSimple> for DeadSimple {
    fn from(value: &DeadSimple) -> Self {
        value.clone()
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
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"^a*$\""]
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
impl ::std::convert::From<&Eh> for Eh {
    fn from(value: &Eh) -> Self {
        value.clone()
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
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"$ref\": \"#/definitions/Value\""]
#[doc = "  },"]
#[doc = "  \"propertyNames\": {"]
#[doc = "    \"format\": \"date\""]
#[doc = "  },"]
#[doc = "  \"$comment\": \"test that a type isn't needed for propertyNames\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithDateKeys(pub ::std::collections::HashMap<::chrono::naive::NaiveDate, Value>);
impl ::std::ops::Deref for MapWithDateKeys {
    type Target = ::std::collections::HashMap<::chrono::naive::NaiveDate, Value>;
    fn deref(&self) -> &::std::collections::HashMap<::chrono::naive::NaiveDate, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithDateKeys>
    for ::std::collections::HashMap<::chrono::naive::NaiveDate, Value>
{
    fn from(value: MapWithDateKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapWithDateKeys> for MapWithDateKeys {
    fn from(value: &MapWithDateKeys) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::chrono::naive::NaiveDate, Value>>
    for MapWithDateKeys
{
    fn from(value: ::std::collections::HashMap<::chrono::naive::NaiveDate, Value>) -> Self {
        Self(value)
    }
}
#[doc = "`MapWithDateTimeKeys`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"$ref\": \"#/definitions/Value\""]
#[doc = "  },"]
#[doc = "  \"propertyNames\": {"]
#[doc = "    \"format\": \"date-time\""]
#[doc = "  },"]
#[doc = "  \"$comment\": \"test that a type isn't needed for propertyNames\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithDateTimeKeys(
    pub ::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
);
impl ::std::ops::Deref for MapWithDateTimeKeys {
    type Target = ::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value>;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithDateTimeKeys>
    for ::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value>
{
    fn from(value: MapWithDateTimeKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapWithDateTimeKeys> for MapWithDateTimeKeys {
    fn from(value: &MapWithDateTimeKeys) -> Self {
        value.clone()
    }
}
impl
    ::std::convert::From<
        ::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
    > for MapWithDateTimeKeys
{
    fn from(
        value: ::std::collections::HashMap<::chrono::DateTime<::chrono::offset::Utc>, Value>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "`MapWithKeys`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"$ref\": \"#/definitions/Value\""]
#[doc = "  },"]
#[doc = "  \"propertyNames\": {"]
#[doc = "    \"$ref\": \"#/definitions/Eh\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MapWithKeys(pub ::std::collections::HashMap<Eh, Value>);
impl ::std::ops::Deref for MapWithKeys {
    type Target = ::std::collections::HashMap<Eh, Value>;
    fn deref(&self) -> &::std::collections::HashMap<Eh, Value> {
        &self.0
    }
}
impl ::std::convert::From<MapWithKeys> for ::std::collections::HashMap<Eh, Value> {
    fn from(value: MapWithKeys) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MapWithKeys> for MapWithKeys {
    fn from(value: &MapWithKeys) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<Eh, Value>> for MapWithKeys {
    fn from(value: ::std::collections::HashMap<Eh, Value>) -> Self {
        Self(value)
    }
}
#[doc = "`Value`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
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
impl ::std::convert::From<&Value> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
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
