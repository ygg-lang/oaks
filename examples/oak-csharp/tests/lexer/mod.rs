use oak_core::helpers::LexerTester;
use oak_csharp::{CSharpLanguage, CSharpLexer};
use std::path::Path;

#[test]
fn test_csharp_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = CSharpLexer::new(&CSharpLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("cs");
    match test_runner.run_tests::<CSharpLanguage, _>(lexer) {
        Ok(()) => println!("C# lexer tests passed!"),
        Err(e) => panic!("C# lexer tests failed: {}", e),
    }
}