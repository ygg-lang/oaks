#![doc = include_str!("readme.md")]
use crate::{ast::TwigRoot, lexer::TwigLexer, parser::TwigParser};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Twig template engine configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TwigMode {
    #[default]
    Template,
    Expression,
    // Other possible modes
}

/// Twig language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwigLanguage {
    pub allow_raw_blocks: bool,
    pub allow_custom_tags: bool,
    pub mode: TwigMode,
}

impl Language for TwigLanguage {
    const NAME: &'static str = "twig";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::TwigTokenType;
    type ElementType = crate::parser::element_type::TwigElementType;
    type TypedRoot = TwigRoot;
}

impl TwigLanguage {
    pub fn new() -> Self {
        Self::standard()
    }

    pub fn standard() -> Self {
        Self { allow_raw_blocks: true, allow_custom_tags: false, mode: TwigMode::Template }
    }

    pub fn lexer(&self) -> TwigLexer<'_> {
        TwigLexer::new(self)
    }

    pub fn parser(&self) -> TwigParser<'_> {
        TwigParser::new(self)
    }
}
