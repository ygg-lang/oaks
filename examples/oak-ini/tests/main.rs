use oak_ini::{IniLanguage, IniLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_ini_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(IniLanguage::default()));
    let lexer = IniLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ini").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<IniLanguage, _>(&lexer) {
        Ok(()) => println!("Ini lexer tests passed!"),
        Err(e) => panic!("Ini lexer tests failed: {}", e),
    }
}

#[test]
fn test_basic_lexing() {
    use oak_core::{Lexer, SourceText};
    use oak_ini::IniLanguage;

    let source = SourceText::new(r#"key = "value""#);
    let language = Box::leak(Box::new(IniLanguage::default()));
    let lexer = IniLexer::new(&language);

    let mut cache = oak_core::parser::session::ParseSession::<IniLanguage>::default();
    let result = lexer.lex(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());
    println!("Lexed {} tokens", tokens.len())
}
