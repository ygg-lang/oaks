use oak_pretty_print::{IndentStyle, LineEnding, PrinterConfig};

#[test]
fn test_printer_config_default() {
    let config = PrinterConfig::default();

    assert_eq!(config.indent_style, IndentStyle::Spaces(4));
    assert_eq!(config.line_ending, LineEnding::Auto);
    assert_eq!(config.max_width, 100);
    assert!(config.insert_final_newline);
    assert!(config.trim_trailing_whitespace);
    assert_eq!(config.indent_size, 4);
}
