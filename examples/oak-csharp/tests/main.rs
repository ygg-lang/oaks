use oak_core::{GreenBuilder, IncrementalCache, Lexer, SourceText, lexer::LexOutput, source::Source};
use oak_csharp::{CSharpLanguage, CSharpLexer, CSharpSyntaxKind};

#[test]
fn test_csharp_lexer_basic() {
    let source = SourceText::new("class Program { }");
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_csharp_lexer_keywords() {
    let source = SourceText::new("public static void Main string int bool");
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_csharp_lexer_operators() {
    let source = SourceText::new("+ - * / % = == != < > <= >= && || ! & | ^ ~ ++ --");
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_csharp_lexer_strings() {
    let source = SourceText::new(r#""Hello, World!" 'c' @"verbatim string""#);
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_csharp_lexer_numbers() {
    let source = SourceText::new("123 456.789 0x1A2B 123L 456.789f 123.456m");
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

#[test]
fn test_csharp_lexer_comments() {
    let source = SourceText::new("// single line comment\n/* multi line comment */");
    let language = CSharpLanguage;
    let lexer = CSharpLexer::new(&language);

    let mut builder = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut builder);
    let result = lexer.lex_incremental(&source, 0, cache);

    assert!(result.result.is_ok());
}

fn main() {
    println!("Running C# lexer tests...");
    test_csharp_lexer_basic();
    test_csharp_lexer_keywords();
    test_csharp_lexer_operators();
    test_csharp_lexer_strings();
    test_csharp_lexer_numbers();
    test_csharp_lexer_comments();
    println!("All C# lexer tests passed!");
}
