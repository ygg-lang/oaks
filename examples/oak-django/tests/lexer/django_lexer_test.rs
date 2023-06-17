use oak_core::helpers::LexerTester;
use oak_django::{DjangoLanguage, DjangoLexer};
use std::path::Path;

#[test]
fn test_django_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = DjangoLexer::new(&DjangoLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("django");
    match test_runner.run_tests::<DjangoLanguage, _>(lexer) {
        Ok(()) => println!("Django lexer tests passed!"),
        Err(e) => panic!("Django lexer tests failed: {}", e),
    }
}