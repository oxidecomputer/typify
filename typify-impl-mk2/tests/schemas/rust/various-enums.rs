//! Code generated from tests/schemas/input/various-enums.json
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum AlternativeEnum {
    Choice1,
    Choice2,
    Choice3,
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AnyOfNoStrings {}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AnyOfNothing {}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CommentedVariants(pub ::serde_json::Value);
impl ::std::ops::Deref for CommentedVariants {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<CommentedVariants> for ::serde_json::Value {
    fn from(value: CommentedVariants) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for CommentedVariants {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DiskAttachment {
    pub alternate: AlternativeEnum,
    pub state: DiskAttachmentState,
}
impl DiskAttachment {
    pub fn builder() -> builder::DiskAttachment {
        Default::default()
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum DiskAttachmentState {
    Detached,
    Destroyed,
    Faulted,
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EmptyObject {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prop: ::std::option::Option<EmptyObjectProp>,
}
impl EmptyObject {
    pub fn builder() -> builder::EmptyObject {
        Default::default()
    }
}
#[derive(Clone, Debug)]
pub struct EmptyObjectProp;
impl ::serde::Serialize for EmptyObjectProp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        ::serde_json::Value::Object(::serde_json::Map::from_iter([]))
            .serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for EmptyObjectProp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let expected = ::serde_json::Value::Object(::serde_json::Map::from_iter([]));
        let value: ::serde_json::Value = ::serde::Deserialize::deserialize(
            deserializer,
        )?;
        if value != expected {
            return Err(
                ::serde::de::Error::custom(
                    format!(
                        "expected unit struct value {}, found {}", "{}",
                        ::serde_json::to_string(& value).unwrap()
                    ),
                ),
            );
        }
        Ok(EmptyObjectProp)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EnumAndConstant(pub ::serde_json::Value);
impl ::std::ops::Deref for EnumAndConstant {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<EnumAndConstant> for ::serde_json::Value {
    fn from(value: EnumAndConstant) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for EnumAndConstant {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct IpNet(pub ::serde_json::Value);
impl ::std::ops::Deref for IpNet {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<IpNet> for ::serde_json::Value {
    fn from(value: IpNet) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for IpNet {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Ipv4Net(::std::string::String);
impl ::std::ops::Deref for Ipv4Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Ipv4Net> for ::std::string::String {
    fn from(value: Ipv4Net) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<&str> for Ipv4Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("4*").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"4*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Ipv4Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for Ipv4Net {
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
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Ipv6Net(::std::string::String);
impl ::std::ops::Deref for Ipv6Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Ipv6Net> for ::std::string::String {
    fn from(value: Ipv6Net) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<&str> for Ipv6Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("6*").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"6*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Ipv6Net {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        ::std::convert::TryFrom::try_from(value.as_str())
    }
}
impl<'de> ::serde::Deserialize<'de> for Ipv6Net {
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
#[serde(transparent)]
pub struct JankNames(pub ::serde_json::Value);
impl ::std::ops::Deref for JankNames {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<JankNames> for ::serde_json::Value {
    fn from(value: JankNames) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for JankNames {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Never {}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NeverEver(pub ::serde_json::Value);
impl ::std::ops::Deref for NeverEver {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<NeverEver> for ::serde_json::Value {
    fn from(value: NeverEver) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for NeverEver {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NeverEverForever(pub ::serde_json::Value);
impl ::std::ops::Deref for NeverEverForever {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<NeverEverForever> for ::serde_json::Value {
    fn from(value: NeverEverForever) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for NeverEverForever {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum NullStringEnumWithUnknownFormat {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OneOfMissingTitle(pub ::serde_json::Value);
impl ::std::ops::Deref for OneOfMissingTitle {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<OneOfMissingTitle> for ::serde_json::Value {
    fn from(value: OneOfMissingTitle) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for OneOfMissingTitle {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OneOfRawType {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<i64> for OneOfRawType {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum OneOfTypes {
    #[serde(rename = "bar")]
    Bar(i64),
    #[serde(rename = "foo")]
    Foo(::std::string::String),
}
impl ::std::convert::From<i64> for OneOfTypes {
    fn from(value: i64) -> Self {
        Self::Bar(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionAnyofConst {
    Variant0(::std::string::String),
    Variant1(::serde_json::Value),
    Variant2(::std::string::String),
}
impl ::std::convert::From<::serde_json::Value> for OptionAnyofConst {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant1(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionAnyofEnum {
    String(::std::string::String),
    Null(OptionAnyofEnumNull),
}
impl ::std::convert::From<OptionAnyofEnumNull> for OptionAnyofEnum {
    fn from(value: OptionAnyofEnumNull) -> Self {
        Self::Null(value)
    }
}
#[derive(Clone, Debug)]
pub struct OptionAnyofEnumNull;
impl ::serde::Serialize for OptionAnyofEnumNull {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        ::serde_json::Value::Null.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OptionAnyofEnumNull {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let expected = ::serde_json::Value::Null;
        let value: ::serde_json::Value = ::serde::Deserialize::deserialize(
            deserializer,
        )?;
        if value != expected {
            return Err(
                ::serde::de::Error::custom(
                    format!(
                        "expected unit struct value {}, found {}", "null",
                        ::serde_json::to_string(& value).unwrap()
                    ),
                ),
            );
        }
        Ok(OptionAnyofEnumNull)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionAnyofNull {
    String(::std::string::String),
    Null(()),
}
impl ::std::convert::From<()> for OptionAnyofNull {
    fn from(value: ()) -> Self {
        Self::Null(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionOneofConst {
    Variant0(::std::string::String),
    Variant1(::serde_json::Value),
}
impl ::std::convert::From<::serde_json::Value> for OptionOneofConst {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant1(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionOneofEnum {
    String(::std::string::String),
    Null(OptionOneofEnumNull),
}
impl ::std::convert::From<OptionOneofEnumNull> for OptionOneofEnum {
    fn from(value: OptionOneofEnumNull) -> Self {
        Self::Null(value)
    }
}
#[derive(Clone, Debug)]
pub struct OptionOneofEnumNull;
impl ::serde::Serialize for OptionOneofEnumNull {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        ::serde_json::Value::Null.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OptionOneofEnumNull {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let expected = ::serde_json::Value::Null;
        let value: ::serde_json::Value = ::serde::Deserialize::deserialize(
            deserializer,
        )?;
        if value != expected {
            return Err(
                ::serde::de::Error::custom(
                    format!(
                        "expected unit struct value {}, found {}", "null",
                        ::serde_json::to_string(& value).unwrap()
                    ),
                ),
            );
        }
        Ok(OptionOneofEnumNull)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum OptionOneofNull {
    String(::std::string::String),
    Null(()),
}
impl ::std::convert::From<()> for OptionOneofNull {
    fn from(value: ()) -> Self {
        Self::Null(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ReferenceDef(pub f64);
impl ::std::ops::Deref for ReferenceDef {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<ReferenceDef> for f64 {
    fn from(value: ReferenceDef) -> Self {
        value.0
    }
}
impl ::std::convert::From<f64> for ReferenceDef {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct References(pub ::serde_json::Value);
impl ::std::ops::Deref for References {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<References> for ::serde_json::Value {
    fn from(value: References) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for References {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ShouldBeExclusive {
    Variant0 {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        reference: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        reference: ::std::option::Option<::std::string::String>,
    },
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StringVersion(pub ::std::string::String);
impl ::std::ops::Deref for StringVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<StringVersion> for ::std::string::String {
    fn from(value: StringVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for StringVersion {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum VariantsDifferByPunct {
    #[serde(rename = "v1.5GBASE-T")]
    V15gbaseT,
    #[serde(rename = "v225GBASE-T")]
    V225GbaseT,
    #[serde(rename = "v32,5,GBASE,T")]
    V325GbaseT,
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DiskAttachment {
        alternate: ::std::result::Result<super::AlternativeEnum, ::std::string::String>,
        state: ::std::result::Result<super::DiskAttachmentState, ::std::string::String>,
    }
    impl ::std::default::Default for DiskAttachment {
        fn default() -> Self {
            Self {
                alternate: Err("no value supplied for alternate".to_string()),
                state: Err("no value supplied for state".to_string()),
            }
        }
    }
    impl DiskAttachment {
        pub fn alternate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AlternativeEnum>,
            T::Error: ::std::fmt::Display,
        {
            self.alternate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alternate: {e}")
                });
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DiskAttachmentState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DiskAttachment> for super::DiskAttachment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DiskAttachment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternate: value.alternate?,
                state: value.state?,
            })
        }
    }
    impl ::std::convert::From<super::DiskAttachment> for DiskAttachment {
        fn from(value: super::DiskAttachment) -> Self {
            Self {
                alternate: Ok(value.alternate),
                state: Ok(value.state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EmptyObject {
        prop: ::std::result::Result<
            ::std::option::Option<super::EmptyObjectProp>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EmptyObject {
        fn default() -> Self {
            Self {
                prop: Ok(Default::default()),
            }
        }
    }
    impl EmptyObject {
        pub fn prop<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EmptyObjectProp>>,
            T::Error: ::std::fmt::Display,
        {
            self.prop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prop: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<EmptyObject> for super::EmptyObject {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EmptyObject,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { prop: value.prop? })
        }
    }
    impl ::std::convert::From<super::EmptyObject> for EmptyObject {
        fn from(value: super::EmptyObject) -> Self {
            Self { prop: Ok(value.prop) }
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
