use oak_core::helpers::LexerTester;
use oak_hlsl::{HlslLanguage, HlslLexer};
use std::path::Path;

#[test]
fn test_hlsl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = HlslLexer::new(&HlslLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("hlsl");
    match test_runner.run_tests::<HlslLanguage, _>(lexer) {
        Ok(()) => println!("HLSL lexer tests passed!"),
        Err(e) => panic!("HLSL lexer tests failed: {}", e),
    }
}
