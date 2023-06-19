use oak_core::helpers::LexerTester;
use oak_julia::{JuliaLanguage, JuliaLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_json_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(JuliaLanguage::default()));
    let lexer = JuliaLexer::new(language);
    // Use `txt` instead of `json` to avoid nested tests
    let test_runner = LexerTester::new(tests).with_extension("jl").with_timeout(Duration::from_secs(30));
    match test_runner.run_tests::<JuliaLanguage, _>(lexer) {
        Ok(()) => println!("Julia lexer tests passed!"),
        Err(e) => panic!("Julia lexer tests failed: {}", e),
    }
}
