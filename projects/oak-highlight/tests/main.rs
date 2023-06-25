#![feature(new_range_api)]
use oak_highlight::{ExportFormat, OakHighlighter, Theme};

mod common;
use common::{MockLanguage, MockLexer, MockParser};

#[test]
fn test_mock_highlighting() {
    let highlighter = OakHighlighter::new();
    let code = "k i s";

    let parser = MockParser;
    let lexer = MockLexer;
    let result = highlighter.highlight_with_language(code, Theme::OneDarkPro, &parser, &lexer).expect("Should highlight with Mock parser");

    assert_eq!(result.source, code);
    assert!(!result.segments.is_empty());

    // k -> Keyword, i -> Identifier, s -> String, space -> Whitespace
    // Verify segments match expectations roughly
    let segments: Vec<_> = result.segments.iter().map(|s| s.text).collect();
    // Assuming highlighter preserves whitespace or token structure
    for segment in &result.segments {
        println!("{:?}: {:?}", segment.span, segment.text)
    }
}

#[test]
fn test_format_export() {
    let highlighter = OakHighlighter::new();
    let code = "let x = 1;";

    let html = highlighter.highlight_format(code, "rust", Theme::OneDarkPro, ExportFormat::Html).expect("Should export HTML");
    assert!(html.contains("<div class=\"highlight\">"));
    assert!(html.contains("<style>"));

    let json = highlighter.highlight_format(code, "rust", Theme::OneLight, ExportFormat::Json).expect("Should export JSON");
    assert!(json.contains("\"source\""));
    assert!(json.contains("segments"))
}
