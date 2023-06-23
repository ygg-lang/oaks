#![doc = include_str!("readme.md")]
use crate::ProtobufTokenType;

pub type ProtobufToken = ProtobufTokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct ProtobufRoot {}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Message(Message),
    Service(Service),
    Enum(Enum),
    Import(Import),
    Package(Package),
    Option(Option),
    Syntax(Syntax),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub field_type: FieldType,
    pub name: String,
    pub number: u32,
    pub options: Vec<FieldOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Scalar(ScalarType),
    Message(String),
    Repeated(Box<FieldType>),
    Optional(Box<FieldType>),
    Required(Box<FieldType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarType {
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub options: Vec<MethodOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
    pub options: Vec<EnumValueOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_public: bool,
    pub is_weak: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Option {
    pub name: String,
    pub value: OptionValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Syntax {
    pub version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    String(String),
    Number(f64),
    Bool(bool),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOption {
    pub name: String,
    pub value: OptionValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodOption {
    pub name: String,
    pub value: OptionValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueOption {
    pub name: String,
    pub value: OptionValue,
}
