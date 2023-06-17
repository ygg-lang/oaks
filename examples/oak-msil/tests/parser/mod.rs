use oak_core::{Lexer, SourceText};
use oak_msil::{MsilLanguage, MsilLexer, MsilParser, MsilToken};

#[test]
fn test_parser_basic() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);
    let mut parser = MsilParser::new();

    // 测试基本MSIL 代码解析
    let input = ".assembly extern mscorlib {}";
    let source = SourceText::new(input);
    let lex_result = lexer.lex(&source);

    if let Ok(tokens) = lex_result.result {
        // 转换 Token<MsilSyntaxKind> 为 MsilToken
        let msil_tokens: Vec<MsilToken> = tokens
            .into_iter()
            .map(|token| {
                let text = source.get_text_in(token.span.clone().into()).unwrap_or("").to_string();
                MsilToken::new(token.kind, token.span.into(), text)
            })
            .collect();

        let parse_result = parser.parse(msil_tokens);
        assert!(parse_result.is_ok());
    }
}

#[test]
fn test_parser_empty_tokens() {
    let mut parser = MsilParser::new();

    // 测试空 token 列表解析
    let tokens = vec![];
    let result = parser.parse(tokens);

    // 空 token 列表应该返回成功结果
    assert!(result.is_ok());
}

#[test]
fn test_parser_class_definition() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);
    let mut parser = MsilParser::new();

    // 测试类定义解
    let input = ".class public MyClass extends [mscorlib]System.Object {}";
    let source = SourceText::new(input);
    let lex_result = lexer.lex(&source);

    if let Ok(tokens) = lex_result.result {
        // 转换 Token<MsilSyntaxKind> 为 MsilToken
        let msil_tokens: Vec<MsilToken> = tokens
            .into_iter()
            .map(|token| {
                let text = source.get_text_in(token.span.clone().into()).unwrap_or("").to_string();
                MsilToken::new(token.kind, token.span.into(), text)
            })
            .collect();

        let parse_result = parser.parse(msil_tokens);
        assert!(parse_result.is_ok());
    }
}

#[test]
fn test_parser_method_definition() {
    let language = MsilLanguage::standard();
    let lexer = MsilLexer::new(&language);
    let mut parser = MsilParser::new();

    // 测试方法定义解析
    let input = ".method public static void Main() { ret }";
    let source = SourceText::new(input);
    let lex_result = lexer.lex(&source);

    if let Ok(tokens) = lex_result.result {
        // 转换 Token<MsilSyntaxKind> 为 MsilToken
        let msil_tokens: Vec<MsilToken> = tokens
            .into_iter()
            .map(|token| {
                let text = source.get_text_in(token.span.clone().into()).unwrap_or("").to_string();
                MsilToken::new(token.kind, token.span.into(), text)
            })
            .collect();

        let parse_result = parser.parse(msil_tokens);
        assert!(parse_result.is_ok());
    }
}
