use oak_ascii_doc::{AsciiDocLanguage, AsciiDocLexer};
use oak_core::helpers::LexerTester;
use std::path::Path;

#[test]
fn test_asciidoc_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = AsciiDocLexer::new(&AsciiDocLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("asciidoc");
    match test_runner.run_tests::<AsciiDocLanguage, _>(lexer) {
        Ok(()) => println!("AsciiDoc lexer tests passed!"),
        Err(e) => panic!("AsciiDoc lexer tests failed: {}", e),
    }
}
