pub use crate::parser::element_type::RegexElementType;

/// Regex parser.
pub struct RegexParser;

impl RegexParser {
    /// Create a new Regex parser.
    pub fn new() -> Self {
        Self
    }
}

pub mod element_type;
