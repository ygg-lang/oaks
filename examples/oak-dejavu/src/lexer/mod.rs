pub use token_type::DejavuTokenType;

/// Dejavu lexer.
pub struct DejavuLexer;

impl DejavuLexer {
    /// Create a new Dejavu lexer.
    pub fn new() -> Self {
        Self
    }
}

pub mod token_type;
