use oak_core::{Lexer, LexerCache, lexer::session::LexSession, source::SourceText};
use oak_llvm_ir::{LLvmLanguage, LLvmLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_lexer_basic() {
    let language = &LLvmLanguage::default();
    let source = &SourceText::new(" %1 = add i32 %0, 1 ; comment\n @global = global i32 42 ".to_string());
    let lexer = LLvmLexer::new(language);
    let mut cache = LexSession::<LLvmLanguage>::default();
    let _output = lexer.lex(source, &[], &mut cache);
}

#[test]
fn test_llir_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(LLvmLanguage::default()));
    let lexer = LLvmLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ll").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<LLvmLanguage, _>(&lexer) {
        Ok(()) => println!("LLIR lexer tests passed!"),
        Err(e) => panic!("LLIR lexer tests failed: {}", e),
    }
}
