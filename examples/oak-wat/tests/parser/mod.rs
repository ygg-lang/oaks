use oak_core::{Lexer, ParseSession, Parser, source::SourceText};
use oak_wat::{WatLanguage, WatLexer, WatParser};

#[test]
fn test_parser_basic() {
    let input = "(module)";
    let source = SourceText::new(input.to_string());
    let language = WatLanguage::default();
    let lexer = WatLexer::new(&language);
    let mut session = ParseSession::<WatLanguage>::default();
    let _lex_output = lexer.lex(&source, &[], &mut session);

    let parser = WatParser::new(&language);
    let parse_output = parser.parse(&source, &[], &mut session);

    assert!(parse_output.result.is_ok())
}
