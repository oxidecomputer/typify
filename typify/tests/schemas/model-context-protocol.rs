#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
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
#[doc = "Base for objects that include optional annotations for the client. The client can use annotations to inform how objects are used or displayed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Base for objects that include optional annotations for the client. The client can use annotations to inform how objects are used or displayed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Annotated {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<AnnotatedAnnotations>,
}
impl From<&Annotated> for Annotated {
    fn from(value: &Annotated) -> Self {
        value.clone()
    }
}
#[doc = "AnnotatedAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AnnotatedAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&AnnotatedAnnotations> for AnnotatedAnnotations {
    fn from(value: &AnnotatedAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "BlobResourceContents"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"blob\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"blob\": {"]
#[doc = "      \"description\": \"A base64-encoded string representing the binary data of the item.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"byte\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BlobResourceContents {
    #[doc = "A base64-encoded string representing the binary data of the item."]
    pub blob: ::std::string::String,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl From<&BlobResourceContents> for BlobResourceContents {
    fn from(value: &BlobResourceContents) -> Self {
        value.clone()
    }
}
#[doc = "Used by the client to invoke a tool provided by the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Used by the client to invoke a tool provided by the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tools/call\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequest {
    pub method: ::std::string::String,
    pub params: CallToolRequestParams,
}
impl From<&CallToolRequest> for CallToolRequest {
    fn from(value: &CallToolRequest) -> Self {
        value.clone()
    }
}
#[doc = "CallToolRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolRequestParams {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub arguments: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub name: ::std::string::String,
}
impl From<&CallToolRequestParams> for CallToolRequestParams {
    fn from(value: &CallToolRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a tool call.\n\nAny errors that originate from the tool SHOULD be reported inside the result\nobject, with `isError` set to true, _not_ as an MCP protocol-level error\nresponse. Otherwise, the LLM would not be able to see that an error occurred\nand self-correct.\n\nHowever, any errors in _finding_ the tool, an error indicating that the\nserver does not support tool calls, or any other exceptional conditions,\nshould be reported as an MCP error response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a tool call.\\n\\nAny errors that originate from the tool SHOULD be reported inside the result\\nobject, with `isError` set to true, _not_ as an MCP protocol-level error\\nresponse. Otherwise, the LLM would not be able to see that an error occurred\\nand self-correct.\\n\\nHowever, any errors in _finding_ the tool, an error indicating that the\\nserver does not support tool calls, or any other exceptional conditions,\\nshould be reported as an MCP error response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/TextContent\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/EmbeddedResource\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"isError\": {"]
#[doc = "      \"description\": \"Whether the tool call ended in an error.\\n\\nIf not set, this is assumed to be false (the call was successful).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CallToolResult {
    pub content: ::std::vec::Vec<CallToolResultContentItem>,
    #[doc = "Whether the tool call ended in an error.\n\nIf not set, this is assumed to be false (the call was successful)."]
    #[serde(
        rename = "isError",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_error: ::std::option::Option<bool>,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&CallToolResult> for CallToolResult {
    fn from(value: &CallToolResult) -> Self {
        value.clone()
    }
}
#[doc = "CallToolResultContentItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmbeddedResource\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CallToolResultContentItem {
    TextContent(TextContent),
    ImageContent(ImageContent),
    EmbeddedResource(EmbeddedResource),
}
impl From<&CallToolResultContentItem> for CallToolResultContentItem {
    fn from(value: &CallToolResultContentItem) -> Self {
        value.clone()
    }
}
impl From<TextContent> for CallToolResultContentItem {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl From<ImageContent> for CallToolResultContentItem {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl From<EmbeddedResource> for CallToolResultContentItem {
    fn from(value: EmbeddedResource) -> Self {
        Self::EmbeddedResource(value)
    }
}
#[doc = "This notification can be sent by either side to indicate that it is cancelling a previously-issued request.\n\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\n\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease.\n\nA client MUST NOT attempt to cancel its `initialize` request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This notification can be sent by either side to indicate that it is cancelling a previously-issued request.\\n\\nThe request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.\\n\\nThis notification indicates that the result will be unused, so any associated processing SHOULD cease.\\n\\nA client MUST NOT attempt to cancel its `initialize` request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/cancelled\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"requestId\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"reason\": {"]
#[doc = "          \"description\": \"An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"requestId\": {"]
#[doc = "          \"description\": \"The ID of the request to cancel.\\n\\nThis MUST correspond to the ID of a request previously issued in the same direction.\","]
#[doc = "          \"$ref\": \"#/definitions/RequestId\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotification {
    pub method: ::std::string::String,
    pub params: CancelledNotificationParams,
}
impl From<&CancelledNotification> for CancelledNotification {
    fn from(value: &CancelledNotification) -> Self {
        value.clone()
    }
}
#[doc = "CancelledNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"requestId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"reason\": {"]
#[doc = "      \"description\": \"An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"requestId\": {"]
#[doc = "      \"description\": \"The ID of the request to cancel.\\n\\nThis MUST correspond to the ID of a request previously issued in the same direction.\","]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelledNotificationParams {
    #[doc = "An optional string describing the reason for the cancellation. This MAY be logged or presented to the user."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the request to cancel.\n\nThis MUST correspond to the ID of a request previously issued in the same direction."]
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
impl From<&CancelledNotificationParams> for CancelledNotificationParams {
    fn from(value: &CancelledNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"experimental\": {"]
#[doc = "      \"description\": \"Experimental, non-standard capabilities that the client supports.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"roots\": {"]
#[doc = "      \"description\": \"Present if the client supports listing roots.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether the client supports notifications for changes to the roots list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"sampling\": {"]
#[doc = "      \"description\": \"Present if the client supports sampling from an LLM.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientCapabilities {
    #[doc = "Experimental, non-standard capabilities that the client supports."]
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub experimental: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub roots: ::std::option::Option<ClientCapabilitiesRoots>,
    #[doc = "Present if the client supports sampling from an LLM."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub sampling: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&ClientCapabilities> for ClientCapabilities {
    fn from(value: &ClientCapabilities) -> Self {
        value.clone()
    }
}
#[doc = "Present if the client supports listing roots."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the client supports listing roots.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether the client supports notifications for changes to the roots list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientCapabilitiesRoots {
    #[doc = "Whether the client supports notifications for changes to the roots list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl From<&ClientCapabilitiesRoots> for ClientCapabilitiesRoots {
    fn from(value: &ClientCapabilitiesRoots) -> Self {
        value.clone()
    }
}
#[doc = "ClientNotification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelledNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ProgressNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RootsListChangedNotification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientNotification {
    CancelledNotification(CancelledNotification),
    InitializedNotification(InitializedNotification),
    ProgressNotification(ProgressNotification),
    RootsListChangedNotification(RootsListChangedNotification),
}
impl From<&ClientNotification> for ClientNotification {
    fn from(value: &ClientNotification) -> Self {
        value.clone()
    }
}
impl From<CancelledNotification> for ClientNotification {
    fn from(value: CancelledNotification) -> Self {
        Self::CancelledNotification(value)
    }
}
impl From<InitializedNotification> for ClientNotification {
    fn from(value: InitializedNotification) -> Self {
        Self::InitializedNotification(value)
    }
}
impl From<ProgressNotification> for ClientNotification {
    fn from(value: ProgressNotification) -> Self {
        Self::ProgressNotification(value)
    }
}
impl From<RootsListChangedNotification> for ClientNotification {
    fn from(value: RootsListChangedNotification) -> Self {
        Self::RootsListChangedNotification(value)
    }
}
#[doc = "ClientRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PingRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourcesRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ReadResourceRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SubscribeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnsubscribeRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListPromptsRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetPromptRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListToolsRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CallToolRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SetLevelRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CompleteRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientRequest {
    InitializeRequest(InitializeRequest),
    PingRequest(PingRequest),
    ListResourcesRequest(ListResourcesRequest),
    ReadResourceRequest(ReadResourceRequest),
    SubscribeRequest(SubscribeRequest),
    UnsubscribeRequest(UnsubscribeRequest),
    ListPromptsRequest(ListPromptsRequest),
    GetPromptRequest(GetPromptRequest),
    ListToolsRequest(ListToolsRequest),
    CallToolRequest(CallToolRequest),
    SetLevelRequest(SetLevelRequest),
    CompleteRequest(CompleteRequest),
}
impl From<&ClientRequest> for ClientRequest {
    fn from(value: &ClientRequest) -> Self {
        value.clone()
    }
}
impl From<InitializeRequest> for ClientRequest {
    fn from(value: InitializeRequest) -> Self {
        Self::InitializeRequest(value)
    }
}
impl From<PingRequest> for ClientRequest {
    fn from(value: PingRequest) -> Self {
        Self::PingRequest(value)
    }
}
impl From<ListResourcesRequest> for ClientRequest {
    fn from(value: ListResourcesRequest) -> Self {
        Self::ListResourcesRequest(value)
    }
}
impl From<ReadResourceRequest> for ClientRequest {
    fn from(value: ReadResourceRequest) -> Self {
        Self::ReadResourceRequest(value)
    }
}
impl From<SubscribeRequest> for ClientRequest {
    fn from(value: SubscribeRequest) -> Self {
        Self::SubscribeRequest(value)
    }
}
impl From<UnsubscribeRequest> for ClientRequest {
    fn from(value: UnsubscribeRequest) -> Self {
        Self::UnsubscribeRequest(value)
    }
}
impl From<ListPromptsRequest> for ClientRequest {
    fn from(value: ListPromptsRequest) -> Self {
        Self::ListPromptsRequest(value)
    }
}
impl From<GetPromptRequest> for ClientRequest {
    fn from(value: GetPromptRequest) -> Self {
        Self::GetPromptRequest(value)
    }
}
impl From<ListToolsRequest> for ClientRequest {
    fn from(value: ListToolsRequest) -> Self {
        Self::ListToolsRequest(value)
    }
}
impl From<CallToolRequest> for ClientRequest {
    fn from(value: CallToolRequest) -> Self {
        Self::CallToolRequest(value)
    }
}
impl From<SetLevelRequest> for ClientRequest {
    fn from(value: SetLevelRequest) -> Self {
        Self::SetLevelRequest(value)
    }
}
impl From<CompleteRequest> for ClientRequest {
    fn from(value: CompleteRequest) -> Self {
        Self::CompleteRequest(value)
    }
}
#[doc = "ClientResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CreateMessageResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListRootsResult\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClientResult {
    Result(Result),
    CreateMessageResult(CreateMessageResult),
    ListRootsResult(ListRootsResult),
}
impl From<&ClientResult> for ClientResult {
    fn from(value: &ClientResult) -> Self {
        value.clone()
    }
}
impl From<Result> for ClientResult {
    fn from(value: Result) -> Self {
        Self::Result(value)
    }
}
impl From<CreateMessageResult> for ClientResult {
    fn from(value: CreateMessageResult) -> Self {
        Self::CreateMessageResult(value)
    }
}
impl From<ListRootsResult> for ClientResult {
    fn from(value: ListRootsResult) -> Self {
        Self::ListRootsResult(value)
    }
}
#[doc = "A request from the client to the server, to ask for completion options."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the client to the server, to ask for completion options.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"completion/complete\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"argument\","]
#[doc = "        \"ref\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"argument\": {"]
#[doc = "          \"description\": \"The argument's information\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"name\","]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"name\": {"]
#[doc = "              \"description\": \"The name of the argument\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"value\": {"]
#[doc = "              \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"ref\": {"]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/ResourceReference\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequest {
    pub method: ::std::string::String,
    pub params: CompleteRequestParams,
}
impl From<&CompleteRequest> for CompleteRequest {
    fn from(value: &CompleteRequest) -> Self {
        value.clone()
    }
}
#[doc = "CompleteRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"argument\","]
#[doc = "    \"ref\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"argument\": {"]
#[doc = "      \"description\": \"The argument's information\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\","]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"The name of the argument\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"ref\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ResourceReference\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParams {
    pub argument: CompleteRequestParamsArgument,
    #[serde(rename = "ref")]
    pub ref_: CompleteRequestParamsRef,
}
impl From<&CompleteRequestParams> for CompleteRequestParams {
    fn from(value: &CompleteRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The argument's information"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The argument's information\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the argument\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The value of the argument to use for completion matching.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteRequestParamsArgument {
    #[doc = "The name of the argument"]
    pub name: ::std::string::String,
    #[doc = "The value of the argument to use for completion matching."]
    pub value: ::std::string::String,
}
impl From<&CompleteRequestParamsArgument> for CompleteRequestParamsArgument {
    fn from(value: &CompleteRequestParamsArgument) -> Self {
        value.clone()
    }
}
#[doc = "CompleteRequestParamsRef"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PromptReference\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceReference\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CompleteRequestParamsRef {
    PromptReference(PromptReference),
    ResourceReference(ResourceReference),
}
impl From<&CompleteRequestParamsRef> for CompleteRequestParamsRef {
    fn from(value: &CompleteRequestParamsRef) -> Self {
        value.clone()
    }
}
impl From<PromptReference> for CompleteRequestParamsRef {
    fn from(value: PromptReference) -> Self {
        Self::PromptReference(value)
    }
}
impl From<ResourceReference> for CompleteRequestParamsRef {
    fn from(value: ResourceReference) -> Self {
        Self::ResourceReference(value)
    }
}
#[doc = "The server's response to a completion/complete request"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a completion/complete request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"completion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"completion\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"values\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"hasMore\": {"]
#[doc = "          \"description\": \"Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"total\": {"]
#[doc = "          \"description\": \"The total number of completion options available. This can exceed the number of values actually sent in the response.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"description\": \"An array of completion values. Must not exceed 100 items.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResult {
    pub completion: CompleteResultCompletion,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&CompleteResult> for CompleteResult {
    fn from(value: &CompleteResult) -> Self {
        value.clone()
    }
}
#[doc = "CompleteResultCompletion"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"hasMore\": {"]
#[doc = "      \"description\": \"Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"description\": \"The total number of completion options available. This can exceed the number of values actually sent in the response.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"An array of completion values. Must not exceed 100 items.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CompleteResultCompletion {
    #[doc = "Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown."]
    #[serde(
        rename = "hasMore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_more: ::std::option::Option<bool>,
    #[doc = "The total number of completion options available. This can exceed the number of values actually sent in the response."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<i64>,
    #[doc = "An array of completion values. Must not exceed 100 items."]
    pub values: ::std::vec::Vec<::std::string::String>,
}
impl From<&CompleteResultCompletion> for CompleteResultCompletion {
    fn from(value: &CompleteResultCompletion) -> Self {
        value.clone()
    }
}
#[doc = "A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"sampling/createMessage\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"maxTokens\","]
#[doc = "        \"messages\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"includeContext\": {"]
#[doc = "          \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"allServers\","]
#[doc = "            \"none\","]
#[doc = "            \"thisServer\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"maxTokens\": {"]
#[doc = "          \"description\": \"The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/SamplingMessage\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"description\": \"Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": true"]
#[doc = "        },"]
#[doc = "        \"modelPreferences\": {"]
#[doc = "          \"description\": \"The server's preferences for which model to select. The client MAY ignore these preferences.\","]
#[doc = "          \"$ref\": \"#/definitions/ModelPreferences\""]
#[doc = "        },"]
#[doc = "        \"stopSequences\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"systemPrompt\": {"]
#[doc = "          \"description\": \"An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"temperature\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequest {
    pub method: ::std::string::String,
    pub params: CreateMessageRequestParams,
}
impl From<&CreateMessageRequest> for CreateMessageRequest {
    fn from(value: &CreateMessageRequest) -> Self {
        value.clone()
    }
}
#[doc = "CreateMessageRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"maxTokens\","]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"includeContext\": {"]
#[doc = "      \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"allServers\","]
#[doc = "        \"none\","]
#[doc = "        \"thisServer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"maxTokens\": {"]
#[doc = "      \"description\": \"The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/SamplingMessage\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"modelPreferences\": {"]
#[doc = "      \"description\": \"The server's preferences for which model to select. The client MAY ignore these preferences.\","]
#[doc = "      \"$ref\": \"#/definitions/ModelPreferences\""]
#[doc = "    },"]
#[doc = "    \"stopSequences\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"systemPrompt\": {"]
#[doc = "      \"description\": \"An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"temperature\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageRequestParams {
    #[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request."]
    #[serde(
        rename = "includeContext",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_context: ::std::option::Option<CreateMessageRequestParamsIncludeContext>,
    #[doc = "The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested."]
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,
    pub messages: ::std::vec::Vec<SamplingMessage>,
    #[doc = "Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The server's preferences for which model to select. The client MAY ignore these preferences."]
    #[serde(
        rename = "modelPreferences",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_preferences: ::std::option::Option<ModelPreferences>,
    #[serde(
        rename = "stopSequences",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub stop_sequences: ::std::vec::Vec<::std::string::String>,
    #[doc = "An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt."]
    #[serde(
        rename = "systemPrompt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_prompt: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<f64>,
}
impl From<&CreateMessageRequestParams> for CreateMessageRequestParams {
    fn from(value: &CreateMessageRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"allServers\","]
#[doc = "    \"none\","]
#[doc = "    \"thisServer\""]
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
pub enum CreateMessageRequestParamsIncludeContext {
    #[serde(rename = "allServers")]
    AllServers,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thisServer")]
    ThisServer,
}
impl From<&CreateMessageRequestParamsIncludeContext> for CreateMessageRequestParamsIncludeContext {
    fn from(value: &CreateMessageRequestParamsIncludeContext) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CreateMessageRequestParamsIncludeContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllServers => write!(f, "allServers"),
            Self::None => write!(f, "none"),
            Self::ThisServer => write!(f, "thisServer"),
        }
    }
}
impl std::str::FromStr for CreateMessageRequestParamsIncludeContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allServers" => Ok(Self::AllServers),
            "none" => Ok(Self::None),
            "thisServer" => Ok(Self::ThisServer),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for CreateMessageRequestParamsIncludeContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"model\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"model\": {"]
