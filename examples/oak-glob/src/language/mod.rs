use oak_core::language::{Language, LanguageCategory};

use crate::{ast::GlobRoot, lexer::token_type::GlobTokenType, parser::element_type::GlobElementType};

/// Language definition for glob pattern syntax.
pub struct GlobLanguage;

impl Language for GlobLanguage {
    const NAME: &'static str = "glob";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = GlobTokenType;
    type ElementType = GlobElementType;
    type TypedRoot = GlobRoot;
}

impl Default for GlobLanguage {
    fn default() -> Self {
        Self
    }
}

impl GlobLanguage {
    /// Returns a new parser for the glob language.
    pub fn parser(&self) -> super::parser::GlobParser {
        super::parser::GlobParser::default()
    }

    /// Returns a new lexer for the glob language.
    pub fn lexer(&self) -> super::lexer::GlobLexer {
        super::lexer::GlobLexer::default()
    }
}
