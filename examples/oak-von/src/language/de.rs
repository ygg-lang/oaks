use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

use crate::language::value::{from_ast, VonValue};
use oak_core::OakError;

/// VON 反序列化实现
pub fn deserialize<'de, D>(deserializer: D) -> Result<VonValue, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(VonDeserializer {})
}

struct VonDeserializer;

impl<'de> Visitor<'de> for VonDeserializer {
    type Value = VonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid VON value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Boolean(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Number(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::String(v.to_string()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::String(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::String(String::from_utf8_lossy(v).to_string()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::String(String::from_utf8_lossy(&v).to_string()))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VonValue::Null)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut elements = Vec::new();
        while let Some(elem) = seq.next_element()? {
            elements.push(elem);
        }
        Ok(VonValue::Array(crate::language::value::VonArray { elements }))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut fields = Vec::new();
        while let Some((key, value)) = map.next_entry()? {
            fields.push(crate::language::value::VonField { 
                name: key, 
                value 
            });
        }
        Ok(VonValue::Object(crate::language::value::VonObject { fields }))
    }

    fn visit_enum<A>(self, _variant: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        Err(de::Error::custom("enums are not supported"))
    }
}

/// Deserializes a VON string into a Rust type.
pub fn from_str<'de, T>(von: &'de str) -> Result<T, OakError>
where
    T: serde::Deserialize<'de>,
{
    // 首先解析 VON 字符串到 AST VonValue
    let ast_value = crate::parse(von).map_err(|e| OakError::custom_error(e))?;
    // 然后转换为纯值 VonValue
    let von_value = from_ast(&ast_value);
    // 最后反序列化到 Rust 类型
    T::deserialize(VonValueDeserializer::new(von_value))
}

struct VonValueDeserializer {
    value: VonValue,
}

impl VonValueDeserializer {
    fn new(value: VonValue) -> Self {
        Self { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for VonValueDeserializer {
    type Error = OakError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Null => visitor.visit_none(),
            VonValue::Boolean(b) => visitor.visit_bool(b),
            VonValue::Number(n) => visitor.visit_f64(n),
            VonValue::String(s) => visitor.visit_str(&s),
            VonValue::Array(seq) => visitor.visit_seq(VonArrayAccess::new(seq.elements)),
            VonValue::Tuple(tuple) => visitor.visit_seq(VonArrayAccess::new(tuple.elements)),
            VonValue::Object(map) => visitor.visit_map(VonMapAccess::new(map.fields)),
            VonValue::Enum(en) => {
                if let Some(payload) = en.payload {
                    // For Option types, if the variant is "Some", return Some(payload)
                    if en.variant == "Some" {
                        visitor.visit_some(VonValueDeserializer::new(*payload))
                    } else {
                        visitor.visit_str(&en.variant)
                    }
                } else {
                    // For Option types, if the variant is "None", return None
                    if en.variant == "None" {
                        visitor.visit_none()
                    } else {
                        visitor.visit_str(&en.variant)
                    }
                }
            }
            VonValue::Undefined => visitor.visit_none(),
            VonValue::Inf => visitor.visit_f64(f64::INFINITY),
            VonValue::Nan => visitor.visit_f64(f64::NAN),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_u8(n as u8),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_u16(n as u16),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_u32(n as u32),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_u64(n as u64),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_f32(n as f32),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            VonValue::Number(n) => visitor.visit_f64(n),
            _ => self.deserialize_any(visitor)
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(OakError::custom_error("Enums are not supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct VonArrayAccess {
    elements: Vec<VonValue>,
    index: usize,
}

impl VonArrayAccess {
    fn new(elements: Vec<VonValue>) -> Self {
        Self { elements, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for VonArrayAccess {
    type Error = OakError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.elements.len() {
            let value = self.elements[self.index].clone();
            self.index += 1;
            seed.deserialize(VonValueDeserializer::new(value)).map(Some)
        }
        else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.elements.len() - self.index)
    }
}

struct VonMapAccess {
    fields: Vec<crate::language::value::VonField>,
    index: usize,
}

impl VonMapAccess {
    fn new(fields: Vec<crate::language::value::VonField>) -> Self {
        Self { fields, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for VonMapAccess {
    type Error = OakError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.fields.len() {
            let key = self.fields[self.index].name.clone();
            seed.deserialize(serde::de::value::StrDeserializer::new(&key)).map(Some)
        }
        else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = self.fields[self.index].value.clone();
        self.index += 1;
        seed.deserialize(VonValueDeserializer::new(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.fields.len() - self.index)
    }
}
