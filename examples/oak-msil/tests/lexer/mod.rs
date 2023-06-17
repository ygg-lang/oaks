use oak_core::{Lexer, SourceText};
use oak_msil::{MsilLanguage, MsilLexer};

#[test]
fn test_lexer_basic() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);

    // 测试基本MSIL 代码词法分析
    let input = ".assembly extern mscorlib {}";
    let source = SourceText::new(input);
    let result = lexer.lex(&source);

    assert!(result.result.is_ok());
    assert!(!result.result.unwrap().is_empty());
}

#[test]
fn test_lexer_keywords() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);

    // 测试关键字识
    let input = ".class .method .field";
    let source = SourceText::new(input);
    let result = lexer.lex(&source);

    assert!(result.result.is_ok());
    assert!(!result.result.unwrap().is_empty());
}

#[test]
fn test_lexer_identifiers() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);

    // 测试标识符识
    let input = "MyClass MyMethod MyField";
    let source = SourceText::new(input);
    let result = lexer.lex(&source);

    assert!(result.result.is_ok());
    assert!(!result.result.unwrap().is_empty());
}

#[test]
fn test_lexer_empty_input() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);

    // 测试空输
    let input = "";
    let source = SourceText::new(input);
    let result = lexer.lex(&source);

    // 空输入应该至少有一个 EOF token
    assert!(result.result.is_ok());
    assert!(!result.result.unwrap().is_empty());
}

#[test]
fn test_lexer_whitespace() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);

    // 测试空白字符处理
    let input = "   \t\n  ";
    let source = SourceText::new(input);
    let result = lexer.lex(&source);

    assert!(result.result.is_ok());
    assert!(!result.result.unwrap().is_empty());
}
