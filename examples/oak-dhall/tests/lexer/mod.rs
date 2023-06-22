use oak_core::helpers::LexerTester;
use oak_dhall::{DHallLanguage, DHallLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_dhall_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(DHallLanguage::default()));
    let lexer = DHallLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("dhall").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<DHallLanguage, _>(&lexer) {
        Ok(()) => println!("Dhall lexer tests passed!"),
        Err(e) => panic!("Dhall lexer tests failed: {}", e),
    }
}
