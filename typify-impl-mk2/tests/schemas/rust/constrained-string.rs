//! Code generated from tests/schemas/input/constrained-string.json
pub struct ConstrainedString(pub ::std::string::String);
impl ::std::ops::Deref for ConstrainedString {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<ConstrainedString> for ::std::string::String {
    fn from(value: ConstrainedString) -> Self {
        value.0
    }
}
impl ::serde::Serialize for ConstrainedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ConstrainedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
fn main() {}
