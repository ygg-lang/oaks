use oak_core::{
    Lexer,
    helpers::LexerTester,
    source::{Source, SourceText},
};
use oak_javascript::{language::JavaScriptLanguage, lexer::JavaScriptLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_javascript_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(JavaScriptLanguage::standard()));
    let lexer = JavaScriptLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("js").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<JavaScriptLanguage, _>(lexer) {
        Ok(()) => println!("JavaScript lexer tests passed!"),
        Err(e) => panic!("JavaScript lexer tests failed: {}", e),
    }
}

#[test]
fn debug_test() {
    println!("Starting debug test...");

    let language = JavaScriptLanguage::standard();
    let lexer = JavaScriptLexer::new(&language);
    let source = SourceText::new("x");

    println!("About to lex source...");
    println!("Source length: {}", (&source).length());
    println!("Source content: '{}'", (&source).get_text_in((0..(&source).length()).into()));

    let result = lexer.lex(&source);
    println!("Lexing completed.");

    match result.result {
        Ok(tokens) => {
            println!("Token count: {}", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                let source_ref = &source;
                let token_text = source_ref.get_text_in(token.span.clone());
                println!("Token {}: kind={:?}, span={:?}, text='{}'", i, token.kind, token.span, token_text);
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    println!("Diagnostic count: {}", result.diagnostics.len());
    for (i, diagnostic) in result.diagnostics.iter().enumerate() {
        println!("Diagnostic {}: {:?}", i, diagnostic);
    }
}
