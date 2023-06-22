use oak_core::{Lexer, ParseSession, Parser, source::SourceText};
use oak_r::{RLanguage, RLexer, RParser, RSyntaxKind};

#[test]
fn test_basic_expression() {
    let source = SourceText::new("x <- 1 + 2");
    let language = RLanguage::default();
    let lexer = RLexer::new(&language);
    let parser = RParser::new(&language);

    // 1. Lexing & Parsing
    let mut session = ParseSession::<RLanguage>::new(128);
    let _lex_output = lexer.lex(&source, &[], &mut session);
    let parse_output = parser.parse(&source, &[], &mut session);

    match &parse_output.result {
        Ok(root) => {
            println!("Parse result: {:?}", root);
            assert_eq!(root.kind, RSyntaxKind::Root);
        }
        Err(e) => {
            panic!("Parsing failed: {:?}", e);
        }
    }
}

#[test]
fn test_control_flow() {
    let source = SourceText::new("if (x > 0) { print(x) } else { print(-x) }");
    let language = RLanguage::default();
    let lexer = RLexer::new(&language);
    let parser = RParser::new(&language);

    let mut session = ParseSession::<RLanguage>::new(128);
    let _lex_output = lexer.lex(&source, &[], &mut session);
    let parse_output = parser.parse(&source, &[], &mut session);

    match &parse_output.result {
        Ok(root) => {
            assert_eq!(root.kind, RSyntaxKind::Root);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_function_definition() {
    let source = SourceText::new("f <- function(a, b = 1) { a + b }");
    let language = RLanguage::default();
    let lexer = RLexer::new(&language);
    let parser = RParser::new(&language);

    let mut session = ParseSession::<RLanguage>::new(128);
    let _lex_output = lexer.lex(&source, &[], &mut session);
    let parse_output = parser.parse(&source, &[], &mut session);

    match &parse_output.result {
        Ok(root) => {
            assert_eq!(root.kind, RSyntaxKind::Root);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}
