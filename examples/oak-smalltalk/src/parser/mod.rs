pub mod element_type;

pub use element_type::SmalltalkElementType;

/// Smalltalk parser.
pub struct SmalltalkParser;

impl SmalltalkParser {
    /// Create a new Smalltalk parser.
    pub fn new() -> Self {
        Self
    }
}
