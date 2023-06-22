use oak_core::errors::OakError;
use oak_delphi::{DelphiLanguage, DelphiLexer};
use oak_testing::lexing::LexerTester;
use std::{path::Path, time::Duration};

#[test]
fn test_delphi_lexer() -> Result<(), OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = DelphiLanguage::default();
    let lexer = DelphiLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("pas").with_timeout(Duration::from_secs(5));
    test_runner.run_tests::<DelphiLanguage, _>(&lexer)?;
    Ok(())
}
