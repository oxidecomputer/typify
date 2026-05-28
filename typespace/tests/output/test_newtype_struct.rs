pub struct MyInt(pub u32);
impl ::std::ops::Deref for MyInt {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<MyInt> for u32 {
    fn from(value: MyInt) -> Self {
        value.0
    }
}
impl ::serde::Serialize for MyInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MyInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
///A newtype wrapping String.
pub struct MyString(pub String);
impl ::std::ops::Deref for MyString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<MyString> for String {
    fn from(value: MyString) -> Self {
        value.0
    }
}
impl ::serde::Serialize for MyString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
