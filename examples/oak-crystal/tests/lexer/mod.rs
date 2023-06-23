use oak_core::{helpers::LexerTester, source::Source};
use oak_crystal::{CrystalLanguage, CrystalLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_crystal_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CrystalLanguage::new()));
    let lexer = CrystalLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("cr").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CrystalLanguage, _>(&lexer) {
        Ok(()) => println!("Crystal lexer tests passed!"),
        Err(e) => panic!("Crystal lexer tests failed: {}", e),
    }
}
