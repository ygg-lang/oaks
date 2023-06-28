use serde::ser::{self, Serialize, Serializer};

use crate::ast::YamlValueNode;
use oak_core::OakError;

/// YAML 序列化实现
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value.serialize(serializer)
}

/// Serializes a Rust type into a YAML string.
pub fn to_string<T>(value: &T) -> Result<String, OakError>
where
    T: serde::Serialize,
{
    let yaml_value = value.serialize(YamlValueSerializer::default())?;
    Ok(format!("{}", yaml_value))
}

struct YamlValueSerializer {}

impl Default for YamlValueSerializer {
    fn default() -> Self {
        Self {}
    }
}

impl serde::ser::Serializer for YamlValueSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    type SerializeSeq = YamlSeqSerializer;
    type SerializeTuple = YamlSeqSerializer;
    type SerializeTupleStruct = YamlSeqSerializer;
    type SerializeTupleVariant = YamlSeqSerializer;
    type SerializeMap = YamlMapSerializer;
    type SerializeStruct = YamlMapSerializer;
    type SerializeStructVariant = YamlMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: String::from_utf8_lossy(v).to_string() }))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: "null".to_string() }))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: "null".to_string() }))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: variant.to_string() }))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(YamlSeqSerializer { items: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(YamlSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(YamlSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(YamlSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(YamlMapSerializer { entries: Vec::with_capacity(len.unwrap_or(0)), current_key: None })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(YamlMapSerializer { entries: Vec::with_capacity(len), current_key: None })
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(YamlMapSerializer { entries: Vec::with_capacity(len), current_key: None })
    }
}

struct YamlSeqSerializer {
    items: Vec<YamlValueNode>,
}

impl serde::ser::SerializeSeq for YamlSeqSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Sequence(crate::ast::YamlSequence { span: (0..0).into(), items: self.items }))
    }
}

impl serde::ser::SerializeTuple for YamlSeqSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Sequence(crate::ast::YamlSequence { span: (0..0).into(), items: self.items }))
    }
}

impl serde::ser::SerializeTupleStruct for YamlSeqSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Sequence(crate::ast::YamlSequence { span: (0..0).into(), items: self.items }))
    }
}

impl serde::ser::SerializeTupleVariant for YamlSeqSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Sequence(crate::ast::YamlSequence { span: (0..0).into(), items: self.items }))
    }
}

struct YamlMapSerializer {
    entries: Vec<crate::ast::YamlMappingEntry>,
    current_key: Option<YamlValueNode>,
}

impl serde::ser::SerializeMap for YamlMapSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let key_value = key.serialize(serializer)?;
        self.current_key = Some(key_value);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = YamlValueSerializer {};
        let value_value = value.serialize(serializer)?;
        if let Some(key) = self.current_key.take() {
            self.entries.push(crate::ast::YamlMappingEntry { span: (0..0).into(), key, value: value_value });
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Mapping(crate::ast::YamlMapping { span: (0..0).into(), entries: self.entries }))
    }
}

impl serde::ser::SerializeStruct for YamlMapSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key_value = YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: key.to_string() });
        let serializer = YamlValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.entries.push(crate::ast::YamlMappingEntry { span: (0..0).into(), key: key_value, value: value_value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Mapping(crate::ast::YamlMapping { span: (0..0).into(), entries: self.entries }))
    }
}

impl serde::ser::SerializeStructVariant for YamlMapSerializer {
    type Ok = YamlValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key_value = YamlValueNode::Scalar(crate::ast::YamlScalar { span: (0..0).into(), value: key.to_string() });
        let serializer = YamlValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.entries.push(crate::ast::YamlMappingEntry { span: (0..0).into(), key: key_value, value: value_value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(YamlValueNode::Mapping(crate::ast::YamlMapping { span: (0..0).into(), entries: self.entries }))
    }
}
