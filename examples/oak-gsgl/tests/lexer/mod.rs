use oak_core::helpers::LexerTester;
use oak_gsgl::{GsglLanguage, GsglLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_gsgl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = GsglLexer::new();
    let test_runner = LexerTester::new(tests).with_extension("gsgl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<GsglLanguage, _>(lexer) {
        Ok(()) => println!("GSGL lexer tests passed!"),
        Err(e) => panic!("GSGL lexer tests failed: {}", e),
    }
}
