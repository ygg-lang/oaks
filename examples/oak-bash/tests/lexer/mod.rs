use oak_bash::{BashLanguage, BashLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_bash_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = BashLexer::new(&BashLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("sh");
    match test_runner.run_tests::<BashLanguage, _>(lexer) {
        Ok(()) => println!("Bash lexer tests passed!"),
        Err(e) => panic!("Bash lexer tests failed: {}", e),
    }
}
