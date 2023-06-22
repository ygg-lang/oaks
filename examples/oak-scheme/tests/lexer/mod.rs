use oak_core::source::Source;
use oak_scheme::{language::SchemeLanguage, lexer::SchemeLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_scheme_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let config = SchemeLanguage::default();
    let lexer = SchemeLexer::new(&config);
    let tester = LexerTester::new(tests).with_extension("scm").with_timeout(Duration::from_secs(5));
    tester.run_tests(&lexer)
}
