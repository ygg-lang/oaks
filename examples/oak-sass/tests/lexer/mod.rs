use oak_core::{Lexer, ParseSession, SourceText};
use oak_sass::{SassLanguage, lexer::SassLexer};

#[test]
fn test_sass_lexer_basic() {
    let source = SourceText::new("a");
    let language = SassLanguage::default();
    let lexer = SassLexer::new(&language);

    let mut session = ParseSession::<SassLanguage>::new(16);
    let result = lexer.lex(&source, &[], &mut session);

    assert!(result.result.is_ok(), "词法分析应该成功");
    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty(), "应该生成至少一个 token");

    // 检查最后一个 token 是否为 Eof
    let last_token = tokens.last().unwrap();
    assert_eq!(last_token.kind, oak_sass::SassSyntaxKind::Eof);
}
