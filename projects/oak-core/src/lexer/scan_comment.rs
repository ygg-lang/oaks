use super::LexerState;
use crate::{Language, source::Source};

/// Configuration for comment scanning.
pub struct CommentConfig {
    /// Marker for line comments (e.g., "//").
    pub line_marker: &'static str,
    /// Marker for block comment start (e.g., "/*").
    pub block_start: &'static str,
    /// Marker for block comment end (e.g., "*/").
    pub block_end: &'static str,
    /// Whether block comments can be nested.
    pub nested_blocks: bool,
}

impl CommentConfig {
    /// Scans for a comment at the current position in the lexer state.
    pub fn scan<S: Source + ?Sized, L: Language>(&self, state: &mut LexerState<S, L>, line_kind: L::TokenType, block_kind: L::TokenType) -> bool {
        let start = state.get_position();

        // Try line comment
        if !self.line_marker.is_empty() && state.starts_with(self.line_marker) {
            state.advance(self.line_marker.len());
            state.take_while_byte(|b| b != b'\n');
            state.add_token(line_kind, start, state.get_position());
            return true;
        }

        // Try block comment
        if !self.block_start.is_empty() && state.starts_with(self.block_start) {
            state.advance(self.block_start.len());
            let mut depth = 1;
            while depth > 0 && state.not_at_end() {
                if self.nested_blocks && !self.block_start.is_empty() && state.starts_with(self.block_start) {
                    depth += 1;
                    state.advance(self.block_start.len())
                }
                else if !self.block_end.is_empty() && state.starts_with(self.block_end) {
                    depth -= 1;
                    state.advance(self.block_end.len())
                }
                else if let Some(ch) = state.current() {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(block_kind, start, state.get_position());
            return true;
        }

        false
    }
}

/// Checks if the given byte slice starts with a line comment marker ("//").
#[inline]
pub fn starts_with_line_comment(bytes: &[u8]) -> bool {
    bytes.starts_with(b"//")
}

/// Checks if the given byte slice starts with a block comment marker ("/*").
#[inline]
pub fn starts_with_block_comment(bytes: &[u8]) -> bool {
    bytes.starts_with(b"/*")
}
