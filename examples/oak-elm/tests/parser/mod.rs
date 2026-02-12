use oak_core::parser::{ParseSession, Parser};
use oak_elm::{language::ElmLanguage, parser::ElmParser};

#[test]
fn test_elm_parser() {
    let lang = ElmLanguage::new();
    let parser = ElmParser::new(&lang);
    let source = "module Main exposing (..)\nimport Html\nmain = Html.text \"Hello\"";
    let mut session = ParseSession::<ElmLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
