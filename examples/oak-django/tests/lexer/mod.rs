use oak_django::{DjangoLanguage, DjangoLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_django_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DjangoLanguage::default()));
    let lexer = DjangoLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("django").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DjangoLanguage, _>(&lexer) {
        Ok(()) => println!("Django lexer tests passed!"),
        Err(e) => panic!("Django lexer tests failed: {}", e),
    }
}
