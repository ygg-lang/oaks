use oak_core::helpers::LexerTester;
use oak_yaml::{YamlLanguage, YamlLexer};
use std::path::Path;

#[test]
fn ready() {
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("lexer");
    let tester = LexerTester::new(test_dir).with_extension("yaml");
    let lexer = YamlLexer::new(&YamlLanguage::default());
    tester.run_tests(&lexer).unwrap();
}
