use oak_core::language::Language;
use oak_formatter::Formatter as FormatterTrait;

/// Formatter for glob pattern syntax.
pub struct GlobFormatter;

impl<L: Language> FormatterTrait<L> for GlobFormatter {
    type State = ();
    type Output = String;

    fn format<'a>(&self, _tree: &oak_core::tree::RedTree<'a, L>, _state: &mut Self::State) -> Self::Output {
        // Simplified implementation
        "".to_string()
    }
}

impl Default for GlobFormatter {
    fn default() -> Self {
        Self
    }
}
