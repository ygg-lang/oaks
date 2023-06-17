use oak_core::helpers::LexerTester;
use oak_fsharp::{FSharpLanguage, FSharpLexer};
use std::path::Path;

#[test]
fn test_fsharp_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests = here.join("tests/lexer");
    let lexer = FSharpLexer::new(&FSharpLanguage::default());
    let test_runner = LexerTester::new(tests).with_extension("fs");
    match test_runner.run_tests::<FSharpLanguage, _>(lexer) {
        Ok(()) => println!("F# lexer tests passed!"),
        Err(e) => panic!("F# lexer tests failed: {}", e),
    }
}