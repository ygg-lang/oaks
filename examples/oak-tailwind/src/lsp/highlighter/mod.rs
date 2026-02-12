//! Highlighter for the Tailwind language.

/// Highlight types for Tailwind language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keywords.
    Keyword,
    /// Comments.
    Comment,
    /// Variables.
    Variable,
}

/// Highlighter for Tailwind language.
#[derive(Debug, Default, Clone, Copy)]
pub struct TailwindHighlighter;

impl TailwindHighlighter {
    /// Creates a new Tailwind highlighter.
    pub fn new() -> Self {
        Self
    }

    /// Highlights the given text and returns a list of ranges with their highlight kinds.
    pub fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        // Simple keyword highlighting
        let keywords = ["if", "else", "endif", "for", "in", "endfor", "set", "extends", "include", "block", "endblock"];
        for keyword in keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let actual_pos = start + pos;
                // Check if it's a standalone word
                let is_start = actual_pos == 0 || !text.as_bytes()[actual_pos - 1].is_ascii_alphanumeric();
                let is_end = actual_pos + keyword.len() == text.len() || !text.as_bytes()[actual_pos + keyword.len()].is_ascii_alphanumeric();

                if is_start && is_end {
                    highlights.push((actual_pos, actual_pos + keyword.len(), HighlightKind::Keyword))
                }
                start = actual_pos + keyword.len()
            }
        }

        // Highlight comments {# ... #}
        let mut start = 0;
        while let Some(pos) = text[start..].find("{#") {
            let actual_start = start + pos;
            if let Some(pos_end) = text[actual_start..].find("#}") {
                let actual_end = actual_start + pos_end + 2;
                highlights.push((actual_start, actual_end, HighlightKind::Comment));
                start = actual_end
            }
            else {
                highlights.push((actual_start, text.len(), HighlightKind::Comment));
                break;
            }
        }

        // Highlight variables {{ ... }}
        let mut start = 0;
        while let Some(pos) = text[start..].find("{{") {
            let actual_start = start + pos;
            if let Some(pos_end) = text[actual_start..].find("}}") {
                let actual_end = actual_start + pos_end + 2;
                highlights.push((actual_start, actual_end, HighlightKind::Variable));
                start = actual_end
            }
            else {
                break;
            }
        }

        highlights.sort_by_key(|h| h.0);
        highlights
    }
}
