use crate::{
    ast::{DsvField, DsvRecord, DsvRoot},
    language::DsvLanguage,
};
use oak_core::OakError as Error;

/// Converts a `DsvRoot` to a deserializable value of type `T`.
pub fn from_value<const LANG: DsvLanguage, T: serde::de::DeserializeOwned>(value: DsvRoot<LANG>) -> Result<T, Error> {
    T::deserialize(value)
}

impl<'de, const LANG: DsvLanguage> serde::de::Deserializer<'de> for DsvRoot<LANG> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(self.records))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.records.is_empty() { visitor.visit_none() } else { visitor.visit_some(self) }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(self.records))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct enum tuple
        tuple_struct map identifier ignored_any struct
    }
}

struct SeqDeserializer<const LANG: DsvLanguage> {
    records: std::vec::IntoIter<DsvRecord<LANG>>,
}

impl<const LANG: DsvLanguage> SeqDeserializer<LANG> {
    fn new(records: Vec<DsvRecord<LANG>>) -> Self {
        Self { records: records.into_iter() }
    }
}

impl<'de, const LANG: DsvLanguage> serde::de::SeqAccess<'de> for SeqDeserializer<LANG> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.records.next() {
            Some(record) => seed.deserialize(record).map(Some),
            None => Ok(None),
        }
    }
}

impl<'de, const LANG: DsvLanguage> serde::de::Deserializer<'de> for DsvRecord<LANG> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(RecordDeserializer::new(self))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct enum tuple
        tuple_struct map identifier ignored_any option seq struct
    }
}

struct RecordDeserializer<const LANG: DsvLanguage> {
    fields: std::vec::IntoIter<DsvField<LANG>>,
}

impl<const LANG: DsvLanguage> RecordDeserializer<LANG> {
    fn new(record: DsvRecord<LANG>) -> Self {
        Self { fields: record.fields.into_iter() }
    }
}

impl<'de, const LANG: DsvLanguage> serde::de::SeqAccess<'de> for RecordDeserializer<LANG> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.fields.next() {
            Some(field) => seed.deserialize(field).map(Some),
            None => Ok(None),
        }
    }
}

impl<'de, const LANG: DsvLanguage> serde::de::Deserializer<'de> for DsvField<LANG> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value.to_lowercase().as_str() {
            "true" | "yes" | "1" => visitor.visit_bool(true),
            "false" | "no" | "0" => visitor.visit_bool(false),
            _ => Err(<Error as serde::de::Error>::custom(format!("invalid boolean value: {}", self.value))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<i8>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_i8(val)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<i16>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_i16(val)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<i32>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_i32(val)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<i64>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_i64(val)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<u8>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_u8(val)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<u16>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_u16(val)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<u32>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_u32(val)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<u64>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_u64(val)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<f32>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_f32(val)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let val = self.value.parse::<f64>().map_err(|e| <Error as serde::de::Error>::custom(e))?;
        visitor.visit_f64(val)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.value.len() == 1 { visitor.visit_char(self.value.chars().next().unwrap()) } else { Err(<Error as serde::de::Error>::custom(format!("invalid char value: {}", self.value))) }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_string(self.value)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bytes(self.value.as_bytes())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.value.into_bytes())
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.value.is_empty() { visitor.visit_none() } else { visitor.visit_some(self) }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(serde::de::IntoDeserializer::into_deserializer(self.value))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
