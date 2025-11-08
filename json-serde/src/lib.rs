use serde_core::{
    Deserialize, Deserializer, Serializer,
    de::Error,
    ser::{Impossible, SerializeSeq},
};

/// A deserializer function for `Option<T>` that always produces `Some(T)` if
/// the value is present.
///
/// It is useful when one wants to distinguish between a field that's absent
/// and a field that's present with a `null` value. For example, the anntation
/// below may be used for a field that may be absent, but may not be `null``.
///
/// ```
/// # #[derive(serde::Deserialize, serde::Serialize)]
/// # struct Foo {
///     #[serde(
///         default,
///         deserialize_with = "::json_serde::deserialize_some",
///         skip_serializing_if = "Option::is_none",
///     )]
///     field: Option<String>,
/// # }
/// ```
///
/// It can also be used to to use a "double-Option" to be able to determine
/// if a field was absent, null, or had a value:
/// ```
/// # #[derive(serde::Deserialize, serde::Serialize)]
/// # struct Foo {
///     #[serde(
///         default,
///         deserialize_with = "::json_serde::deserialize_some",
///         skip_serializing_if = "Option::is_none",
///     )]
///     field: Option<Option<String>>,
/// # }
/// ```
///
/// In the first case, a `null` value results in an error because a `String`
/// cannot be deserialized from `null`. In the second case, a `null` value
/// results in `field` having a value of `Some(None)` since `Option<String>`
/// *can* be deserialized from `null`.
pub fn deserialize_some<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

/// A Serializer used to flatten or embed sequences among other elements of a
/// sequence.
pub struct FlattenedSequenceSerializer<'a, S>(&'a mut S);

impl<'a, S> FlattenedSequenceSerializer<'a, S>
where
    S: serde_core::ser::SerializeSeq,
{
    pub fn new(seq_serializer: &'a mut S) -> Self {
        Self(seq_serializer)
    }

    fn wrong_type_error<T>() -> Result<T, S::Error> {
        Err(serde_core::ser::Error::custom(format!("xxx")))
    }
}

impl<'a, S> Serializer for FlattenedSequenceSerializer<'a, S>
where
    S: serde_core::ser::SerializeSeq,
{
    type Ok = ();
    type Error = S::Error;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = serde_core::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = serde_core::ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = serde_core::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(serde_core::ser::Error::custom(
            "FlatArraySerializer does not support maps",
        ))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(serde_core::ser::Error::custom(
            "FlatArraySerializer does not support structs",
        ))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(serde_core::ser::Error::custom(
            "FlatArraySerializer does not support struct variants",
        ))
    }

    fn serialize_bool(self, __v: bool) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Self::wrong_type_error()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        Self::wrong_type_error()
    }
}

impl<'a, S> SerializeSeq for FlattenedSequenceSerializer<'a, S>
where
    S: serde_core::ser::SerializeSeq,
{
    type Ok = ();

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde_core::Serialize,
    {
        self.0.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// A Deserializer used to extract flattened sequences from the end of a
/// sequence into a type that can be serialized as a sequence.
pub struct FlattenedSequenceDeserializer<'a, S>(&'a mut S);

impl<'a, S> FlattenedSequenceDeserializer<'a, S> {
    pub fn new(seq_access: &'a mut S) -> Self {
        Self(seq_access)
    }
}

impl<'de, 'a, S> Deserializer<'de> for FlattenedSequenceDeserializer<'a, S>
where
    S: serde_core::de::SeqAccess<'de>,
{
    type Error = S::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, S::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        Err(S::Error::custom("type must expect a sequence"))
    }

    serde_core::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple
        tuple_struct map struct enum identifier ignored_any
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde_core::de::Visitor<'de>,
    {
        visitor.visit_seq(self.0)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize, ser::SerializeSeq};

    use crate::{FlattenedSequenceDeserializer, FlattenedSequenceSerializer};

    #[test]
    fn flatten_tuple_vec() {
        #[derive(Debug, Eq, PartialEq)]
        struct TestType(u32, String, Vec<u32>);

        impl Serialize for TestType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(None)?;

                seq.serialize_element(&self.0)?;
                seq.serialize_element(&self.1)?;

                self.2
                    .serialize(FlattenedSequenceSerializer::new(&mut seq))?;

                seq.end()
            }
        }

        impl<'de> Deserialize<'de> for TestType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = TestType;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a flattened tuple vec")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let v_0 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("expected first element"))?;
                        let v_1 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::custom("expected second element"))?;

                        let rest =
                            Deserialize::deserialize(FlattenedSequenceDeserializer::new(&mut seq))?;

                        Ok(TestType(v_0, v_1, rest))
                    }
                }
                deserializer.deserialize_seq(Visitor)
            }
        }

        let value = TestType(42, "Hello".to_string(), vec![1, 2, 3]);
        let serialized = serde_json::to_string(&value).unwrap();

        assert_eq!(serialized, "[42,\"Hello\",1,2,3]");

        let de_value = serde_json::from_str::<TestType>(&serialized).unwrap();

        assert_eq!(value, de_value);

        let value = TestType(7, "World".to_string(), vec![]);
        let serialized = serde_json::to_string(&value).unwrap();

        assert_eq!(serialized, "[7,\"World\"]");

        let de_value = serde_json::from_str::<TestType>(&serialized).unwrap();

        assert_eq!(value, de_value);

        let input = "[1, \"Two\", \"Three\", 4, 5, 6]";
        let de_result = serde_json::from_str::<TestType>(&input);
        assert!(de_result.is_err());
    }
}
