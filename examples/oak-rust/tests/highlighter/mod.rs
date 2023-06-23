use oak_rust::{Highlighter, RustHighlighter};

#[test]
fn test_rust_highlighter() -> Result<(), oak_core::OakError> {
    let highlighter = RustHighlighter::new();
    let source = r#"
fn main() {
    let x = 42; // This is a comment
    println!("Hello, {}", x)
}
"#;

    let highlights = highlighter.highlight(source);
    assert!(!highlights.is_empty(), "Highlighter should produce highlights");

    println!("Rust highlighter test passed - {} highlights generated", highlights.len());
    Ok(())
}

#[test]
fn test_rust_highlighter_with_parser() -> Result<(), oak_core::OakError> {
    let highlighter = RustHighlighter::new();
    let source = r#"
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
"#;

    let highlights = highlighter.highlight(source);
    assert!(!highlights.is_empty(), "Should highlight complex code");

    println!("Rust highlighter with files test passed - {} highlights generated", highlights.len());
    Ok(())
}
