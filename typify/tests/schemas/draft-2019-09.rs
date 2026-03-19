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
#[doc = "`Color`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"red\","]
#[doc = "    \"green\","]
#[doc = "    \"blue\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
}
impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Red => f.write_str("red"),
            Self::Green => f.write_str("green"),
            Self::Blue => f.write_str("blue"),
        }
    }
}
impl ::std::str::FromStr for Color {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Color {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Color {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Color {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Point`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"dependencies\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"properties\": {"]
#[doc = "        \"opacity\": {"]
#[doc = "          \"maximum\": 1,"]
#[doc = "          \"minimum\": 0,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"$ref\": \"#/$defs/color\""]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"y\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"x\","]
#[doc = "    \"y\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Point {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<Color>,
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn builder() -> builder::Point {
        Default::default()
    }
}
#[doc = "`Shape`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"points\": {"]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/point\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"points\""]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Shape {
    pub kind: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub points: ::std::vec::Vec<Point>,
}
impl Shape {
    pub fn builder() -> builder::Shape {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Point {
        color: ::std::result::Result<::std::option::Option<super::Color>, ::std::string::String>,
        x: ::std::result::Result<f64, ::std::string::String>,
        y: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Point {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl Point {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Color>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {e}"));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Point> for super::Point {
        type Error = super::error::ConversionError;
        fn try_from(value: Point) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl ::std::convert::From<super::Point> for Point {
        fn from(value: super::Point) -> Self {
            Self {
                color: Ok(value.color),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Shape {
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        points: ::std::result::Result<::std::vec::Vec<super::Point>, ::std::string::String>,
    }
    impl ::std::default::Default for Shape {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                points: Ok(Default::default()),
            }
        }
    }
    impl Shape {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {e}"));
            self
        }
        pub fn points<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Point>>,
            T::Error: ::std::fmt::Display,
        {
            self.points = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for points: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Shape> for super::Shape {
        type Error = super::error::ConversionError;
        fn try_from(value: Shape) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                points: value.points?,
            })
        }
    }
    impl ::std::convert::From<super::Shape> for Shape {
        fn from(value: super::Shape) -> Self {
            Self {
                kind: Ok(value.kind),
                points: Ok(value.points),
            }
        }
    }
}
fn main() {}
