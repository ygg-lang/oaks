#![doc = include_str!("readme.md")]
use core::range::Range;

/// Identifier with name and source span.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// Identifier name.
    pub name: String,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Root node of the HLSL AST.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HlslRoot {
    /// Top-level declarations.
    pub declarations: Vec<Declaration>,
}

/// Top-level declaration types in HLSL.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Declaration {
    /// Function declaration.
    Function(Function),
    /// Variable declaration.
    Variable(Variable),
    /// Struct declaration.
    Struct(Struct),
}

/// Function declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    /// Function name.
    pub name: Identifier,
    /// Return type.
    pub return_type: String,
    /// Function parameters.
    pub parameters: Vec<Parameter>,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function parameter.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Parameter {
    /// Parameter name.
    pub name: Identifier,
    /// Parameter type.
    pub type_name: String,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Variable declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    /// Variable name.
    pub name: Identifier,
    /// Variable type.
    pub type_name: String,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Struct declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Struct {
    /// Struct name.
    pub name: Identifier,
    /// Struct members.
    pub members: Vec<Variable>,
    /// Source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
