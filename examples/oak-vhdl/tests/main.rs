use oak_core::{GreenBuilder, IncrementalCache, Lexer, source::SourceText};
use oak_vhdl::{VhdlLanguage, VhdlLexer};

#[test]
fn test_vhdl_lexer_simple() {
    let lexer = VhdlLexer::new(&VhdlLanguage);
    let source = SourceText::new("entity test is end entity;");
    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    // 检查是否成功生成了 tokens
    match &result.result {
        Ok(tokens) => {
            assert!(!tokens.is_empty(), "Lexer should produce tokens");
            println!("Generated {} tokens", tokens.len());

            // 打印前几个 tokens 用于调试
            for (i, token) in tokens.iter().take(5).enumerate() {
                println!("Token {}: {:?}", i, token);
            }
        }
        Err(e) => {
            panic!("Lexer failed with error: {}", e);
        }
    }

    // 检查是否有诊断信息
    if !result.diagnostics.is_empty() {
        println!("Diagnostics: {:?}", result.diagnostics);
    }
}
