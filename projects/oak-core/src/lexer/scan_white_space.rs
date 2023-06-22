use super::LexerState;
use crate::{
    Language,
    source::{SimdScanner, Source},
};

/// Configuration for whitespace scanning.
pub struct WhitespaceConfig {
    /// Whether to recognize Unicode whitespace characters.
    pub unicode_whitespace: bool,
}

impl WhitespaceConfig {
    /// Scans for whitespace at the current position in the lexer state.
    pub fn scan<S: Source + ?Sized, L: Language>(&self, state: &mut LexerState<S, L>, kind: L::TokenType) -> bool {
        let start = state.get_position();
        let range = if self.unicode_whitespace { state.take_while(|c| c.is_whitespace()) } else { state.skip_ascii_whitespace() };

        if range.end > start {
            state.add_token(kind, start, range.end);
            true
        }
        else {
            false
        }
    }
}

/// Counts how many spaces or tabs are at the start of the byte slice.
#[inline]
pub fn count_space_tab_prefix(bytes: &[u8]) -> usize {
    SimdScanner::skip_two_bytes(bytes, b' ', b'\t')
}
