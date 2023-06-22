use core::range::Range;
use serde::{Deserialize, Serialize};

/// Identifier in Delphi
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root for Delphi language
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelphiRoot {
    pub items: Vec<DelphiItem>,
}

/// Top-level items in Delphi language
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DelphiItem {
    Program(DelphiProgram),
    Unit(DelphiUnit),
    Statement(DelphiStatement),
}

/// Represents a Delphi program
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelphiProgram {
    pub name: Identifier,
    pub statements: Vec<DelphiStatement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a Delphi unit
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DelphiUnit {
    pub name: Identifier,
    pub interface_section: Vec<DelphiStatement>,
    pub implementation_section: Vec<DelphiStatement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents various statements in Delphi language
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DelphiStatement {
    /// Empty statement
    Empty {
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}
