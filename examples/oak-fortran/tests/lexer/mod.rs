use oak_fortran::{FortranLanguage, FortranLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_fortran_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = FortranLanguage::default();
    let lexer = FortranLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer/fixtures")).with_extension("f90").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText, parser::session::ParseSession};
    use oak_fortran::FortranLanguage;

    let source = SourceText::new("NESTED_CONSTANT");
    let _cache = ParseSession::<FortranLanguage>::default();
    let mut state = LexerState::<SourceText, FortranLanguage>::new(&source);

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
fn test_fortran_program_parsing() {
    use oak_core::{Lexer, SourceText, parser::session::ParseSession};
    use oak_fortran::{FortranLanguage, FortranLexer};

    let source = SourceText::new("program hello\n  print *, 'Hello, World!'\nend program hello");
    let language = FortranLanguage::default();
    let lexer = FortranLexer::new(&language);

    let mut cache = ParseSession::<FortranLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    println!("Testing Fortran program parsing:");
    println!("Source code: '{}'", (&source).get_text_from(0));

    let tokens = result.result.expect("Lexing should succeed");
    assert!(!tokens.is_empty(), "Should parse at least one token");

    let first_token = &tokens[0];
    let source_ref = &source;
    let token_text = source_ref.get_text_in(first_token.span.clone());

    println!("First token: Kind={:?}, Text='{}'", first_token.kind, token_text);

    println!("✅ Fortran program parsing test passed!")
}
