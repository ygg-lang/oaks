use super::LexerState;
use crate::{
    Language,
    source::{SimdScanner, Source},
};

/// Configuration for string literal scanning.
pub struct StringConfig {
    /// Characters that can start and end a string (e.g., '"', '\'').
    pub quotes: &'static [char],
    /// Character used for escaping (e.g., '\\').
    pub escape: Option<char>,
}

impl StringConfig {
    /// Scans for a string literal at the current position in the lexer state.
    pub fn scan<S: Source + ?Sized, L: Language>(&self, state: &mut LexerState<S, L>, kind: L::TokenType) -> bool {
        let start = state.get_position();
        let quote = match state.current() {
            Some(c) if self.quotes.contains(&c) => c,
            _ => return false,
        };

        state.advance(quote.len_utf8());

        // Fast path for ASCII strings
        if quote.is_ascii() && self.escape.map_or(true, |c| c.is_ascii()) {
            let q_byte = quote as u8;
            let e_byte = self.escape.map(|c| c as u8).unwrap_or(q_byte);

            loop {
                let (rest_len, found_info) = {
                    let rest = state.rest();
                    if rest.is_empty() {
                        (0, None)
                    }
                    else {
                        let bytes = rest.as_bytes();
                        if let Some(pos) = find_first_of_4(bytes, q_byte, e_byte, q_byte, e_byte) { (rest.len(), Some((pos, bytes[pos]))) } else { (rest.len(), None) }
                    }
                };

                if rest_len == 0 {
                    break;
                }

                if let Some((pos, found)) = found_info {
                    state.advance(pos);
                    if found == q_byte {
                        state.advance(1);
                        state.add_token(kind, start, state.get_position());
                        return true;
                    }
                    else {
                        state.advance(1);
                        if let Some(next) = state.current() {
                            state.advance(next.len_utf8());
                        }
                    }
                }
                else {
                    state.advance(rest_len);
                }
            }

            // Unterminated string
            state.add_token(kind, start, state.get_position());
            return true;
        }

        while let Some(ch) = state.current() {
            if Some(ch) == self.escape {
                state.advance(ch.len_utf8());
                if let Some(next) = state.current() {
                    state.advance(next.len_utf8());
                }
            }
            else if ch == quote {
                state.advance(ch.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        // Unterminated string
        state.add_token(kind, start, state.get_position());
        true
    }
}

/// Finds the first occurrence of any of the four given bytes.
#[inline]
pub fn find_first_of_4(bytes: &[u8], a: u8, b: u8, c: u8, d: u8) -> Option<usize> {
    SimdScanner::find_first_of_4(bytes, a, b, c, d)
}
