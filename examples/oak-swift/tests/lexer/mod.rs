use oak_swift::{SwiftLanguage, SwiftLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_swift_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = SwiftLexer::new(&SwiftLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("swift");
    match test_runner.run_tests::<SwiftLanguage, _>(lexer) {
        Ok(()) => println!("Swift lexer tests passed!"),
        Err(e) => panic!("Swift lexer tests failed: {}", e),
    }
}