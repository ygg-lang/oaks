use oak_core::Source;
use oak_rust::{RustLanguage, RustLexer, lexer::RustTokenType};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_rust_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("rust").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};
    use oak_rust::RustLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let mut state = LexerState::<SourceText, RustLanguage>::new(&source);

    println!("Initial state:");
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\nAfter advancing 1 char:");
    state.advance(1);
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\nAfter advancing 1 char:");
    state.advance(1);
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek())
}

#[test]
fn test_nested_constant_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let source = SourceText::new("NESTED_CONSTANT");
    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("Testing NESTED_CONSTANT parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("Lexing should succeed");
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}', Position={}..{}", first_token.kind, token_text, first_token.span.start, first_token.span.end);

    // Verify identifier type
    assert!(matches!(first_token.kind, RustTokenType::Identifier), "Should be Identifier type");
    assert_eq!(token_text, "NESTED_CONSTANT", "Identifier should be parsed as NESTED_CONSTANT");
    assert_eq!(first_token.span.start, 0, "Token should start at position 0");
    assert_eq!(first_token.span.end, 15, "Token should end at position 15");

    println!("✅ NESTED_CONSTANT parsing test passed!")
}

#[test]
fn test_rust_lexer_keywords() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new("fn let mut pub struct enum trait impl use");
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    // Should have tokens for each keyword plus whitespace
    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(tokens.len() >= 8, "Should tokenize all keywords");

    println!("Rust lexer keywords test passed - {} tokens generated", tokens.len())
}

#[test]
fn test_rust_lexer_literals() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new(r#"42 3.14 "hello" 'a' true false"#);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty(), "Should tokenize literals");

    println!("Rust lexer literals test passed - {} tokens generated", tokens.len())
}

#[test]
fn test_rust_lexer_comments() {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);

    let source = SourceText::new(
        r#"
// Line comment
/* Block comment */
/// Doc comment
//! Module doc
"#,
    );
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);

    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty(), "Should tokenize comments");

    println!("Rust lexer comments test passed - {} tokens generated", tokens.len())
}
