//! Predefined themes and theme loading logic.
//!
//! This module provides a set of built-in themes for syntax highlighting,
//! such as One Dark Pro and One Light.

/// One Dark Pro theme implementation.
pub mod one_dark_pro;
/// One Light theme implementation.
pub mod one_light;

use crate::highlighter::HighlightTheme;

/// Predefined theme types for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
