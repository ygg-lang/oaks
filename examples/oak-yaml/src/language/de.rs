use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

use crate::ast::{YamlMapping, YamlMappingEntry, YamlRoot, YamlScalar, YamlSequence, YamlValueNode};
use oak_core::OakError;

/// YAML 反序列化实现
pub fn deserialize<'de, D>(deserializer: D) -> Result<YamlValueNode, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(YamlDeserializer {})
}

struct YamlDeserializer;

impl<'de> Visitor<'de> for YamlDeserializer {
    type Value = YamlValueNode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid YAML value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: v }))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: String::from_utf8_lossy(v).to_string() }))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: String::from_utf8_lossy(&v).to_string() }))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: "null".to_string() }))
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
        Ok(YamlValueNode::Scalar(YamlScalar { span: (0..0).into(), value: "null".to_string() }))
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
        Ok(YamlValueNode::Sequence(YamlSequence { span: (0..0).into(), items }))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut entries = Vec::new();
        while let Some((key, value)) = map.next_entry()? {
            entries.push(YamlMappingEntry { span: (0..0).into(), key, value });
        }
        Ok(YamlValueNode::Mapping(YamlMapping { span: (0..0).into(), entries }))
    }

    fn visit_enum<A>(self, _variant: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        Err(de::Error::custom("enums are not supported"))
    }
}

/// Deserializes a YAML string into a Rust type.
pub fn from_str<'de, T>(yaml: &'de str) -> Result<T, OakError>
where
    T: serde::Deserialize<'de>,
{
    // 由于 oak-yaml 的 parser 还在开发中，这里使用一个简单的实现来处理基本类型和简单结构
    // 对于复杂类型，我们将在 parser 完成后更新此实现
    
    // 移除首尾空白
    let yaml = yaml.trim();
    
    // 处理 null
    if yaml == "null" || yaml == "Null" || yaml == "NULL" || yaml == "~" {
        let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: "null".to_string() });
        let deserializer = YamlValueDeserializer::new(value);
        return T::deserialize(deserializer);
    }
    
    // 处理布尔值
    if yaml == "true" || yaml == "True" || yaml == "TRUE" {
        let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: "true".to_string() });
        let deserializer = YamlValueDeserializer::new(value);
        return T::deserialize(deserializer);
    }
    if yaml == "false" || yaml == "False" || yaml == "FALSE" {
        let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: "false".to_string() });
        let deserializer = YamlValueDeserializer::new(value);
        return T::deserialize(deserializer);
    }
    
    // 处理数字
    if yaml.parse::<i64>().is_ok() || yaml.parse::<f64>().is_ok() {
        let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: yaml.to_string() });
        let deserializer = YamlValueDeserializer::new(value);
        return T::deserialize(deserializer);
    }
    
    // 处理字符串
    if (yaml.starts_with('"') && yaml.ends_with('"')) || (yaml.starts_with('\'') && yaml.ends_with('\'')) {
        let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: yaml.to_string() });
        let deserializer = YamlValueDeserializer::new(value);
        return T::deserialize(deserializer);
    }
    
    // 处理简单的 YAML 映射（仅支持一级映射）
    if yaml.contains(':') && !yaml.starts_with('[') && !yaml.starts_with('{') {
        let mut entries = Vec::new();
        let lines = yaml.lines();
        
        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(colon_idx) = line.find(':') {
                let key_str = line[..colon_idx].trim();
                let value_str = line[colon_idx + 1..].trim();
                
                // 简单处理键值对
                let key = YamlValueNode::Scalar(YamlScalar { span: (0..key_str.len()).into(), value: key_str.to_string() });
                
                // 简单处理值
                let value = YamlValueNode::Scalar(YamlScalar { span: (0..value_str.len()).into(), value: value_str.to_string() });
                entries.push(YamlMappingEntry { span: (0..line.len()).into(), key, value });
            }
        }
        
        if !entries.is_empty() {
            let value = YamlValueNode::Mapping(YamlMapping { span: (0..yaml.len()).into(), entries });
            let deserializer = YamlValueDeserializer::new(value);
            return T::deserialize(deserializer);
        }
    }
    
    // 对于其他复杂类型，暂时使用一个简单的占位符实现
    // 注意：这只是一个临时解决方案，当 parser 完成后会更新
    let value = YamlValueNode::Scalar(YamlScalar { span: (0..yaml.len()).into(), value: yaml.to_string() });
    let deserializer = YamlValueDeserializer::new(value);
    T::deserialize(deserializer)
}

struct YamlValueDeserializer {
    value: YamlValueNode,
}

impl YamlValueDeserializer {
    fn new(value: YamlValueNode) -> Self {
        Self { value }
    }
}

impl<'de> serde::de::Deserializer<'de> for YamlValueDeserializer {
    type Error = OakError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            YamlValueNode::Scalar(scalar) => {
                let s = scalar.value;
                if s == "true" {
                    visitor.visit_bool(true)
                }
                else if s == "false" {
                    visitor.visit_bool(false)
                }
                else if s == "null" {
                    visitor.visit_none()
                }
                else if let Ok(n) = s.parse::<i64>() {
                    visitor.visit_i64(n)
                }
                else if let Ok(n) = s.parse::<f64>() {
                    visitor.visit_f64(n)
                }
                else {
                    visitor.visit_str(&s)
                }
            }
            YamlValueNode::Sequence(seq) => visitor.visit_seq(YamlSequenceAccess::new(seq.items)),
            YamlValueNode::Mapping(map) => visitor.visit_map(YamlMapAccess::new(map.entries)),
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

struct YamlSequenceAccess {
    items: Vec<YamlValueNode>,
    index: usize,
}

impl YamlSequenceAccess {
    fn new(items: Vec<YamlValueNode>) -> Self {
        Self { items, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for YamlSequenceAccess {
    type Error = OakError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.items.len() {
            let value = self.items[self.index].clone();
            self.index += 1;
            seed.deserialize(YamlValueDeserializer::new(value)).map(Some)
        }
        else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.items.len() - self.index)
    }
}

struct YamlMapAccess {
    entries: Vec<YamlMappingEntry>,
    index: usize,
}

impl YamlMapAccess {
    fn new(entries: Vec<YamlMappingEntry>) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'de> serde::de::MapAccess<'de> for YamlMapAccess {
    type Error = OakError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index < self.entries.len() {
            let key = self.entries[self.index].key.clone();
            seed.deserialize(YamlValueDeserializer::new(key)).map(Some)
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
        seed.deserialize(YamlValueDeserializer::new(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.entries.len() - self.index)
    }
}
