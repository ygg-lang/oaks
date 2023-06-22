use oak_core::{Lexer, Parser, source::SourceText};
use oak_wolfram::{WolframLanguage, WolframLexer, WolframParser};

#[test]
fn test_parser_basic() {
    let input = "f[x, {1, 2, 3}]";
    let source = SourceText::new(input.to_string());
    let language = WolframLanguage::default();
    let lexer = WolframLexer::new(&language);

    let mut cache = oak_core::ParseSession::<WolframLanguage>::default();
    let _lex_output = lexer.lex(&source, &[], &mut cache);

    let parser = WolframParser::new(&language);
    let diagnostics = parser.parse(&source, &[], &mut cache);

    assert!(diagnostics.result.is_ok());
}
