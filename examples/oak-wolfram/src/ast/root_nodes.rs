//! Root-level owned AST nodes for Wolfram Language.

use crate::ast::expression_nodes::Expression;

/// Source span (`start..end` byte offsets).
pub type Span = oak_core::Range<usize>;

/// A Wolfram symbol / identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// Symbol spelling.
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

/// Root of a Wolfram source unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframRoot {
    /// Top-level expressions in source order.
    pub expressions: Vec<Expression>,
    /// Span covering the whole unit.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
