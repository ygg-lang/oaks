use crate::ast::{XmlAttribute, XmlElement, XmlValue};
use serde::{
    de::{self, Visitor},
    ser::{self, Serialize},
};
use std::fmt;

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        e.0
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

pub struct Serializer;

pub fn to_value<T: Serialize>(value: &T) -> Result<XmlValue, Error> {
    value.serialize(Serializer)
}

pub fn from_value<T: de::DeserializeOwned>(value: XmlValue) -> Result<T, Error> {
    T::deserialize(value)
}

impl ser::Serializer for Serializer {
    type Ok = XmlValue;
    type Error = Error;

    type SerializeSeq = SerializeArray;
    type SerializeTuple = SerializeArray;
    type SerializeTupleStruct = SerializeArray;
    type SerializeTupleVariant = SerializeVariant;
    type SerializeMap = SerializeObject;
    type SerializeStruct = SerializeObject;
    type SerializeStructVariant = SerializeVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Text(v.to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Text(v.to_string()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Text(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let elements = v.iter().map(|&b| XmlValue::Text(b.to_string())).collect();
        Ok(XmlValue::Element(XmlElement { name: "bytes".to_string(), attributes: Vec::new(), children: elements, span: (0..0).into() }))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // In XML, None is usually just an absent element.
        // But if we are serializing to a Value, maybe an empty string or special marker?
        // Let's use an empty text for now.
        Ok(XmlValue::Text(String::new()))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        let mut children = Vec::new();
        children.push(value.serialize(self)?);
        Ok(XmlValue::Element(XmlElement { name: variant.to_string(), attributes: Vec::new(), children, span: (0..0).into() }))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeArray { elements: Vec::new() })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeVariant { name: variant.to_string(), elements: Vec::new() })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeObject {
            name: "root".to_string(), // Default name, should be overridden by struct name
            attributes: Vec::new(),
            children: Vec::new(),
            current_key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeObject { name: name.to_string(), attributes: Vec::new(), children: Vec::new(), current_key: None })
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeVariant { name: variant.to_string(), elements: Vec::new() })
    }
}

pub struct SerializeArray {
    elements: Vec<XmlValue>,
}

impl ser::SerializeSeq for SerializeArray {
    type Ok = XmlValue;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.elements.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Fragment(self.elements))
    }
}

impl ser::SerializeTuple for SerializeArray {
    type Ok = XmlValue;
    type Error = Error;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeArray {
    type Ok = XmlValue;
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

pub struct SerializeVariant {
    name: String,
    elements: Vec<XmlValue>,
}

impl ser::SerializeTupleVariant for SerializeVariant {
    type Ok = XmlValue;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.elements.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Element(XmlElement { name: self.name, attributes: Vec::new(), children: self.elements, span: (0..0).into() }))
    }
}

impl ser::SerializeStructVariant for SerializeVariant {
    type Ok = XmlValue;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        let val = value.serialize(Serializer)?;
        self.elements.push(XmlValue::Element(XmlElement { name: key.to_string(), attributes: Vec::new(), children: vec![val], span: (0..0).into() }));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Element(XmlElement { name: self.name, attributes: Vec::new(), children: self.elements, span: (0..0).into() }))
    }
}

pub struct SerializeObject {
    name: String,
    attributes: Vec<XmlAttribute>,
    children: Vec<XmlValue>,
    current_key: Option<String>,
}

