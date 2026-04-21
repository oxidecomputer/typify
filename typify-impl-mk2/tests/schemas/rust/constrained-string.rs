//! Code generated from tests/schemas/input/constrained-string.json
pub struct SchemaRoot(pub ::std::string::String);
impl ::std::ops::Deref for SchemaRoot {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<SchemaRoot> for ::std::string::String {
    fn from(value: SchemaRoot) -> Self {
        value.0
    }
}
impl ::serde::Serialize for SchemaRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
fn main() {}
