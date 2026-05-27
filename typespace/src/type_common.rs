use crate::TypespaceTraitSet;

/// Common properties of named, generated types.
#[derive(Debug, Clone)]
pub struct TypeCommon {
    pub name: String,
    pub description: Option<String>,
    pub default: Option<JsonValue>,
    pub traits: TypespaceTraitSet,
}

impl TypeCommon {
    pub fn new(name: impl Into<String>, description: Option<String>) -> Self {
        Self {
            name: name.into(),
            description,
            default: None,
            traits: TypespaceTraitSet::empty(),
        }
    }
}

/// A wrapper around `serde_json::Value` that implements `Ord` (trivially).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValue(pub serde_json::Value);

impl JsonValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl Ord for JsonValue {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for JsonValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
