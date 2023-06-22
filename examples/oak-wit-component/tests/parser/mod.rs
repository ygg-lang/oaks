use oak_core::{parser::Parser, source::SourceText};
use oak_wit::{WitLanguage, WitParser};

#[test]
fn test_parser_basic() {
    let input = "package example:test; world my-world {}";
    let source = SourceText::new(input);
    let language = WitLanguage {};
    let parser = WitParser::new(&language);

    let mut session = oak_core::parser::session::ParseSession::<WitLanguage>::default();
    let diagnostics = parser.parse(&source, &[], &mut session);

    assert!(diagnostics.result.is_ok());
}
