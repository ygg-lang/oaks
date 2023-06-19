use oak_core::helpers::LexerTester;
use oak_markdown::{MarkdownLanguage, MarkdownLexer};
use std::path::Path;

#[test]
fn test_markdown_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(MarkdownLanguage::default()));
    let lexer = MarkdownLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("markdown");
    match test_runner.run_tests::<MarkdownLanguage, _>(lexer) {
        Ok(()) => println!("Markdown lexer tests passed!"),
        Err(e) => panic!("Markdown lexer tests failed: {}", e),
    }
}
