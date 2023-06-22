use oak_core::{Lexer, SourceText, parser::session::ParseSession};
use oak_matlab::{MatlabLanguage, MatlabLexer, MatlabSyntaxKind};

#[test]
fn test_basic_identifier() {
    let language = MatlabLanguage::default();
    let lexer = MatlabLexer::new(&language);
    let source = SourceText::new("x");
    let mut cache = ParseSession::default();

    let result = lexer.lex(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(tokens.len() >= 1); // at least one kind
    // Just check that we got some tokens without infinite loop
}

#[test]
fn test_keywords() {
    let language = MatlabLanguage::default();
    let lexer = MatlabLexer::new(&language);
    let source = SourceText::new("function end if else while for");
    let mut cache = ParseSession::default();

    let result = lexer.lex(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查关键字
    assert_eq!(tokens[0].kind, MatlabSyntaxKind::Function);
    assert_eq!(tokens[2].kind, MatlabSyntaxKind::End); // 跳过空白
    assert_eq!(tokens[4].kind, MatlabSyntaxKind::If);
    assert_eq!(tokens[6].kind, MatlabSyntaxKind::Else);
    assert_eq!(tokens[8].kind, MatlabSyntaxKind::While);
    assert_eq!(tokens[10].kind, MatlabSyntaxKind::For);
}
