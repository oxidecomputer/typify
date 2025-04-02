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
#[doc = "`AnythingWorks`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AnythingWorks {
    pub value: ::serde_json::Value,
}
impl ::std::convert::From<&AnythingWorks> for AnythingWorks {
    fn from(value: &AnythingWorks) -> Self {
        value.clone()
    }
}
impl AnythingWorks {
    pub fn builder() -> builder::AnythingWorks {
        Default::default()
    }
}
#[doc = "`FloatsArentTerribleImTold`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"flush_timeout\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"float\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FloatsArentTerribleImTold {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub flush_timeout: ::std::option::Option<f32>,
}
impl ::std::convert::From<&FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
    fn from(value: &FloatsArentTerribleImTold) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FloatsArentTerribleImTold {
    fn default() -> Self {
        Self {
            flush_timeout: Default::default(),
        }
    }
}
impl FloatsArentTerribleImTold {
    pub fn builder() -> builder::FloatsArentTerribleImTold {
        Default::default()
    }
}
#[doc = "`JustOne`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct JustOne(pub ::std::string::String);
impl ::std::ops::Deref for JustOne {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<JustOne> for ::std::string::String {
    fn from(value: JustOne) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JustOne> for JustOne {
    fn from(value: &JustOne) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for JustOne {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for JustOne {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for JustOne {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`UintMinimumAndMaximum`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"max\","]
#[doc = "    \"min\","]
#[doc = "    \"min_and_max\","]
#[doc = "    \"min_non_zero\","]
#[doc = "    \"min_uint_non_zero\","]
#[doc = "    \"no_bounds\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"max\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"maximum\": 256.0"]
#[doc = "    },"]
#[doc = "    \"min\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"min_and_max\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"maximum\": 256.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"min_non_zero\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"min_uint_non_zero\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"no_bounds\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UintMinimumAndMaximum {
    pub max: u64,
    pub min: u64,
    pub min_and_max: ::std::num::NonZeroU64,
    pub min_non_zero: ::std::num::NonZeroU64,
    pub min_uint_non_zero: ::std::num::NonZeroU64,
    pub no_bounds: u64,
}
impl ::std::convert::From<&UintMinimumAndMaximum> for UintMinimumAndMaximum {
    fn from(value: &UintMinimumAndMaximum) -> Self {
        value.clone()
    }
}
impl UintMinimumAndMaximum {
    pub fn builder() -> builder::UintMinimumAndMaximum {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AnythingWorks {
        value: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for AnythingWorks {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl AnythingWorks {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AnythingWorks> for super::AnythingWorks {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AnythingWorks,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AnythingWorks> for AnythingWorks {
        fn from(value: super::AnythingWorks) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FloatsArentTerribleImTold {
        flush_timeout: ::std::result::Result<::std::option::Option<f32>, ::std::string::String>,
    }
    impl ::std::default::Default for FloatsArentTerribleImTold {
        fn default() -> Self {
            Self {
                flush_timeout: Ok(Default::default()),
            }
        }
    }
    impl FloatsArentTerribleImTold {
        pub fn flush_timeout<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f32>>,
            T::Error: ::std::fmt::Display,
        {
            self.flush_timeout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flush_timeout: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FloatsArentTerribleImTold> for super::FloatsArentTerribleImTold {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FloatsArentTerribleImTold,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flush_timeout: value.flush_timeout?,
            })
        }
    }
    impl ::std::convert::From<super::FloatsArentTerribleImTold> for FloatsArentTerribleImTold {
        fn from(value: super::FloatsArentTerribleImTold) -> Self {
            Self {
                flush_timeout: Ok(value.flush_timeout),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UintMinimumAndMaximum {
        max: ::std::result::Result<u64, ::std::string::String>,
        min: ::std::result::Result<u64, ::std::string::String>,
        min_and_max: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        min_non_zero: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        min_uint_non_zero: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        no_bounds: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for UintMinimumAndMaximum {
        fn default() -> Self {
            Self {
                max: Err("no value supplied for max".to_string()),
                min: Err("no value supplied for min".to_string()),
                min_and_max: Err("no value supplied for min_and_max".to_string()),
                min_non_zero: Err("no value supplied for min_non_zero".to_string()),
                min_uint_non_zero: Err("no value supplied for min_uint_non_zero".to_string()),
                no_bounds: Err("no value supplied for no_bounds".to_string()),
            }
        }
    }
    impl UintMinimumAndMaximum {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
        pub fn min_and_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.min_and_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_and_max: {}", e));
            self
        }
        pub fn min_non_zero<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.min_non_zero = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_non_zero: {}", e));
            self
        }
        pub fn min_uint_non_zero<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.min_uint_non_zero = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for min_uint_non_zero: {}",
                    e
                )
            });
            self
        }
        pub fn no_bounds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.no_bounds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_bounds: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UintMinimumAndMaximum> for super::UintMinimumAndMaximum {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UintMinimumAndMaximum,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                min: value.min?,
                min_and_max: value.min_and_max?,
                min_non_zero: value.min_non_zero?,
                min_uint_non_zero: value.min_uint_non_zero?,
                no_bounds: value.no_bounds?,
            })
        }
    }
    impl ::std::convert::From<super::UintMinimumAndMaximum> for UintMinimumAndMaximum {
        fn from(value: super::UintMinimumAndMaximum) -> Self {
            Self {
                max: Ok(value.max),
                min: Ok(value.min),
                min_and_max: Ok(value.min_and_max),
                min_non_zero: Ok(value.min_non_zero),
                min_uint_non_zero: Ok(value.min_uint_non_zero),
                no_bounds: Ok(value.no_bounds),
            }
        }
    }
}
fn main() {}
