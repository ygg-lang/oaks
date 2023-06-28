use oak_core::{highlight::Highlighter as HighlighterTrait, lexer::Token};

use crate::lexer::GlobTokenType;

/// Highlighter for glob pattern syntax.
pub struct GlobHighlighter;

impl HighlighterTrait for GlobHighlighter {
    type TokenType = GlobTokenType;

    fn highlight(&self, token: &Token<Self::TokenType>) -> Option<&'static str> {
        match token.token_type {
            GlobTokenType::Comment => Some("comment"),
            GlobTokenType::Rule => Some("string"),
            _ => None,
        }
    }
}

impl Default for GlobHighlighter {
    fn default() -> Self {
        Self
    }
}
