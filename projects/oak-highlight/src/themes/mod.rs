pub mod one_dark_pro;
pub mod one_light;

use crate::highlighter::HighlightTheme;

/// Predefined theme types for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    /// One Dark Pro theme (Atom inspired)
    OneDarkPro,
    /// One Light theme (Atom inspired)
    OneLight,
    /// Default theme
    Default,
}

impl Theme {
    /// Returns the corresponding [HighlightTheme] configuration for this theme.
    pub fn get_theme(self) -> HighlightTheme {
        match self {
            Theme::OneDarkPro => HighlightTheme::one_dark_pro(),
            Theme::OneLight => HighlightTheme::one_light(),
            Theme::Default => HighlightTheme::default(),
        }
    }
}
