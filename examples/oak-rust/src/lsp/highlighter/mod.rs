#![doc = include_str!("readme.md")]

/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords
    Keyword,
    /// Strings
    String,
    /// Numbers
    Number,
    /// Comments
    Comment,
    /// Macros
    Macro,
    /// Identifiers
    Identifier,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlights the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Rust syntax highlighter
///
/// `RustHighlighter` implements the `Highlighter` trait, providing syntax highlighting for Rust code.
pub struct RustHighlighter {
    /// Whether to use parser-based highlighting for better accuracy
    pub use_parser: bool,
}

impl Default for RustHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl RustHighlighter {
    /// Creates a new Rust highlighter instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a highlighter instance that uses a parser
    pub fn with_parser() -> Self {
        Self { use_parser: true }
    }

    /// Highlights Rust keywords
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let keywords = [
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
            "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "union", "raw",
        ];

        for keyword in &keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let absolute_pos = start + pos;
                let end_pos = absolute_pos + keyword.len();

                // Ensure this is a full word
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
            match ch {
                '"' => {
                    let start = i;
                    let mut end = i + 1;
                    let mut escaped = false;

                    while let Some((j, next_ch)) = chars.next() {
                        end = j + next_ch.len_utf8();
                        if escaped {
                            escaped = false;
                        }
                        else if next_ch == '\\' {
                            escaped = true;
                        }
                        else if next_ch == '"' {
                            break;
                        }
                    }

                    highlights.push((start, end, HighlightKind::String))
                }
                '\'' => {
                    let start = i;
                    let mut end = i + 1;
                    let mut escaped = false;

                    while let Some((j, next_ch)) = chars.next() {
                        end = j + next_ch.len_utf8();
                        if escaped {
                            escaped = false;
                        }
                        else if next_ch == '\\' {
                            escaped = true;
                        }
                        else if next_ch == '\'' {
                            break;
                        }
                    }

                    highlights.push((start, end, HighlightKind::String))
                }
                _ => {}
            }
        }

        highlights
    }

    /// Highlights number literals
    fn highlight_numbers(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch.is_ascii_digit() {
                let start = i;
                let mut end = i + 1;

                // Continue reading number characters
                while let Some(&(j, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == '_' {
                        end = j + next_ch.len_utf8();
                        chars.next();
                    }
                    else {
                        break;
                    }
                }

                highlights.push((start, end, HighlightKind::Number))
            }
        }

        highlights
    }

    /// Highlights comments
    fn highlight_comments(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut pos = 0;

        for line in lines {
            if let Some(comment_start) = line.find("//") {
                let start = pos + comment_start;
                let end = pos + line.len();
                highlights.push((start, end, HighlightKind::Comment))
            }
            pos += line.len() + 1; // +1 for newline
        }

        highlights
    }
}

impl Highlighter for RustHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_keywords(text));
        highlights.extend(self.highlight_strings(text));
        highlights.extend(self.highlight_numbers(text));
        highlights.extend(self.highlight_comments(text));

        // Sort by position
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
