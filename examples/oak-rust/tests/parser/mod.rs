use oak_core::{LexerCache, source::Source};

#[test]
fn test_simple_function_parsing() -> Result<(), oak_core::OakError> {
    use oak_core::{Lexer, Parser, SourceText};
    use oak_rust::{RustLanguage, RustLexer, RustParser};

    let source = SourceText::new("fn main() { println!(\"Hello, world!\"); }");
    let language = RustLanguage::default();
    let parser = RustParser::new(language);

    // 先测试词法分析器
    println!("测试词法分析器:");
    let lexer = RustLexer::new(&language);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);
    match &lex_output.result {
        Ok(tokens) => {
            println!("生成的 tokens: {:?}", tokens);
            println!("token 数量: {}", tokens.len());

            cache.set_lex_output(lex_output.clone());

            // 使用带有 token 的缓存进行解析
            let parse_output = parser.parse(&source, &[], &mut cache);

            println!("测试简单函数解析:");
            println!("源代码: '{}'", (&source).get_text_from(0));
            match &parse_output.result {
                Ok(root) => {
                    println!("解析结果: {:?}", root);
                    println!("✅ 简单函数解析测试通过！");
                }
                Err(e) => {
                    println!("❌ 解析失败: {:?}", e);
                    return Err(e.clone());
                }
            }
        }
        Err(e) => {
            println!("❌ 词法分析失败: {:?}", e);
            return Err(e.clone());
        }
    }
    Ok(())
}
