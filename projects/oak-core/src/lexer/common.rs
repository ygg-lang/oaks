//! Common lexing utilities shared across languages.
//!
//! This module provides reusable scanners for whitespace, line comments,
//! block comments, and string literals. Language-specific lexers can call
//! these helpers to avoid re-implementing the same logic.

use crate::{SourceText, errors::OakError};
use alloc::format;

/// Configuration for whitespace scanning
#[derive(Debug, Clone, Copy)]
pub struct WhitespaceConfig {
    /// Whether to include Unicode whitespace characters
    pub unicode_whitespace: bool,
    /// Whether to treat specific characters as whitespace
    pub custom_whitespace: &'static [char],
}

impl Default for WhitespaceConfig {
    fn default() -> Self {
        Self { unicode_whitespace: true, custom_whitespace: &[] }
    }
}

/// Configuration for comment scanning
#[derive(Debug, Clone)]
pub struct CommentConfig {
    /// Single-line comment markers (e.g., ["//", "#", ";"])
    pub line_markers: &'static [&'static str],
    /// Block comment start/end pairs (e.g., [("/*", "*/"), ("<!--", "-->")])
    pub block_markers: &'static [(&'static str, &'static str)],
    /// Whether block comments can be nested
    pub nested_blocks: bool,
}

impl Default for CommentConfig {
    fn default() -> Self {
        Self { line_markers: &["//"], block_markers: &[("/*", "*/")], nested_blocks: false }
    }
}

/// Configuration for string scanning
#[derive(Debug, Clone)]
pub struct StringConfig {
    /// Quote characters that can start/end strings
    pub quotes: &'static [char],
    /// Whether to allow multiline strings
    pub multiline: bool,
    /// Whether to process escape sequences
    pub escape_sequences: bool,
    /// Custom escape characters (default is backslash)
    pub escape_char: char,
}

impl Default for StringConfig {
    fn default() -> Self {
        Self { quotes: &['"'], multiline: false, escape_sequences: true, escape_char: '\\' }
    }
}

/// Scans ASCII and Unicode whitespace starting at `start`.
/// Returns the end offset after consuming whitespace.
pub fn skip_whitespace(source: &SourceText, start: usize) -> usize {
    scan_whitespace_with_config(source, start, &WhitespaceConfig::default())
}

/// Scans whitespace with custom configuration.
pub fn scan_whitespace_with_config(source: &SourceText, mut start: usize, config: &WhitespaceConfig) -> usize {
    let len = source.len();
    while start < len {
        match source.get_char_at(start) {
            Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                start += 1; // ASCII whitespace
            }
            Some(c) if config.unicode_whitespace && c.is_whitespace() => {
                start += c.len_utf8();
            }
            Some(c) if config.custom_whitespace.contains(&c) => {
                start += c.len_utf8();
            }
            _ => break,
        }
    }
    start
}

/// Scans a line comment from `start` until a newline or end-of-input.
/// The caller should ensure the comment marker (e.g. `;`, `//`) is at `start`.
/// The returned end offset points to the first character after the comment text,
/// typically the newline character (which is not included in the comment span).
pub fn scan_line_comment(source: &SourceText, mut start: usize) -> usize {
    let len = source.len();
    while start < len {
        match source.get_char_at(start) {
            Some('\n') | Some('\r') => break,
            Some(c) => start += c.len_utf8(),
            None => break,
        }
    }
    start
}

/// Scans line comment with marker detection.
pub fn scan_line_comment_with_marker(source: &SourceText, start: usize, marker: &str) -> Option<usize> {
    let text = source.get_text_at(start)?;
    if text.starts_with(marker) { Some(scan_line_comment(source, start + marker.len())) } else { None }
}

/// Scans a block comment starting at `start`, where `start` should point to the
/// `/` character of a `/*` sequence. Returns the end offset after the closing `*/`.
/// If the comment is unterminated, returns an error and sets end to the end-of-input.
pub fn scan_block_comment(source: &SourceText, start: usize) -> (usize, Option<OakError>) {
    scan_block_comment_with_markers(source, start, "/*", "*/")
}

/// Scans block comment with custom start/end markers.
pub fn scan_block_comment_with_markers(
    source: &SourceText,
    start: usize,
    start_marker: &str,
    end_marker: &str,
) -> (usize, Option<OakError>) {
    let len = source.len();
    let mut i = start;

    // Check for start marker
    let text = source.get_text_at(i).unwrap_or("");
    if !text.starts_with(start_marker) {
        return (i, Some(source.syntax_error(&format!("Expected '{}' to start block comment", start_marker), i)));
    }

    i += start_marker.len();
    let mut closed = false;

    while i < len {
        let remaining = source.get_text_at(i).unwrap_or("");
        if remaining.starts_with(end_marker) {
            i += end_marker.len();
            closed = true;
            break;
        }

        match source.get_char_at(i) {
            Some(c) => i += c.len_utf8(),
            None => break,
        }
    }

    if !closed { (len, Some(source.syntax_error("Unterminated block comment", start))) } else { (i, None) }
}

