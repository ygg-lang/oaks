use oak_core::parser::{ParseSession, Parser};
use oak_racket::{language::RacketLanguage, parser::RacketParser};

#[test]
fn test_racket_parser() {
    let lang = RacketLanguage::new();
    let parser = RacketParser::new(&lang);
    let source = "(define (hello) (display \"Hello\"))\n(hello)";
    let mut session = ParseSession::<RacketLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.result.is_ok());
    let green = output.result.unwrap();
    assert!(green.children.len() > 0);
}
