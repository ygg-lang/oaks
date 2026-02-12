pub use element_type::DejavuElementType;

/// Dejavu parser.
pub struct DejavuParser;

impl DejavuParser {
    /// Create a new Dejavu parser.
    pub fn new() -> Self {
        Self
    }
}

pub mod element_type;
