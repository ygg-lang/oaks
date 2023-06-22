use oak_julia::{JuliaLanguage, JuliaLexer};
use oak_testing::lexing::LexerTester;
use std::time::Duration;

#[test]
fn test_julia_lexer() -> Result<(), oak_core::OakError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = JuliaLanguage::default();
    let lexer = JuliaLexer::new(&language);
    // Use `txt` instead of `json` to avoid nested tests
    let test_runner = LexerTester::new(tests).with_extension("jl").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
