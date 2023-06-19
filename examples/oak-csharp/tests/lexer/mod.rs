#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_csharp::{CSharpLanguage, CSharpLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_csharp_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(CSharpLanguage::default()));
    let lexer = CSharpLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("cs").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<CSharpLanguage, _>(lexer) {
        Ok(()) => println!("C# lexer tests passed!"),
        Err(e) => panic!("C# lexer tests failed: {}", e),
    }
}