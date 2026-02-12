#![doc = include_str!("readme.md")]
use crate::ast::WgslRoot;
use oak_core::{Language, LanguageCategory};

/// WGSL language definition.
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WgslLanguage {}

impl WgslLanguage {
    /// Creates a new WGSL language definition.
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WgslLanguage {
    const NAME: &'static str = "wgsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::WgslTokenType;
    type ElementType = crate::parser::element_type::WgslElementType;
    type TypedRoot = WgslRoot;
}
