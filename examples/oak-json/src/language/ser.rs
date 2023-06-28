use serde::ser::{self, Serialize, Serializer};

use crate::ast::JsonValueNode;
use oak_core::OakError;

/// JSON 序列化实现
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value.serialize(serializer)
}

/// Serializes a Rust type into a JSON string.
pub fn to_string<T>(value: &T) -> Result<String, OakError>
where
    T: serde::Serialize,
{
    // 直接使用 serde_json 来序列化 Rust 类型
    serde_json::to_string(value).map_err(|e| OakError::custom_error(format!("{:?}", e)))
}

struct JsonValueSerializer {}

impl Default for JsonValueSerializer {
    fn default() -> Self {
        Self {}
    }
}

impl serde::ser::Serializer for JsonValueSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    type SerializeSeq = JsonSeqSerializer;
    type SerializeTuple = JsonSeqSerializer;
    type SerializeTupleStruct = JsonSeqSerializer;
    type SerializeTupleVariant = JsonSeqSerializer;
    type SerializeMap = JsonMapSerializer;
    type SerializeStruct = JsonMapSerializer;
    type SerializeStructVariant = JsonMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Boolean(crate::ast::JsonBoolean { span: (0..0).into(), value: v }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v as f64 }))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Number(crate::ast::JsonNumber { span: (0..0).into(), value: v }))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: v.to_string() }))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: String::from_utf8_lossy(v).to_string() }))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Null(crate::ast::JsonNull { span: (0..0).into() }))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Null(crate::ast::JsonNull { span: (0..0).into() }))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::String(crate::ast::JsonString { span: (0..0).into(), value: variant.to_string() }))
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
        Ok(JsonSeqSerializer { elements: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(JsonSeqSerializer { elements: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(JsonSeqSerializer { elements: Vec::with_capacity(len) })
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(JsonSeqSerializer { elements: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(JsonMapSerializer { fields: Vec::with_capacity(len.unwrap_or(0)), current_key: None })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(JsonMapSerializer { fields: Vec::with_capacity(len), current_key: None })
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(JsonMapSerializer { fields: Vec::with_capacity(len), current_key: None })
    }
}

struct JsonSeqSerializer {
    elements: Vec<JsonValueNode>,
}

impl serde::ser::SerializeSeq for JsonSeqSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let item = value.serialize(serializer)?;
        self.elements.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Array(crate::ast::JsonArray { elements: self.elements, span: (0..0).into() }))
    }
}

impl serde::ser::SerializeTuple for JsonSeqSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let item = value.serialize(serializer)?;
        self.elements.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Array(crate::ast::JsonArray { elements: self.elements, span: (0..0).into() }))
    }
}

impl serde::ser::SerializeTupleStruct for JsonSeqSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let item = value.serialize(serializer)?;
        self.elements.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Array(crate::ast::JsonArray { elements: self.elements, span: (0..0).into() }))
    }
}

impl serde::ser::SerializeTupleVariant for JsonSeqSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let item = value.serialize(serializer)?;
        self.elements.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Array(crate::ast::JsonArray { elements: self.elements, span: (0..0).into() }))
    }
}

struct JsonMapSerializer {
    fields: Vec<crate::ast::JsonField>,
    current_key: Option<JsonValueNode>,
}

impl serde::ser::SerializeMap for JsonMapSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let key_value = key.serialize(serializer)?;
        self.current_key = Some(key_value);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = JsonValueSerializer {};
        let value_value = value.serialize(serializer)?;
        if let Some(key) = self.current_key.take() {
            // Convert JsonValueNode key to JsonString
            let json_string = match key {
                JsonValueNode::String(s) => s,
                _ => return Err(OakError::custom_error("JSON keys must be strings")),
            };
            self.fields.push(crate::ast::JsonField { span: (0..0).into(), name: json_string, value: value_value });
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Object(crate::ast::JsonObject { fields: self.fields, span: (0..0).into() }))
    }
}

impl serde::ser::SerializeStruct for JsonMapSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let json_string = crate::ast::JsonString { span: (0..0).into(), value: key.to_string() };
        let serializer = JsonValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.fields.push(crate::ast::JsonField { span: (0..0).into(), name: json_string, value: value_value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Object(crate::ast::JsonObject { fields: self.fields, span: (0..0).into() }))
    }
}

impl serde::ser::SerializeStructVariant for JsonMapSerializer {
    type Ok = JsonValueNode;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let json_string = crate::ast::JsonString { span: (0..0).into(), value: key.to_string() };
        let serializer = JsonValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.fields.push(crate::ast::JsonField { span: (0..0).into(), name: json_string, value: value_value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(JsonValueNode::Object(crate::ast::JsonObject { fields: self.fields, span: (0..0).into() }))
    }
}
