#![doc = include_str!("readme.md")]
use crate::ProtobufTokenType;

/// Alias for ProtobufTokenType for convenience.
pub type ProtobufToken = ProtobufTokenType;

/// Represents the root of a Protobuf document.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtobufRoot {
    /// The definitions in the file.
    pub definitions: Vec<Definition>,
}

/// Represents a definition in a Protobuf file.
#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    /// A message definition.
    Message(Message),
    /// A service definition.
    Service(Service),
    /// An enum definition.
    Enum(Enum),
    /// An import statement.
    Import(Import),
    /// A package statement.
    Package(Package),
    /// A top-level option statement.
    Option(ProtobufOption),
    /// A syntax statement.
    Syntax(Syntax),
}

/// A message definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// The name of the message.
    pub name: String,
    /// The fields within the message.
    pub fields: Vec<Field>,
}

/// A field within a message.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The type of the field.
    pub field_type: FieldType,
    /// The name of the field.
    pub name: String,
    /// The field number.
    pub number: u32,
    /// Options associated with the field.
    pub options: Vec<FieldOption>,
}

/// Represents the type of a field.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    /// A scalar type (e.g., int32, string).
    Scalar(ScalarType),
    /// A reference to another message type.
    Message(String),
    /// A repeated field.
    Repeated(Box<FieldType>),
    /// An optional field (proto2).
    Optional(Box<FieldType>),
    /// A required field (proto2).
    Required(Box<FieldType>),
}

/// Scalar types in Protobuf.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarType {
    /// double
    Double,
    /// float
    Float,
    /// int32
    Int32,
    /// int64
    Int64,
    /// uint32
    Uint32,
    /// uint64
    Uint64,
    /// sint32
    Sint32,
    /// sint64
    Sint64,
    /// fixed32
    Fixed32,
    /// fixed64
    Fixed64,
    /// sfixed32
    Sfixed32,
    /// sfixed64
    Sfixed64,
    /// bool
    Bool,
    /// string
    String,
    /// bytes
    Bytes,
}

/// A service definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    /// The name of the service.
    pub name: String,
    /// The methods within the service.
    pub methods: Vec<Method>,
}

/// A method definition within a service.
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    /// The name of the method.
    pub name: String,
    /// The input message type.
    pub input_type: String,
    /// The output message type.
    pub output_type: String,
    /// Options associated with the method.
    pub options: Vec<MethodOption>,
}

/// An enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values within the enum.
    pub values: Vec<EnumValue>,
}

/// A value within an enum.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {
    /// The name of the enum value.
    pub name: String,
    /// The numeric value.
    pub number: i32,
    /// Options associated with the enum value.
    pub options: Vec<EnumValueOption>,
}

/// An import statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    /// The path to the imported file.
    pub path: String,
    /// Whether the import is public.
    pub is_public: bool,
    /// Whether the import is weak.
    pub is_weak: bool,
}

/// A package statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    /// The name of the package.
    pub name: String,
}

/// A top-level option statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ProtobufOption {
    /// The name of the option.
    pub name: String,
    /// The value of the option.
    pub value: OptionValue,
}

/// A syntax statement.
#[derive(Debug, Clone, PartialEq)]
pub struct Syntax {
    /// The version of the syntax (e.g., "proto2", "proto3").
    pub version: String,
}

/// Represents the value of an option.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    /// A string value.
    String(String),
    /// A numeric value.
    Number(f64),
    /// A boolean value.
    Bool(bool),
    /// An identifier value.
    Identifier(String),
}

/// An option associated with a field.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldOption {
    /// The name of the option.
    pub name: String,
    /// The value of the option.
    pub value: OptionValue,
}

/// An option associated with a method.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodOption {
    /// The name of the option.
    pub name: String,
    /// The value of the option.
    pub value: OptionValue,
}

/// An option associated with an enum value.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueOption {
    /// The name of the option.
    pub name: String,
    /// The value of the option.
    pub value: OptionValue,
}
