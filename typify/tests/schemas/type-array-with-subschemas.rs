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
#[doc = "`TypeArrayAllOfRefinement`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"minItems\": 1"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayAllOfRefinement {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for TypeArrayAllOfRefinement {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayAnyOfItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayAnyOfItems {
    Variant0(TypeArrayAnyOfItemsVariant0),
    Variant1(TypeArrayAnyOfItemsVariant1),
}
impl ::std::convert::From<TypeArrayAnyOfItemsVariant0> for TypeArrayAnyOfItems {
    fn from(value: TypeArrayAnyOfItemsVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TypeArrayAnyOfItemsVariant1> for TypeArrayAnyOfItems {
    fn from(value: TypeArrayAnyOfItemsVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`TypeArrayAnyOfItemsVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayAnyOfItemsVariant0 {
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<f64> for TypeArrayAnyOfItemsVariant0 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for TypeArrayAnyOfItemsVariant0 {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayAnyOfItemsVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayAnyOfItemsVariant1 {
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<f64>),
}
impl ::std::convert::From<f64> for TypeArrayAnyOfItemsVariant1 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for TypeArrayAnyOfItemsVariant1 {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayFullyUnsatisfiableOneOf`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\""]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"k\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"Every branch conflicts with the outer type union; must resolve cleanly rather than panic.\""]
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
#[serde(deny_unknown_fields)]
pub enum TypeArrayFullyUnsatisfiableOneOf {}
#[doc = "`TypeArrayNotExclusion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"not\": {"]
#[doc = "    \"type\": \"object\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayNotExclusion {
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<::serde_json::Value>),
}
impl ::std::convert::From<f64> for TypeArrayNotExclusion {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::serde_json::Value>> for TypeArrayNotExclusion {
    fn from(value: ::std::vec::Vec<::serde_json::Value>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayOneOfAndAllOf`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"oneOf and allOf on the same object, plus a multi-type `type` array.\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfAndAllOf {
    Variant0(TypeArrayOneOfAndAllOfVariant0),
    Variant1(TypeArrayOneOfAndAllOfVariant1),
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant0> for TypeArrayOneOfAndAllOf {
    fn from(value: TypeArrayOneOfAndAllOfVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant1> for TypeArrayOneOfAndAllOf {
    fn from(value: TypeArrayOneOfAndAllOfVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`TypeArrayOneOfAndAllOfVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"array\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfAndAllOfVariant0 {
    String(TypeArrayOneOfAndAllOfVariant0String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant0String> for TypeArrayOneOfAndAllOfVariant0 {
    fn from(value: TypeArrayOneOfAndAllOfVariant0String) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for TypeArrayOneOfAndAllOfVariant0
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayOneOfAndAllOfVariant0String`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeArrayOneOfAndAllOfVariant0String(::std::string::String);
impl ::std::ops::Deref for TypeArrayOneOfAndAllOfVariant0String {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant0String> for ::std::string::String {
    fn from(value: TypeArrayOneOfAndAllOfVariant0String) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TypeArrayOneOfAndAllOfVariant0String {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeArrayOneOfAndAllOfVariant0String {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeArrayOneOfAndAllOfVariant0String {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeArrayOneOfAndAllOfVariant0String {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeArrayOneOfAndAllOfVariant0String {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`TypeArrayOneOfAndAllOfVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"array\""]
#[doc = "      ],"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfAndAllOfVariant1 {
    String(TypeArrayOneOfAndAllOfVariant1String),
    Array(::std::vec::Vec<f64>),
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant1String> for TypeArrayOneOfAndAllOfVariant1 {
    fn from(value: TypeArrayOneOfAndAllOfVariant1String) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for TypeArrayOneOfAndAllOfVariant1 {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayOneOfAndAllOfVariant1String`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TypeArrayOneOfAndAllOfVariant1String(::std::string::String);
impl ::std::ops::Deref for TypeArrayOneOfAndAllOfVariant1String {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TypeArrayOneOfAndAllOfVariant1String> for ::std::string::String {
    fn from(value: TypeArrayOneOfAndAllOfVariant1String) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TypeArrayOneOfAndAllOfVariant1String {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TypeArrayOneOfAndAllOfVariant1String {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeArrayOneOfAndAllOfVariant1String {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeArrayOneOfAndAllOfVariant1String {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TypeArrayOneOfAndAllOfVariant1String {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`TypeArrayOneOfExplicitArrayBranches`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"Each oneOf branch pins `type: array`; the non-array variants from the outer union should be pruned.\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfExplicitArrayBranches {
    Variant0(::std::vec::Vec<::std::string::String>),
    Variant1(::std::vec::Vec<f64>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for TypeArrayOneOfExplicitArrayBranches
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for TypeArrayOneOfExplicitArrayBranches {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`TypeArrayOneOfItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"boolean\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfItems {
    Variant0(TypeArrayOneOfItemsVariant0),
    Variant1(TypeArrayOneOfItemsVariant1),
    Variant2(TypeArrayOneOfItemsVariant2),
}
impl ::std::convert::From<TypeArrayOneOfItemsVariant0> for TypeArrayOneOfItems {
    fn from(value: TypeArrayOneOfItemsVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TypeArrayOneOfItemsVariant1> for TypeArrayOneOfItems {
    fn from(value: TypeArrayOneOfItemsVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<TypeArrayOneOfItemsVariant2> for TypeArrayOneOfItems {
    fn from(value: TypeArrayOneOfItemsVariant2) -> Self {
        Self::Variant2(value)
    }
}
#[doc = "`TypeArrayOneOfItemsVariant0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"boolean\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfItemsVariant0 {
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<bool> for TypeArrayOneOfItemsVariant0 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for TypeArrayOneOfItemsVariant0 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for TypeArrayOneOfItemsVariant0 {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayOneOfItemsVariant1`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"boolean\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfItemsVariant1 {
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<f64>),
}
impl ::std::convert::From<bool> for TypeArrayOneOfItemsVariant1 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for TypeArrayOneOfItemsVariant1 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<f64>> for TypeArrayOneOfItemsVariant1 {
    fn from(value: ::std::vec::Vec<f64>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayOneOfItemsVariant2`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"boolean\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayOneOfItemsVariant2 {
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
    Array(::std::vec::Vec<bool>),
}
impl ::std::convert::From<bool> for TypeArrayOneOfItemsVariant2 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for TypeArrayOneOfItemsVariant2 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<bool>> for TypeArrayOneOfItemsVariant2 {
    fn from(value: ::std::vec::Vec<bool>) -> Self {
        Self::Array(value)
    }
}
#[doc = "`TypeArrayPartiallyUnsatisfiableOneOf`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"array\""]
#[doc = "  ],"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$comment\": \"Two oneOf branches conflict with the outer type union and should be dropped during merge.\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TypeArrayPartiallyUnsatisfiableOneOf {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for TypeArrayPartiallyUnsatisfiableOneOf
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
fn main() {}
