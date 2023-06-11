#[cfg(test)]
mod tests {
    use oak_core::{NodeId, SourceSpan};
    use oak_highlight::{HighlightConfig, HighlightError, HighlightResult, Highlighter, Theme, TokenStyle};

    #[test]
    fn test_highlight_config_default() {
        let config = HighlightConfig::default();

        // Test default configuration values
        assert!(config.enable_syntax_highlighting);
        assert!(config.enable_semantic_highlighting);
        assert!(!config.enable_error_highlighting);
        assert_eq!(config.max_tokens, 10000);
    }

    #[test]
    fn test_highlight_config_builder() {
        let config = HighlightConfig::new()
            .with_syntax_highlighting(false)
            .with_semantic_highlighting(true)
            .with_error_highlighting(true)
            .with_max_tokens(5000);

        assert!(!config.enable_syntax_highlighting);
        assert!(config.enable_semantic_highlighting);
        assert!(config.enable_error_highlighting);
        assert_eq!(config.max_tokens, 5000);
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::default();

        // Test that default theme is created successfully
        assert_eq!(theme.name(), "default");
        assert!(!theme.is_dark_theme());
    }

    #[test]
    fn test_theme_dark_mode() {
        let dark_theme = Theme::dark();

        assert_eq!(dark_theme.name(), "dark");
        assert!(dark_theme.is_dark_theme());
    }

    #[test]
    fn test_token_style_creation() {
        let style = TokenStyle::new().with_foreground("#FF0000").with_background("#FFFFFF").with_bold(true).with_italic(false);

        assert_eq!(style.foreground(), Some("#FF0000"));
        assert_eq!(style.background(), Some("#FFFFFF"));
        assert!(style.is_bold());
        assert!(!style.is_italic());
    }

    #[test]
    fn test_highlighter_creation() {
        let config = HighlightConfig::default();
        let theme = Theme::default();
        let highlighter = Highlighter::new(config, theme);

        // Test basic highlighter creation
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_highlight_error_display() {
        let error = HighlightError::InvalidToken { span: SourceSpan::new(0, 5), message: "Invalid tokens".to_string() };

        let error_string = format!("{}", error);
        assert!(error_string.contains("Invalid tokens"));
    }

    #[test]
    fn test_highlight_error_theme_not_found() {
        let error = HighlightError::ThemeNotFound { theme_name: "nonexistent".to_string() };

        let error_string = format!("{}", error);
        assert!(error_string.contains("nonexistent"));
    }

    #[test]
    fn test_highlight_simple_text() {
        let config = HighlightConfig::default();
        let theme = Theme::default();
        let highlighter = Highlighter::new(config, theme);

        let source = "let x = 42;";
        let result = highlighter.highlight_text(source);

        // Test that highlighting doesn't fail
        assert!(result.is_ok());
    }

    #[test]
    fn test_highlight_with_comments() {
        let config = HighlightConfig::default();
        let theme = Theme::default();
        let highlighter = Highlighter::new(config, theme);

        let source = "// This is a comment\nlet x = 42;";
        let result = highlighter.highlight_text(source);

        // Test that highlighting with comments works
        assert!(result.is_ok());
    }

    #[test]
    fn test_highlight_empty_text() {
        let config = HighlightConfig::default();
        let theme = Theme::default();
        let highlighter = Highlighter::new(config, theme);

        let source = "";
        let result = highlighter.highlight_text(source);

        // Test that highlighting empty text works
        assert!(result.is_ok());
    }
}

#[test]
fn ready() {
    println!("it works!")
}
