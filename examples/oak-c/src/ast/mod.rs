#![doc = include_str!("readme.md")]
/// C language abstract syntax.
mod declaration_nodes;
mod expression_nodes;
mod statement_nodes;

pub use declaration_nodes::*;
pub use expression_nodes::*;
pub use statement_nodes::*;

/// C language AST root node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CRoot {
    /// The translation unit containing the source code structure.
    pub translation_unit: TranslationUnit,
    /// The source span of the root node.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Translation unit (the top-level structure of a C program).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit {
    /// List of external declarations.
    pub external_declarations: Vec<ExternalDeclaration>,
    /// The source span of the translation unit.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

impl CRoot {
    /// Creates a new AST
    pub fn new(translation_unit: TranslationUnit, span: core::range::Range<usize>) -> Self {
        Self { translation_unit, span }
    }
}

impl TranslationUnit {
    /// Creates a new translation unit
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: core::range::Range<usize>) -> Self {
        Self { external_declarations, span }
    }
}
