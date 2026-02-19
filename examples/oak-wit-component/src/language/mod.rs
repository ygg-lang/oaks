#![doc = include_str!("readme.md")]
// use crate::{lexer::token_type::WitTokenType, parser::element_type::WitElementType};
use oak_core::{Language, LanguageCategory};

/// Language configuration for WIT (WebAssembly Interface Types) Component.
///
/// This struct implements the `Language` trait and provides the configuration
/// for lexing and parsing WIT component definition files.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone, Copy)]
pub struct WitLanguage {}

impl Language for WitLanguage {
    const NAME: &'static str = "wit-component";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::lexer::token_type::WitTokenType;
    type ElementType = crate::parser::element_type::WitElementType;
    type TypedRoot = crate::ast::WitRoot;
}
