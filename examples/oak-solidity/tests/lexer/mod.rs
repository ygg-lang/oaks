use oak_core::{Lexer, SourceText};
use oak_solidity::{SolidityLanguage, SolidityLexer};

#[test]
fn test_solidity_lexer_simple() {
    let language = SolidityLanguage::default();
    let lexer = SolidityLexer::new(&language);
    let source = SourceText::new("contract");
    let mut cache = oak_core::ParseSession::<SolidityLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Tokens: {:?}", tokens);
}
