#![doc = include_str!("readme.md")]
use crate::VerilogLanguage;

/// Formatter implementation for Verilog.
pub struct VerilogFormatter {}

impl VerilogFormatter {
    /// Formats the given source.
    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }
}
