#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "ArrayBs"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"multipleOf\": 100.0,"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"boolean\""]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {}"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArrayBs(pub Vec<bool>);
impl std::ops::Deref for ArrayBs {
    type Target = Vec<bool>;
    fn deref(&self) -> &Vec<bool> {
        &self.0
    }
}
impl From<ArrayBs> for Vec<bool> {
    fn from(value: ArrayBs) -> Self {
        value.0
    }
}
impl From<&ArrayBs> for ArrayBs {
    fn from(value: &ArrayBs) -> Self {
        value.clone()
    }
}
impl From<Vec<bool>> for ArrayBs {
    fn from(value: Vec<bool>) -> Self {
        Self(value)
    }
}
#[doc = "IntegerBs"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"minimum\": 0.0,"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {}"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegerBs(pub u64);
impl std::ops::Deref for IntegerBs {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl From<IntegerBs> for u64 {
    fn from(value: IntegerBs) -> Self {
        value.0
    }
}
impl From<&IntegerBs> for IntegerBs {
    fn from(value: &IntegerBs) -> Self {
        value.clone()
    }
}
impl From<u64> for IntegerBs {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for IntegerBs {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for IntegerBs {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntegerBs {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntegerBs {
    type Error = <u64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for IntegerBs {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "ObjectBs"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"multipleOf\": 100.0,"]
#[doc = "  \"maxLength\": 100,"]
#[doc = "  \"maxItems\": 100,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ok\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectBs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
}
impl From<&ObjectBs> for ObjectBs {
    fn from(value: &ObjectBs) -> Self {
        value.clone()
    }
}
fn main() {}
