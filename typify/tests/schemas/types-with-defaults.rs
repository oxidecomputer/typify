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
#[doc = "`Doodad`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"when\": {"]
#[doc = "      \"default\": \"1970-01-01T00:00:00Z\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Doodad {
    #[serde(default = "defaults::doodad_when")]
    pub when: ::chrono::DateTime<::chrono::offset::Utc>,
}
impl ::std::convert::From<&Doodad> for Doodad {
    fn from(value: &Doodad) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Doodad {
    fn default() -> Self {
        Self {
            when: defaults::doodad_when(),
        }
    }
}
impl Doodad {
    pub fn builder() -> builder::Doodad {
        Default::default()
    }
}
#[doc = "`MrDefaultNumbers`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"big_nullable\": {"]
#[doc = "      \"default\": 1,"]
#[doc = "      \"type\": ["]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"format\": \"uint64\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"little_u16\": {"]
#[doc = "      \"default\": 3,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint16\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"little_u8\": {"]
#[doc = "      \"default\": 2,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"uint8\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MrDefaultNumbers {
    #[serde(default = "defaults::mr_default_numbers_big_nullable")]
    pub big_nullable: ::std::option::Option<::std::num::NonZeroU64>,
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU16, 3>")]
    pub little_u16: ::std::num::NonZeroU16,
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU8, 2>")]
    pub little_u8: ::std::num::NonZeroU8,
}
impl ::std::convert::From<&MrDefaultNumbers> for MrDefaultNumbers {
    fn from(value: &MrDefaultNumbers) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MrDefaultNumbers {
    fn default() -> Self {
        Self {
            big_nullable: defaults::mr_default_numbers_big_nullable(),
            little_u16: defaults::default_nzu64::<::std::num::NonZeroU16, 3>(),
            little_u8: defaults::default_nzu64::<::std::num::NonZeroU8, 2>(),
        }
    }
}
impl MrDefaultNumbers {
    pub fn builder() -> builder::MrDefaultNumbers {
        Default::default()
    }
}
#[doc = "`OuterThing`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"thing\": {"]
#[doc = "      \"title\": \"ThingWithDefaults\","]
#[doc = "      \"default\": {"]
#[doc = "        \"type\": \"bee\""]
#[doc = "      },"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"a\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OuterThing {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thing: ::std::option::Option<ThingWithDefaults>,
}
impl ::std::convert::From<&OuterThing> for OuterThing {
    fn from(value: &OuterThing) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OuterThing {
    fn default() -> Self {
        Self {
            thing: Default::default(),
        }
    }
}
impl OuterThing {
    pub fn builder() -> builder::OuterThing {
        Default::default()
    }
}
#[doc = "`TestBed`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"any\": {"]
#[doc = "      \"default\": ["]
#[doc = "        ["]
#[doc = "          8,"]
#[doc = "          6,"]
#[doc = "          7"]
#[doc = "        ],"]
#[doc = "        ["]
#[doc = "          5,"]
#[doc = "          3,"]
#[doc = "          0,"]
#[doc = "          9"]
#[doc = "        ]"]
#[doc = "      ],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {}"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"default\": \"abc123-is-this-a-uuid\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uuid\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestBed {
    #[serde(default = "defaults::test_bed_any")]
    pub any: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default = "defaults::test_bed_id")]
    pub id: ::uuid::Uuid,
}
impl ::std::convert::From<&TestBed> for TestBed {
    fn from(value: &TestBed) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TestBed {
    fn default() -> Self {
        Self {
            any: defaults::test_bed_any(),
            id: defaults::test_bed_id(),
        }
    }
}
impl TestBed {
    pub fn builder() -> builder::TestBed {
        Default::default()
    }
}
#[doc = "`ThingWithDefaults`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThingWithDefaults\","]
#[doc = "  \"default\": {"]
#[doc = "    \"type\": \"bee\""]
#[doc = "  },"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"a\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ThingWithDefaults {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub a: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ThingWithDefaults> for ThingWithDefaults {
    fn from(value: &ThingWithDefaults) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ThingWithDefaults {
    fn default() -> Self {
        ThingWithDefaults {
            a: Default::default(),
            type_: ::std::option::Option::Some("bee".to_string()),
        }
    }
}
impl ThingWithDefaults {
    pub fn builder() -> builder::ThingWithDefaults {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Doodad {
        when:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
    }
    impl ::std::default::Default for Doodad {
        fn default() -> Self {
            Self {
                when: Ok(super::defaults::doodad_when()),
            }
        }
    }
    impl Doodad {
        pub fn when<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.when = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for when: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Doodad> for super::Doodad {
        type Error = super::error::ConversionError;
        fn try_from(value: Doodad) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { when: value.when? })
        }
    }
    impl ::std::convert::From<super::Doodad> for Doodad {
        fn from(value: super::Doodad) -> Self {
            Self {
                when: Ok(value.when),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MrDefaultNumbers {
        big_nullable: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        little_u16: ::std::result::Result<::std::num::NonZeroU16, ::std::string::String>,
        little_u8: ::std::result::Result<::std::num::NonZeroU8, ::std::string::String>,
    }
    impl ::std::default::Default for MrDefaultNumbers {
        fn default() -> Self {
            Self {
                big_nullable: Ok(super::defaults::mr_default_numbers_big_nullable()),
                little_u16: Ok(super::defaults::default_nzu64::<::std::num::NonZeroU16, 3>()),
                little_u8: Ok(super::defaults::default_nzu64::<::std::num::NonZeroU8, 2>()),
            }
        }
    }
    impl MrDefaultNumbers {
        pub fn big_nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.big_nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for big_nullable: {}", e));
            self
        }
        pub fn little_u16<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU16>,
            T::Error: ::std::fmt::Display,
        {
            self.little_u16 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for little_u16: {}", e));
            self
        }
        pub fn little_u8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU8>,
            T::Error: ::std::fmt::Display,
        {
            self.little_u8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for little_u8: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MrDefaultNumbers> for super::MrDefaultNumbers {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MrDefaultNumbers,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                big_nullable: value.big_nullable?,
                little_u16: value.little_u16?,
                little_u8: value.little_u8?,
            })
        }
    }
    impl ::std::convert::From<super::MrDefaultNumbers> for MrDefaultNumbers {
        fn from(value: super::MrDefaultNumbers) -> Self {
            Self {
                big_nullable: Ok(value.big_nullable),
                little_u16: Ok(value.little_u16),
                little_u8: Ok(value.little_u8),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OuterThing {
        thing: ::std::result::Result<
            ::std::option::Option<super::ThingWithDefaults>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OuterThing {
        fn default() -> Self {
            Self {
                thing: Ok(Default::default()),
            }
        }
    }
    impl OuterThing {
        pub fn thing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ThingWithDefaults>>,
            T::Error: ::std::fmt::Display,
        {
            self.thing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thing: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OuterThing> for super::OuterThing {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OuterThing,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                thing: value.thing?,
            })
        }
    }
    impl ::std::convert::From<super::OuterThing> for OuterThing {
        fn from(value: super::OuterThing) -> Self {
            Self {
                thing: Ok(value.thing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TestBed {
        any: ::std::result::Result<::std::vec::Vec<::serde_json::Value>, ::std::string::String>,
        id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
    }
    impl ::std::default::Default for TestBed {
        fn default() -> Self {
            Self {
                any: Ok(super::defaults::test_bed_any()),
                id: Ok(super::defaults::test_bed_id()),
            }
        }
    }
    impl TestBed {
        pub fn any<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.any = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for any: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::uuid::Uuid>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TestBed> for super::TestBed {
        type Error = super::error::ConversionError;
        fn try_from(value: TestBed) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                any: value.any?,
                id: value.id?,
            })
        }
    }
    impl ::std::convert::From<super::TestBed> for TestBed {
        fn from(value: super::TestBed) -> Self {
            Self {
                any: Ok(value.any),
                id: Ok(value.id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThingWithDefaults {
        a: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ThingWithDefaults {
        fn default() -> Self {
            Self {
                a: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl ThingWithDefaults {
        pub fn a<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.a = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for a: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ThingWithDefaults> for super::ThingWithDefaults {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThingWithDefaults,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                a: value.a?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ThingWithDefaults> for ThingWithDefaults {
        fn from(value: super::ThingWithDefaults) -> Self {
            Self {
                a: Ok(value.a),
                type_: Ok(value.type_),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    pub(super) fn doodad_when() -> ::chrono::DateTime<::chrono::offset::Utc> {
        ::serde_json::from_str::<::chrono::DateTime<::chrono::offset::Utc>>(
            "\"1970-01-01T00:00:00Z\"",
        )
        .unwrap()
    }
    pub(super) fn mr_default_numbers_big_nullable() -> ::std::option::Option<::std::num::NonZeroU64>
    {
        ::std::option::Option::Some(::std::num::NonZeroU64::new(1).unwrap())
    }
    pub(super) fn test_bed_any() -> ::std::vec::Vec<::serde_json::Value> {
        vec![
            ::serde_json::from_str::<::serde_json::Value>("[8,6,7]").unwrap(),
            ::serde_json::from_str::<::serde_json::Value>("[5,3,0,9]").unwrap(),
        ]
    }
    pub(super) fn test_bed_id() -> ::uuid::Uuid {
        ::serde_json::from_str::<::uuid::Uuid>("\"abc123-is-this-a-uuid\"").unwrap()
    }
}
fn main() {}