#[doc = "      \"description\": \"The name of the model that generated the message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    },"]
#[doc = "    \"stopReason\": {"]
#[doc = "      \"description\": \"The reason why sampling stopped, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CreateMessageResult {
    pub content: CreateMessageResultContent,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The name of the model that generated the message."]
    pub model: ::std::string::String,
    pub role: Role,
    #[doc = "The reason why sampling stopped, if known."]
    #[serde(
        rename = "stopReason",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stop_reason: ::std::option::Option<::std::string::String>,
}
impl From<&CreateMessageResult> for CreateMessageResult {
    fn from(value: &CreateMessageResult) -> Self {
        value.clone()
    }
}
#[doc = "CreateMessageResultContent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CreateMessageResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
}
impl From<&CreateMessageResultContent> for CreateMessageResultContent {
    fn from(value: &CreateMessageResultContent) -> Self {
        value.clone()
    }
}
impl From<TextContent> for CreateMessageResultContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl From<ImageContent> for CreateMessageResultContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
#[doc = "An opaque token used to represent a cursor for pagination."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An opaque token used to represent a cursor for pagination.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct Cursor(pub ::std::string::String);
impl ::std::ops::Deref for Cursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl From<Cursor> for ::std::string::String {
    fn from(value: Cursor) -> Self {
        value.0
    }
}
impl From<&Cursor> for Cursor {
    fn from(value: &Cursor) -> Self {
        value.clone()
    }
}
impl From<::std::string::String> for Cursor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Cursor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = "The contents of a resource, embedded into a prompt or tool call result.\n\nIt is up to the client how best to render embedded resources for the benefit\nof the LLM and/or the user."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contents of a resource, embedded into a prompt or tool call result.\\n\\nIt is up to the client how best to render embedded resources for the benefit\\nof the LLM and/or the user.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resource\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"resource\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resource\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmbeddedResource {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<EmbeddedResourceAnnotations>,
    pub resource: EmbeddedResourceResource,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl From<&EmbeddedResource> for EmbeddedResource {
    fn from(value: &EmbeddedResource) -> Self {
        value.clone()
    }
}
#[doc = "EmbeddedResourceAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmbeddedResourceAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&EmbeddedResourceAnnotations> for EmbeddedResourceAnnotations {
    fn from(value: &EmbeddedResourceAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "EmbeddedResourceResource"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl From<&EmbeddedResourceResource> for EmbeddedResourceResource {
    fn from(value: &EmbeddedResourceResource) -> Self {
        value.clone()
    }
}
impl From<TextResourceContents> for EmbeddedResourceResource {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl From<BlobResourceContents> for EmbeddedResourceResource {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "EmptyResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$ref\": \"#/definitions/Result\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct EmptyResult(pub Result);
impl ::std::ops::Deref for EmptyResult {
    type Target = Result;
    fn deref(&self) -> &Result {
        &self.0
    }
}
impl From<EmptyResult> for Result {
    fn from(value: EmptyResult) -> Self {
        value.0
    }
}
impl From<&EmptyResult> for EmptyResult {
    fn from(value: &EmptyResult) -> Self {
        value.clone()
    }
}
impl From<Result> for EmptyResult {
    fn from(value: Result) -> Self {
        Self(value)
    }
}
#[doc = "Used by the client to get a prompt provided by the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Used by the client to get a prompt provided by the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"prompts/get\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"arguments\": {"]
#[doc = "          \"description\": \"Arguments to use for templating the prompt.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"The name of the prompt or prompt template.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequest {
    pub method: ::std::string::String,
    pub params: GetPromptRequestParams,
}
impl From<&GetPromptRequest> for GetPromptRequest {
    fn from(value: &GetPromptRequest) -> Self {
        value.clone()
    }
}
#[doc = "GetPromptRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"Arguments to use for templating the prompt.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the prompt or prompt template.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptRequestParams {
    #[doc = "Arguments to use for templating the prompt."]
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub arguments: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The name of the prompt or prompt template."]
    pub name: ::std::string::String,
}
impl From<&GetPromptRequestParams> for GetPromptRequestParams {
    fn from(value: &GetPromptRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a prompts/get request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a prompts/get request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"messages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PromptMessage\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetPromptResult {
    #[doc = "An optional description for the prompt."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub messages: ::std::vec::Vec<PromptMessage>,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&GetPromptResult> for GetPromptResult {
    fn from(value: &GetPromptResult) -> Self {
        value.clone()
    }
}
#[doc = "An image provided to or from an LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An image provided to or from an LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"mimeType\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The base64-encoded image data.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"byte\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the image. Different providers may support different image types.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"image\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImageContent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<ImageContentAnnotations>,
    #[doc = "The base64-encoded image data."]
    pub data: ::std::string::String,
    #[doc = "The MIME type of the image. Different providers may support different image types."]
    #[serde(rename = "mimeType")]
    pub mime_type: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl From<&ImageContent> for ImageContent {
    fn from(value: &ImageContent) -> Self {
        value.clone()
    }
}
#[doc = "ImageContentAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImageContentAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&ImageContentAnnotations> for ImageContentAnnotations {
    fn from(value: &ImageContentAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "Describes the name and version of an MCP implementation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes the name and version of an MCP implementation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Implementation {
    pub name: ::std::string::String,
    pub version: ::std::string::String,
}
impl From<&Implementation> for Implementation {
    fn from(value: &Implementation) -> Self {
        value.clone()
    }
}
#[doc = "This request is sent from the client to the server when it first connects, asking it to begin initialization."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This request is sent from the client to the server when it first connects, asking it to begin initialization.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"initialize\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"capabilities\","]
#[doc = "        \"clientInfo\","]
#[doc = "        \"protocolVersion\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"capabilities\": {"]
#[doc = "          \"$ref\": \"#/definitions/ClientCapabilities\""]
#[doc = "        },"]
#[doc = "        \"clientInfo\": {"]
#[doc = "          \"$ref\": \"#/definitions/Implementation\""]
#[doc = "        },"]
#[doc = "        \"protocolVersion\": {"]
#[doc = "          \"description\": \"The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeRequest {
    pub method: ::std::string::String,
    pub params: InitializeRequestParams,
}
impl From<&InitializeRequest> for InitializeRequest {
    fn from(value: &InitializeRequest) -> Self {
        value.clone()
    }
}
#[doc = "InitializeRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"clientInfo\","]
#[doc = "    \"protocolVersion\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"$ref\": \"#/definitions/ClientCapabilities\""]
#[doc = "    },"]
#[doc = "    \"clientInfo\": {"]
#[doc = "      \"$ref\": \"#/definitions/Implementation\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeRequestParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    #[doc = "The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
}
impl From<&InitializeRequestParams> for InitializeRequestParams {
    fn from(value: &InitializeRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "After receiving an initialize request from the client, the server sends this response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"After receiving an initialize request from the client, the server sends this response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"protocolVersion\","]
#[doc = "    \"serverInfo\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"$ref\": \"#/definitions/ServerCapabilities\""]
#[doc = "    },"]
#[doc = "    \"instructions\": {"]
#[doc = "      \"description\": \"Instructions describing how to use the server and its features.\\n\\nThis can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a \\\"hint\\\" to the model. For example, this information MAY be added to the system prompt.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"serverInfo\": {"]
#[doc = "      \"$ref\": \"#/definitions/Implementation\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[doc = "Instructions describing how to use the server and its features.\n\nThis can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a \"hint\" to the model. For example, this information MAY be added to the system prompt."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
}
impl From<&InitializeResult> for InitializeResult {
    fn from(value: &InitializeResult) -> Self {
        value.clone()
    }
}
#[doc = "This notification is sent from the client to the server after initialization has finished."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"This notification is sent from the client to the server after initialization has finished.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/initialized\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<InitializedNotificationParams>,
}
impl From<&InitializedNotification> for InitializedNotification {
    fn from(value: &InitializedNotification) -> Self {
        value.clone()
    }
}
#[doc = "InitializedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InitializedNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&InitializedNotificationParams> for InitializedNotificationParams {
    fn from(value: &InitializedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "A response to a request that indicates an error occurred."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A response to a request that indicates an error occurred.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\","]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"code\","]
