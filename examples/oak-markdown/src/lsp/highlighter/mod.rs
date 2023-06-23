#![doc = include_str!("readme.md")]
//! Markdown syntax highlighter.
//!
//! This module provides syntax highlighting for Markdown source code,
//! supporting headings, emphasis, code blocks, links, etc.

/// Local definition for highlight kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Heading.
    Heading,
    /// Emphasis (italics).
    Emphasis,
    /// Strong (bold).
    Strong,
    /// Code block or inline code.
    Code,
    /// Link.
    Link,
    /// List marker.
    ListMarker,
    /// Blockquote marker.
    BlockquoteMarker,
    /// Comment.
    Comment,
}

/// Highlighter trait for processing text into highlights.
pub trait Highlighter {
    /// Processes the given text and returns a list of highlights.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Markdown syntax highlighter.
pub struct MarkdownHighlighter;

impl Default for MarkdownHighlighter {
    fn default() -> Self {
        Self
    }
}

impl MarkdownHighlighter {
    /// Creates a new MarkdownHighlighter instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlights headings in the text.
    fn highlight_headings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        for line in text.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let pos = text.find(line).unwrap();
                highlights.push((pos, pos + line.len(), HighlightKind::Heading));
            }
        }
        highlights
    }

    /// Highlights code blocks.
    fn highlight_code_blocks(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut in_code_block = false;
        let mut start_pos = 0;

        for line in text.lines() {
            let pos = text.find(line).unwrap();
            if line.trim_start().starts_with("```") || line.trim_start().starts_with("~~~") {
                if in_code_block {
                    highlights.push((start_pos, pos + line.len(), HighlightKind::Code));
                    in_code_block = false;
                }
                else {
                    start_pos = pos;
                    in_code_block = true;
                }
            }
        }
        highlights
    }
}

impl Highlighter for MarkdownHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_headings(text));
        highlights.extend(self.highlight_code_blocks(text));

        // Sort by position.
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
