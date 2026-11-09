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
#[doc = "`DefaultedEnum`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"beta\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"alpha\","]
#[doc = "    \"beta\","]
#[doc = "    \"gamma\""]
#[doc = "  ]"]
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
pub enum DefaultedEnum {
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "gamma")]
    Gamma,
}
impl ::std::fmt::Display for DefaultedEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alpha => f.write_str("alpha"),
            Self::Beta => f.write_str("beta"),
            Self::Gamma => f.write_str("gamma"),
        }
    }
}
impl ::std::str::FromStr for DefaultedEnum {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alpha" => Ok(Self::Alpha),
            "beta" => Ok(Self::Beta),
            "gamma" => Ok(Self::Gamma),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DefaultedEnum {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DefaultedEnum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for DefaultedEnum {
    fn default() -> Self {
        DefaultedEnum::Beta
    }
}
#[doc = "`DefaultedStruct`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": {"]
#[doc = "    \"a\": \"hello\""]
#[doc = "  },"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"a\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DefaultedStruct {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub a: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for DefaultedStruct {
    fn default() -> Self {
        DefaultedStruct {
            a: ::std::option::Option::Some("hello".to_string()),
        }
    }
}
impl DefaultedStruct {
    pub fn builder() -> builder::DefaultedStruct {
        Default::default()
    }
}
#[doc = "`Root`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enum_via_ref\": {"]
#[doc = "      \"$ref\": \"#/definitions/DefaultedEnum\""]
#[doc = "    },"]
#[doc = "    \"override_at_ref\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"a\": \"override\""]
#[doc = "      },"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/DefaultedStruct\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"via_ref\": {"]
#[doc = "      \"$ref\": \"#/definitions/DefaultedStruct\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Root {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enum_via_ref: ::std::option::Option<DefaultedEnum>,
    #[serde(default = "defaults::root_override_at_ref")]
    pub override_at_ref: DefaultedStruct,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub via_ref: ::std::option::Option<DefaultedStruct>,
}
impl ::std::default::Default for Root {
    fn default() -> Self {
        Self {
            enum_via_ref: Default::default(),
            override_at_ref: defaults::root_override_at_ref(),
            via_ref: Default::default(),
        }
    }
}
impl Root {
    pub fn builder() -> builder::Root {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DefaultedStruct {
        a: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DefaultedStruct {
        fn default() -> Self {
            Self {
                a: Ok(Default::default()),
            }
        }
    }
    impl DefaultedStruct {
        pub fn a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for a: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DefaultedStruct> for super::DefaultedStruct {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DefaultedStruct,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { a: value.a? })
        }
    }
    impl ::std::convert::From<super::DefaultedStruct> for DefaultedStruct {
        fn from(value: super::DefaultedStruct) -> Self {
            Self { a: Ok(value.a) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Root {
        enum_via_ref: ::std::result::Result<
            ::std::option::Option<super::DefaultedEnum>,
            ::std::string::String,
        >,
        override_at_ref: ::std::result::Result<super::DefaultedStruct, ::std::string::String>,
        via_ref: ::std::result::Result<
            ::std::option::Option<super::DefaultedStruct>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Root {
        fn default() -> Self {
            Self {
                enum_via_ref: Ok(Default::default()),
                override_at_ref: Ok(super::defaults::root_override_at_ref()),
                via_ref: Ok(Default::default()),
            }
        }
    }
    impl Root {
        pub fn enum_via_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DefaultedEnum>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_via_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_via_ref: {e}"));
            self
        }
        pub fn override_at_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DefaultedStruct>,
            T::Error: ::std::fmt::Display,
        {
            self.override_at_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for override_at_ref: {e}"));
            self
        }
        pub fn via_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DefaultedStruct>>,
            T::Error: ::std::fmt::Display,
        {
            self.via_ref = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for via_ref: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Root> for super::Root {
        type Error = super::error::ConversionError;
        fn try_from(value: Root) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enum_via_ref: value.enum_via_ref?,
                override_at_ref: value.override_at_ref?,
                via_ref: value.via_ref?,
            })
        }
    }
    impl ::std::convert::From<super::Root> for Root {
        fn from(value: super::Root) -> Self {
            Self {
                enum_via_ref: Ok(value.enum_via_ref),
                override_at_ref: Ok(value.override_at_ref),
                via_ref: Ok(value.via_ref),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn root_override_at_ref() -> super::DefaultedStruct {
        super::DefaultedStruct {
            a: ::std::option::Option::Some("override".to_string()),
        }
    }
}
fn main() {}
