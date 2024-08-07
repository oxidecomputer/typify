#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Baz"]
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
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
)]
pub struct Baz(pub String);
impl std::ops::Deref for Baz {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Baz> for String {
    fn from(value: Baz) -> Self {
        value.0
    }
}
impl From<&Baz> for Baz {
    fn from(value: &Baz) -> Self {
        value.clone()
    }
}
impl From<String> for Baz {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Baz {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Baz {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Foo"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"Bar\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/FooBar\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Baz\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
pub struct Foo {
    #[serde(rename = "Bar", default, skip_serializing_if = "Option::is_none")]
    pub bar: Option<FooBar>,
}
impl From<&Foo> for Foo {
    fn from(value: &Foo) -> Self {
        value.clone()
    }
}
#[doc = "FooBar"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FooBar\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Baz\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
pub enum FooBar {
    FooBar(FooBar2),
    Baz(Baz),
}
impl From<&FooBar> for FooBar {
    fn from(value: &FooBar) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FooBar {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::FooBar(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Baz(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for FooBar {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FooBar {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FooBar {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ToString for FooBar {
    fn to_string(&self) -> String {
        match self {
            Self::FooBar(x) => x.to_string(),
            Self::Baz(x) => x.to_string(),
        }
    }
}
impl From<FooBar2> for FooBar {
    fn from(value: FooBar2) -> Self {
        Self::FooBar(value)
    }
}
impl From<Baz> for FooBar {
    fn from(value: Baz) -> Self {
        Self::Baz(value)
    }
}
#[doc = "FooBar2"]
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
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde :: Deserialize, serde :: Serialize,
)]
pub struct FooBar2(pub String);
impl std::ops::Deref for FooBar2 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<FooBar2> for String {
    fn from(value: FooBar2) -> Self {
        value.0
    }
}
impl From<&FooBar2> for FooBar2 {
    fn from(value: &FooBar2) -> Self {
        value.clone()
    }
}
impl From<String> for FooBar2 {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for FooBar2 {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for FooBar2 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
fn main() {}
