use oak_highlight::{ExportFormat, OakHighlighter, Theme};

#[test]
fn test_rust_highlighting() {
    let highlighter = OakHighlighter::new();
    let code = r#"
        fn main() {
            let x = 1;
            println!("Hello, {}!", x)
        }
    "#;

    let language = oak_rust::RustLanguage::default();
    let parser = oak_rust::parser::RustParser::new(&language);
    let lexer = oak_rust::lexer::RustLexer::new(&language);
    let result = highlighter.highlight_with_language(code, Theme::OneDarkPro, &parser, &lexer).expect("Should highlight with Rust parser");

    assert_eq!(result.source, code);
    assert!(!result.segments.is_empty());

    // Print segments for manual verification if needed
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
