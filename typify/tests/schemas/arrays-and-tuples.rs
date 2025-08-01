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
#[doc = "`ArraySansItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"minItems\": 1,"]
#[doc = "  \"uniqueItems\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ArraySansItems(pub Vec<::serde_json::Value>);
impl ::std::ops::Deref for ArraySansItems {
    type Target = Vec<::serde_json::Value>;
    fn deref(&self) -> &Vec<::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ArraySansItems> for Vec<::serde_json::Value> {
    fn from(value: ArraySansItems) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ArraySansItems> for ArraySansItems {
    fn from(value: &ArraySansItems) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::serde_json::Value>> for ArraySansItems {
    fn from(value: Vec<::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`LessSimpleTwoTuple`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"maxItems\": 2,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct LessSimpleTwoTuple(pub (::std::string::String, ::std::string::String));
impl ::std::ops::Deref for LessSimpleTwoTuple {
    type Target = (::std::string::String, ::std::string::String);
    fn deref(&self) -> &(::std::string::String, ::std::string::String) {
        &self.0
    }
}
impl ::std::convert::From<LessSimpleTwoTuple> for (::std::string::String, ::std::string::String) {
    fn from(value: LessSimpleTwoTuple) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LessSimpleTwoTuple> for LessSimpleTwoTuple {
    fn from(value: &LessSimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<(::std::string::String, ::std::string::String)> for LessSimpleTwoTuple {
    fn from(value: (::std::string::String, ::std::string::String)) -> Self {
        Self(value)
    }
}
#[doc = "`SimpleTwoArray`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 2,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SimpleTwoArray(pub [::std::string::String; 2usize]);
impl ::std::ops::Deref for SimpleTwoArray {
    type Target = [::std::string::String; 2usize];
    fn deref(&self) -> &[::std::string::String; 2usize] {
        &self.0
    }
}
impl ::std::convert::From<SimpleTwoArray> for [::std::string::String; 2usize] {
    fn from(value: SimpleTwoArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SimpleTwoArray> for SimpleTwoArray {
    fn from(value: &SimpleTwoArray) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<[::std::string::String; 2usize]> for SimpleTwoArray {
    fn from(value: [::std::string::String; 2usize]) -> Self {
        Self(value)
    }
}
#[doc = "`SimpleTwoTuple`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"maxItems\": 2,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SimpleTwoTuple(pub (::std::string::String, ::std::string::String));
impl ::std::ops::Deref for SimpleTwoTuple {
    type Target = (::std::string::String, ::std::string::String);
    fn deref(&self) -> &(::std::string::String, ::std::string::String) {
        &self.0
    }
}
impl ::std::convert::From<SimpleTwoTuple> for (::std::string::String, ::std::string::String) {
    fn from(value: SimpleTwoTuple) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SimpleTwoTuple> for SimpleTwoTuple {
    fn from(value: &SimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<(::std::string::String, ::std::string::String)> for SimpleTwoTuple {
    fn from(value: (::std::string::String, ::std::string::String)) -> Self {
        Self(value)
    }
}
#[doc = "`UnsimpleTwoTuple`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"additionalItems\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 2,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct UnsimpleTwoTuple(pub (::std::string::String, ::std::string::String));
impl ::std::ops::Deref for UnsimpleTwoTuple {
    type Target = (::std::string::String, ::std::string::String);
    fn deref(&self) -> &(::std::string::String, ::std::string::String) {
        &self.0
    }
}
impl ::std::convert::From<UnsimpleTwoTuple> for (::std::string::String, ::std::string::String) {
    fn from(value: UnsimpleTwoTuple) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnsimpleTwoTuple> for UnsimpleTwoTuple {
    fn from(value: &UnsimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<(::std::string::String, ::std::string::String)> for UnsimpleTwoTuple {
    fn from(value: (::std::string::String, ::std::string::String)) -> Self {
        Self(value)
    }
}
#[doc = "`YoloTwoArray`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"additionalItems\": {"]
#[doc = "    \"type\": \"string\","]
#[doc = "    \"$comment\": \"ignored\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 2,"]
#[doc = "  \"minItems\": 2"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct YoloTwoArray(pub [::serde_json::Value; 2usize]);
impl ::std::ops::Deref for YoloTwoArray {
    type Target = [::serde_json::Value; 2usize];
    fn deref(&self) -> &[::serde_json::Value; 2usize] {
        &self.0
    }
}
impl ::std::convert::From<YoloTwoArray> for [::serde_json::Value; 2usize] {
    fn from(value: YoloTwoArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<&YoloTwoArray> for YoloTwoArray {
    fn from(value: &YoloTwoArray) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<[::serde_json::Value; 2usize]> for YoloTwoArray {
    fn from(value: [::serde_json::Value; 2usize]) -> Self {
        Self(value)
    }
}
fn main() {}
