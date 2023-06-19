#![feature(new_range_api)]

use oak_bash::{BashLanguage, BashLexer};
use oak_core::helpers::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_bash_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(BashLanguage::default()));
    let lexer = BashLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("sh").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<BashLanguage, _>(lexer) {
        Ok(()) => println!("Bash lexer tests passed!"),
        Err(e) => panic!("Bash lexer tests failed: {}", e),
    }
}
