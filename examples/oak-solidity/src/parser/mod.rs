/// Solidity element types.
pub mod element_type;

pub use element_type::SolidityElementType;

/// Solidity parser.
pub struct SolidityParser;

impl SolidityParser {
    /// Create a new Solidity parser.
    pub fn new() -> Self {
        Self
    }
}
