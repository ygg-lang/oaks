#![feature(new_range_api)]

use oak_core::helpers::LexerTester;
use oak_ini::{language::IniLanguage, lexer::IniLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_ini_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(IniLanguage::default()));
    let lexer = IniLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ini").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<IniLanguage, _>(lexer) {
        Ok(()) => println!("Ini lexer tests passed!"),
        Err(e) => panic!("Ini lexer tests failed: {}", e),
    }
}

#[test]
fn test_basic_lexing() {
    use oak_core::{Lexer, SourceText};
    use oak_ini::language::IniLanguage;

    let source = SourceText::new(r#"key = "value""#);
    let language = Box::leak(Box::new(IniLanguage::default()));
    let lexer = IniLexer::new(language);

    let result = lexer.lex(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len());
}
