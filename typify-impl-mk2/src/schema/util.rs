//! This is intended to provide better error messages during deserialization
//! for this particular untagged enum where we know the type (i.e. bool v. map)
//! is distinguishing.

use std::marker::PhantomData;

use serde::{de::Visitor, forward_to_deserialize_any, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ObjectOrBool<T> {
    Bool(bool),
    Object(Box<T>),
}

impl<'de, T> Deserialize<'de> for ObjectOrBool<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ObjectOrBoolVisitor::<T>::new())
    }
}

struct ObjectOrBoolVisitor<T>(PhantomData<T>);
impl<T> ObjectOrBoolVisitor<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}
impl<'de, T> Visitor<'de> for ObjectOrBoolVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = ObjectOrBool<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "object or bool")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ObjectOrBool::Bool(v))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let xxx = MapWrapperDeserializer(map);

        let yyy = T::deserialize(xxx)?;

        Ok(ObjectOrBool::Object(Box::new(yyy)))
    }
}

struct MapWrapperDeserializer<A>(A);
impl<'de, A> Deserializer<'de> for MapWrapperDeserializer<A>
where
    A: serde::de::MapAccess<'de>,
{
    type Error = A::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct enum identifier ignored_any
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self.0)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self.0)
    }
}
