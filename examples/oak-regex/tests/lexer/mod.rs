use oak_core::{Lexer, SourceText, source::Source};
use oak_regex::{RegexLanguage, RegexLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_regex_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RegexLanguage::default();
    let lexer = RegexLexer::new(&language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("regex").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};

    let source = SourceText::new(r"[a-z]+");
    let mut state = LexerState::<SourceText, RegexLanguage>::new(&source);

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
fn test_character_class_parsing() {
    let source = SourceText::new(r"[a-z]+");
    let language = RegexLanguage::default();
    let lexer = RegexLexer::new(&language);

    let mut cache = oak_core::ParseSession::<RegexLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("Testing character class parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("Lexing should succeed");
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}', Position={}..{}", first_token.kind, token_text, first_token.span.start, first_token.span.end);

    assert_eq!(token_text, "[", "First token should be left bracket");
    assert_eq!(first_token.span.start, 0, "Token should start at position 0");
    assert_eq!(first_token.span.end, 1, "Token should end at position 1");

    println!("✅ Character class parsing test passed!")
}