#[doc = "        \"message\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"code\": {"]
#[doc = "          \"description\": \"The error type that occurred.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"data\": {"]
#[doc = "          \"description\": \"Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).\""]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"description\": \"A short description of the error. The message SHOULD be limited to a concise single sentence.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcError {
    pub error: JsonrpcErrorError,
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
}
impl From<&JsonrpcError> for JsonrpcError {
    fn from(value: &JsonrpcError) -> Self {
        value.clone()
    }
}
#[doc = "JsonrpcErrorError"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error type that occurred.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A short description of the error. The message SHOULD be limited to a concise single sentence.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcErrorError {
    #[doc = "The error type that occurred."]
    pub code: i64,
    #[doc = "Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A short description of the error. The message SHOULD be limited to a concise single sentence."]
    pub message: ::std::string::String,
}
impl From<&JsonrpcErrorError> for JsonrpcErrorError {
    fn from(value: &JsonrpcErrorError) -> Self {
        value.clone()
    }
}
#[doc = "JsonrpcMessage"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCError\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcMessage {
    Request(JsonrpcRequest),
    Notification(JsonrpcNotification),
    Response(JsonrpcResponse),
    Error(JsonrpcError),
}
impl From<&JsonrpcMessage> for JsonrpcMessage {
    fn from(value: &JsonrpcMessage) -> Self {
        value.clone()
    }
}
impl From<JsonrpcRequest> for JsonrpcMessage {
    fn from(value: JsonrpcRequest) -> Self {
        Self::Request(value)
    }
}
impl From<JsonrpcNotification> for JsonrpcMessage {
    fn from(value: JsonrpcNotification) -> Self {
        Self::Notification(value)
    }
}
impl From<JsonrpcResponse> for JsonrpcMessage {
    fn from(value: JsonrpcResponse) -> Self {
        Self::Response(value)
    }
}
impl From<JsonrpcError> for JsonrpcMessage {
    fn from(value: JsonrpcError) -> Self {
        Self::Error(value)
    }
}
#[doc = "A notification which does not expect a response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification which does not expect a response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcNotification {
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<JsonrpcNotificationParams>,
}
impl From<&JsonrpcNotification> for JsonrpcNotification {
    fn from(value: &JsonrpcNotification) -> Self {
        value.clone()
    }
}
#[doc = "JsonrpcNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&JsonrpcNotificationParams> for JsonrpcNotificationParams {
    fn from(value: &JsonrpcNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "A request that expects a response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request that expects a response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequest {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<JsonrpcRequestParams>,
}
impl From<&JsonrpcRequest> for JsonrpcRequest {
    fn from(value: &JsonrpcRequest) -> Self {
        value.clone()
    }
}
#[doc = "JsonrpcRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<JsonrpcRequestParamsMeta>,
}
impl From<&JsonrpcRequestParams> for JsonrpcRequestParams {
    fn from(value: &JsonrpcRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "JsonrpcRequestParamsMeta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
}
impl From<&JsonrpcRequestParamsMeta> for JsonrpcRequestParamsMeta {
    fn from(value: &JsonrpcRequestParamsMeta) -> Self {
        value.clone()
    }
}
#[doc = "A successful (non-error) response to a request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A successful (non-error) response to a request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/definitions/RequestId\""]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcResponse {
    pub id: RequestId,
    pub jsonrpc: ::std::string::String,
    pub result: Result,
}
impl From<&JsonrpcResponse> for JsonrpcResponse {
    fn from(value: &JsonrpcResponse) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request a list of prompts and prompt templates the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of prompts and prompt templates the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"prompts/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListPromptsRequestParams>,
}
impl From<&ListPromptsRequest> for ListPromptsRequest {
    fn from(value: &ListPromptsRequest) -> Self {
        value.clone()
    }
}
#[doc = "ListPromptsRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl From<&ListPromptsRequestParams> for ListPromptsRequestParams {
    fn from(value: &ListPromptsRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a prompts/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a prompts/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"prompts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"prompts\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Prompt\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListPromptsResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub prompts: ::std::vec::Vec<Prompt>,
}
impl From<&ListPromptsResult> for ListPromptsResult {
    fn from(value: &ListPromptsResult) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request a list of resource templates the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of resource templates the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/templates/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListResourceTemplatesRequestParams>,
}
impl From<&ListResourceTemplatesRequest> for ListResourceTemplatesRequest {
    fn from(value: &ListResourceTemplatesRequest) -> Self {
        value.clone()
    }
}
#[doc = "ListResourceTemplatesRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl From<&ListResourceTemplatesRequestParams> for ListResourceTemplatesRequestParams {
    fn from(value: &ListResourceTemplatesRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a resources/templates/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/templates/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resourceTemplates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resourceTemplates\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ResourceTemplate\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourceTemplatesResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: ::std::vec::Vec<ResourceTemplate>,
}
impl From<&ListResourceTemplatesResult> for ListResourceTemplatesResult {
    fn from(value: &ListResourceTemplatesResult) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request a list of resources the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of resources the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListResourcesRequestParams>,
}
impl From<&ListResourcesRequest> for ListResourcesRequest {
    fn from(value: &ListResourcesRequest) -> Self {
        value.clone()
    }
}
#[doc = "ListResourcesRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl From<&ListResourcesRequestParams> for ListResourcesRequestParams {
    fn from(value: &ListResourcesRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a resources/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"resources\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"resources\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Resource\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListResourcesResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub resources: ::std::vec::Vec<Resource>,
}
impl From<&ListResourcesResult> for ListResourcesResult {
    fn from(value: &ListResourcesResult) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the server to request a list of root URIs from the client. Roots allow\nservers to ask for specific directories or files to operate on. A common example\nfor roots is providing a set of repositories or directories a server should operate\non.\n\nThis request is typically used when the server needs to understand the file system\nstructure or access specific locations that the client has permission to read from."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the server to request a list of root URIs from the client. Roots allow\\nservers to ask for specific directories or files to operate on. A common example\\nfor roots is providing a set of repositories or directories a server should operate\\non.\\n\\nThis request is typically used when the server needs to understand the file system\\nstructure or access specific locations that the client has permission to read from.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"roots/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListRootsRequestParams>,
}
impl From<&ListRootsRequest> for ListRootsRequest {
    fn from(value: &ListRootsRequest) -> Self {
        value.clone()
    }
}
#[doc = "ListRootsRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<ListRootsRequestParamsMeta>,
}
impl From<&ListRootsRequestParams> for ListRootsRequestParams {
    fn from(value: &ListRootsRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "ListRootsRequestParamsMeta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
}
impl From<&ListRootsRequestParamsMeta> for ListRootsRequestParamsMeta {
    fn from(value: &ListRootsRequestParamsMeta) -> Self {
        value.clone()
    }
}
#[doc = "The client's response to a roots/list request from the server.\nThis result contains an array of Root objects, each representing a root directory\nor file that the server can operate on."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The client's response to a roots/list request from the server.\\nThis result contains an array of Root objects, each representing a root directory\\nor file that the server can operate on.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"roots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"roots\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Root\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListRootsResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub roots: ::std::vec::Vec<Root>,
}
impl From<&ListRootsResult> for ListRootsResult {
    fn from(value: &ListRootsResult) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request a list of tools the server has."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request a list of tools the server has.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tools/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ListToolsRequestParams>,
}
impl From<&ListToolsRequest> for ListToolsRequest {
    fn from(value: &ListToolsRequest) -> Self {
        value.clone()
    }
}
#[doc = "ListToolsRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl From<&ListToolsRequestParams> for ListToolsRequestParams {
    fn from(value: &ListToolsRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a tools/list request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a tools/list request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tools\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Tool\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListToolsResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
    pub tools: ::std::vec::Vec<Tool>,
}
impl From<&ListToolsResult> for ListToolsResult {
    fn from(value: &ListToolsResult) -> Self {
        value.clone()
    }
}
#[doc = "The severity of a log message.\n\nThese map to syslog message severities, as specified in RFC-5424:\nhttps://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The severity of a log message.\\n\\nThese map to syslog message severities, as specified in RFC-5424:\\nhttps://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"alert\","]
#[doc = "    \"critical\","]
#[doc = "    \"debug\","]
#[doc = "    \"emergency\","]
#[doc = "    \"error\","]
#[doc = "    \"info\","]
#[doc = "    \"notice\","]
#[doc = "    \"warning\""]
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
pub enum LoggingLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl From<&LoggingLevel> for LoggingLevel {
    fn from(value: &LoggingLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LoggingLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alert => write!(f, "alert"),
            Self::Critical => write!(f, "critical"),
            Self::Debug => write!(f, "debug"),
            Self::Emergency => write!(f, "emergency"),
            Self::Error => write!(f, "error"),
            Self::Info => write!(f, "info"),
            Self::Notice => write!(f, "notice"),
            Self::Warning => write!(f, "warning"),
        }
    }
}
impl std::str::FromStr for LoggingLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alert" => Ok(Self::Alert),
            "critical" => Ok(Self::Critical),
            "debug" => Ok(Self::Debug),
            "emergency" => Ok(Self::Emergency),
            "error" => Ok(Self::Error),
            "info" => Ok(Self::Info),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for LoggingLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/message\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"data\","]
