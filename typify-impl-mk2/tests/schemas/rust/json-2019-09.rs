//! Code generated from tests/schemas/input/json-2019-09
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NonNegativeInteger(pub i64);
impl ::std::ops::Deref for NonNegativeInteger {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<NonNegativeInteger> for i64 {
    fn from(value: NonNegativeInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<i64> for NonNegativeInteger {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SchemaArray(Vec<SchemaRoot>);
impl ::std::ops::Deref for SchemaArray {
    type Target = Vec<SchemaRoot>;
    fn deref(&self) -> &Vec<SchemaRoot> {
        &self.0
    }
}
impl ::std::convert::From<SchemaArray> for Vec<SchemaRoot> {
    fn from(value: SchemaArray) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<Vec<SchemaRoot>> for SchemaArray {
    type Error = self::error::ConversionError;
    fn try_from(
        value: Vec<SchemaRoot>,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.is_empty() {
            return Err("at least one item required".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaArray {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<Vec<SchemaRoot>>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SchemaRoot {
    Boolean(bool),
    Object {
        #[serde(
            rename = "$anchor",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        anchor: Option<SchemaRootObjectAnchor>,
        #[serde(
            rename = "$comment",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        comment: Option<String>,
        #[serde(
            rename = "$defs",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        defs: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            rename = "$id",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        id: Option<SchemaRootObjectId>,
        #[serde(
            rename = "$recursiveAnchor",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        recursive_anchor: Option<bool>,
        #[serde(
            rename = "$recursiveRef",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        recursive_ref: Option<::url::Url>,
        #[serde(
            rename = "$ref",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        ref_: Option<::url::Url>,
        #[serde(
            rename = "$schema",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        schema: Option<::url::Url>,
        #[serde(
            rename = "$vocabulary",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        vocabulary: Option<::std::collections::BTreeMap<String, bool>>,
        #[serde(
            rename = "additionalItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        additional_items: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "additionalProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "allOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        all_of: Option<SchemaArray>,
        #[serde(
            rename = "anyOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        any_of: Option<SchemaArray>,
        #[serde(
            rename = "const",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        const_: Option<::serde_json::Value>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        contains: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "contentEncoding",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_encoding: Option<String>,
        #[serde(
            rename = "contentMediaType",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_media_type: Option<String>,
        #[serde(
            rename = "contentSchema",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        content_schema: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        default: Option<::serde_json::Value>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        definitions: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependencies: Option<
            ::std::collections::BTreeMap<String, SchemaRootObjectDependenciesValue>,
        >,
        #[serde(
            rename = "dependentRequired",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependent_required: Option<::std::collections::BTreeMap<String, StringArray>>,
        #[serde(
            rename = "dependentSchemas",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        dependent_schemas: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        deprecated: Option<bool>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        description: Option<String>,
        #[serde(
            rename = "else",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        else_: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "enum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        enum_: Option<Vec<::serde_json::Value>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        examples: Option<Vec<::serde_json::Value>>,
        #[serde(
            rename = "exclusiveMaximum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_maximum: Option<serde_json::Number>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        exclusive_minimum: Option<serde_json::Number>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        format: Option<String>,
        #[serde(
            rename = "if",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        if_: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        items: Option<Box<SchemaRootObjectItems>>,
        #[serde(
            rename = "maxContains",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_contains: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_items: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_length: Option<NonNegativeInteger>,
        #[serde(
            rename = "maxProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        max_properties: Option<NonNegativeInteger>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        maximum: Option<serde_json::Number>,
        #[serde(
            rename = "minContains",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_contains: Option<NonNegativeInteger>,
        #[serde(
            rename = "minItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_items: Option<NonNegativeInteger>,
        #[serde(
            rename = "minLength",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_length: Option<NonNegativeInteger>,
        #[serde(
            rename = "minProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        min_properties: Option<NonNegativeInteger>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        minimum: Option<serde_json::Number>,
        #[serde(
            rename = "multipleOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        multiple_of: Option<serde_json::Number>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        not: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "oneOf",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        one_of: Option<SchemaArray>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pattern: Option<String>,
        #[serde(
            rename = "patternProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        pattern_properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        properties: Option<::std::collections::BTreeMap<String, SchemaRoot>>,
        #[serde(
            rename = "propertyNames",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        property_names: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "readOnly",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        read_only: Option<bool>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        required: Option<StringArray>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        then: Option<Box<SchemaRoot>>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        title: Option<String>,
        #[serde(
            rename = "type",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        type_: Option<SchemaRootObjectType>,
        #[serde(
            rename = "unevaluatedItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_items: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "unevaluatedProperties",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_properties: Option<Box<SchemaRoot>>,
        #[serde(
            rename = "uniqueItems",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        unique_items: Option<bool>,
        #[serde(
            rename = "writeOnly",
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "Option::is_none"
        )]
        write_only: Option<bool>,
    },
}
impl ::std::convert::From<bool> for SchemaRoot {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SchemaRootObjectAnchor(String);
impl ::std::ops::Deref for SchemaRootObjectAnchor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl ::std::convert::From<SchemaRootObjectAnchor> for String {
    fn from(value: SchemaRootObjectAnchor) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<&str> for SchemaRootObjectAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[A-Za-z][-A-Za-z0-9.:_]*$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z][-A-Za-z0-9.:_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SchemaRootObjectAnchor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaRootObjectAnchor {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(
                ::std::string::String::deserialize(deserializer)?,
            )
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SchemaRootObjectDependenciesValue {
    Variant0(SchemaRoot),
    Variant1(StringArray),
}
impl ::std::convert::From<SchemaRoot> for SchemaRootObjectDependenciesValue {
    fn from(value: SchemaRoot) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<StringArray> for SchemaRootObjectDependenciesValue {
    fn from(value: StringArray) -> Self {
        Self::Variant1(value)
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SchemaRootObjectId(::url::Url);
impl ::std::ops::Deref for SchemaRootObjectId {
    type Target = ::url::Url;
    fn deref(&self) -> &::url::Url {
        &self.0
    }
}
impl ::std::convert::From<SchemaRootObjectId> for ::url::Url {
    fn from(value: SchemaRootObjectId) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<&str> for SchemaRootObjectId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]*#?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]*#?$\"".into());
        }
        value
            .parse()
            .map(Self)
            .map_err(|e| { ::std::string::ToString::to_string(&e).into() })
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SchemaRootObjectId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaRootObjectId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::convert::TryFrom::try_from(
                ::std::string::String::deserialize(deserializer)?,
            )
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SchemaRootObjectItems {
    Variant0(SchemaRoot),
    Variant1(SchemaArray),
}
impl ::std::convert::From<SchemaRoot> for SchemaRootObjectItems {
    fn from(value: SchemaRoot) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<SchemaArray> for SchemaRootObjectItems {
    fn from(value: SchemaArray) -> Self {
        Self::Variant1(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SchemaRootObjectType {
    String(SimpleTypes),
    Array(SchemaRootObjectTypeArray),
}
impl ::std::convert::From<SimpleTypes> for SchemaRootObjectType {
    fn from(value: SimpleTypes) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<SchemaRootObjectTypeArray> for SchemaRootObjectType {
    fn from(value: SchemaRootObjectTypeArray) -> Self {
        Self::Array(value)
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SchemaRootObjectTypeArray(Vec<SimpleTypes>);
impl ::std::ops::Deref for SchemaRootObjectTypeArray {
    type Target = Vec<SimpleTypes>;
    fn deref(&self) -> &Vec<SimpleTypes> {
        &self.0
    }
}
impl ::std::convert::From<SchemaRootObjectTypeArray> for Vec<SimpleTypes> {
    fn from(value: SchemaRootObjectTypeArray) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<Vec<SimpleTypes>> for SchemaRootObjectTypeArray {
    type Error = self::error::ConversionError;
    fn try_from(
        value: Vec<SimpleTypes>,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.is_empty() {
            return Err("at least one item required".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaRootObjectTypeArray {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<Vec<SimpleTypes>>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SimpleTypes {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StringArray(pub Vec<String>);
impl ::std::ops::Deref for StringArray {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}
impl ::std::convert::From<StringArray> for Vec<String> {
    fn from(value: StringArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<Vec<String>> for StringArray {
    fn from(value: Vec<String>) -> Self {
        Self(value)
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
