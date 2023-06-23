//! Source buffer for code generation and minification.

/// A trait for types that can be converted to source code.
pub trait ToSource {
    /// Writes the source code representation of this type to the provided buffer.
    fn to_source(&self, buffer: &mut SourceBuffer);

    /// Converts this type to a source code string.
    fn to_source_string(&self) -> String {
        let mut buffer = SourceBuffer::new();
        self.to_source(&mut buffer);
        buffer.finish()
    }
}

/// A buffer for building source code with intelligent spacing for minification.
#[derive(Debug, Clone, Default)]
pub struct SourceBuffer {
    inner: String,
    last_char: Option<char>,
}

impl std::fmt::Display for SourceBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl SourceBuffer {
    /// Creates a new, empty source buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a string to the buffer, automatically adding a space if necessary to prevent token merging.
    pub fn push(&mut self, s: &str) {
        if let Some(first) = s.chars().next() {
            // If both the last character and the first character of the new string are "word" characters,
            // we must insert a space to prevent them from merging into a single token (e.g., "let" + "a" -> "let a").
            if self.is_word_char(self.last_char) && self.is_word_char(Some(first)) {
                self.inner.push(' ')
            }
            self.inner.push_str(s);
            self.last_char = s.chars().last()
        }
    }

    /// Returns the accumulated source code as a string.
    pub fn finish(self) -> String {
        self.inner
    }

    /// Helper to check if a character is a "word" character (alphanumeric or underscore).
    fn is_word_char(&self, c: Option<char>) -> bool {
        c.map_or(false, |c| c.is_alphanumeric() || c == '_')
    }
}

impl From<SourceBuffer> for String {
    fn from(buffer: SourceBuffer) -> Self {
        buffer.finish()
    }
}
