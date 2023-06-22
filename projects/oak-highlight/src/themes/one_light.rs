use crate::highlighter::{HighlightStyle, HighlightTheme};
use std::{collections::HashMap, string::ToString};

impl HighlightTheme {
    /// Creates a One Light theme based on the Atom One Light color scheme.
    pub fn one_light() -> Self {
        let mut styles = HashMap::new();

        // Token Styles
        styles.insert(
            "keyword".to_string(),
            HighlightStyle {
                color: Some("#A626A4".to_string()), // Purple
                bold: true,
                ..Default::default()
            },
        );
        styles.insert(
            "name".to_string(),
            HighlightStyle {
                color: Some("#E45649".to_string()), // Red
                ..Default::default()
            },
        );
        styles.insert(
            "literal".to_string(),
            HighlightStyle {
                color: Some("#50A14F".to_string()), // Green
                ..Default::default()
            },
        );
        styles.insert(
            "escape".to_string(),
            HighlightStyle {
                color: Some("#986801".to_string()), // Orange
                ..Default::default()
            },
        );
        styles.insert(
            "operator".to_string(),
            HighlightStyle {
                color: Some("#0184BC".to_string()), // Blue
                ..Default::default()
            },
        );
        styles.insert(
            "punctuation".to_string(),
            HighlightStyle {
                color: Some("#383A42".to_string()), // Dark Gray
                ..Default::default()
            },
        );
        styles.insert(
            "comment".to_string(),
            HighlightStyle {
                color: Some("#A0A1A7".to_string()), // Light Gray
                italic: true,
                ..Default::default()
            },
        );
        styles.insert("whitespace".to_string(), HighlightStyle::default());

        // Element Styles
        styles.insert(
            "definition".to_string(),
            HighlightStyle {
                color: Some("#4078F2".to_string()), // Blue
                bold: true,
                ..Default::default()
            },
        );
        styles.insert(
            "binding".to_string(),
            HighlightStyle {
                color: Some("#E45649".to_string()), // Red
                ..Default::default()
            },
        );
        styles.insert(
            "reference".to_string(),
            HighlightStyle {
                color: Some("#383A42".to_string()), // Dark Gray
                ..Default::default()
            },
        );
        styles.insert(
            "typing".to_string(),
            HighlightStyle {
                color: Some("#986801".to_string()), // Yellow/Orange
                ..Default::default()
            },
        );
        styles.insert(
            "documentation".to_string(),
            HighlightStyle {
                color: Some("#A0A1A7".to_string()), // Light Gray
                italic: true,
                ..Default::default()
            },
        );
        styles.insert(
            "metadata".to_string(),
            HighlightStyle {
                color: Some("#A626A4".to_string()), // Purple
                ..Default::default()
            },
        );
        styles.insert(
            "attribute".to_string(),
            HighlightStyle {
                color: Some("#A626A4".to_string()), // Purple
                ..Default::default()
            },
        );
        styles.insert(
            "attribute.key".to_string(),
            HighlightStyle {
                color: Some("#986801".to_string()), // Orange
                ..Default::default()
            },
        );
        styles.insert(
            "call".to_string(),
            HighlightStyle {
                color: Some("#4078F2".to_string()), // Blue
                ..Default::default()
            },
        );
        styles.insert(
            "value".to_string(),
            HighlightStyle {
                color: Some("#50A14F".to_string()), // Green
                ..Default::default()
            },
        );

        // Common
        styles.insert(
            "error".to_string(),
            HighlightStyle {
                color: Some("#E45649".to_string()),            // Red
                background_color: Some("#FFEAEA".to_string()), // Light Red Background
                ..Default::default()
            },
        );
        styles.insert("none".to_string(), HighlightStyle::default());

        Self { name: "One Light".to_string(), styles }
    }
}
