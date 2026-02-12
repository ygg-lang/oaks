pub use element_type::ValkyrieElementType;

/// Valkyrie parser.
pub struct ValkyrieParser;

impl ValkyrieParser {
    /// Create a new Valkyrie parser.
    pub fn new() -> Self {
        Self
    }
}

pub mod element_type;
