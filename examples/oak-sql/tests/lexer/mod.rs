use oak_core::{Lexer, ParseSession, SourceText};
use oak_sql::{SqlLanguage, SqlLexer};

#[test]
fn test_sql_lexer_basic() {
    let source = SourceText::new("SELECT * FROM table");
    let language = SqlLanguage::default();
    let lexer = SqlLexer::new(&language);

    let mut session = ParseSession::<SqlLanguage>::new(16);
    let result = lexer.lex(&source, &[], &mut session);

    assert!(result.result.is_ok(), "词法分析应该成功");
    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty(), "应该生成至少一个 token");

    // 检查最后一个 token 是否为 Eof
    let last_token = tokens.last().unwrap();
    assert_eq!(last_token.kind, oak_sql::SqlSyntaxKind::Eof);
}
