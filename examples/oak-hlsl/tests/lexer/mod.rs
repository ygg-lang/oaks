use oak_core::helpers::LexerTester;
use oak_hlsl::{HlslLanguage, HlslLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_hlsl_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let language = Box::leak(Box::new(HlslLanguage::default()));
    let lexer = HlslLexer::new(language);
    let test_runner = LexerTester::new(tests).with_extension("hlsl").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<HlslLanguage, _>(&lexer) {
        Ok(()) => println!("HLSL lexer tests passed!"),
        Err(e) => panic!("HLSL lexer tests failed: {}", e),
    }
}
