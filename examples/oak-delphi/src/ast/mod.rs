#![doc = include_str!("readme.md")]
use core::range::Range;

/// Identifier in Delphi
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,
    /// The span of the identifier in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Delphi AST root node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DelphiRoot {
    /// Items in the Delphi source.
    pub items: Vec<DelphiItem>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

impl DelphiRoot {
    /// Creates a new `DelphiRoot`.
    pub fn new(items: Vec<DelphiItem>, span: core::range::Range<usize>) -> Self {
        Self { items, span }
    }
}

/// Top-level items in Delphi language
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelphiItem {
    /// A Delphi program.
    Program(DelphiProgram),
    /// A Delphi unit.
    Unit(DelphiUnit),
    /// A Delphi statement.
    Statement(DelphiStatement),
}

/// Represents a Delphi program
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelphiProgram {
    /// The name of the program.
    pub name: Identifier,
    /// The statements in the program.
    pub statements: Vec<DelphiStatement>,
    /// The span of the program in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a Delphi unit
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelphiUnit {
    /// The name of the unit.
    pub name: Identifier,
    /// The interface section of the unit.
    pub interface_section: Vec<DelphiStatement>,
    /// The implementation section of the unit.
    pub implementation_section: Vec<DelphiStatement>,
    /// The span of the unit in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents various statements in Delphi language
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DelphiStatement {
    /// Empty statement
    Empty {
        /// The span of the empty statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}
