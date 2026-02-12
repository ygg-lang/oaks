//! Abstract Syntax Tree (AST) definitions for Groovy.

use core::range::Range;

/// Groovy program root node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroovyRoot {
    /// Top-level declarations
    pub declarations: Vec<Declaration>,
    /// Source location
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Groovy declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Declaration {
    /// Class declaration
    Class {
        /// Class name
        name: String,
        /// Members
        members: Vec<Declaration>,
        /// Source location
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Method declaration
    Method {
        /// Method name
        name: String,
        /// Parameters
        params: Vec<Parameter>,
        /// Source location
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Variable declaration
    Variable {
        /// Variable name
        name: String,
        /// Source location
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

/// Groovy parameter
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_name: Option<String>,
    /// Source location
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
