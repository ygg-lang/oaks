#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root of the IDL AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdlRoot {
    /// Items defined at the top level.
    pub items: Vec<IdlItem>,
}

/// A top-level item in an IDL file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdlItem {
    /// A module definition.
    Module(Module),
    /// An interface definition.
    Interface(Interface),
    /// A struct definition.
    Struct(Struct),
    /// An enum definition.
    Enum(Enum),
    /// A type alias definition.
    Typedef(Typedef),
    /// A constant definition.
    Const(Const),
}

/// A module in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Module {
    /// The name of the module.
    pub name: String,
    /// Items contained within the module.
    pub items: Vec<IdlItem>,
    /// The source range of this module.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An interface in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interface {
    /// The name of the interface.
    pub name: String,
    /// Members of the interface (attributes and operations).
    pub members: Vec<IdlMember>,
    /// The source range of this interface.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A member of an IDL interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdlMember {
    /// An attribute.
    Attribute(Attribute),
    /// An operation (method).
    Operation(Operation),
}

/// An attribute in an interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attribute {
    /// The name of the attribute.
    pub name: String,
    /// The type of the attribute.
    pub type_name: String,
    /// Whether the attribute is read-only.
    pub readonly: bool,
}

/// An operation (method) in an interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Operation {
    /// The name of the operation.
    pub name: String,
    /// The return type of the operation.
    pub return_type: String,
    /// Parameters of the operation.
    pub params: Vec<Param>,
}

/// A parameter of an operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    /// The name of the parameter.
    pub name: String,
    /// The type of the parameter.
    pub type_name: String,
    /// The direction of the parameter (in, out, inout).
    pub direction: ParamDirection,
}

/// Parameter passing direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParamDirection {
    /// Input parameter.
    In,
    /// Output parameter.
    Out,
    /// Input/Output parameter.
    Inout,
}

/// A struct definition in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Struct {
    /// The name of the struct.
    pub name: String,
    /// Fields of the struct.
    pub fields: Vec<Field>,
    /// The source range of this struct.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A field in a struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub type_name: String,
}

/// An enum definition in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// Variants of the enum.
    pub variants: Vec<String>,
}

/// A type alias in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Typedef {
    /// The name of the new type.
    pub name: String,
    /// The existing type it aliases.
    pub type_name: String,
}

/// A constant definition in IDL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Const {
    /// The name of the constant.
    pub name: String,
    /// The type of the constant.
    pub type_name: String,
    /// The value of the constant.
    pub value: String,
}
