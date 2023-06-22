use oak_core::helpers::LexerTester;
use oak_llir::{LLvmLanguage, LlvmLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_llir_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let _language = Box::leak(Box::new(LLvmLanguage::default()));
    let lexer = LlvmLexer;
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("ll").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<LLvmLanguage, _>(&lexer) {
        Ok(()) => println!("LLIR lexer tests passed!"),
        Err(e) => panic!("LLIR lexer tests failed: {}", e),
    }
}
