use oak_core::{GreenBuilder, IncrementalCache, Lexer, SourceText};
use oak_cpp::{CppLanguage, CppLexer};

#[test]
fn test_cpp_lexer_basic() {
    let source = SourceText::new("int main() { return 0; }");
    let language = CppLanguage::new();
    let lexer = CppLexer::new(&language);
    let mut builder = GreenBuilder::new(1024);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);
    assert!(result.diagnostics.is_empty());
    println!("✓ C++ 基本词法分析测试通过");
}

#[test]
fn test_cpp_lexer_keywords() {
    let source = SourceText::new("class MyClass { public: int value; };");
    let language = CppLanguage::new();
    let lexer = CppLexer::new(&language);
    let mut builder = GreenBuilder::new(1024);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);
    assert!(result.diagnostics.is_empty());
    println!("✓ C++ 关键字词法分析测试通过");
}

#[test]
fn test_cpp_lexer_operators() {
    let source = SourceText::new("a += b * c / d - e;");
    let language = CppLanguage::new();
    let lexer = CppLexer::new(&language);
    let mut builder = GreenBuilder::new(1024);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);
    assert!(result.diagnostics.is_empty());
    println!("✓ C++ 操作符词法分析测试通过");
}

#[test]
fn test_cpp_lexer_strings() {
    let source = SourceText::new(r#"const char* str = "Hello, World!";"#);
    let language = CppLanguage::new();
    let lexer = CppLexer::new(&language);
    let mut builder = GreenBuilder::new(1024);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);
    assert!(result.diagnostics.is_empty());
    println!("✓ C++ 字符串词法分析测试通过");
}

#[test]
fn test_cpp_lexer_comments() {
    let source = SourceText::new("// Single line comment\n/* Multi line comment */\nint x;");
    let language = CppLanguage::new();
    let lexer = CppLexer::new(&language);
    let mut builder = GreenBuilder::new(1024);
    let cache = IncrementalCache::new(&mut builder);

    let result = lexer.lex_incremental(&source, 0, cache);
    assert!(result.diagnostics.is_empty());
    println!("✓ C++ 注释词法分析测试通过");
}
