use oak_core::{helpers::LexerTester, source::Source};
use oak_objective_c::{ObjectiveCLanguage, ObjectiveCLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_objective_c_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(ObjectiveCLanguage::default()));
    let lexer = ObjectiveCLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("m").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<ObjectiveCLanguage, _>(lexer) {
        Ok(()) => println!("Objective-C lexer tests passed!"),
        Err(e) => panic!("Objective-C lexer tests failed: {}", e),
    }
}
