use oak_core::parser::{ParseSession, Parser};
use oak_purescript::{language::PurescriptLanguage, parser::PurescriptParser};

#[test]
fn test_purescript_parser() {
    let lang = PurescriptLanguage::new();
    let parser = PurescriptParser::new(&lang);
    let source = "module Main where\nimport Prelude\nx = 1 + 2";
    let mut session = ParseSession::<PurescriptLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
