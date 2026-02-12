#![doc = include_str!("readme.md")]
//! Matlab syntax highlighter.
//!
//! This module provides syntax highlighting for Matlab source code, supporting keywords, comments, strings, etc.

use oak_highlight::highlighter::{HighlightKind, Highlighter};

/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword
    Keyword,
    /// String
    String,
    /// Number
    Number,
    /// Comment
    Comment,
    /// Identifier
    Identifier,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// MATLAB syntax highlighter
///
/// `MatlabHighlighter` implements `Highlighter` trait, providing syntax highlighting for MATLAB code.
pub struct MatlabHighlighter;

impl Default for MatlabHighlighter {
    fn default() -> Self {
        Self
    }
}

impl MatlabHighlighter {
    /// Creates a new MATLAB highlighter instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights MATLAB keywords
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["break", "case", "catch", "classdef", "continue", "else", "elseif", "end", "for", "function", "global", "if", "otherwise", "parfor", "persistent", "return", "spmd", "switch", "try", "while"];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                let is_word_boundary_before = absolute_pos == 0 || !text.chars().nth(absolute_pos - 1).unwrap_or(' ').is_alphanumeric();
                let is_word_boundary_after = end_pos >= text.len() || !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();

                if is_word_boundary_before && is_word_boundary_after {
                    highlights.push((absolute_pos, end_pos, HighlightKind::Keyword))
                }
                start = absolute_pos + 1
            }
        }
        highlights
    }

    /// Highlights string literals
    fn highlight_strings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '\'' {
                let start = i;
                let mut end = i + 1;
                while let Some((j, next_ch)) = chars.next() {
                    end = j + next_ch.len_utf8();
                    if next_ch == '\'' {
                        // Check for double quote escape
                        if let Some(&(_, peek_ch)) = chars.peek() {
                            if peek_ch == '\'' {
                                chars.next();
                                continue;
                            }
                        }
                        break;
                    }
                }
                highlights.push((start, end, HighlightKind::String))
            }
        }
        highlights
    }
}

impl Highlighter for MatlabHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_strings(text));

        // Sort by position
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
