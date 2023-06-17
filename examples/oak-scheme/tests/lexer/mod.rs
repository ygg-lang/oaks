use oak_scheme::{SchemeLanguage, SchemeLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_scheme_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = SchemeLexer::new(&SchemeLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("scm");
    match test_runner.run_tests::<SchemeLanguage, _>(lexer) {
        Ok(()) => println!("Scheme lexer tests passed!"),
        Err(e) => panic!("Scheme lexer tests failed: {}", e),
    }
}