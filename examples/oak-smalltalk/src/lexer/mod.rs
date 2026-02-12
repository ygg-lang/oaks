pub mod token_type;

pub use token_type::SmalltalkTokenType;

/// Smalltalk lexer.
pub struct SmalltalkLexer;

impl SmalltalkLexer {
    /// Create a new Smalltalk lexer.
    pub fn new() -> Self {
        Self
    }
}
