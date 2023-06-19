use oak_core::{Lexer, SourceText};
use oak_solidity::{SolidityLanguage, SolidityLexer};

#[test]
fn test_solidity_lexer_simple() {
    let config = SolidityLanguage::default();
    let lexer = SolidityLexer::new(&config);
    let source = SourceText::new("contract");

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Tokens: {:?}", tokens);
}
