#![doc = include_str!("readme.md")]

/// MSIL language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MsilLanguage;

impl MsilLanguage {
    /// Creates a new `MsilLanguage`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for MsilLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl oak_core::Language for MsilLanguage {
    type TokenType = crate::lexer::token_type::MsilTokenType;
    type ElementType = crate::parser::element_type::MsilElementType;
    type TypedRoot = crate::ast::MsilRoot;
    const NAME: &'static str = "msil";
}
