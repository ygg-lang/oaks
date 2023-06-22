use oak_core::helpers::ParserTester;
use oak_ruby::{language::RubyLanguage, parser::RubyParser};
use std::path::Path;
use std::time::Duration;

#[test]
fn test_ruby_parser() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = RubyLanguage;
    let parser = RubyParser::new(&language);
    let lexer = oak_ruby::lexer::RubyLexer::new(&language);
    // use `rb` extension for Ruby source files
    let test_runner = ParserTester::new(here.join("tests/parser")).with_extension("rb")
        .with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<RubyLanguage, _>(parser) {
        Ok(()) => println!("Ruby files tests passed!"),
        Err(e) => panic!("Ruby files tests failed: {}", e),
    }
}