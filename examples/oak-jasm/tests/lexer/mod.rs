use oak_core::helpers::LexerTester;
use oak_jasm::{JasmLanguage, JasmLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_jasm_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(JasmLanguage::default()));
    let lexer = JasmLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("jasm").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<JasmLanguage, _>(&lexer) {
        Ok(()) => println!("JASM lexer tests passed!"),
        Err(e) => panic!("JASM lexer tests failed: {}", e),
    }
}
