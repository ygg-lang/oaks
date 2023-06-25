use crate::{
    ast::{DsvField, DsvRecord, DsvRoot},
    language::DsvLanguage,
};
use oak_core::OakError as Error;

/// Serializer for converting Rust types to DSV values.
pub struct Serializer<const LANG: DsvLanguage>;

impl<const LANG: DsvLanguage> Default for Serializer<LANG> {
    fn default() -> Self {
        Self
    }
}

impl<const LANG: DsvLanguage> Serializer<LANG> {
    /// Creates a new serializer.
    pub fn new() -> Self {
        Self
    }
}

/// Converts a serializable value to a `DsvRoot` with the given configuration.
pub fn to_value<const LANG: DsvLanguage, T: serde::Serialize>(value: &T) -> Result<DsvRoot<LANG>, Error> {
    value.serialize(Serializer::<LANG>::new())
}

impl<const LANG: DsvLanguage> serde::Serializer for Serializer<LANG> {
    type Ok = DsvRoot<LANG>;
    type Error = Error;

    type SerializeSeq = SerializeSeq<LANG>;
    type SerializeTuple = SerializeSeq<LANG>;
    type SerializeTupleStruct = SerializeSeq<LANG>;
    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeMap = serde::ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = SerializeRecord<LANG>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: vec![DsvField { value: v.to_string(), is_quoted: false, span: (0..0).into() }], span: (0..0).into() }] })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: vec![DsvField { value: v.to_string(), is_quoted: false, span: (0..0).into() }], span: (0..0).into() }] })
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: vec![DsvField { value: v.to_string(), is_quoted: false, span: (0..0).into() }], span: (0..0).into() }] })
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: vec![DsvField { value: v.to_string(), is_quoted: false, span: (0..0).into() }], span: (0..0).into() }] })
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: vec![DsvField { value: v.to_owned(), is_quoted: v.contains(LANG.field_separator) || v.contains(LANG.quote_char) || v.contains('\n'), span: (0..0).into() }], span: (0..0).into() }] })
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&String::from_utf8_lossy(v))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![] })
    }
    fn serialize_some<T: ?Sized + serde::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
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
    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq::new())
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(serde::ser::Error::custom("tuple variants are not supported"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(serde::ser::Error::custom("maps are not supported"))
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeRecord::new())
    }
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(serde::ser::Error::custom("struct variants are not supported"))
    }
}

pub struct SerializeSeq<const LANG: DsvLanguage> {
    records: Vec<DsvRecord<LANG>>,
}

impl<const LANG: DsvLanguage> SerializeSeq<LANG> {
    fn new() -> Self {
        Self { records: Vec::new() }
    }
}

impl<const LANG: DsvLanguage> serde::ser::SerializeSeq for SerializeSeq<LANG> {
    type Ok = DsvRoot<LANG>;
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let root: DsvRoot<LANG> = value.serialize(Serializer::<LANG>::new())?;
        self.records.extend(root.records);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: self.records })
    }
}

impl<const LANG: DsvLanguage> serde::ser::SerializeTuple for SerializeSeq<LANG> {
    type Ok = DsvRoot<LANG>;
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<const LANG: DsvLanguage> serde::ser::SerializeTupleStruct for SerializeSeq<LANG> {
    type Ok = DsvRoot<LANG>;
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeRecord<const LANG: DsvLanguage> {
    fields: Vec<DsvField<LANG>>,
}

impl<const LANG: DsvLanguage> SerializeRecord<LANG> {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }
}

impl<const LANG: DsvLanguage> serde::ser::SerializeStruct for SerializeRecord<LANG> {
    type Ok = DsvRoot<LANG>;
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::Serialize>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> {
        let root: DsvRoot<LANG> = value.serialize(Serializer::<LANG>::new())?;
        for record in root.records {
            self.fields.extend(record.fields);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DsvRoot { records: vec![DsvRecord { fields: self.fields, span: (0..0).into() }] })
    }
}
