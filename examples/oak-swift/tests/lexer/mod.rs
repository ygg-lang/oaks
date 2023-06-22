use oak_core::helpers::LexerTester;
use oak_swift::{lexer::SwiftLexer, language::SwiftLanguage};

use std::{path::Path, time::Duration};

#[test]
fn test_swift_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config = SwiftLanguage::default();
    let lexer = SwiftLexer::new(&config);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("swift").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<SwiftLanguage, _>(&lexer) {
        Ok(()) => println!("Swift lexer tests passed!"),
        Err(e) => panic!("Swift lexer tests failed: {}", e),
    }
}