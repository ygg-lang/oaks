//! Root-level owned AST nodes for MATLAB.

use crate::ast::statement_nodes::Statement;

/// Source span (`start..end` byte offsets).
pub type Span = oak_core::Range<usize>;

/// A MATLAB identifier / name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// Name spelling.
    pub name: String,
    /// Source span of the name token.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

impl Default for Identifier {
    fn default() -> Self {
        Self { name: String::new(), span: Span::default() }
    }
}

/// Root of a MATLAB source unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatlabRoot {
    /// Top-level statements / expressions.
    pub items: Vec<Statement>,
    /// Span covering the whole unit.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
