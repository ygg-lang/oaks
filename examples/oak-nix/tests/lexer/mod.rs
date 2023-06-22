use oak_core::helpers::LexerTester;
use oak_nix::{NixLanguage, NixLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_nix_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(NixLanguage::default()));
    let lexer = NixLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("nix").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<NixLanguage, _>(&lexer) {
        Ok(()) => println!("Nix lexer tests passed!"),
        Err(e) => panic!("Nix lexer tests failed: {}", e),
    }
}
