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
