#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LessSimpleTwoTuple(pub (String, String));
impl std::ops::Deref for LessSimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<LessSimpleTwoTuple> for (String, String) {
    fn from(value: LessSimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&LessSimpleTwoTuple> for LessSimpleTwoTuple {
    fn from(value: &LessSimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for LessSimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleTwoTuple(pub (String, String));
impl std::ops::Deref for SimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<SimpleTwoTuple> for (String, String) {
    fn from(value: SimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&SimpleTwoTuple> for SimpleTwoTuple {
    fn from(value: &SimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for SimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplerTwoTuple(pub (String, String));
impl std::ops::Deref for SimplerTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<SimplerTwoTuple> for (String, String) {
    fn from(value: SimplerTwoTuple) -> Self {
        value.0
    }
}
impl From<&SimplerTwoTuple> for SimplerTwoTuple {
    fn from(value: &SimplerTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for SimplerTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnsimpleTwoTuple(pub (String, String));
impl std::ops::Deref for UnsimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<UnsimpleTwoTuple> for (String, String) {
    fn from(value: UnsimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&UnsimpleTwoTuple> for UnsimpleTwoTuple {
    fn from(value: &UnsimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for UnsimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YoloTwoTuple(pub (serde_json::Value, serde_json::Value));
impl std::ops::Deref for YoloTwoTuple {
    type Target = (serde_json::Value, serde_json::Value);
    fn deref(&self) -> &(serde_json::Value, serde_json::Value) {
        &self.0
    }
}
impl From<YoloTwoTuple> for (serde_json::Value, serde_json::Value) {
    fn from(value: YoloTwoTuple) -> Self {
        value.0
    }
}
impl From<&YoloTwoTuple> for YoloTwoTuple {
    fn from(value: &YoloTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(serde_json::Value, serde_json::Value)> for YoloTwoTuple {
    fn from(value: (serde_json::Value, serde_json::Value)) -> Self {
        Self(value)
    }
}
fn main() {}
