use crate::{SyntaxKind, Token};
use std::range::Range;

/// Configuration for string scanning
#[derive(Debug, Clone)]
pub struct StringConfig {
    /// Quote characters that can start/end strings
    pub quotes: &'static [char],
    /// Custom escape characters (default is backslash)
    pub escape: Option<char>,
}

/// Configuration for multiline string scanning
pub struct StringMultilineConfig {
    /// Opening delimiters for multiline strings (e.g., ["\"\"\"", "'''"])
    pub open_delimiters: &'static [&'static str],
    /// Closing delimiters for multiline strings (e.g., ["\"\"\"", "'''"])
    pub close_delimiters: &'static [&'static str],
    /// Whether to allow escape sequences in multiline strings
    pub escape: Option<char>,
}

impl Default for StringConfig {
    fn default() -> Self {
        Self { quotes: &['"'], escape: Some('\\') }
    }
}

impl Default for StringMultilineConfig {
    fn default() -> Self {
        Self { open_delimiters: &["\"\"\"", "'''"], close_delimiters: &["\"\"\"", "'''"], escape: Some('\\') }
    }
}

impl StringConfig {
    /// Scan for a string at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the string
    ///
    /// # Returns
    ///
    /// A token if a string is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        for quote in self.quotes {
            if view.starts_with(*quote) {
                let end_index = view[start + 1..].find(*quote);
                if let Some(end_index) = end_index {
                    return Some(Token { kind, span: Range { start, end: start + 1 + end_index } });
                }
            }
        }
        None
    }
}
impl StringMultilineConfig {
    /// Scan for a multiline string at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the string
    ///
    /// # Returns
    ///
    /// A token if a multiline string is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        let remaining = &view[start..];

        // Try each opening delimiter
        for (i, open_delim) in self.open_delimiters.iter().enumerate() {
            if remaining.starts_with(open_delim) {
                let close_delim = self.close_delimiters[i];
                let mut pos = open_delim.len();

                // Scan until we find the closing delimiter
                while pos < remaining.len() {
                    // Check for closing delimiter
                    if remaining[pos..].starts_with(close_delim) {
                        pos += close_delim.len();
                        return Some(Token { kind, span: Range { start, end: start + pos } });
                    }

                    // Handle escape sequences if enabled
                    if let Some(escape_char) = self.escape {
                        if remaining[pos..].starts_with(escape_char) && pos + 1 < remaining.len() {
                            // Skip the escape character and the next character
                            pos += 2;
                            continue;
                        }
                    }

                    // Move to next character
                    let ch = remaining[pos..].chars().next().unwrap_or('\0');
                    pos += ch.len_utf8();
                }

                // If we reach here, we didn't find a closing delimiter
                // Return the token anyway, but it will be incomplete
                return Some(Token { kind, span: Range { start, end: start + remaining.len() } });
            }
        }

        None
    }
}
