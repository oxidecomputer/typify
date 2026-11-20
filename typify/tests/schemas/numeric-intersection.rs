#![deny(warnings)]
#[doc = "`NumericIntersection`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NumericIntersection {
    pub n: i64,
}
impl NumericIntersection {
    pub fn builder() -> builder::NumericIntersection {
        Default::default()
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct NumericIntersection {
        n: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for NumericIntersection {
        fn default() -> Self {
            Self {
                n: Err("no value supplied for n".to_string()),
            }
        }
    }
    impl NumericIntersection {
        pub fn n<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.n = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for n: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<NumericIntersection> for super::NumericIntersection {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NumericIntersection,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { n: value.n? })
        }
    }
    impl ::std::convert::From<super::NumericIntersection> for NumericIntersection {
        fn from(value: super::NumericIntersection) -> Self {
            Self { n: Ok(value.n) }
        }
    }
}
#[doc = " Error types."]
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
fn main() {}
