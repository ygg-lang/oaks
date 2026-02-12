#![doc = include_str!("readme.md")]
use core::range::Range;

/// Vala root node.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaRoot {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Top-level items.
    pub items: Vec<ValaItem>,
}

/// Vala top-level item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValaItem {
    /// Namespace.
    Namespace(ValaNamespace),
    /// Class.
    Class(ValaClass),
    /// Interface.
    Interface(ValaInterface),
    /// Method.
    Method(ValaMethod),
}

/// Vala namespace.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaNamespace {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Namespace name.
    pub name: String,
    /// Items in the namespace.
    pub items: Vec<ValaItem>,
}

/// Vala class.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaClass {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Class name.
    pub name: String,
    /// Parent class name.
    pub parent: Option<String>,
    /// Implemented interfaces.
    pub interfaces: Vec<String>,
    /// Class members.
    pub members: Vec<ValaMember>,
}

/// Vala interface.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaInterface {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Interface name.
    pub name: String,
    /// Interface members.
    pub members: Vec<ValaMember>,
}

/// Vala member.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValaMember {
    /// Field.
    Field(String),
    /// Property.
    Property(String),
    /// Method.
    Method(ValaMethod),
}

/// Vala method.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaMethod {
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Method name.
    pub name: String,
    /// Return type.
    pub return_type: String,
    /// Parameter list.
    pub params: Vec<ValaParam>,
}

/// Vala parameter
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValaParam {
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
}
