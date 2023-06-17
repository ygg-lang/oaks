use oak_zig::{ZigLanguage, ZigLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_zig_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = ZigLexer::new(&ZigLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("zig");
    match test_runner.run_tests::<ZigLanguage, _>(lexer) {
        Ok(()) => println!("Zig lexer tests passed!"),
        Err(e) => panic!("Zig lexer tests failed: {}", e),
    }
}