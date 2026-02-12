pub use token_type::ValkyrieTokenType;

/// Valkyrie lexer.
pub struct ValkyrieLexer;

impl ValkyrieLexer {
    /// Create a new Valkyrie lexer.
    pub fn new() -> Self {
        Self
    }
}

pub mod token_type;
