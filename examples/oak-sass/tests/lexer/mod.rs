use oak_core::{GreenBuilder, IncrementalCache, Lexer, SourceText};
use oak_sass::{SassLanguage, lexer::SassLexer};

#[test]
fn test_sass_lexer_basic() {
    let source = SourceText::new("a");
    let language = SassLanguage::default();
    let lexer = SassLexer::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok(), "词法分析应该成功");
    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty(), "应该生成至少一个 token");

    // 检查最后一个 token 是否为 Eof
    let last_token = tokens.last().unwrap();
    assert_eq!(last_token.kind, oak_sass::SassSyntaxKind::Eof);
}
