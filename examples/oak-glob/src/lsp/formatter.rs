use oak_core::{formatter::Formatter as FormatterTrait, source::SourceText};

/// Formatter for glob pattern syntax.
pub struct GlobFormatter;

impl FormatterTrait for GlobFormatter {
    fn format<S: SourceText>(&self, text: &S) -> String {
        text.to_string()
    }
}

impl Default for GlobFormatter {
    fn default() -> Self {
        Self
    }
}
