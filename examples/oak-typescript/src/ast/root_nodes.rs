use crate::ast::Statement;
use core::range::Range;

/// TypeScript AST root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeScriptRoot {
    /// Statements in the root.
    pub statements: Vec<Statement>,
    /// Source span of the root.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl TypeScriptRoot {
    /// Creates a new `TypeScriptRoot`.
    pub fn new(span: Range<usize>) -> Self {
        Self { statements: vec![], span }
    }
}
