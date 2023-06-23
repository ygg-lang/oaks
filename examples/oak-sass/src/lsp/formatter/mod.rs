#![doc = include_str!("readme.md")]
use crate::ast::SassRoot;

/// Sass Code Formatter
pub struct SassFormatter {}

impl SassFormatter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn format(&self, source: &str) -> String {
        source.to_string()
    }

    pub fn format_root(&self, _root: &SassRoot) -> String {
        String::new()
    }
}

impl Default for SassFormatter {
    fn default() -> Self {
        Self::new()
    }
}
