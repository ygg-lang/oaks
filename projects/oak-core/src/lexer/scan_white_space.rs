//! Common lexing utilities shared across languages.
//!
//! This module provides reusable scanners for whitespace, line comments,
//! block comments, and string literals. Language-specific lexers can call
//! these helpers to avoid re-implementing the same logic.

use crate::{SyntaxKind, Token};
use std::range::Range;

/// Configuration for whitespace scanning
#[derive(Debug, Clone, Copy)]
pub struct WhitespaceConfig {
    /// Whether to include newline characters in the whitespace scan
    ///
    /// Notice whitespace includes newline characters
    pub unicode_whitespace: bool,
}
/// Configuration for custom whitespace scanning
pub struct WhitespaceCustom {
    /// Custom characters to treat as whitespace
    pub custom_whitespace: &'static [char],
}

impl Default for WhitespaceConfig {
    fn default() -> Self {
        Self {
            // 大多数语言不支持 Unicode 空白字符
            unicode_whitespace: false,
        }
    }
}

impl Default for WhitespaceCustom {
    fn default() -> Self {
        Self { custom_whitespace: &[' ', '\t', '\n', '\t'] }
    }
}

impl WhitespaceConfig {
    /// Scan for whitespace at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the whitespace
    ///
    /// # Returns
    ///
    /// A token if whitespace is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        let mut length = 0;
        let mut iter = view.chars();
        while let Some(c) = iter.next() {
            if self.unicode_whitespace && c.is_whitespace() {
                length += c.len_utf8();
            }
            else if c.is_ascii_whitespace() {
                length += c.len_utf8();
            }
            else {
                break;
            }
        }
        if length == 0 {
            return None;
        }
        Some(Token { kind, span: Range { start, end: start + length } })
    }
}

impl WhitespaceCustom {
    /// Scan for custom whitespace at the given position
    ///
    /// # Arguments
    ///
    /// * `view` - The text view to scan
    /// * `start` - The starting byte position
    /// * `kind` - The token kind to assign to the whitespace
    ///
    /// # Returns
    ///
    /// A token if custom whitespace is found, `None` otherwise
    pub fn scan<K: SyntaxKind>(&self, view: &str, start: usize, kind: K) -> Option<Token<K>> {
        let mut length = 0;
        let mut iter = view.chars();
        while let Some(c) = iter.next() {
            if self.custom_whitespace.contains(&c) {
                length += c.len_utf8();
            }
            else {
                break;
            }
        }
        if length == 0 {
            return None;
        }
        Some(Token { kind, span: Range { start, end: start + length } })
    }
}
