use oak_core::errors::OakError;
use oak_testing::lexing::LexerTester;
use oak_vue::{VueLanguage, VueLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_vue_lexer() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lexer = VueLexer::new(VueLanguage::default());
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("vue").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)?;
    Ok(())
}
