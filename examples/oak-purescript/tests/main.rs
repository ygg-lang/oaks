use oak_purescript::{PurescriptLanguage, PurescriptLexer};

mod lexer;
mod parser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_purescript_lexer_basic() {
    // 简单测试：创建词法分析器并验证它可以正常工作
    let language = PurescriptLanguage::default();
    let _lexer = PurescriptLexer::new(&language);

    // 测试一个简单的 PureScript 代码片段
    let test_code = "module Main where\n\nmain :: IO ()\nmain = log \"Hello, World!\"";

    // 这里只是验证词法分析器可以创建，不进行复杂的测试
    println!("PureScript lexer created successfully");
    println!("Test code: {}", test_code)
}
