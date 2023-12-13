#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "DeadSimple"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"DeadSimple\",\n  \"type\": \"object\",\n  \"$comment\": \"usual case of a map whose name must come from its title\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeadSimple(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for DeadSimple {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<DeadSimple> for serde_json::Map<String, serde_json::Value> {
    fn from(value: DeadSimple) -> Self {
        value.0
    }
}
impl From<&DeadSimple> for DeadSimple {
    fn from(value: &DeadSimple) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for DeadSimple {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "Eh"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"format\": \"^a*$\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Eh(pub String);
impl std::ops::Deref for Eh {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Eh> for String {
    fn from(value: Eh) -> Self {
        value.0
    }
}
impl From<&Eh> for Eh {
    fn from(value: &Eh) -> Self {
        value.clone()
    }
}
impl From<String> for Eh {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Eh {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Eh {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "MapWithDateKeys"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"additionalProperties\": {\n    \"$ref\": \"#/definitions/Value\"\n  },\n  \"propertyNames\": {\n    \"format\": \"date\"\n  },\n  \"$comment\": \"test that a type isn't needed for propertyNames\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapWithDateKeys(pub std::collections::HashMap<chrono::naive::NaiveDate, Value>);
impl std::ops::Deref for MapWithDateKeys {
    type Target = std::collections::HashMap<chrono::naive::NaiveDate, Value>;
    fn deref(&self) -> &std::collections::HashMap<chrono::naive::NaiveDate, Value> {
        &self.0
    }
}
impl From<MapWithDateKeys> for std::collections::HashMap<chrono::naive::NaiveDate, Value> {
    fn from(value: MapWithDateKeys) -> Self {
        value.0
    }
}
impl From<&MapWithDateKeys> for MapWithDateKeys {
    fn from(value: &MapWithDateKeys) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<chrono::naive::NaiveDate, Value>> for MapWithDateKeys {
    fn from(value: std::collections::HashMap<chrono::naive::NaiveDate, Value>) -> Self {
        Self(value)
    }
}
#[doc = "MapWithDateTimeKeys"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"additionalProperties\": {\n    \"$ref\": \"#/definitions/Value\"\n  },\n  \"propertyNames\": {\n    \"format\": \"date-time\"\n  },\n  \"$comment\": \"test that a type isn't needed for propertyNames\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapWithDateTimeKeys(
    pub std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value>,
);
impl std::ops::Deref for MapWithDateTimeKeys {
    type Target = std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value>;
    fn deref(&self) -> &std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value> {
        &self.0
    }
}
impl From<MapWithDateTimeKeys>
    for std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value>
{
    fn from(value: MapWithDateTimeKeys) -> Self {
        value.0
    }
}
impl From<&MapWithDateTimeKeys> for MapWithDateTimeKeys {
    fn from(value: &MapWithDateTimeKeys) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value>>
    for MapWithDateTimeKeys
{
    fn from(
        value: std::collections::HashMap<chrono::DateTime<chrono::offset::Utc>, Value>,
    ) -> Self {
        Self(value)
    }
}
#[doc = "MapWithKeys"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"object\",\n  \"additionalProperties\": {\n    \"$ref\": \"#/definitions/Value\"\n  },\n  \"propertyNames\": {\n    \"$ref\": \"#/definitions/Eh\"\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapWithKeys(pub std::collections::HashMap<Eh, Value>);
impl std::ops::Deref for MapWithKeys {
    type Target = std::collections::HashMap<Eh, Value>;
    fn deref(&self) -> &std::collections::HashMap<Eh, Value> {
        &self.0
    }
}
impl From<MapWithKeys> for std::collections::HashMap<Eh, Value> {
    fn from(value: MapWithKeys) -> Self {
        value.0
    }
}
impl From<&MapWithKeys> for MapWithKeys {
    fn from(value: &MapWithKeys) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<Eh, Value>> for MapWithKeys {
    fn from(value: std::collections::HashMap<Eh, Value>) -> Self {
        Self(value)
    }
}
#[doc = "Value"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Value(pub String);
impl std::ops::Deref for Value {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Value> for String {
    fn from(value: Value) -> Self {
        value.0
    }
}
impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
    }
}
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Value {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Value {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
fn main() {}
