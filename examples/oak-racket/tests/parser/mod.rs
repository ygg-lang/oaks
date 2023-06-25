use oak_core::parser::{ParseSession, Parser};
use oak_scheme::{language::SchemeLanguage, parser::SchemeParser};

#[test]
fn test_scheme_parser() {
    let lang = SchemeLanguage::new();
    let parser = SchemeParser::new(&lang);
    let source = "(define (hello) (display \"Hello\"))\n(hello)";
    let mut session = ParseSession::<SchemeLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.is_ok());
    let green = output.into_result().unwrap();
    assert!(green.children.len() > 0);
}
