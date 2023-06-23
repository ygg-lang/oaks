use oak_json::{JsonLanguage, JsonLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
#[cfg(feature = "serde")]
fn test_json_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = JsonLanguage::default();
    let lexer = JsonLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("json").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
