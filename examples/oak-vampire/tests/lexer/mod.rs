use oak_vampire::{VampireLanguage, VampireLexer};
use oak_testing::lexing::LexerTester;
use std::path::Path;
use std::time::Duration;

#[test]
fn test_vampire_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = VampireLanguage::default();
    let lexer = VampireLexer::new(&language);
    let test_runner = LexerTester::new(tests)
        .with_extension("tptp")
        .with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}