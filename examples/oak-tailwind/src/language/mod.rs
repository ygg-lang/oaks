#![doc = include_str!("readme.md")]
use crate::{ast::TailwindRoot, lexer::TailwindLexer, parser::TailwindParser};
use oak_core::{Language, LanguageCategory};

/// Tailwind engine configuration modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TailwindMode {
    /// Template mode (e.g., HTML with Tailwind classes).
    #[default]
    Template,
    /// Expression mode (e.g., inside a template tag).
    Expression,
}

/// Tailwind language definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailwindLanguage {
    /// Whether to allow raw blocks.
    pub allow_raw_blocks: bool,
    /// Whether to allow custom tags.
    pub allow_custom_tags: bool,
    /// The current mode of the engine.
    pub mode: TailwindMode,
}

impl Language for TailwindLanguage {
    const NAME: &'static str = "tailwind";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;

    type TokenType = crate::lexer::token_type::TailwindTokenType;
    type ElementType = crate::parser::element_type::TailwindElementType;
    type TypedRoot = TailwindRoot;
}

impl TailwindLanguage {
    /// Creates a new `TailwindLanguage` with standard settings.
    pub fn new() -> Self {
        Self::standard()
    }

    /// Creates a new `TailwindLanguage` with standard settings.
    pub fn standard() -> Self {
        Self { allow_raw_blocks: true, allow_custom_tags: false, mode: TailwindMode::Template }
    }

    /// Creates a new lexer for this language configuration.
    pub fn lexer(&self) -> TailwindLexer<'_> {
        TailwindLexer::new(self)
    }

    /// Creates a new parser for this language configuration.
    pub fn parser(&self) -> TailwindParser<'_> {
        TailwindParser::new(self)
    }
}
