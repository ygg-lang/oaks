#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WgslLanguage {}

impl WgslLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for WgslLanguage {
    const NAME: &'static str = "wgsl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::WgslTokenType;
    type ElementType = crate::parser::element_type::WgslElementType;
    type TypedRoot = ();
}
