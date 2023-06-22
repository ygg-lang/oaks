use oak_core::{Lexer, ParseSession, Parser, source::SourceText};
use oak_wgsl::{WgslLanguage, WgslLexer, WgslParser};

#[test]
fn test_parser_basic() {
    let input = "fn main() {}";
    let source = SourceText::new(input.to_string());
    let language = WgslLanguage;
    let lexer = WgslLexer::new(&language);
    let mut session = ParseSession::<WgslLanguage>::default();
    let _lex_output = lexer.lex(&source, &[], &mut session);

    let parser = WgslParser::new(&language);
    let diagnostics = parser.parse(&source, &[], &mut session);

    assert!(diagnostics.result.is_ok());
}
