use oak_core::helpers::LexerTester;
use oak_rust::{RustLanguage, RustLexer};
use std::path::Path;

#[test]
fn test_rust_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    // don't use `rs` here to avoid confusion with Rust source files
    let test_runner = LexerTester::new(tests).with_extension("txt");
    match test_runner.run_tests::<RustLanguage, _>(lexer) {
        Ok(()) => println!("Rust lexer tests passed!"),
        Err(e) => panic!("Rust lexer tests failed: {}", e),
    }
}
