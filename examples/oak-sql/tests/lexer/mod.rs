use oak_core::{GreenBuilder, IncrementalCache, Lexer, SourceText};
use oak_sql::{SqlLanguage, SqlLexer};

#[test]
fn test_sql_lexer_basic() {
    let source = SourceText::new("SELECT * FROM table");
    let language = SqlLanguage::default();
    let lexer = SqlLexer::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok(), "词法分析应该成功");
    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty(), "应该生成至少一个 token");

    // 检查最后一个 token 是否为 Eof
    let last_token = tokens.last().unwrap();
    assert_eq!(last_token.kind, oak_sql::SqlSyntaxKind::Eof);
}
