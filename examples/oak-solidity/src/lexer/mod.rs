/// Solidity token types.
pub mod token_type;

pub use token_type::SolidityTokenType;

/// Solidity lexer.
pub struct SolidityLexer;

impl SolidityLexer {
    /// Create a new Solidity lexer.
    pub fn new() -> Self {
        Self
    }
}
