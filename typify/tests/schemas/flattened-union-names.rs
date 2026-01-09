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
#[doc = "`BlockStep`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"block\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"block\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"prompt\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BlockStep {
    pub block: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
}
impl BlockStep {
    pub fn builder() -> builder::BlockStep {
        Default::default()
    }
}
#[doc = "`CommandStep`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"command\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"command\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CommandStep {
    pub command: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
}
impl CommandStep {
    pub fn builder() -> builder::CommandStep {
        Default::default()
    }
}
#[doc = "`TestFlattenedUnion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TestFlattenedUnion\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"steps\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"steps\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/CommandStep\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/WaitStep\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/BlockStep\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TestFlattenedUnion {
    pub steps: ::std::vec::Vec<TestFlattenedUnionStepsItem>,
}
impl TestFlattenedUnion {
    pub fn builder() -> builder::TestFlattenedUnion {
        Default::default()
    }
}
#[doc = "`TestFlattenedUnionStepsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CommandStep\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/WaitStep\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlockStep\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TestFlattenedUnionStepsItem {
    CommandStep(CommandStep),
    WaitStep(WaitStep),
    BlockStep(BlockStep),
}
impl ::std::convert::From<CommandStep> for TestFlattenedUnionStepsItem {
    fn from(value: CommandStep) -> Self {
        Self::CommandStep(value)
    }
}
impl ::std::convert::From<WaitStep> for TestFlattenedUnionStepsItem {
    fn from(value: WaitStep) -> Self {
        Self::WaitStep(value)
    }
}
impl ::std::convert::From<BlockStep> for TestFlattenedUnionStepsItem {
    fn from(value: BlockStep) -> Self {
        Self::BlockStep(value)
    }
}
#[doc = "`WaitStep`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"wait\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WaitStep {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wait: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WaitStep {
    fn default() -> Self {
        Self {
            wait: Default::default(),
        }
    }
}
impl WaitStep {
    pub fn builder() -> builder::WaitStep {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BlockStep {
        block: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BlockStep {
        fn default() -> Self {
            Self {
                block: Err("no value supplied for block".to_string()),
                prompt: Ok(Default::default()),
            }
        }
    }
    impl BlockStep {
        pub fn block<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.block = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for block: {e}"));
            self
        }
        pub fn prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<BlockStep> for super::BlockStep {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BlockStep,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                block: value.block?,
                prompt: value.prompt?,
            })
        }
    }
    impl ::std::convert::From<super::BlockStep> for BlockStep {
        fn from(value: super::BlockStep) -> Self {
            Self {
                block: Ok(value.block),
                prompt: Ok(value.prompt),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CommandStep {
        command: ::std::result::Result<::std::string::String, ::std::string::String>,
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CommandStep {
        fn default() -> Self {
            Self {
                command: Err("no value supplied for command".to_string()),
                label: Ok(Default::default()),
            }
        }
    }
    impl CommandStep {
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for command: {e}"));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CommandStep> for super::CommandStep {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CommandStep,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                command: value.command?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::CommandStep> for CommandStep {
        fn from(value: super::CommandStep) -> Self {
            Self {
                command: Ok(value.command),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TestFlattenedUnion {
        steps: ::std::result::Result<
            ::std::vec::Vec<super::TestFlattenedUnionStepsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TestFlattenedUnion {
        fn default() -> Self {
            Self {
                steps: Err("no value supplied for steps".to_string()),
            }
        }
    }
    impl TestFlattenedUnion {
        pub fn steps<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TestFlattenedUnionStepsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.steps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for steps: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<TestFlattenedUnion> for super::TestFlattenedUnion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TestFlattenedUnion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                steps: value.steps?,
            })
        }
    }
    impl ::std::convert::From<super::TestFlattenedUnion> for TestFlattenedUnion {
        fn from(value: super::TestFlattenedUnion) -> Self {
            Self {
                steps: Ok(value.steps),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WaitStep {
        wait: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WaitStep {
        fn default() -> Self {
            Self {
                wait: Ok(Default::default()),
            }
        }
    }
    impl WaitStep {
        pub fn wait<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.wait = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wait: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<WaitStep> for super::WaitStep {
        type Error = super::error::ConversionError;
        fn try_from(value: WaitStep) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { wait: value.wait? })
        }
    }
    impl ::std::convert::From<super::WaitStep> for WaitStep {
        fn from(value: super::WaitStep) -> Self {
            Self {
                wait: Ok(value.wait),
            }
        }
    }
}
fn main() {}
