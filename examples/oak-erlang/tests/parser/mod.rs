use oak_core::parser::{ParseSession, Parser};
use oak_erlang::{language::ErlangLanguage, parser::ErlangParser};

#[test]
fn test_erlang_parser() {
    let lang = ErlangLanguage::new();
    let parser = ErlangParser::new(&lang);
    let source = "-module(hello).\n-export([hello/0]).\nhello() -> ok.";
    let mut session = ParseSession::<ErlangLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
