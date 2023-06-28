use serde::ser::{self, Serialize, Serializer};

use crate::language::Value;
use oak_core::OakError;

/// TOML 序列化实现
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    value.serialize(serializer)
}

/// Serializes a Rust type into a TOML string.
pub fn to_string<T>(value: &T) -> Result<String, OakError>
where
    T: serde::Serialize,
{
    // 使用自定义序列化器将值转换为 Value
    let serializer = ValueSerializer {};
    let toml_value = value.serialize(serializer)?;
    
    // 将 Value 转换为字符串
    Ok(format!("{}", toml_value))
}

struct ValueSerializer {}

impl Default for ValueSerializer {
    fn default() -> Self {
        Self {}
    }
}

impl serde::ser::Serializer for ValueSerializer {
    type Ok = Value;
    type Error = OakError;

    type SerializeSeq = ValueSeqSerializer;
    type SerializeTuple = ValueSeqSerializer;
    type SerializeTupleStruct = ValueSeqSerializer;
    type SerializeTupleVariant = ValueSeqSerializer;
    type SerializeMap = ValueMapSerializer;
    type SerializeStruct = ValueMapSerializer;
    type SerializeStructVariant = ValueMapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v as i64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(String::from_utf8_lossy(v).to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // TOML doesn't have a null type, so we'll use a string "null"
        Ok(Value::String("null".to_string()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // TOML doesn't have a unit type, so we'll use a string "null"
        Ok(Value::String("null".to_string()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(variant.to_string()))
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
        Ok(ValueSeqSerializer { items: Vec::with_capacity(len.unwrap_or(0)) })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(ValueSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ValueSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(ValueSeqSerializer { items: Vec::with_capacity(len) })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(ValueMapSerializer { entries: std::collections::HashMap::with_capacity(len.unwrap_or(0)), current_key: None })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(ValueMapSerializer { entries: std::collections::HashMap::with_capacity(len), current_key: None })
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(ValueMapSerializer { entries: std::collections::HashMap::with_capacity(len), current_key: None })
    }
}

struct ValueSeqSerializer {
    items: Vec<Value>,
}

impl serde::ser::SerializeSeq for ValueSeqSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

impl serde::ser::SerializeTuple for ValueSeqSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

impl serde::ser::SerializeTupleStruct for ValueSeqSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

impl serde::ser::SerializeTupleVariant for ValueSeqSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let item = value.serialize(serializer)?;
        self.items.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.items))
    }
}

struct ValueMapSerializer {
    entries: std::collections::HashMap<String, Value>,
    current_key: Option<String>,
}

impl serde::ser::SerializeMap for ValueMapSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key_str = key.to_string();
        self.current_key = Some(key_str);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let value_value = value.serialize(serializer)?;
        if let Some(key) = self.current_key.take() {
            self.entries.insert(key, value_value);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Table(self.entries))
    }
}

impl serde::ser::SerializeStruct for ValueMapSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.entries.insert(key.to_string(), value_value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Table(self.entries))
    }
}

impl serde::ser::SerializeStructVariant for ValueMapSerializer {
    type Ok = Value;
    type Error = OakError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let serializer = ValueSerializer {};
        let value_value = value.serialize(serializer)?;
        self.entries.insert(key.to_string(), value_value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Table(self.entries))
    }
}
