use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

use crate::ast::{JsonArray, JsonField, JsonObject, JsonRoot, JsonValueNode};
use oak_core::OakError;

/// JSON 反序列化实现
pub fn deserialize<'de, D>(deserializer: D) -> Result<JsonValueNode, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(JsonDeserializer {})
}

struct JsonDeserializer;

impl<'de> Visitor<'de> for JsonDeserializer {
    type Value = JsonValueNode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid JSON value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Boolean(crate::ast::JsonBoolean { span: (0..0).into(), value: v }))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v }))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: v }))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: String::from_utf8_lossy(v).to_string() }))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: String::from_utf8_lossy(&v).to_string() }))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(JsonValueNode::Null(crate::ast::JsonNull { span: (0..0).into() }))
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
        Ok(JsonValueNode::Null(crate::ast::JsonNull { span: (0..0).into() }))
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
        Ok(JsonValueNode::Array(JsonArray { span: (0..0).into(), elements }))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut fields = Vec::new();
        while let Some((key, value)) = map.next_entry()? {
            let json_string = crate::ast::JsonString { span: (0..0).into(), value: key };
            fields.push(JsonField { span: (0..0).into(), name: json_string, value });
        }
        Ok(JsonValueNode::Object(JsonObject { fields, span: (0..0).into() }))
    }

    fn visit_enum<A>(self, _variant: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        Err(de::Error::custom("enums are not supported"))
    }
}

/// Deserializes a JSON string into a Rust type.
pub fn from_str<'de, T>(json: &'de str) -> Result<T, OakError>
where
    T: serde::Deserialize<'de>,
{
    // 直接使用 serde_json 来反序列化 JSON 字符串
    serde_json::from_str(json).map_err(|e| OakError::custom_error(format!("{:?}", e)))
}

struct JsonValueDeserializer {
    value: JsonValueNode,
}

impl JsonValueDeserializer {
    fn new(value: JsonValueNode) -> Self {
        Self { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for JsonValueDeserializer {
    type Error = OakError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            JsonValueNode::Null(_) => visitor.visit_none(),
            JsonValueNode::Boolean(b) => visitor.visit_bool(b.value),
            JsonValueNode::Number(n) => visitor.visit_f64(n.value),
            JsonValueNode::String(s) => visitor.visit_str(&s.value),
            JsonValueNode::Array(seq) => visitor.visit_seq(JsonArrayAccess::new(seq.elements)),
            JsonValueNode::Object(map) => visitor.visit_map(JsonMapAccess::new(map.fields)),
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
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
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

struct JsonArrayAccess {
    elements: Vec<JsonValueNode>,
    index: usize,
}

impl JsonArrayAccess {
    fn new(elements: Vec<JsonValueNode>) -> Self {
        Self { elements, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for JsonArrayAccess {
    type Error = OakError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.elements.len() {
            let value = self.elements[self.index].clone();
            self.index += 1;
            seed.deserialize(JsonValueDeserializer::new(value)).map(Some)
        }
        else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.elements.len() - self.index)
    }
}

struct JsonMapAccess {
    fields: Vec<JsonField>,
    index: usize,
}

impl JsonMapAccess {
    fn new(fields: Vec<JsonField>) -> Self {
        Self { fields, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for JsonMapAccess {
    type Error = OakError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.fields.len() {
            let key = self.fields[self.index].name.value.clone();
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
        seed.deserialize(JsonValueDeserializer::new(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.fields.len() - self.index)
    }
}
