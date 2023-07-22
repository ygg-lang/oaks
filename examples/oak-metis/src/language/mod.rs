use oak_core::language::{Language, LanguageCategory};

use crate::{ast::MetisRoot, lexer::token_type::MetisTokenType, parser::element_type::MetisElementType};

/// Metis island language definition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MetisLanguage;

impl Language for MetisLanguage {
    const NAME: &'static str = "metis";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = MetisTokenType;
    type ElementType = MetisElementType;
    type TypedRoot = MetisRoot;
}

impl MetisLanguage {
    /// Lexer for this language.
    pub fn lexer(&self) -> super::lexer::MetisLexer {
        super::lexer::MetisLexer::default()
    }

    /// Parser for this language.
    pub fn parser(&self) -> super::parser::MetisParser {
        super::parser::MetisParser::default()
    }
}
