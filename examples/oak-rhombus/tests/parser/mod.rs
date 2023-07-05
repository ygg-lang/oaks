use oak_core::parser::{ParseSession, Parser};
use oak_rhombus::{RhombusLanguage, RhombusParser};

#[test]
fn test_rhombus_parser() {
    let lang = RhombusLanguage::new();
    let parser = RhombusParser::new(&lang);
    let source = "#lang rhombus\n1 + 2";
    let mut session = ParseSession::<RhombusLanguage>::new(16);
    let output = parser.parse(source, &[], &mut session);
    assert!(output.result.is_ok());
}
