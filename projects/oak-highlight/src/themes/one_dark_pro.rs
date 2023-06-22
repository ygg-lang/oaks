use crate::highlighter::{HighlightStyle, HighlightTheme};
use std::{collections::HashMap, string::ToString};

impl HighlightTheme {
    /// Creates a One Dark Pro theme based on the Atom One Dark Pro color scheme.
    pub fn one_dark_pro() -> Self {
        let mut styles = HashMap::new();

        // Token Styles
        styles.insert(
            "keyword".to_string(),
            HighlightStyle {
                color: Some("#C678DD".to_string()), // Purple
                bold: true,
                ..Default::default()
            },
        );
        styles.insert(
            "name".to_string(),
            HighlightStyle {
                color: Some("#E06C75".to_string()), // Red
                ..Default::default()
            },
        );
        styles.insert(
            "literal".to_string(),
            HighlightStyle {
                color: Some("#98C379".to_string()), // Green
                ..Default::default()
            },
        );
        styles.insert(
            "escape".to_string(),
            HighlightStyle {
                color: Some("#D19A66".to_string()), // Orange
                ..Default::default()
            },
        );
        styles.insert(
            "operator".to_string(),
            HighlightStyle {
                color: Some("#56B6C2".to_string()), // Cyan
                ..Default::default()
            },
        );
        styles.insert(
            "punctuation".to_string(),
            HighlightStyle {
                color: Some("#ABB2BF".to_string()), // Light Gray
                ..Default::default()
            },
        );
        styles.insert(
            "comment".to_string(),
            HighlightStyle {
                color: Some("#5C6370".to_string()), // Gray
                italic: true,
                ..Default::default()
            },
        );
        styles.insert("whitespace".to_string(), HighlightStyle::default());

        // Element Styles
        styles.insert(
            "definition".to_string(),
            HighlightStyle {
                color: Some("#61AFEF".to_string()), // Blue
                bold: true,
                ..Default::default()
            },
        );
        styles.insert(
            "binding".to_string(),
            HighlightStyle {
                color: Some("#E06C75".to_string()), // Red
                ..Default::default()
            },
        );
        styles.insert(
            "reference".to_string(),
            HighlightStyle {
                color: Some("#ABB2BF".to_string()), // Light Gray
                ..Default::default()
            },
        );
        styles.insert(
            "typing".to_string(),
            HighlightStyle {
                color: Some("#E5C07B".to_string()), // Yellow
                ..Default::default()
            },
        );
        styles.insert(
            "documentation".to_string(),
            HighlightStyle {
                color: Some("#5C6370".to_string()), // Gray
                italic: true,
                ..Default::default()
            },
        );
        styles.insert(
            "metadata".to_string(),
            HighlightStyle {
                color: Some("#C678DD".to_string()), // Purple
                ..Default::default()
            },
        );
        styles.insert(
            "attribute".to_string(),
            HighlightStyle {
                color: Some("#C678DD".to_string()), // Purple
                ..Default::default()
            },
        );
        styles.insert(
            "attribute.key".to_string(),
            HighlightStyle {
                color: Some("#D19A66".to_string()), // Orange
                ..Default::default()
            },
        );
        styles.insert(
            "call".to_string(),
            HighlightStyle {
                color: Some("#61AFEF".to_string()), // Blue
                ..Default::default()
            },
        );
        styles.insert(
            "value".to_string(),
            HighlightStyle {
                color: Some("#98C379".to_string()), // Green
                ..Default::default()
            },
        );

        // Common
        styles.insert(
            "error".to_string(),
            HighlightStyle {
                color: Some("#E06C75".to_string()),            // Red
                background_color: Some("#3E2723".to_string()), // Dark Red Background
                ..Default::default()
            },
        );
        styles.insert("none".to_string(), HighlightStyle::default());

        Self { name: "One Dark Pro".to_string(), styles }
    }
}
