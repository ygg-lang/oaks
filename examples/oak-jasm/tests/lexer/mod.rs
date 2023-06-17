use oak_core::helpers::LexerTester;
use oak_jasm::{JasmLanguage, JasmLexer};
use std::path::Path;

#[test]
fn test_jasm_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = JasmLexer::new(&JasmLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("jasm");
    match test_runner.run_tests::<JasmLanguage, _>(lexer) {
        Ok(()) => println!("JASM lexer tests passed!"),
        Err(e) => panic!("JASM lexer tests failed: {}", e),
    }
}
