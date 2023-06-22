#![feature(new_range_api)]

mod builder;
mod lexer;
mod parser;

#[test]
fn ready() {
    println!("Valkyrie tests are ready!");
}

#[test]
fn test_valkyrie_integration() {
    use oak_core::SourceText;
    use oak_valkyrie::{ValkyrieLanguage, ValkyrieLexer, ValkyrieParser};

    let language = ValkyrieLanguage::default();
    let lexer = ValkyrieLexer::new(&language);
    let _parser = ValkyrieParser::new(&language);

    // Test basic integration
    let source = SourceText::new("namespace Test { micro main() { let x = 42; } }");

    // Test lexer
    let tokens: Vec<_> = lexer.tokenize(&source).collect();
    assert!(!tokens.is_empty(), "Lexer should produce tokens");

    println!("Valkyrie integration test passed - {} tokens generated", tokens.len());
}
