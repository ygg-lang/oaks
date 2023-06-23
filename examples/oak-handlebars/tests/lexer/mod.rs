use oak_handlebars::{HandlebarsLanguage, HandlebarsLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_handlebars_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(HandlebarsLanguage::default()));
    let lexer = HandlebarsLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("hbs").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
