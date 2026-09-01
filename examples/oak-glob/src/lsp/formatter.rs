use crate::language::GlobLanguage;
use oak_core::tree::RedNode;

/// Formatter for glob pattern syntax.
#[derive(Default, Clone)]
pub struct GlobFormatter;

impl GlobFormatter {
    /// Formats the glob pattern source.
    pub fn format(&self, _root: &RedNode<GlobLanguage>, source: &str) -> String {
        source.to_string()
    }
}
