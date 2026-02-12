#![doc = include_str!("readme.md")]
//! MSIL syntax highlighter.
//!
//! This module provides syntax highlighting for MSIL source code, supporting keywords, directives, opcodes, comments, etc.

/// Local definition of highlight kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords (e.g., public, private, static).
    Keyword,
    /// Directives (starting with ., e.g., .assembly, .class, .method).
    Directive,
    /// Opcodes (e.g., ldstr, call, ret).
    Instruction,
    /// Strings.
    String,
    /// Numbers.
    Number,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
}

/// Highlighter trait.
pub trait Highlighter {
    /// Highlights the given text.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// MSIL syntax highlighter.
pub struct MsilHighlighter;

impl MsilHighlighter {
    /// Creates a new MSIL highlighter instance.
    pub fn new() -> Self {
        Self
    }

    /// Highlights MSIL keywords.
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = ["public", "private", "static", "hidebysig", "cil", "managed", "instance", "void", "extends", "implements"];

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

    /// Highlights directives (starting with .).
    fn highlight_directives(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch == '.' {
                let start = i;
                let mut end = i + 1;
                while let Some(&(j, next_ch)) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        end = j + next_ch.len_utf8();
                        chars.next();
                    }
                    else {
                        break;
                    }
                }
                highlights.push((start, end, HighlightKind::Directive))
            }
        }
        highlights
    }
}

impl Default for MsilHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter for MsilHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_directives(text));

        // Sort by position
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