#[doc = "        \"level\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"data\": {"]
#[doc = "          \"description\": \"The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.\""]
#[doc = "        },"]
#[doc = "        \"level\": {"]
#[doc = "          \"description\": \"The severity of this log message.\","]
#[doc = "          \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "        },"]
#[doc = "        \"logger\": {"]
#[doc = "          \"description\": \"An optional name of the logger issuing this message.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotification {
    pub method: ::std::string::String,
    pub params: LoggingMessageNotificationParams,
}
impl From<&LoggingMessageNotification> for LoggingMessageNotification {
    fn from(value: &LoggingMessageNotification) -> Self {
        value.clone()
    }
}
#[doc = "LoggingMessageNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"level\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.\""]
#[doc = "    },"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"The severity of this log message.\","]
#[doc = "      \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "    },"]
#[doc = "    \"logger\": {"]
#[doc = "      \"description\": \"An optional name of the logger issuing this message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct LoggingMessageNotificationParams {
    #[doc = "The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here."]
    pub data: ::serde_json::Value,
    #[doc = "The severity of this log message."]
    pub level: LoggingLevel,
    #[doc = "An optional name of the logger issuing this message."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logger: ::std::option::Option<::std::string::String>,
}
impl From<&LoggingMessageNotificationParams> for LoggingMessageNotificationParams {
    fn from(value: &LoggingMessageNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Hints to use for model selection.\n\nKeys not declared here are currently left unspecified by the spec and are up\nto the client to interpret."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Hints to use for model selection.\\n\\nKeys not declared here are currently left unspecified by the spec and are up\\nto the client to interpret.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A hint for a model name.\\n\\nThe client SHOULD treat this as a substring of a model name; for example:\\n - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`\\n - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.\\n - `claude` should match any Claude model\\n\\nThe client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:\\n - `gemini-1.5-flash` could match `claude-3-haiku-20240307`\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelHint {
    #[doc = "A hint for a model name.\n\nThe client SHOULD treat this as a substring of a model name; for example:\n - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`\n - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.\n - `claude` should match any Claude model\n\nThe client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:\n - `gemini-1.5-flash` could match `claude-3-haiku-20240307`"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl From<&ModelHint> for ModelHint {
    fn from(value: &ModelHint) -> Self {
        value.clone()
    }
}
#[doc = "The server's preferences for model selection, requested of the client during sampling.\n\nBecause LLMs can vary along multiple dimensions, choosing the \"best\" model is\nrarely straightforward.  Different models excel in different areas—some are\nfaster but less capable, others are more capable but more expensive, and so\non. This interface allows servers to express their priorities across multiple\ndimensions to help clients make an appropriate selection for their use case.\n\nThese preferences are always advisory. The client MAY ignore them. It is also\nup to the client to decide how to interpret these preferences and how to\nbalance them against other considerations."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's preferences for model selection, requested of the client during sampling.\\n\\nBecause LLMs can vary along multiple dimensions, choosing the \\\"best\\\" model is\\nrarely straightforward.  Different models excel in different areas—some are\\nfaster but less capable, others are more capable but more expensive, and so\\non. This interface allows servers to express their priorities across multiple\\ndimensions to help clients make an appropriate selection for their use case.\\n\\nThese preferences are always advisory. The client MAY ignore them. It is also\\nup to the client to decide how to interpret these preferences and how to\\nbalance them against other considerations.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"costPriority\": {"]
#[doc = "      \"description\": \"How much to prioritize cost when selecting a model. A value of 0 means cost\\nis not important, while a value of 1 means cost is the most important\\nfactor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"hints\": {"]
#[doc = "      \"description\": \"Optional hints to use for model selection.\\n\\nIf multiple hints are specified, the client MUST evaluate them in order\\n(such that the first match is taken).\\n\\nThe client SHOULD prioritize these hints over the numeric priorities, but\\nMAY still use the priorities to select from ambiguous matches.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ModelHint\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"intelligencePriority\": {"]
#[doc = "      \"description\": \"How much to prioritize intelligence and capabilities when selecting a\\nmodel. A value of 0 means intelligence is not important, while a value of 1\\nmeans intelligence is the most important factor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"speedPriority\": {"]
#[doc = "      \"description\": \"How much to prioritize sampling speed (latency) when selecting a model. A\\nvalue of 0 means speed is not important, while a value of 1 means speed is\\nthe most important factor.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ModelPreferences {
    #[serde(
        rename = "costPriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cost_priority: ::std::option::Option<f64>,
    #[doc = "Optional hints to use for model selection.\n\nIf multiple hints are specified, the client MUST evaluate them in order\n(such that the first match is taken).\n\nThe client SHOULD prioritize these hints over the numeric priorities, but\nMAY still use the priorities to select from ambiguous matches."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub hints: ::std::vec::Vec<ModelHint>,
    #[serde(
        rename = "intelligencePriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intelligence_priority: ::std::option::Option<f64>,
    #[serde(
        rename = "speedPriority",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_priority: ::std::option::Option<f64>,
}
impl From<&ModelPreferences> for ModelPreferences {
    fn from(value: &ModelPreferences) -> Self {
        value.clone()
    }
}
#[doc = "Notification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Notification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<NotificationParams>,
}
impl From<&Notification> for Notification {
    fn from(value: &Notification) -> Self {
        value.clone()
    }
}
#[doc = "NotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct NotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&NotificationParams> for NotificationParams {
    fn from(value: &NotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "PaginatedRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"cursor\": {"]
