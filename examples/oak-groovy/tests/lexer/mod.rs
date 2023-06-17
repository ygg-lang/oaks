use oak_core::helpers::LexerTester;
use oak_groovy::{GroovyLanguage, GroovyLexer};
use std::path::Path;

#[test]
fn test_groovy_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = GroovyLexer::new(&GroovyLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("groovy");
    match test_runner.run_tests::<GroovyLanguage, _>(lexer) {
        Ok(()) => println!("Groovy lexer tests passed!"),
        Err(e) => panic!("Groovy lexer tests failed: {}", e),
    }
}