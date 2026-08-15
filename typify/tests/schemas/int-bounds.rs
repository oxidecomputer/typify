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
#[doc = "A range spanning negative and positive."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A range spanning negative and positive.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"int32\","]
#[doc = "  \"maximum\": 1000.0,"]
#[doc = "  \"minimum\": -1000.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BoundedMixedSign(i32);
impl ::std::ops::Deref for BoundedMixedSign {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}
impl ::std::convert::From<BoundedMixedSign> for i32 {
    fn from(value: BoundedMixedSign) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i32> for BoundedMixedSign {
    type Error = self::error::ConversionError;
    fn try_from(value: i32) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < -1000 {
            return Err("value must be at least -1000".into());
        }
        if value > 1000 {
            return Err("value must be at most 1000".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for BoundedMixedSign {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i32>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A negative-only range."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A negative-only range.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"int16\","]
#[doc = "  \"maximum\": -10.0,"]
#[doc = "  \"minimum\": -50.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BoundedNegative(i16);
impl ::std::ops::Deref for BoundedNegative {
    type Target = i16;
    fn deref(&self) -> &i16 {
        &self.0
    }
}
impl ::std::convert::From<BoundedNegative> for i16 {
    fn from(value: BoundedNegative) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i16> for BoundedNegative {
    type Error = self::error::ConversionError;
    fn try_from(value: i16) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < -50 {
            return Err("value must be at least -50".into());
        }
        if value > -10 {
            return Err("value must be at most -10".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for BoundedNegative {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i16>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A uint8 value between 0 and 63."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A uint8 value between 0 and 63.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 63.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BoundedUint(u8);
impl ::std::ops::Deref for BoundedUint {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<BoundedUint> for u8 {
    fn from(value: BoundedUint) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for BoundedUint {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value > 63 {
            return Err("value must be at most 63".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for BoundedUint {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "min=1, max=255 matches NonZeroU8 exactly: should not produce a constrained newtype."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"min=1, max=255 matches NonZeroU8 exactly: should not produce a constrained newtype.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 255.0,"]
#[doc = "  \"minimum\": 1.0"]
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
#[serde(transparent)]
pub struct ExactNonzeroU8Match(pub ::std::num::NonZeroU8);
impl ::std::ops::Deref for ExactNonzeroU8Match {
    type Target = ::std::num::NonZeroU8;
    fn deref(&self) -> &::std::num::NonZeroU8 {
        &self.0
    }
}
impl ::std::convert::From<ExactNonzeroU8Match> for ::std::num::NonZeroU8 {
    fn from(value: ExactNonzeroU8Match) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::num::NonZeroU8> for ExactNonzeroU8Match {
    fn from(value: ::std::num::NonZeroU8) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ExactNonzeroU8Match {
    type Err = <::std::num::NonZeroU8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ExactNonzeroU8Match {
    type Error = <::std::num::NonZeroU8 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ExactNonzeroU8Match {
    type Error = <::std::num::NonZeroU8 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ExactNonzeroU8Match {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "Exact u8 bounds: should not produce a constrained newtype."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Exact u8 bounds: should not produce a constrained newtype.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 255.0,"]
#[doc = "  \"minimum\": 0.0"]
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
#[serde(transparent)]
pub struct ExactU8Match(pub u8);
impl ::std::ops::Deref for ExactU8Match {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<ExactU8Match> for u8 {
    fn from(value: ExactU8Match) -> Self {
        value.0
    }
}
impl ::std::convert::From<u8> for ExactU8Match {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ExactU8Match {
    type Err = <u8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ExactU8Match {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ExactU8Match {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ExactU8Match {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "exclusiveMinimum=0.5 should produce min=1, exclusiveMaximum=63.5 should produce max=63."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"exclusiveMinimum=0.5 should produce min=1, exclusiveMaximum=63.5 should produce max=63.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"exclusiveMaximum\": 63.5,"]
#[doc = "  \"exclusiveMinimum\": 0.5"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExclusiveFractionalBounds(u8);
impl ::std::ops::Deref for ExclusiveFractionalBounds {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<ExclusiveFractionalBounds> for u8 {
    fn from(value: ExclusiveFractionalBounds) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for ExclusiveFractionalBounds {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 1 {
            return Err("value must be at least 1".into());
        }
        if value > 63 {
            return Err("value must be at most 63".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for ExclusiveFractionalBounds {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "exclusiveMinimum=0, exclusiveMaximum=64 should produce min=1, max=63."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"exclusiveMinimum=0, exclusiveMaximum=64 should produce min=1, max=63.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"exclusiveMaximum\": 64.0,"]
#[doc = "  \"exclusiveMinimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ExclusiveIntegerBounds(u8);
impl ::std::ops::Deref for ExclusiveIntegerBounds {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<ExclusiveIntegerBounds> for u8 {
    fn from(value: ExclusiveIntegerBounds) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for ExclusiveIntegerBounds {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 1 {
            return Err("value must be at least 1".into());
        }
        if value > 63 {
            return Err("value must be at most 63".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for ExclusiveIntegerBounds {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Negative fractional bounds: min=-10.3 -> ceil -> -10, max=-0.1 -> floor -> -1."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Negative fractional bounds: min=-10.3 -> ceil -> -10, max=-0.1 -> floor -> -1.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"int8\","]
#[doc = "  \"maximum\": -0.1,"]
#[doc = "  \"minimum\": -10.3"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FractionalBoundsNegative(i8);
impl ::std::ops::Deref for FractionalBoundsNegative {
    type Target = i8;
    fn deref(&self) -> &i8 {
        &self.0
    }
}
impl ::std::convert::From<FractionalBoundsNegative> for i8 {
    fn from(value: FractionalBoundsNegative) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i8> for FractionalBoundsNegative {
    type Error = self::error::ConversionError;
    fn try_from(value: i8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < -10 {
            return Err("value must be at least -10".into());
        }
        if value > -1 {
            return Err("value must be at most -1".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for FractionalBoundsNegative {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Fractional bounds that must be rounded: min=0.5 -> ceil -> 1, max=63.7 -> floor -> 63."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Fractional bounds that must be rounded: min=0.5 -> ceil -> 1, max=63.7 -> floor -> 63.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 63.7,"]
#[doc = "  \"minimum\": 0.5"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FractionalBoundsPositive(u8);
impl ::std::ops::Deref for FractionalBoundsPositive {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<FractionalBoundsPositive> for u8 {
    fn from(value: FractionalBoundsPositive) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for FractionalBoundsPositive {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 1 {
            return Err("value must be at least 1".into());
        }
        if value > 63 {
            return Err("value must be at most 63".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for FractionalBoundsPositive {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Only a max bound, no min. max=200 with uint8 format."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Only a max bound, no min. max=200 with uint8 format.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 200.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MaxOnlySubRange(u8);
impl ::std::ops::Deref for MaxOnlySubRange {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<MaxOnlySubRange> for u8 {
    fn from(value: MaxOnlySubRange) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for MaxOnlySubRange {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value > 200 {
            return Err("value must be at most 200".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxOnlySubRange {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Only a min bound, no max. min=10 with uint8 format."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Only a min bound, no max. min=10 with uint8 format.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"minimum\": 10.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MinOnlySubRange(u8);
impl ::std::ops::Deref for MinOnlySubRange {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<MinOnlySubRange> for u8 {
    fn from(value: MinOnlySubRange) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for MinOnlySubRange {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 10 {
            return Err("value must be at least 10".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for MinOnlySubRange {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "No format hint, large unsigned range: should pick u32."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"No format hint, large unsigned range: should pick u32.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 100000.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NoFormatLargeUnsigned(u32);
impl ::std::ops::Deref for NoFormatLargeUnsigned {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}
impl ::std::convert::From<NoFormatLargeUnsigned> for u32 {
    fn from(value: NoFormatLargeUnsigned) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u32> for NoFormatLargeUnsigned {
    type Error = self::error::ConversionError;
    fn try_from(value: u32) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value > 100000 {
            return Err("value must be at most 100000".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for NoFormatLargeUnsigned {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u32>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "No format, min=1, max=10: should be a constrained newtype, not NonZero."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"No format, min=1, max=10: should be a constrained newtype, not NonZero.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 10.0,"]
#[doc = "  \"minimum\": 1.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NoFormatMin1WithMax(u8);
impl ::std::ops::Deref for NoFormatMin1WithMax {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<NoFormatMin1WithMax> for u8 {
    fn from(value: NoFormatMin1WithMax) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for NoFormatMin1WithMax {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 1 {
            return Err("value must be at least 1".into());
        }
        if value > 10 {
            return Err("value must be at most 10".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for NoFormatMin1WithMax {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "No format hint, signed range: should pick i8."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"No format hint, signed range: should pick i8.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 50.0,"]
#[doc = "  \"minimum\": -50.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NoFormatSigned(i8);
impl ::std::ops::Deref for NoFormatSigned {
    type Target = i8;
    fn deref(&self) -> &i8 {
        &self.0
    }
}
impl ::std::convert::From<NoFormatSigned> for i8 {
    fn from(value: NoFormatSigned) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<i8> for NoFormatSigned {
    type Error = self::error::ConversionError;
    fn try_from(value: i8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < -50 {
            return Err("value must be at least -50".into());
        }
        if value > 50 {
            return Err("value must be at most 50".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for NoFormatSigned {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "No format hint, non-negative range: should pick u8."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"No format hint, non-negative range: should pick u8.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"maximum\": 63.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NoFormatUnsigned(u8);
impl ::std::ops::Deref for NoFormatUnsigned {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<NoFormatUnsigned> for u8 {
    fn from(value: NoFormatUnsigned) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for NoFormatUnsigned {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value > 63 {
            return Err("value must be at most 63".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for NoFormatUnsigned {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "min=1 with sub-range max: should be a constrained newtype, not NonZeroU8."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"min=1 with sub-range max: should be a constrained newtype, not NonZeroU8.\","]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"format\": \"uint8\","]
#[doc = "  \"maximum\": 100.0,"]
#[doc = "  \"minimum\": 1.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NonzeroWithMax(u8);
impl ::std::ops::Deref for NonzeroWithMax {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<NonzeroWithMax> for u8 {
    fn from(value: NonzeroWithMax) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<u8> for NonzeroWithMax {
    type Error = self::error::ConversionError;
    fn try_from(value: u8) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value < 1 {
            return Err("value must be at least 1".into());
        }
        if value > 100 {
            return Err("value must be at most 100".into());
        }
        Ok(Self(value))
    }
}
impl<'de> ::serde::Deserialize<'de> for NonzeroWithMax {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<u8>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
fn main() {}
