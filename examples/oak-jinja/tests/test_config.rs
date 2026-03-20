use oak_core::{
    lexer::{Lexer, LexerSession},
    parser::{ParseSession, Parser},
    source::SourceText,
};
use oak_jinja::{JinjaLanguage, JinjaLexer, JinjaParser};

fn main() {
    println!("=== 测试 Jinja 配置机制 ===\n");

    // 1. 测试默认配置
    println!("1. 测试默认 Jinja 配置:");
    test_default_config();

    println!("\n2. 测试自定义分隔符配置:");
    test_custom_delimiters();
}

fn test_default_config() {
    let source = SourceText::new("Hello {{ name }}! {% if is_admin %}Admin{% endif %}");
    let language = JinjaLanguage::default();

    println!("  语言配置: {:?}", language);
    println!("  源文本: {}", source);

    // 测试词法分析
    let lexer = JinjaLexer::new(&language);
    let mut lex_cache = LexerSession::default();
    let lex_result = lexer.lex(&source, &[], &mut lex_cache);

    println!("  Token 数量: {}", lex_result.tokens.len());
    for (i, token) in lex_result.tokens.iter().enumerate() {
        let text = source.get_text_in(token.span);
        println!("    Token {}: {:?} - '{}'", i, token.kind, text);
    }

    // 测试语法分析
    let parser = JinjaParser::new(&language);
    let mut parse_cache = ParseSession::default();
    let parse_result = parser.parse(&source, &[], &mut parse_cache);

    if parse_result.diagnostics.is_empty() {
        println!("  ✓ 解析成功!");
    }
    else {
        println!("  ✗ 解析有错误:");
        for diag in &parse_result.diagnostics {
            println!("    - {:?}", diag);
        }
    }
}

fn test_custom_delimiters() {
    // 创建自定义配置
    let mut language = JinjaLanguage::new();
    language.variable_start = "[[".to_string();
    language.variable_end = "]]".to_string();
    language.tag_start = "[%".to_string();
    language.tag_end = "%]".to_string();
    language.comment_start = "[#".to_string();
    language.comment_end = "#]".to_string();

    let source = SourceText::new("Hello [[ name ]]! [% if is_admin %]Admin[% endif %]");

    println!("  自定义语言配置: {:?}", language);
    println!("  源文本: {}", source);

    // 测试词法分析
    let lexer = JinjaLexer::new(&language);
    let mut lex_cache = LexerSession::default();
    let lex_result = lexer.lex(&source, &[], &mut lex_cache);

    println!("  Token 数量: {}", lex_result.tokens.len());
    for (i, token) in lex_result.tokens.iter().enumerate() {
        let text = source.get_text_in(token.span);
        println!("    Token {}: {:?} - '{}'", i, token.kind, text);
    }

    // 测试语法分析
    let parser = JinjaParser::new(&language);
    let mut parse_cache = ParseSession::default();
    let parse_result = parser.parse(&source, &[], &mut parse_cache);

    if parse_result.diagnostics.is_empty() {
        println!("  ✓ 自定义配置解析成功!");
    }
    else {
        println!("  ✗ 自定义配置解析有错误:");
        for diag in &parse_result.diagnostics {
            println!("    - {:?}", diag);
        }
    }
}
