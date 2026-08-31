use oak_core::{
    parser::{ParseSession, Parser as _},
    source::SourceText,
};
use oak_racket::parser::Parser;

#[test]
fn test_racket_parser() {
    let parser = Parser;
    let source = SourceText::new("(define (hello) (display \"Hello\"))\n(hello)".to_string());
    let mut session = ParseSession::new(16);
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());
    let green = output.result.unwrap();
    assert!(!green.children.is_empty());
}
