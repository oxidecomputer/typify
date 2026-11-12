//! Code generated from tests/schemas/input/simple-required-fields-1.json
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RootSchema {
    pub bar: f64,
    pub foo: ::std::string::String,
}
impl RootSchema {
    pub fn builder() -> builder::RootSchema {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct RootSchema {
        bar: ::std::result::Result<f64, ::std::string::String>,
        foo: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for RootSchema {
        fn default() -> Self {
            Self {
                bar: Err("no value supplied for bar".to_string()),
                foo: Err("no value supplied for foo".to_string()),
            }
        }
    }
    impl RootSchema {
        pub fn bar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bar: {e}"));
            self
        }
        pub fn foo<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.foo = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for foo: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<RootSchema> for super::RootSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RootSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bar: value.bar?,
                foo: value.foo?,
            })
        }
    }
    impl ::std::convert::From<super::RootSchema> for RootSchema {
        fn from(value: super::RootSchema) -> Self {
            Self {
                bar: Ok(value.bar),
                foo: Ok(value.foo),
            }
        }
    }
}
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
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
