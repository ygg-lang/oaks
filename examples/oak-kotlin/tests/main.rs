use oak_kotlin::{KotlinLanguage, KotlinLexer};
use std::path::Path;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_kotlin_lexer_basic() {
    // 简单测试：创建词法分析器并验证它可以正常工作
    let language = KotlinLanguage::default();
    let lexer = KotlinLexer::new(&language);

    // 测试一个简单的 Kotlin 代码片段
    let test_code = "fun main() {\n    println(\"Hello, World!\")\n}";

    // 这里只是验证词法分析器可以创建，不进行复杂的测试
    println!("Kotlin lexer created successfully");
    println!("Test code: {}", test_code);
}
