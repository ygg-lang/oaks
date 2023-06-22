use oak_core::helpers::LexerTester;
use oak_pascal::{language::PascalLanguage, lexer::PascalLexer};
use std::{path::Path, time::Duration};

static PASCAL_LANGUAGE: PascalLanguage = PascalLanguage {};

#[test]
fn test_pascal_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lexer = PascalLexer::new(&PASCAL_LANGUAGE);

    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("pas").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<PascalLanguage, _>(&lexer) {
        Ok(_) => println!("Pascal lexer tests passed"),
        Err(e) => panic!("Pascal lexer tests failed: {}", e),
    }
}
