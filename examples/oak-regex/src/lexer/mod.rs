pub use crate::lexer::token_type::RegexTokenType;

/// Regex lexer.
pub struct RegexLexer;

impl RegexLexer {
    /// Create a new Regex lexer.
    pub fn new() -> Self {
        Self
    }
}

pub mod token_type;
