use oak_pretty_print::{FormatConfig, IndentStyle, LineEnding};

#[test]
fn test_format_config_default() {
    let config = FormatConfig::default();

    assert_eq!(config.indent_style, IndentStyle::Spaces(4));
    assert_eq!(config.line_ending, LineEnding::Auto);
    assert_eq!(config.max_width, 100);
    assert!(config.insert_final_newline);
    assert!(config.trim_trailing_whitespace);
    assert!(config.preserve_blank_lines);
    assert_eq!(config.max_blank_lines, 2);
    assert!(config.format_comments);
    assert!(!config.format_strings)
}
