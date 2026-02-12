use oak_core::Lexer;
use oak_erlang::{ErlangLanguage, ErlangLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_erlang_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ErlangLanguage::default()));
    let lexer = ErlangLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("erl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ErlangLanguage, _>(&lexer) {
        Ok(()) => println!("Erlang lexer tests passed!"),
        Err(e) => panic!("Erlang lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{LexerState, SourceText};

    let source = SourceText::new("hello world");
    let mut state = LexerState::<SourceText, ErlangLanguage>::new(&source);

    println!("Initial state:");
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\nAfter advancing 1 character:");
    state.advance(1);
    println!("Position: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek())
}

#[test]
fn test_erlang_module_parsing() {
    use oak_core::SourceText;
    use oak_erlang::{ErlangLanguage, ErlangLexer, ErlangSyntaxKind};

    let source = SourceText::new(
        r#"
-module(test).
-export([hello/0]).

hello() ->
    "Hello, World!".
"#,
    );

    let language = Box::leak(Box::new(ErlangLanguage::default()));
    let lexer = ErlangLexer::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::<ErlangLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);

    // Verify tokens containing module declarations
    let tokens = result.result.expect("Lexical analysis should succeed");
    assert!(!tokens.is_empty());

    println!("Parsed {} tokens:", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} at {:?}", i, token.kind, token.span)
    }

    // Check if there is an EOF token
    assert_eq!(tokens.last().unwrap().kind, ErlangSyntaxKind::Eof);

    // Verify expected token types are present
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&ErlangSyntaxKind::Minus));
    assert!(token_kinds.contains(&ErlangSyntaxKind::Atom))
}
