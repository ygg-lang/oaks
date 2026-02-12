#![doc = include_str!("readme.md")]
/// Solidity code formatter.
pub struct SolidityFormatter;

impl SolidityFormatter {
    /// Creates a new Solidity formatter.
    pub fn new() -> Self {
        Self
    }

    /// Formats Solidity source code.
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}
