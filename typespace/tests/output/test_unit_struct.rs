#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct MyUnitStruct;
impl ::serde::Serialize for MyUnitStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let value: ::serde_json::Value = ::serde_json::from_str("\"<<+>>\"").unwrap();
        value.serialize(serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MyUnitStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let expected: ::serde_json::Value = ::serde_json::from_str("\"<<+>>\"").unwrap();
        let value: ::serde_json::Value = ::serde::Deserialize::deserialize(
            deserializer,
        )?;
        if value != expected {
            return Err(
                ::serde::de::Error::custom(
                    format!(
                        "expected {}, found {}", "\"<<+>>\"", ::serde_json::to_string(&
                        value).unwrap()
                    ),
                ),
            );
        }
        Ok(MyUnitStruct)
    }
}
