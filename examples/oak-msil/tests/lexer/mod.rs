use oak_core::helpers::LexerTester;
use oak_msil::{MsilLanguage, MsilLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_msil_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(MsilLanguage::default()));
    let lexer = MsilLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("il").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<MsilLanguage, _>(lexer) {
        Ok(()) => println!("Msil lexer tests passed!"),
        Err(e) => panic!("Msil lexer tests failed: {}", e),
    }
}
