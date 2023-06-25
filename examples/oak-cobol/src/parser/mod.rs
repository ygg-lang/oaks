pub mod element_type;

pub use element_type::CobolElementType;

/// COBOL parser.
pub struct CobolParser;

impl CobolParser {
    /// Create a new COBOL parser.
    pub fn new() -> Self {
        Self
    }
}
