use oak_core::helpers::LexerTester;
use oak_ruby::{language::RubyLanguage, lexer::RubyLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_ruby_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(RubyLanguage));
    let lexer = RubyLexer::new(language);
    // use `rb` extension for Ruby source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("rb").with_timeout(Duration::from_secs(30));
    match test_runner.run_tests::<RubyLanguage, _>(&lexer) {
        Ok(()) => println!("Ruby lexer tests passed!"),
        Err(e) => panic!("Ruby lexer tests failed: {}", e),
    }
}
