use oak_ascii_doc::{AsciiDocLanguage, AsciiDocLexer};
use oak_diagnostic::testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_asciidoc_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(AsciiDocLanguage::default()));
    let lexer = AsciiDocLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("asciidoc").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<AsciiDocLanguage, _>(&lexer) {
        Ok(()) => println!("AsciiDoc lexer tests passed!"),
        Err(e) => panic!("AsciiDoc lexer tests failed: {}", e),
    }
}
