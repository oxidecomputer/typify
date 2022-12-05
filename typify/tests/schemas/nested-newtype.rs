use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NestedNewType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<NestedNewTypeField>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NestedNewTypeField(NestedNewTypeFieldInner);
impl std::ops::Deref for NestedNewTypeField {
    type Target = NestedNewTypeFieldInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::cmp::PartialEq for NestedNewTypeFieldInner {
    #[inline]
    fn eq(&self, other: &NestedNewTypeFieldInner) -> bool {
        self.0 == other.0
    }
}
impl std::convert::TryFrom<NestedNewTypeFieldInner> for NestedNewTypeField {
    type Error = &'static str;
    fn try_from(value: NestedNewTypeFieldInner) -> Result<Self, Self::Error> {
        if ![
            super::NestedNewTypeFieldInner("a".to_string()),
            super::NestedNewTypeFieldInner("b".to_string()),
        ]
        .contains(&value)
        {
            Err("invalid value")
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct NestedNewTypeFieldInner(String);
impl std::ops::Deref for NestedNewTypeFieldInner {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::convert::TryFrom<&str> for NestedNewTypeFieldInner {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 25usize {
            return Err("longer than 25 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&String> for NestedNewTypeFieldInner {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl std::convert::TryFrom<String> for NestedNewTypeFieldInner {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NestedNewTypeFieldInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(String::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}

fn main() {}
