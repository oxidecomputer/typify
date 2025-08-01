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
#[doc = "`CoreSchemaMetaSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"http://json-schema.org/draft-07/schema\","]
#[doc = "  \"title\": \"Core schema meta-schema\","]
#[doc = "  \"default\": true,"]
#[doc = "  \"type\": ["]
#[doc = "    \"object\","]
#[doc = "    \"boolean\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$comment\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"$id\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\""]
#[doc = "    },"]
#[doc = "    \"$ref\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-reference\""]
#[doc = "    },"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"additionalItems\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"additionalProperties\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"allOf\": {"]
#[doc = "      \"$ref\": \"#/definitions/schemaArray\""]
#[doc = "    },"]
#[doc = "    \"anyOf\": {"]
#[doc = "      \"$ref\": \"#/definitions/schemaArray\""]
#[doc = "    },"]
#[doc = "    \"const\": true,"]
#[doc = "    \"contains\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"contentEncoding\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"contentMediaType\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"default\": true,"]
#[doc = "    \"definitions\": {"]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"dependencies\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/stringArray\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"else\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": true,"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"examples\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": true"]
#[doc = "    },"]
#[doc = "    \"exclusiveMaximum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"exclusiveMinimum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"if\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"default\": true,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/schemaArray\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"maxItems\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeInteger\""]
#[doc = "    },"]
#[doc = "    \"maxLength\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeInteger\""]
#[doc = "    },"]
#[doc = "    \"maxProperties\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeInteger\""]
#[doc = "    },"]
#[doc = "    \"maximum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"minItems\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeIntegerDefault0\""]
#[doc = "    },"]
#[doc = "    \"minLength\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeIntegerDefault0\""]
#[doc = "    },"]
#[doc = "    \"minProperties\": {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeIntegerDefault0\""]
#[doc = "    },"]
#[doc = "    \"minimum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"multipleOf\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"not\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"oneOf\": {"]
#[doc = "      \"$ref\": \"#/definitions/schemaArray\""]
#[doc = "    },"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"regex\""]
#[doc = "    },"]
#[doc = "    \"patternProperties\": {"]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"format\": \"regex\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"properties\": {"]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"propertyNames\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"readOnly\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"$ref\": \"#/definitions/stringArray\""]
#[doc = "    },"]
#[doc = "    \"then\": {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/simpleTypes\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/simpleTypes\""]
#[doc = "          },"]
#[doc = "          \"minItems\": 1,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"uniqueItems\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$comment\": \"tests reflexive schemas with the json schema draft 7 schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchema {
    Boolean(bool),
    Object {
        #[serde(
            rename = "additionalItems",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        additional_items: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "additionalProperties",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        additional_properties: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "allOf",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        all_of: ::std::option::Option<SchemaArray>,
        #[serde(
            rename = "anyOf",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        any_of: ::std::option::Option<SchemaArray>,
        #[serde(
            rename = "$comment",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        comment: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "const",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        const_: ::std::option::Option<::serde_json::Value>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        contains: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "contentEncoding",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        content_encoding: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "contentMediaType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        content_media_type: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        default: ::std::option::Option<::serde_json::Value>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        definitions: ::std::collections::HashMap<::std::string::String, CoreSchemaMetaSchema>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        dependencies: ::std::collections::HashMap<
            ::std::string::String,
            CoreSchemaMetaSchemaObjectDependenciesValue,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "else",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        else_: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "enum",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        enum_: ::std::option::Option<Vec<::serde_json::Value>>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        examples: ::std::vec::Vec<::serde_json::Value>,
        #[serde(
            rename = "exclusiveMaximum",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        exclusive_maximum: ::std::option::Option<f64>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        exclusive_minimum: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        format: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "$id",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "if",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        if_: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(default = "defaults::core_schema_meta_schema_object_items")]
        items: CoreSchemaMetaSchemaObjectItems,
        #[serde(
            rename = "maxItems",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        max_items: ::std::option::Option<NonNegativeInteger>,
        #[serde(
            rename = "maxLength",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        max_length: ::std::option::Option<NonNegativeInteger>,
        #[serde(
            rename = "maxProperties",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        max_properties: ::std::option::Option<NonNegativeInteger>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        maximum: ::std::option::Option<f64>,
        #[serde(
            rename = "minItems",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        min_items: ::std::option::Option<NonNegativeIntegerDefault0>,
        #[serde(
            rename = "minLength",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        min_length: ::std::option::Option<NonNegativeIntegerDefault0>,
        #[serde(
            rename = "minProperties",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        min_properties: ::std::option::Option<NonNegativeIntegerDefault0>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        minimum: ::std::option::Option<f64>,
        #[serde(
            rename = "multipleOf",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        multiple_of: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        not: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(
            rename = "oneOf",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        one_of: ::std::option::Option<SchemaArray>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pattern: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "patternProperties",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pattern_properties:
            ::std::collections::HashMap<::std::string::String, CoreSchemaMetaSchema>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        properties: ::std::collections::HashMap<::std::string::String, CoreSchemaMetaSchema>,
        #[serde(
            rename = "propertyNames",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        property_names: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(rename = "readOnly", default)]
        read_only: bool,
        #[serde(
            rename = "$ref",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        ref_: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        required: ::std::option::Option<StringArray>,
        #[serde(
            rename = "$schema",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        schema: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        then: ::std::option::Option<::std::boxed::Box<CoreSchemaMetaSchema>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        title: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        type_: ::std::option::Option<CoreSchemaMetaSchemaObjectType>,
        #[serde(rename = "uniqueItems", default)]
        unique_items: bool,
    },
}
impl ::std::convert::From<&Self> for CoreSchemaMetaSchema {
    fn from(value: &CoreSchemaMetaSchema) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CoreSchemaMetaSchema {
    fn default() -> Self {
        CoreSchemaMetaSchema::Boolean(true)
    }
}
impl ::std::convert::From<bool> for CoreSchemaMetaSchema {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
#[doc = "`CoreSchemaMetaSchemaObjectDependenciesValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/stringArray\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectDependenciesValue {
    Variant0(CoreSchemaMetaSchema),
    Variant1(StringArray),
}
impl ::std::convert::From<&Self> for CoreSchemaMetaSchemaObjectDependenciesValue {
    fn from(value: &CoreSchemaMetaSchemaObjectDependenciesValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<CoreSchemaMetaSchema> for CoreSchemaMetaSchemaObjectDependenciesValue {
    fn from(value: CoreSchemaMetaSchema) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<StringArray> for CoreSchemaMetaSchemaObjectDependenciesValue {
    fn from(value: StringArray) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`CoreSchemaMetaSchemaObjectItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": true,"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/schemaArray\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectItems {
    Variant0(::std::boxed::Box<CoreSchemaMetaSchema>),
    Variant1(SchemaArray),
}
impl ::std::convert::From<&Self> for CoreSchemaMetaSchemaObjectItems {
    fn from(value: &CoreSchemaMetaSchemaObjectItems) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CoreSchemaMetaSchemaObjectItems {
    fn default() -> Self {
        CoreSchemaMetaSchemaObjectItems::Variant0(::std::boxed::Box::new(
            CoreSchemaMetaSchema::Boolean(true),
        ))
    }
}
impl ::std::convert::From<::std::boxed::Box<CoreSchemaMetaSchema>>
    for CoreSchemaMetaSchemaObjectItems
{
    fn from(value: ::std::boxed::Box<CoreSchemaMetaSchema>) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<SchemaArray> for CoreSchemaMetaSchemaObjectItems {
    fn from(value: SchemaArray) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`CoreSchemaMetaSchemaObjectType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/simpleTypes\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/simpleTypes\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CoreSchemaMetaSchemaObjectType {
    Variant0(SimpleTypes),
    Variant1(Vec<SimpleTypes>),
}
impl ::std::convert::From<&Self> for CoreSchemaMetaSchemaObjectType {
    fn from(value: &CoreSchemaMetaSchemaObjectType) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<SimpleTypes> for CoreSchemaMetaSchemaObjectType {
    fn from(value: SimpleTypes) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<Vec<SimpleTypes>> for CoreSchemaMetaSchemaObjectType {
    fn from(value: Vec<SimpleTypes>) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`NonNegativeInteger`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"integer\","]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NonNegativeInteger(pub u64);
impl ::std::ops::Deref for NonNegativeInteger {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<NonNegativeInteger> for u64 {
    fn from(value: NonNegativeInteger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NonNegativeInteger> for NonNegativeInteger {
    fn from(value: &NonNegativeInteger) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<u64> for NonNegativeInteger {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NonNegativeInteger {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NonNegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NonNegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NonNegativeInteger {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NonNegativeInteger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`NonNegativeIntegerDefault0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/nonNegativeInteger\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"default\": 0"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NonNegativeIntegerDefault0(pub NonNegativeInteger);
impl ::std::ops::Deref for NonNegativeIntegerDefault0 {
    type Target = NonNegativeInteger;
    fn deref(&self) -> &NonNegativeInteger {
        &self.0
    }
}
impl ::std::convert::From<NonNegativeIntegerDefault0> for NonNegativeInteger {
    fn from(value: NonNegativeIntegerDefault0) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NonNegativeIntegerDefault0> for NonNegativeIntegerDefault0 {
    fn from(value: &NonNegativeIntegerDefault0) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<NonNegativeInteger> for NonNegativeIntegerDefault0 {
    fn from(value: NonNegativeInteger) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for NonNegativeIntegerDefault0 {
    type Err = <NonNegativeInteger as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for NonNegativeIntegerDefault0 {
    type Error = <NonNegativeInteger as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for NonNegativeIntegerDefault0 {
    type Error = <NonNegativeInteger as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for NonNegativeIntegerDefault0 {
    type Error = <NonNegativeInteger as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for NonNegativeIntegerDefault0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "`SchemaArray`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$ref\": \"#\""]
#[doc = "  },"]
#[doc = "  \"minItems\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SchemaArray(pub ::std::vec::Vec<CoreSchemaMetaSchema>);
impl ::std::ops::Deref for SchemaArray {
    type Target = ::std::vec::Vec<CoreSchemaMetaSchema>;
    fn deref(&self) -> &::std::vec::Vec<CoreSchemaMetaSchema> {
        &self.0
    }
}
impl ::std::convert::From<SchemaArray> for ::std::vec::Vec<CoreSchemaMetaSchema> {
    fn from(value: SchemaArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SchemaArray> for SchemaArray {
    fn from(value: &SchemaArray) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<CoreSchemaMetaSchema>> for SchemaArray {
    fn from(value: ::std::vec::Vec<CoreSchemaMetaSchema>) -> Self {
        Self(value)
    }
}
#[doc = "`SimpleTypes`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"array\","]
#[doc = "    \"boolean\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\","]
#[doc = "    \"number\","]
#[doc = "    \"object\","]
#[doc = "    \"string\""]
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
impl ::std::convert::From<&Self> for SimpleTypes {
    fn from(value: &SimpleTypes) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SimpleTypes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Array => f.write_str("array"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Null => f.write_str("null"),
            Self::Number => f.write_str("number"),
            Self::Object => f.write_str("object"),
            Self::String => f.write_str("string"),
        }
    }
}
impl ::std::str::FromStr for SimpleTypes {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "array" => Ok(Self::Array),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "null" => Ok(Self::Null),
            "number" => Ok(Self::Number),
            "object" => Ok(Self::Object),
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SimpleTypes {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SimpleTypes {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SimpleTypes {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`StringArray`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": [],"]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"uniqueItems\": true"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct StringArray(pub Vec<::std::string::String>);
impl ::std::ops::Deref for StringArray {
    type Target = Vec<::std::string::String>;
    fn deref(&self) -> &Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<StringArray> for Vec<::std::string::String> {
    fn from(value: StringArray) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StringArray> for StringArray {
    fn from(value: &StringArray) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::std::string::String>> for StringArray {
    fn from(value: Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn core_schema_meta_schema_object_items() -> super::CoreSchemaMetaSchemaObjectItems {
        super::CoreSchemaMetaSchemaObjectItems::Variant0(::std::boxed::Box::new(
            super::CoreSchemaMetaSchema::Boolean(true),
        ))
    }
}
fn main() {}
