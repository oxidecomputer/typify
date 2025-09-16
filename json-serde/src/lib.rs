use serde::{Deserialize, Deserializer};

/// A deserializer for `Option<T>` that always produces `Some(T)` if the value
/// is present. It is intended to be used in concert #[]
pub fn deserialize_some<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {}
