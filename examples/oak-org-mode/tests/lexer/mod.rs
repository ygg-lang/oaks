use oak_core::helpers::LexerTester;
use oak_org_mode::{language::OrgModeLanguage, lexer::OrgModeLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_org_mode_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(OrgModeLanguage::default()));
    let lexer = OrgModeLexer::new(language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("org").with_timeout(Duration::from_secs(5));

    match test_runner.run_tests::<OrgModeLanguage, _>(&lexer) {
        Ok(()) => println!("Org-mode lexer tests passed!"),
        Err(e) => panic!("Org-mode lexer tests failed: {}", e),
    }
}
