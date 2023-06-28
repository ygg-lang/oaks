use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

use crate::ast::{TomlArray, TomlInlineTable, TomlKeyValue, TomlRoot, TomlValueNode};
use oak_core::OakError;

/// TOML 反序列化实现
pub fn deserialize<'de, D>(deserializer: D) -> Result<TomlValueNode, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(TomlDeserializer {})
}

struct TomlDeserializer;

impl<'de> Visitor<'de> for TomlDeserializer {
    type Value = TomlValueNode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid TOML value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Boolean(crate::ast::TomlBoolean { span: (0..0).into(), value: v }))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Integer(crate::ast::TomlInteger { span: (0..0).into(), value: v as i64, format: crate::ast::IntegerFormat::Decimal }))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Float(crate::ast::TomlFloat { span: (0..0).into(), value: v as f64 }))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::Float(crate::ast::TomlFloat { span: (0..0).into(), value: v }))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: v.to_string(), is_multiline: false, is_literal: false }))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: v.to_string(), is_multiline: false, is_literal: false }))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: v, is_multiline: false, is_literal: false }))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: String::from_utf8_lossy(v).to_string(), is_multiline: false, is_literal: false }))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: String::from_utf8_lossy(&v).to_string(), is_multiline: false, is_literal: false }))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // TOML doesn't have a null type, so we'll use a string "null"
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: "null".to_string(), is_multiline: false, is_literal: false }))
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
        // TOML doesn't have a unit type, so we'll use a string "null"
        Ok(TomlValueNode::String(crate::ast::TomlString { span: (0..0).into(), value: "null".to_string(), is_multiline: false, is_literal: false }))
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
        let mut items = Vec::new();
        while let Some(elem) = seq.next_element()? {
            items.push(elem);
        }
        Ok(TomlValueNode::Array(TomlArray { span: (0..0).into(), items }))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some((key, value)) = map.next_entry()? {
            // Convert string key to TomlKey
            let toml_key = crate::ast::TomlKey { span: (0..0).into(), segments: vec![crate::ast::TomlKeySegment::Bare(crate::ast::TomlBareKey { span: (0..0).into(), name: key })] };
            entries.push(TomlKeyValue { span: (0..0).into(), key: toml_key, value });
        }
        Ok(TomlValueNode::InlineTable(TomlInlineTable { span: (0..0).into(), items: entries }))
    }

    fn visit_enum<A>(self, _variant: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        Err(de::Error::custom("enums are not supported"))
    }
}

/// Deserializes a TOML string into a Rust type.
pub fn from_str<'de, T>(toml: &'de str) -> Result<T, OakError>
where
    T: serde::Deserialize<'de>,
{
    // 首先使用 oak-toml 解析器解析 TOML 字符串
    let root = crate::parse(toml)?;
    
    // 将解析结果转换为 TomlValueNode
    let toml_value = root.into_value();
    
    // 使用自定义反序列化器将 TomlValueNode 转换为目标类型
    let deserializer = TomlValueDeserializer::new(toml_value);
    T::deserialize(deserializer)
}

struct TomlValueDeserializer {
    value: TomlValueNode,
}

impl TomlValueDeserializer {
    fn new(value: TomlValueNode) -> Self {
        Self { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for TomlValueDeserializer {
    type Error = OakError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            TomlValueNode::String(s) => visitor.visit_str(&s.value),
            TomlValueNode::Integer(i) => visitor.visit_i64(i.value),
            TomlValueNode::Float(f) => visitor.visit_f64(f.value),
            TomlValueNode::Boolean(b) => visitor.visit_bool(b.value),
            TomlValueNode::DateTime(dt) => visitor.visit_str(&dt.value),
            TomlValueNode::Array(seq) => visitor.visit_seq(TomlArrayAccess::new(seq.items)),
            TomlValueNode::InlineTable(map) => visitor.visit_map(TomlMapAccess::new(map.items)),
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

struct TomlArrayAccess {
    items: Vec<TomlValueNode>,
    index: usize,
}

impl TomlArrayAccess {
    fn new(items: Vec<TomlValueNode>) -> Self {
        Self { items, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for TomlArrayAccess {
    type Error = OakError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.items.len() {
            let value = self.items[self.index].clone();
            self.index += 1;
            seed.deserialize(TomlValueDeserializer::new(value)).map(Some)
        }
        else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.items.len() - self.index)
    }
}

struct TomlMapAccess {
    entries: Vec<TomlKeyValue>,
    index: usize,
}

impl TomlMapAccess {
    fn new(entries: Vec<TomlKeyValue>) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for TomlMapAccess {
    type Error = OakError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.entries.len() {
            let key = self.entries[self.index].key.to_string();
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
        let value = self.entries[self.index].value.clone();
        self.index += 1;
        seed.deserialize(TomlValueDeserializer::new(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.entries.len() - self.index)
    }
}
