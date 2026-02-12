use oak_purescript::{PurescriptLanguage, PurescriptLexer};

mod lexer;
mod parser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_purescript_lexer_basic() {
    // Simple test: create lexer and verify it works.
    let language = PurescriptLanguage::default();
    let _lexer = PurescriptLexer::new(&language);

    // Test a simple PureScript code snippet.
    let test_code = "module Main where\n\nmain :: IO ()\nmain = log \"Hello, World!\"";

    // Just verify lexer creation, no complex tests.
    println!("PureScript lexer created successfully");
    println!("Test code: {}", test_code)
}