#[doc = "          \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PaginatedRequestParams>,
}
impl From<&PaginatedRequest> for PaginatedRequest {
    fn from(value: &PaginatedRequest) -> Self {
        value.clone()
    }
}
#[doc = "PaginatedRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the current pagination position.\\nIf provided, the server should return results starting after this cursor.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedRequestParams {
    #[doc = "An opaque token representing the current pagination position.\nIf provided, the server should return results starting after this cursor."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
}
impl From<&PaginatedRequestParams> for PaginatedRequestParams {
    fn from(value: &PaginatedRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "PaginatedResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"nextCursor\": {"]
#[doc = "      \"description\": \"An opaque token representing the pagination position after the last returned result.\\nIf present, there may be more results available.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PaginatedResult {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An opaque token representing the pagination position after the last returned result.\nIf present, there may be more results available."]
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_cursor: ::std::option::Option<::std::string::String>,
}
impl From<&PaginatedResult> for PaginatedResult {
    fn from(value: &PaginatedResult) -> Self {
        value.clone()
    }
}
#[doc = "A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ping\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequest {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PingRequestParams>,
}
impl From<&PingRequest> for PingRequest {
    fn from(value: &PingRequest) -> Self {
        value.clone()
    }
}
#[doc = "PingRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<PingRequestParamsMeta>,
}
impl From<&PingRequestParams> for PingRequestParams {
    fn from(value: &PingRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "PingRequestParamsMeta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PingRequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
}
impl From<&PingRequestParamsMeta> for PingRequestParamsMeta {
    fn from(value: &PingRequestParamsMeta) -> Self {
        value.clone()
    }
}
#[doc = "An out-of-band notification used to inform the receiver of a progress update for a long-running request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An out-of-band notification used to inform the receiver of a progress update for a long-running request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/progress\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"progress\","]
#[doc = "        \"progressToken\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"progress\": {"]
#[doc = "          \"description\": \"The progress thus far. This should increase every time progress is made, even if the total is unknown.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        },"]
#[doc = "        \"total\": {"]
#[doc = "          \"description\": \"Total number of items to process (or total progress required), if known.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotification {
    pub method: ::std::string::String,
    pub params: ProgressNotificationParams,
}
impl From<&ProgressNotification> for ProgressNotification {
    fn from(value: &ProgressNotification) -> Self {
        value.clone()
    }
}
#[doc = "ProgressNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"progress\","]
#[doc = "    \"progressToken\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"progress\": {"]
#[doc = "      \"description\": \"The progress thus far. This should increase every time progress is made, even if the total is unknown.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"description\": \"Total number of items to process (or total progress required), if known.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ProgressNotificationParams {
    pub progress: f64,
    #[doc = "The progress token which was given in the initial request, used to associate this notification with the request that is proceeding."]
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub total: ::std::option::Option<f64>,
}
impl From<&ProgressNotificationParams> for ProgressNotificationParams {
    fn from(value: &ProgressNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "A progress token, used to associate progress notifications with the original request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A progress token, used to associate progress notifications with the original request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ProgressToken {
    String(::std::string::String),
    Integer(i64),
}
impl From<&ProgressToken> for ProgressToken {
    fn from(value: &ProgressToken) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ProgressToken {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for ProgressToken {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ProgressToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl From<i64> for ProgressToken {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "A prompt or prompt template that the server offers."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A prompt or prompt template that the server offers.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"arguments\": {"]
#[doc = "      \"description\": \"A list of arguments to use for templating the prompt.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PromptArgument\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description of what this prompt provides\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the prompt or prompt template.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Prompt {
    #[doc = "A list of arguments to use for templating the prompt."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub arguments: ::std::vec::Vec<PromptArgument>,
    #[doc = "An optional description of what this prompt provides"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the prompt or prompt template."]
    pub name: ::std::string::String,
}
impl From<&Prompt> for Prompt {
    fn from(value: &Prompt) -> Self {
        value.clone()
    }
}
#[doc = "Describes an argument that a prompt can accept."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes an argument that a prompt can accept.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the argument.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the argument.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Whether this argument must be provided.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptArgument {
    #[doc = "A human-readable description of the argument."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the argument."]
    pub name: ::std::string::String,
    #[doc = "Whether this argument must be provided."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
}
impl From<&PromptArgument> for PromptArgument {
    fn from(value: &PromptArgument) -> Self {
        value.clone()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/prompts/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<PromptListChangedNotificationParams>,
}
impl From<&PromptListChangedNotification> for PromptListChangedNotification {
    fn from(value: &PromptListChangedNotification) -> Self {
        value.clone()
    }
}
#[doc = "PromptListChangedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptListChangedNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&PromptListChangedNotificationParams> for PromptListChangedNotificationParams {
    fn from(value: &PromptListChangedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Describes a message returned as part of a prompt.\n\nThis is similar to `SamplingMessage`, but also supports the embedding of\nresources from the MCP server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes a message returned as part of a prompt.\\n\\nThis is similar to `SamplingMessage`, but also supports the embedding of\\nresources from the MCP server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/EmbeddedResource\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptMessage {
    pub content: PromptMessageContent,
    pub role: Role,
}
impl From<&PromptMessage> for PromptMessage {
    fn from(value: &PromptMessage) -> Self {
        value.clone()
    }
}
#[doc = "PromptMessageContent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/EmbeddedResource\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PromptMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    EmbeddedResource(EmbeddedResource),
}
impl From<&PromptMessageContent> for PromptMessageContent {
    fn from(value: &PromptMessageContent) -> Self {
        value.clone()
    }
}
impl From<TextContent> for PromptMessageContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl From<ImageContent> for PromptMessageContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
impl From<EmbeddedResource> for PromptMessageContent {
    fn from(value: EmbeddedResource) -> Self {
        Self::EmbeddedResource(value)
    }
}
#[doc = "Identifies a prompt."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifies a prompt.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the prompt or prompt template\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ref/prompt\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PromptReference {
    #[doc = "The name of the prompt or prompt template"]
    pub name: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl From<&PromptReference> for PromptReference {
    fn from(value: &PromptReference) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to the server, to read a specific resource URI."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to the server, to read a specific resource URI.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/read\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequest {
    pub method: ::std::string::String,
    pub params: ReadResourceRequestParams,
}
impl From<&ReadResourceRequest> for ReadResourceRequest {
    fn from(value: &ReadResourceRequest) -> Self {
        value.clone()
    }
}
#[doc = "ReadResourceRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceRequestParams {
    #[doc = "The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl From<&ReadResourceRequestParams> for ReadResourceRequestParams {
    fn from(value: &ReadResourceRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "The server's response to a resources/read request from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The server's response to a resources/read request from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contents\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"contents\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReadResourceResult {
    pub contents: ::std::vec::Vec<ReadResourceResultContentsItem>,
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&ReadResourceResult> for ReadResourceResult {
    fn from(value: &ReadResourceResult) -> Self {
        value.clone()
    }
}
#[doc = "ReadResourceResultContentsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextResourceContents\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlobResourceContents\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReadResourceResultContentsItem {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
impl From<&ReadResourceResultContentsItem> for ReadResourceResultContentsItem {
    fn from(value: &ReadResourceResultContentsItem) -> Self {
        value.clone()
    }
}
impl From<TextResourceContents> for ReadResourceResultContentsItem {
    fn from(value: TextResourceContents) -> Self {
        Self::TextResourceContents(value)
    }
}
impl From<BlobResourceContents> for ReadResourceResultContentsItem {
    fn from(value: BlobResourceContents) -> Self {
        Self::BlobResourceContents(value)
    }
}
#[doc = "Request"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"progressToken\": {"]
#[doc = "              \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "              \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Request {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<RequestParams>,
}
impl From<&Request> for Request {
    fn from(value: &Request) -> Self {
        value.clone()
    }
}
#[doc = "A uniquely identifying ID for a request in JSON-RPC."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A uniquely identifying ID for a request in JSON-RPC.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestId {
    String(::std::string::String),
    Integer(i64),
}
impl From<&RequestId> for RequestId {
    fn from(value: &RequestId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for RequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl From<i64> for RequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "RequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"progressToken\": {"]
#[doc = "          \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "          \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestParams {
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub meta: ::std::option::Option<RequestParamsMeta>,
}
impl From<&RequestParams> for RequestParams {
    fn from(value: &RequestParams) -> Self {
        value.clone()
    }
}
#[doc = "RequestParamsMeta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"progressToken\": {"]
#[doc = "      \"description\": \"If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.\","]
#[doc = "      \"$ref\": \"#/definitions/ProgressToken\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RequestParamsMeta {
    #[doc = "If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications."]
    #[serde(
        rename = "progressToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub progress_token: ::std::option::Option<ProgressToken>,
}
impl From<&RequestParamsMeta> for RequestParamsMeta {
    fn from(value: &RequestParamsMeta) -> Self {
        value.clone()
    }
}
#[doc = "A known resource that the server is capable of reading."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A known resource that the server is capable of reading.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of what this resource represents.\\n\\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human-readable name for this resource.\\n\\nThis can be used by clients to populate UI elements.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<ResourceAnnotations>,
    #[doc = "A description of what this resource represents.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "A human-readable name for this resource.\n\nThis can be used by clients to populate UI elements."]
    pub name: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl From<&Resource> for Resource {
    fn from(value: &Resource) -> Self {
        value.clone()
    }
}
#[doc = "ResourceAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&ResourceAnnotations> for ResourceAnnotations {
    fn from(value: &ResourceAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "The contents of a specific resource or sub-resource."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The contents of a specific resource or sub-resource.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceContents {
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl From<&ResourceContents> for ResourceContents {
    fn from(value: &ResourceContents) -> Self {
        value.clone()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/resources/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ResourceListChangedNotificationParams>,
}
impl From<&ResourceListChangedNotification> for ResourceListChangedNotification {
    fn from(value: &ResourceListChangedNotification) -> Self {
        value.clone()
    }
}
#[doc = "ResourceListChangedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceListChangedNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&ResourceListChangedNotificationParams> for ResourceListChangedNotificationParams {
    fn from(value: &ResourceListChangedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "A reference to a resource or resource template definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A reference to a resource or resource template definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ref/resource\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI or URI template of the resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-template\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceReference {
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[doc = "The URI or URI template of the resource."]
    pub uri: ::std::string::String,
}
impl From<&ResourceReference> for ResourceReference {
    fn from(value: &ResourceReference) -> Self {
        value.clone()
    }
}
#[doc = "A template description for resources available on the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A template description for resources available on the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"uriTemplate\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of what this template is for.\\n\\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \\\"hint\\\" to the model.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human-readable name for the type of resource this template refers to.\\n\\nThis can be used by clients to populate UI elements.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uriTemplate\": {"]
#[doc = "      \"description\": \"A URI template (according to RFC 6570) that can be used to construct resource URIs.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri-template\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<ResourceTemplateAnnotations>,
    #[doc = "A description of what this template is for.\n\nThis can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a \"hint\" to the model."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "A human-readable name for the type of resource this template refers to.\n\nThis can be used by clients to populate UI elements."]
    pub name: ::std::string::String,
    #[doc = "A URI template (according to RFC 6570) that can be used to construct resource URIs."]
    #[serde(rename = "uriTemplate")]
    pub uri_template: ::std::string::String,
}
impl From<&ResourceTemplate> for ResourceTemplate {
    fn from(value: &ResourceTemplate) -> Self {
        value.clone()
    }
}
#[doc = "ResourceTemplateAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceTemplateAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&ResourceTemplateAnnotations> for ResourceTemplateAnnotations {
    fn from(value: &ResourceTemplateAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/resources/updated\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotification {
    pub method: ::std::string::String,
    pub params: ResourceUpdatedNotificationParams,
}
impl From<&ResourceUpdatedNotification> for ResourceUpdatedNotification {
    fn from(value: &ResourceUpdatedNotification) -> Self {
        value.clone()
    }
}
#[doc = "ResourceUpdatedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ResourceUpdatedNotificationParams {
    #[doc = "The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to."]
    pub uri: ::std::string::String,
}
impl From<&ResourceUpdatedNotificationParams> for ResourceUpdatedNotificationParams {
    fn from(value: &ResourceUpdatedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Result"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Result {
    #[doc = "This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&Result> for Result {
    fn from(value: &Result) -> Self {
        value.clone()
    }
}
#[doc = "The sender or recipient of messages and data in a conversation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The sender or recipient of messages and data in a conversation.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"assistant\","]
#[doc = "    \"user\""]
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
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
impl From<&Role> for Role {
    fn from(value: &Role) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => write!(f, "assistant"),
            Self::User => write!(f, "user"),
        }
    }
}
impl std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a root directory or file that the server can operate on."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a root directory or file that the server can operate on.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the root. This can be used to provide a human-readable\\nidentifier for the root, which may be useful for display purposes or for\\nreferencing the root in other parts of the application.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI identifying the root. This *must* start with file:// for now.\\nThis restriction may be relaxed in future versions of the protocol to allow\\nother URI schemes.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Root {
    #[doc = "An optional name for the root. This can be used to provide a human-readable\nidentifier for the root, which may be useful for display purposes or for\nreferencing the root in other parts of the application."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The URI identifying the root. This *must* start with file:// for now.\nThis restriction may be relaxed in future versions of the protocol to allow\nother URI schemes."]
    pub uri: ::std::string::String,
}
impl From<&Root> for Root {
    fn from(value: &Root) -> Self {
        value.clone()
    }
}
#[doc = "A notification from the client to the server, informing it that the list of roots has changed.\nThis notification should be sent whenever the client adds, removes, or modifies any root.\nThe server should then request an updated list of roots using the ListRootsRequest."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A notification from the client to the server, informing it that the list of roots has changed.\\nThis notification should be sent whenever the client adds, removes, or modifies any root.\\nThe server should then request an updated list of roots using the ListRootsRequest.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/roots/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RootsListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<RootsListChangedNotificationParams>,
}
impl From<&RootsListChangedNotification> for RootsListChangedNotification {
    fn from(value: &RootsListChangedNotification) -> Self {
        value.clone()
    }
}
#[doc = "RootsListChangedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct RootsListChangedNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&RootsListChangedNotificationParams> for RootsListChangedNotificationParams {
    fn from(value: &RootsListChangedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Describes a message issued to or received from an LLM API."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes a message issued to or received from an LLM API.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"content\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TextContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$ref\": \"#/definitions/Role\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SamplingMessage {
    pub content: SamplingMessageContent,
    pub role: Role,
}
impl From<&SamplingMessage> for SamplingMessage {
    fn from(value: &SamplingMessage) -> Self {
        value.clone()
    }
}
#[doc = "SamplingMessageContent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextContent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ImageContent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SamplingMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
}
impl From<&SamplingMessageContent> for SamplingMessageContent {
    fn from(value: &SamplingMessageContent) -> Self {
        value.clone()
    }
}
impl From<TextContent> for SamplingMessageContent {
    fn from(value: TextContent) -> Self {
        Self::TextContent(value)
    }
}
impl From<ImageContent> for SamplingMessageContent {
    fn from(value: ImageContent) -> Self {
        Self::ImageContent(value)
    }
}
#[doc = "Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"experimental\": {"]
#[doc = "      \"description\": \"Experimental, non-standard capabilities that the server supports.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"logging\": {"]
#[doc = "      \"description\": \"Present if the server supports sending log messages to the client.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    },"]
#[doc = "    \"prompts\": {"]
#[doc = "      \"description\": \"Present if the server offers any prompt templates.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the prompt list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"resources\": {"]
#[doc = "      \"description\": \"Present if the server offers any resources to read.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the resource list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"subscribe\": {"]
#[doc = "          \"description\": \"Whether this server supports subscribing to resource updates.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tools\": {"]
#[doc = "      \"description\": \"Present if the server offers any tools to call.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"listChanged\": {"]
#[doc = "          \"description\": \"Whether this server supports notifications for changes to the tool list.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ServerCapabilities {
    #[doc = "Experimental, non-standard capabilities that the server supports."]
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub experimental: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[doc = "Present if the server supports sending log messages to the client."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub logging: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prompts: ::std::option::Option<ServerCapabilitiesPrompts>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<ServerCapabilitiesResources>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<ServerCapabilitiesTools>,
}
impl From<&ServerCapabilities> for ServerCapabilities {
    fn from(value: &ServerCapabilities) -> Self {
        value.clone()
    }
}
#[doc = "Present if the server offers any prompt templates."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any prompt templates.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the prompt list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ServerCapabilitiesPrompts {
    #[doc = "Whether this server supports notifications for changes to the prompt list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl From<&ServerCapabilitiesPrompts> for ServerCapabilitiesPrompts {
    fn from(value: &ServerCapabilitiesPrompts) -> Self {
        value.clone()
    }
}
#[doc = "Present if the server offers any resources to read."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any resources to read.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the resource list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"subscribe\": {"]
#[doc = "      \"description\": \"Whether this server supports subscribing to resource updates.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ServerCapabilitiesResources {
    #[doc = "Whether this server supports notifications for changes to the resource list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
    #[doc = "Whether this server supports subscribing to resource updates."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subscribe: ::std::option::Option<bool>,
}
impl From<&ServerCapabilitiesResources> for ServerCapabilitiesResources {
    fn from(value: &ServerCapabilitiesResources) -> Self {
        value.clone()
    }
}
#[doc = "Present if the server offers any tools to call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Present if the server offers any tools to call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"listChanged\": {"]
#[doc = "      \"description\": \"Whether this server supports notifications for changes to the tool list.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ServerCapabilitiesTools {
    #[doc = "Whether this server supports notifications for changes to the tool list."]
    #[serde(
        rename = "listChanged",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub list_changed: ::std::option::Option<bool>,
}
impl From<&ServerCapabilitiesTools> for ServerCapabilitiesTools {
    fn from(value: &ServerCapabilitiesTools) -> Self {
        value.clone()
    }
}
#[doc = "ServerNotification"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelledNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ProgressNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ResourceUpdatedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PromptListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ToolListChangedNotification\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/LoggingMessageNotification\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerNotification {
    CancelledNotification(CancelledNotification),
    ProgressNotification(ProgressNotification),
    ResourceListChangedNotification(ResourceListChangedNotification),
    ResourceUpdatedNotification(ResourceUpdatedNotification),
    PromptListChangedNotification(PromptListChangedNotification),
    ToolListChangedNotification(ToolListChangedNotification),
    LoggingMessageNotification(LoggingMessageNotification),
}
impl From<&ServerNotification> for ServerNotification {
    fn from(value: &ServerNotification) -> Self {
        value.clone()
    }
}
impl From<CancelledNotification> for ServerNotification {
    fn from(value: CancelledNotification) -> Self {
        Self::CancelledNotification(value)
    }
}
impl From<ProgressNotification> for ServerNotification {
    fn from(value: ProgressNotification) -> Self {
        Self::ProgressNotification(value)
    }
}
impl From<ResourceListChangedNotification> for ServerNotification {
    fn from(value: ResourceListChangedNotification) -> Self {
        Self::ResourceListChangedNotification(value)
    }
}
impl From<ResourceUpdatedNotification> for ServerNotification {
    fn from(value: ResourceUpdatedNotification) -> Self {
        Self::ResourceUpdatedNotification(value)
    }
}
impl From<PromptListChangedNotification> for ServerNotification {
    fn from(value: PromptListChangedNotification) -> Self {
        Self::PromptListChangedNotification(value)
    }
}
impl From<ToolListChangedNotification> for ServerNotification {
    fn from(value: ToolListChangedNotification) -> Self {
        Self::ToolListChangedNotification(value)
    }
}
impl From<LoggingMessageNotification> for ServerNotification {
    fn from(value: LoggingMessageNotification) -> Self {
        Self::LoggingMessageNotification(value)
    }
}
#[doc = "ServerRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PingRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CreateMessageRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListRootsRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerRequest {
    PingRequest(PingRequest),
    CreateMessageRequest(CreateMessageRequest),
    ListRootsRequest(ListRootsRequest),
}
impl From<&ServerRequest> for ServerRequest {
    fn from(value: &ServerRequest) -> Self {
        value.clone()
    }
}
impl From<PingRequest> for ServerRequest {
    fn from(value: PingRequest) -> Self {
        Self::PingRequest(value)
    }
}
impl From<CreateMessageRequest> for ServerRequest {
    fn from(value: CreateMessageRequest) -> Self {
        Self::CreateMessageRequest(value)
    }
}
impl From<ListRootsRequest> for ServerRequest {
    fn from(value: ListRootsRequest) -> Self {
        Self::ListRootsRequest(value)
    }
}
#[doc = "ServerResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Result\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InitializeResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListResourcesResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ReadResourceResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListPromptsResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetPromptResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListToolsResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CallToolResult\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CompleteResult\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerResult {
    Result(Result),
    InitializeResult(InitializeResult),
    ListResourcesResult(ListResourcesResult),
    ReadResourceResult(ReadResourceResult),
    ListPromptsResult(ListPromptsResult),
    GetPromptResult(GetPromptResult),
    ListToolsResult(ListToolsResult),
    CallToolResult(CallToolResult),
    CompleteResult(CompleteResult),
}
impl From<&ServerResult> for ServerResult {
    fn from(value: &ServerResult) -> Self {
        value.clone()
    }
}
impl From<Result> for ServerResult {
    fn from(value: Result) -> Self {
        Self::Result(value)
    }
}
impl From<InitializeResult> for ServerResult {
    fn from(value: InitializeResult) -> Self {
        Self::InitializeResult(value)
    }
}
impl From<ListResourcesResult> for ServerResult {
    fn from(value: ListResourcesResult) -> Self {
        Self::ListResourcesResult(value)
    }
}
impl From<ReadResourceResult> for ServerResult {
    fn from(value: ReadResourceResult) -> Self {
        Self::ReadResourceResult(value)
    }
}
impl From<ListPromptsResult> for ServerResult {
    fn from(value: ListPromptsResult) -> Self {
        Self::ListPromptsResult(value)
    }
}
impl From<GetPromptResult> for ServerResult {
    fn from(value: GetPromptResult) -> Self {
        Self::GetPromptResult(value)
    }
}
impl From<ListToolsResult> for ServerResult {
    fn from(value: ListToolsResult) -> Self {
        Self::ListToolsResult(value)
    }
}
impl From<CallToolResult> for ServerResult {
    fn from(value: CallToolResult) -> Self {
        Self::CallToolResult(value)
    }
}
impl From<CompleteResult> for ServerResult {
    fn from(value: CompleteResult) -> Self {
        Self::CompleteResult(value)
    }
}
#[doc = "A request from the client to the server, to enable or adjust logging."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A request from the client to the server, to enable or adjust logging.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"logging/setLevel\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"level\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"level\": {"]
#[doc = "          \"description\": \"The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/logging/message.\","]
#[doc = "          \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetLevelRequest {
    pub method: ::std::string::String,
    pub params: SetLevelRequestParams,
}
impl From<&SetLevelRequest> for SetLevelRequest {
    fn from(value: &SetLevelRequest) -> Self {
        value.clone()
    }
}
#[doc = "SetLevelRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"level\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/logging/message.\","]
#[doc = "      \"$ref\": \"#/definitions/LoggingLevel\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetLevelRequestParams {
    #[doc = "The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/logging/message."]
    pub level: LoggingLevel,
}
impl From<&SetLevelRequestParams> for SetLevelRequestParams {
    fn from(value: &SetLevelRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request resources/updated notifications from the server whenever a particular resource changes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/subscribe\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeRequest {
    pub method: ::std::string::String,
    pub params: SubscribeRequestParams,
}
impl From<&SubscribeRequest> for SubscribeRequest {
    fn from(value: &SubscribeRequest) -> Self {
        value.clone()
    }
}
#[doc = "SubscribeRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SubscribeRequestParams {
    #[doc = "The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it."]
    pub uri: ::std::string::String,
}
impl From<&SubscribeRequestParams> for SubscribeRequestParams {
    fn from(value: &SubscribeRequestParams) -> Self {
        value.clone()
    }
}
#[doc = "Text provided to or from an LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Text provided to or from an LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"annotations\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"audience\": {"]
#[doc = "          \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Role\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"priority\": {"]
#[doc = "          \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text content of the message.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"text\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextContent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations: ::std::option::Option<TextContentAnnotations>,
    #[doc = "The text content of the message."]
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl From<&TextContent> for TextContent {
    fn from(value: &TextContent) -> Self {
        value.clone()
    }
}
#[doc = "TextContentAnnotations"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"audience\": {"]
#[doc = "      \"description\": \"Describes who the intended customer of this object or data is.\\n\\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\\\"user\\\", \\\"assistant\\\"]`).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Role\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"priority\": {"]
#[doc = "      \"description\": \"Describes how important this data is for operating the server.\\n\\nA value of 1 means \\\"most important,\\\" and indicates that the data is\\neffectively required, while 0 means \\\"least important,\\\" and indicates that\\nthe data is entirely optional.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextContentAnnotations {
    #[doc = "Describes who the intended customer of this object or data is.\n\nIt can include multiple entries to indicate content useful for multiple audiences (e.g., `[\"user\", \"assistant\"]`)."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub audience: ::std::vec::Vec<Role>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub priority: ::std::option::Option<f64>,
}
impl From<&TextContentAnnotations> for TextContentAnnotations {
    fn from(value: &TextContentAnnotations) -> Self {
        value.clone()
    }
}
#[doc = "TextResourceContents"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of this resource, if known.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The text of the item. This must only be set if the item can actually be represented as text (not binary data).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of this resource.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextResourceContents {
    #[doc = "The MIME type of this resource, if known."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "The text of the item. This must only be set if the item can actually be represented as text (not binary data)."]
    pub text: ::std::string::String,
    #[doc = "The URI of this resource."]
    pub uri: ::std::string::String,
}
impl From<&TextResourceContents> for TextResourceContents {
    fn from(value: &TextResourceContents) -> Self {
        value.clone()
    }
}
#[doc = "Definition for a tool the client can call."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Definition for a tool the client can call.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"inputSchema\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the tool.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"inputSchema\": {"]
#[doc = "      \"description\": \"A JSON Schema object defining the expected parameters for the tool.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"properties\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": true"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"object\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the tool.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Tool {
    #[doc = "A human-readable description of the tool."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    #[doc = "The name of the tool."]
    pub name: ::std::string::String,
}
impl From<&Tool> for Tool {
    fn from(value: &Tool) -> Self {
        value.clone()
    }
}
#[doc = "A JSON Schema object defining the expected parameters for the tool."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A JSON Schema object defining the expected parameters for the tool.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"properties\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolInputSchema {
    #[serde(default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
    pub properties: ::std::collections::HashMap<
        ::std::string::String,
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl From<&ToolInputSchema> for ToolInputSchema {
    fn from(value: &ToolInputSchema) -> Self {
        value.clone()
    }
}
#[doc = "An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"notifications/tools/list_changed\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_meta\": {"]
#[doc = "          \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {}"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolListChangedNotification {
    pub method: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<ToolListChangedNotificationParams>,
}
impl From<&ToolListChangedNotification> for ToolListChangedNotification {
    fn from(value: &ToolListChangedNotification) -> Self {
        value.clone()
    }
}
#[doc = "ToolListChangedNotificationParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_meta\": {"]
#[doc = "      \"description\": \"This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {}"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ToolListChangedNotificationParams {
    #[doc = "This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications."]
    #[serde(
        rename = "_meta",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub meta: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl From<&ToolListChangedNotificationParams> for ToolListChangedNotificationParams {
    fn from(value: &ToolListChangedNotificationParams) -> Self {
        value.clone()
    }
}
#[doc = "Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"method\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"resources/unsubscribe\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"uri\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"uri\": {"]
#[doc = "          \"description\": \"The URI of the resource to unsubscribe from.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeRequest {
    pub method: ::std::string::String,
    pub params: UnsubscribeRequestParams,
}
impl From<&UnsubscribeRequest> for UnsubscribeRequest {
    fn from(value: &UnsubscribeRequest) -> Self {
        value.clone()
    }
}
#[doc = "UnsubscribeRequestParams"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The URI of the resource to unsubscribe from.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsubscribeRequestParams {
    #[doc = "The URI of the resource to unsubscribe from."]
    pub uri: ::std::string::String,
}
impl From<&UnsubscribeRequestParams> for UnsubscribeRequestParams {
    fn from(value: &UnsubscribeRequestParams) -> Self {
        value.clone()
    }
}
fn main() {}
