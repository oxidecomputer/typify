//! Code generated from tests/schemas/input/various-enums.json
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum AlternativeEnum {
    Choice1,
    Choice2,
    Choice3,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum AnyOfNoStrings {}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum AnyOfNothing {}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct DiskAttachment {
    pub alternate: AlternativeEnum,
    pub state: DiskAttachmentState,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum DiskAttachmentState {
    Detached,
    Destroyed,
    Faulted,
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub struct EmptyObject {
    #[serde(
        default,
        deserialize_with = "::json_serde::deserialize_some",
        skip_serializing_if = ":: std :: option :: Option::is_none"
    )]
    pub prop: ::std::option::Option<EmptyObjectProp>,
}
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
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
        let value: serde_json::Value = ::serde::Deserialize::deserialize(deserializer)?;
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
pub struct IpNet(pub ::serde_json::Value);
impl ::std::ops::Deref for IpNet {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<IpNet> for ::serde_json::Value {
    fn from(value: IpNet) -> Self {
        value.0
    }
}
impl ::serde::Serialize for IpNet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for IpNet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct Ipv4Net(pub ::std::string::String);
impl ::std::ops::Deref for Ipv4Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<Ipv4Net> for ::std::string::String {
    fn from(value: Ipv4Net) -> Self {
        value.0
    }
}
impl ::serde::Serialize for Ipv4Net {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for Ipv4Net {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct Ipv6Net(pub ::std::string::String);
impl ::std::ops::Deref for Ipv6Net {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<Ipv6Net> for ::std::string::String {
    fn from(value: Ipv6Net) -> Self {
        value.0
    }
}
impl ::serde::Serialize for Ipv6Net {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for Ipv6Net {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct JankNames(pub ::serde_json::Value);
impl ::std::ops::Deref for JankNames {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<JankNames> for ::serde_json::Value {
    fn from(value: JankNames) -> Self {
        value.0
    }
}
impl ::serde::Serialize for JankNames {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for JankNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum Never {}
pub struct NeverEver(pub ::serde_json::Value);
impl ::std::ops::Deref for NeverEver {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<NeverEver> for ::serde_json::Value {
    fn from(value: NeverEver) -> Self {
        value.0
    }
}
impl ::serde::Serialize for NeverEver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NeverEver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct NeverEverForever(pub ::serde_json::Value);
impl ::std::ops::Deref for NeverEverForever {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<NeverEverForever> for ::serde_json::Value {
    fn from(value: NeverEverForever) -> Self {
        value.0
    }
}
impl ::serde::Serialize for NeverEverForever {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NeverEverForever {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum NullStringEnumWithUnknownFormat {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
}
pub struct ReferenceDef(pub f64);
impl ::std::ops::Deref for ReferenceDef {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<ReferenceDef> for f64 {
    fn from(value: ReferenceDef) -> Self {
        value.0
    }
}
impl ::serde::Serialize for ReferenceDef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ReferenceDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct References(pub ::serde_json::Value);
impl ::std::ops::Deref for References {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<References> for ::serde_json::Value {
    fn from(value: References) -> Self {
        value.0
    }
}
impl ::serde::Serialize for References {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for References {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum ShouldBeExclusive {
    Variant0 {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = ":: std :: option :: Option::is_none"
        )]
        id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = ":: std :: option :: Option::is_none"
        )]
        reference: ::std::option::Option<::std::string::String>,
    },
    Variant1 {
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = ":: std :: option :: Option::is_none"
        )]
        id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            deserialize_with = "::json_serde::deserialize_some",
            skip_serializing_if = ":: std :: option :: Option::is_none"
        )]
        reference: ::std::option::Option<::std::string::String>,
    },
}
pub struct StringVersion(pub ::std::string::String);
impl ::std::ops::Deref for StringVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<StringVersion> for ::std::string::String {
    fn from(value: StringVersion) -> Self {
        value.0
    }
}
impl ::serde::Serialize for StringVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StringVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct CommentedVariants(pub ::serde_json::Value);
impl ::std::ops::Deref for CommentedVariants {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<CommentedVariants> for ::serde_json::Value {
    fn from(value: CommentedVariants) -> Self {
        value.0
    }
}
impl ::serde::Serialize for CommentedVariants {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CommentedVariants {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct EnumAndConstant(pub ::serde_json::Value);
impl ::std::ops::Deref for EnumAndConstant {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<EnumAndConstant> for ::serde_json::Value {
    fn from(value: EnumAndConstant) -> Self {
        value.0
    }
}
impl ::serde::Serialize for EnumAndConstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for EnumAndConstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
pub struct OneOfMissingTitle(pub ::serde_json::Value);
impl ::std::ops::Deref for OneOfMissingTitle {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::convert::From<OneOfMissingTitle> for ::serde_json::Value {
    fn from(value: OneOfMissingTitle) -> Self {
        value.0
    }
}
impl ::serde::Serialize for OneOfMissingTitle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OneOfMissingTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(Self(::serde::Deserialize::deserialize(deserializer)?))
    }
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OneOfRawType {
    String(::std::string::String),
    Integer(i64),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum OneOfTypes {
    #[serde(rename = "bar")]
    Bar(i64),
    #[serde(rename = "foo")]
    Foo(::std::string::String),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionAnyofConst {
    Variant0(::std::string::String),
    Variant1(::serde_json::Value),
    Variant2(::std::string::String),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionAnyofEnum {
    String(::std::string::String),
    Null(OptionAnyofEnumNull),
}
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
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
        let value: serde_json::Value = ::serde::Deserialize::deserialize(deserializer)?;
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
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionAnyofNull {
    String(::std::string::String),
    Null(()),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionOneofConst {
    Variant0(::std::string::String),
    Variant1(::serde_json::Value),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionOneofEnum {
    String(::std::string::String),
    Null(OptionOneofEnumNull),
}
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
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
        let value: serde_json::Value = ::serde::Deserialize::deserialize(deserializer)?;
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
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(untagged)]
pub enum OptionOneofNull {
    String(::std::string::String),
    Null(()),
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
pub enum VariantsDifferByPunct {
    #[serde(rename = "v1.5GBASE-T")]
    V15gbaseT,
    #[serde(rename = "v225GBASE-T")]
    V225GbaseT,
    #[serde(rename = "v32,5,GBASE,T")]
    V325GbaseT,
}
fn main() {}
