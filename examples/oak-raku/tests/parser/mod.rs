use oak_core::parser::{ParseSession, Parser};
use oak_raku::{language::RakuLanguage, parser::RakuParser};

#[test]
fn test_raku_parser() {
    let lang = RakuLanguage::new();
    let parser = RakuParser::new(lang);
    let source = "sub hello() { say \"Hello\" }\nhello();";
    let mut session = ParseSession::<RakuLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