/// Scans comments based on configuration.
pub fn scan_comment_with_config(
    source: &SourceText,
    start: usize,
    config: &CommentConfig,
) -> Option<(usize, Option<OakError>)> {
    let text = source.get_text_at(start).unwrap_or("");

    // Try line comments first
    for &marker in config.line_markers {
        if text.starts_with(marker) {
            let end = scan_line_comment(source, start + marker.len());
            return Some((end, None));
        }
    }

    // Try block comments
    for &(start_marker, end_marker) in config.block_markers {
        if text.starts_with(start_marker) {
            let (end, error) = scan_block_comment_with_markers(source, start, start_marker, end_marker);
            return Some((end, error));
        }
    }

    None
}

/// Scans a string literal body starting just after the opening quote at `start`.
/// Returns the end offset (pointing just after the closing quote) and an optional error
/// if the string is unterminated or has an invalid escape at the end of input.
pub fn scan_string_chars(source: &SourceText, start: usize, quote: char) -> (usize, Option<OakError>) {
    scan_string_chars_with_config(source, start, quote, &StringConfig::default())
}

/// Scans string with custom configuration.
pub fn scan_string_chars_with_config(
    source: &SourceText,
    start: usize,
    quote: char,
    config: &StringConfig,
) -> (usize, Option<OakError>) {
    let len = source.len();
    let literal_start = start - quote.len_utf8();
    let mut error: Option<OakError> = None;
    let mut current_start = start;

    while current_start < len {
        match source.get_char_at(current_start) {
            Some(c) if config.escape_sequences && c == config.escape_char => {
                current_start += c.len_utf8(); // skip escape char
                if current_start >= len {
                    error = Some(source.syntax_error("Invalid escape", current_start - c.len_utf8()));
                    break;
                }
                match source.get_char_at(current_start) {
                    Some(c) => current_start += c.len_utf8(),
                    None => break,
                }
            }
            Some('\n') | Some('\r') if !config.multiline => {
                error = Some(source.syntax_error("Unterminated string", literal_start));
                break;
            }
            Some(c) if c == quote => {
                current_start += c.len_utf8();
                break;
            }
            Some(c) => current_start += c.len_utf8(),
            None => break,
        }
    }

    if current_start >= len && error.is_none() {
        error = Some(source.syntax_error("Unterminated string", literal_start));
    }

    (current_start, error)
}

/// Scans string literal with quote detection.
pub fn scan_string_with_config(source: &SourceText, start: usize, config: &StringConfig) -> Option<(usize, Option<OakError>)> {
    if let Some(quote) = source.get_char_at(start) {
        if config.quotes.contains(&quote) {
            let (end, error) = scan_string_chars_with_config(source, start + quote.len_utf8(), quote, config);
            return Some((end, error));
        }
    }
    None
}

/// Utility function to check if a character is an identifier start character.
pub fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Utility function to check if a character can continue an identifier.
pub fn is_identifier_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Scans an identifier starting at `start`.
pub fn scan_identifier(source: &SourceText, mut start: usize) -> usize {
    let len = source.len();

    // Check first character
    if let Some(c) = source.get_char_at(start) {
        if !is_identifier_start(c) {
            return start;
        }
        start += c.len_utf8();
    }
    else {
        return start;
    }

    // Continue with identifier characters
    while start < len {
        match source.get_char_at(start) {
            Some(c) if is_identifier_continue(c) => {
                start += c.len_utf8();
            }
            _ => break,
        }
    }

    start
}

/// Scans a number starting at `start`. Supports integers, floats, hex, binary, and octal.
pub fn scan_number(source: &SourceText, mut start: usize) -> usize {
    let len = source.len();

    // Handle optional sign
    if let Some(c) = source.get_char_at(start) {
        if c == '+' || c == '-' {
            start += 1; // Increment start position
        }
    }

    // Check for hex, binary, or octal prefix
    if start + 1 < len {
        if let (Some('0'), Some(prefix)) = (source.get_char_at(start), source.get_char_at(start + 1)) {
            match prefix {
                'x' | 'X' => {
                    start += 2;
                    while start < len {
                        match source.get_char_at(start) {
                            Some(c) if c.is_ascii_hexdigit() => start += 1,
                            _ => break,
                        }
                    }
                    return start;
                }
                'b' | 'B' => {
                    start += 2;
                    while start < len {
                        match source.get_char_at(start) {
                            Some('0') | Some('1') => start += 1,
                            _ => break,
                        }
                    }
                    return start;
                }
                'o' | 'O' => {
                    start += 2;
                    while start < len {
                        match source.get_char_at(start) {
                            Some(c) if c >= '0' && c <= '7' => start += 1,
                            _ => break,
                        }
                    }
                    return start;
                }
                _ => {}
            }
        }
    }

    // Scan decimal digits
    while start < len {
        match source.get_char_at(start) {
            Some(c) if c.is_ascii_digit() => start += 1,
            _ => break,
        }
    }

    // Handle decimal point
    if start < len && source.get_char_at(start) == Some('.') {
        start += 1;
        while start < len {
            match source.get_char_at(start) {
                Some(c) if c.is_ascii_digit() => start += 1,
                _ => break,
            }
        }
    }

    // Handle scientific notation
    if start < len {
        if let Some(c) = source.get_char_at(start) {
            if c == 'e' || c == 'E' {
                start += 1;
                if start < len {
                    if let Some(c) = source.get_char_at(start) {
                        if c == '+' || c == '-' {
                            start += 1;
                        }
                    }
                }
                while start < len {
                    match source.get_char_at(start) {
                        Some(c) if c.is_ascii_digit() => start += 1,
                        _ => break,
                    }
                }
            }
        }
    }

    start
}
