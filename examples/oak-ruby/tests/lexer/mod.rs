use oak_ruby::{language::RubyLanguage, lexer::RubyLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_ruby_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RubyLanguage;
    let lexer = RubyLexer::new(&language);
    // use `rb` extension for Ruby source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("rb").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
