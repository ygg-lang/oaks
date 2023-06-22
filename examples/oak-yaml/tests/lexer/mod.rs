use oak_testing::lexing::LexerTester;
use oak_yaml::{YamlLanguage, YamlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_yaml_lexer() -> Result<(), oak_core::OakError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = YamlLanguage::default();
    let lexer = YamlLexer::new(&language);
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("yaml").with_timeout(Duration::from_secs(5));
    test_runner.run_tests(&lexer)
}