impl ser::SerializeMap for SerializeObject {
    type Ok = XmlValue;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        self.current_key = Some(key.serialize(StringSerializer)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let key = self.current_key.take().unwrap();
        let val = value.serialize(Serializer)?;

        if key.starts_with('@') {
            self.attributes.push(XmlAttribute {
                name: key[1..].to_string(),
                value: match val {
                    XmlValue::Text(t) => t,
                    XmlValue::Fragment(_) => return Err(ser::Error::custom("Cannot use fragment as attribute value")),
                    _ => val.as_str().unwrap_or("").to_string(),
                },
                span: (0..0).into(),
            });
        }
        else if key == "$value" {
            match val {
                XmlValue::Fragment(elements) => {
                    self.children.extend(elements);
                }
                _ => {
                    self.children.push(val);
                }
            }
        }
        else {
            match val {
                XmlValue::Fragment(elements) => {
                    for element in elements {
                        match element {
                            XmlValue::Element(mut e) => {
                                e.name = key.clone();
                                self.children.push(XmlValue::Element(e));
                            }
                            _ => {
                                self.children.push(XmlValue::Element(XmlElement { name: key.clone(), attributes: Vec::new(), children: vec![element], span: (0..0).into() }));
                            }
                        }
                    }
                }
                XmlValue::Element(mut e) => {
                    e.name = key;
                    self.children.push(XmlValue::Element(e));
                }
                _ => {
                    self.children.push(XmlValue::Element(XmlElement { name: key, attributes: Vec::new(), children: vec![val], span: (0..0).into() }));
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(XmlValue::Element(XmlElement { name: self.name, attributes: self.attributes, children: self.children, span: (0..0).into() }))
    }
}

impl ser::SerializeStruct for SerializeObject {
    type Ok = XmlValue;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}

struct StringSerializer;

impl ser::Serializer for StringSerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = ser::Impossible<String, Error>;
    type SerializeTuple = ser::Impossible<String, Error>;
    type SerializeTupleStruct = ser::Impossible<String, Error>;
    type SerializeTupleVariant = ser::Impossible<String, Error>;
    type SerializeMap = ser::Impossible<String, Error>;
    type SerializeStruct = ser::Impossible<String, Error>;
    type SerializeStructVariant = ser::Impossible<String, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(format!("{:?}", v))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _name: &'static str, _variant_index: u32, variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("Impossible"))
    }
}

impl<'de> de::Deserializer<'de> for XmlValue {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            XmlValue::Text(t) => visitor.visit_string(t),
            XmlValue::Element(e) => visitor.visit_map(MapDeserializer::new(e)),
            XmlValue::Fragment(fs) => visitor.visit_seq(SeqDeserializer::new(fs)),
            _ => Err(de::Error::custom("Unsupported XML value type for deserialize_any")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            XmlValue::Text(ref t) if t.is_empty() => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            XmlValue::Text(t) => visitor.visit_enum(EnumDeserializer::new(t, None)),
            XmlValue::Element(e) => visitor.visit_enum(EnumDeserializer::new(e.name.clone(), Some(XmlValue::Element(e)))),
            _ => Err(de::Error::custom("Expected text or element for enum")),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            XmlValue::Text(t) => visitor.visit_string(t),
            XmlValue::Element(e) => {
                if e.attributes.is_empty() && e.children.iter().all(|c| matches!(c, XmlValue::Text(_))) {
                    let mut text = String::new();
                    for child in e.children {
                        if let XmlValue::Text(t) = child {
                            text.push_str(&t);
                        }
                    }
                    visitor.visit_string(text)
                }
                else {
                    XmlValue::Element(e).deserialize_any(visitor)
                }
            }
            XmlValue::Fragment(fs) => {
                let mut text = String::new();
                for f in fs {
                    text.push_str(&f.to_string());
                }
                visitor.visit_string(text)
            }
            other => other.deserialize_any(visitor),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            XmlValue::Element(e) => visitor.visit_seq(SeqDeserializer::new(e.children)),
            XmlValue::Fragment(fs) => visitor.visit_seq(SeqDeserializer::new(fs)),
            _ => Err(de::Error::custom("Expected element or fragment for seq")),
        }
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct MapDeserializer {
    entries: std::vec::IntoIter<(String, XmlValue)>,
    current_value: Option<XmlValue>,
}

impl MapDeserializer {
    fn new(element: XmlElement) -> Self {
        let mut entries: Vec<(String, Vec<XmlValue>)> = Vec::new();

        for attr in element.attributes {
            let key = "@".to_string() + &attr.name;
            entries.push((key, vec![XmlValue::Text(attr.value)]));
        }

        for child in element.children {
            match child {
                XmlValue::Element(e) => {
                    if let Some(entry) = entries.iter_mut().find(|(k, _)| k == &e.name) {
                        entry.1.push(XmlValue::Element(e));
                    }
                    else {
                        let name = e.name.clone();
                        entries.push((name, vec![XmlValue::Element(e)]));
                    }
                }
                XmlValue::Text(t) => {
                    if let Some(entry) = entries.iter_mut().find(|(k, _)| k == "$value") {
                        entry.1.push(XmlValue::Text(t.clone()));
                    }
                    else {
                        entries.push(("$value".to_string(), vec![XmlValue::Text(t.clone())]));
                    }
                }
                _ => {}
            }
        }

        let final_entries = entries
            .into_iter()
            .map(|(k, mut v)| {
                let val = if k == "$value" {
                    if v.len() == 1 { v.remove(0) } else { XmlValue::Fragment(v) }
                }
                else if v.len() == 1 {
                    v.remove(0)
                }
                else {
                    XmlValue::Fragment(v)
                };
                println!("MapEntry: key={}, value={:?}", k, val);
                (k, val)
            })
            .collect::<Vec<_>>();

        Self { entries: final_entries.into_iter(), current_value: None }
    }
}

impl<'de> de::MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.entries.next() {
            self.current_value = Some(value);
            return seed.deserialize(de::value::StrDeserializer::<Error>::new(&key)).map(Some);
        }

        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.current_value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("next_value_seed called before next_key_seed")),
        }
    }
}

struct SeqDeserializer {
    iter: std::vec::IntoIter<XmlValue>,
}

impl SeqDeserializer {
    fn new(elements: Vec<XmlValue>) -> Self {
        Self { iter: elements.into_iter() }
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option<XmlValue>,
}

impl EnumDeserializer {
    fn new(variant: String, value: Option<XmlValue>) -> Self {
        Self { variant, value }
    }
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(de::value::StrDeserializer::<Error>::new(&self.variant))?;
        Ok((variant, VariantDeserializer { value: self.value }))
    }
}

struct VariantDeserializer {
    value: Option<XmlValue>,
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("Expected newtype variant")),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(XmlValue::Element(e)) => visitor.visit_seq(SeqDeserializer::new(e.children)),
            _ => Err(de::Error::custom("Expected element for tuple variant")),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(XmlValue::Element(e)) => visitor.visit_map(MapDeserializer::new(e)),
            _ => Err(de::Error::custom("Expected element for struct variant")),
        }
    }
}
