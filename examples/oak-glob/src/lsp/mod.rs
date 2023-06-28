#[cfg(feature = "lsp")]
pub mod highlighter;

#[cfg(feature = "lsp")]
pub mod formatter;

#[cfg(feature = "lsp")]
pub use highlighter::GlobHighlighter;

#[cfg(feature = "lsp")]
pub use formatter::GlobFormatter;

#[cfg(feature = "lsp")]
use oak_lsp::LanguageService;

#[cfg(feature = "lsp")]
/// Language service for glob patterns.
pub struct GlobLanguageService;

#[cfg(feature = "lsp")]
impl LanguageService for GlobLanguageService {
    type Language = crate::language::GlobLanguage;

    fn language(&self) -> Self::Language {
        Self::Language::default()
    }
}

#[cfg(feature = "lsp")]
impl Default for GlobLanguageService {
    fn default() -> Self {
        Self
    }
}
