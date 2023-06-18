use crate::{SyntaxKind, Token};
use std::range::Range;

/// Configuration for comment scanning
#[derive(Debug, Clone)]
pub struct CommentLine {
    /// Single-line comment markers (e.g., ["//", "#", ";"])
    pub line_markers: &'static [&'static str],
}

/// Configuration for block comment scanning
#[derive(Debug, Clone)]
pub struct CommentBlock {
    /// Block comment start/end pairs (e.g., [("/*", "*/"), ("<!--", "-->")])
    pub block_markers: &'static [(&'static str, &'static str)],
    /// Whether block comments can be nested
    pub nested_blocks: bool,
}

impl Default for CommentLine {
    fn default() -> Self {
        Self { line_markers: &["//"] }
    }
}

impl Default for CommentBlock {
    fn default() -> Self {
        Self { block_markers: &[("/*", "*/")], nested_blocks: false }
    }
}

impl CommentLine {
    /// Scan for a line comment at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the comment
    ///
    /// # Returns
    ///
    /// A token if a line comment is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        for marker in self.line_markers {
            if view.starts_with(marker) {
                return Some(Token { kind, span: Range { start, end: start + marker.len() } });
            }
        }
        None
    }
}

impl CommentBlock {
    /// Scan for a block comment at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the comment
    ///
    /// # Returns
    ///
    /// A token if a block comment is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        for (start_marker, end_marker) in self.block_markers {
            if view.starts_with(start_marker) {
                let end_index = view[start + start_marker.len()..].find(end_marker);
                if let Some(end_index) = end_index {
                    return Some(Token {
                        kind,
                        span: Range { start, end: start + start_marker.len() + end_index + end_marker.len() },
                    });
                }
            }
        }
        None
    }
}
