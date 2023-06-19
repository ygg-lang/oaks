use oak_core::helpers::LexerTester;
use oak_perl::{language::PerlLanguage, lexer::PerlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_perl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(PerlLanguage::default()));
    let lexer = PerlLexer::new(language);

    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("pl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PerlLanguage, _>(lexer) {
        Ok(()) => println!("Perl lexer tests passed!"),
        Err(e) => panic!("Perl lexer tests failed: {}", e),
    }
}
