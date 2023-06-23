#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Identifier in Delphi
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root for Delphi language
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DelphiRoot {
    pub items: Vec<DelphiItem>,
}

/// Top-level items in Delphi language
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DelphiItem {
    Program(DelphiProgram),
    Unit(DelphiUnit),
    Statement(DelphiStatement),
}

/// Represents a Delphi program
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DelphiProgram {
    pub name: Identifier,
    pub statements: Vec<DelphiStatement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a Delphi unit
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DelphiUnit {
    pub name: Identifier,
    pub interface_section: Vec<DelphiStatement>,
    pub implementation_section: Vec<DelphiStatement>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents various statements in Delphi language
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DelphiStatement {
    /// Empty statement
    Empty {
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
