#![doc = include_str!("readme.md")]
// use crate::{lexer::token_type::WitTokenType, parser::element_type::WitElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy)]
pub struct WitLanguage {}

impl Language for WitLanguage {
    const NAME: &'static str = "wit-component";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::lexer::token_type::WitTokenType;
    type ElementType = crate::parser::element_type::WitElementType;
    type TypedRoot = ();
}
