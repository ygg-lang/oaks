use oak_core::{Builder, SourceText, errors::OakErrorKind};
use oak_json::{language::JsonLanguage, parser::JsonParser};

#[test]
fn parse_trailing_comma_error_location_standard() {
    let source = SourceText::new("{\"a\":1,}");
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let out = parser.parse(&source);
    // find the error anchored at '}' position
    let loc = out
        .diagnostics
        .iter()
        .find_map(|e| match e.kind() {
            OakErrorKind::SyntaxError { message, source } if message.contains("Trailing comma not allowed") => {
                Some(source.clone())
            }
            _ => None,
        })
        .expect("expected trailing comma kind error at '}'");
    assert_eq!((loc.line, loc.column), (1, 7));
}

#[test]
fn parse_unexpected_token_on_bare_key() {
    let source = SourceText::new("{a:1}");
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let out = parser.parse(&source);
    assert!(
        out.diagnostics
            .iter()
            .any(|e| matches!(e.kind(), OakErrorKind::UnexpectedCharacter { .. } | OakErrorKind::SyntaxError { .. })),
        "expected error for bare key"
    );
    let loc = out
        .diagnostics
        .iter()
        .find_map(|e| match e.kind() {
            OakErrorKind::UnexpectedCharacter { source, .. } => Some(source.clone()),
            OakErrorKind::SyntaxError { source, .. } => Some(source.clone()),
            _ => None,
        })
        .unwrap();
    assert_eq!((loc.line, loc.column), (1, 1)); // points to 'a'
}

#[test]
fn parse_unexpected_eof_in_object() {
    let source = SourceText::new("{\"a\":");
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let out = parser.parse(&source);
    assert!(
        out.diagnostics.iter().any(
            |e| matches!(e.kind(), OakErrorKind::SyntaxError { message, .. } if message.contains("Unexpected end of file"))
        ),
        "expected Unexpected end of file"
    );
}
