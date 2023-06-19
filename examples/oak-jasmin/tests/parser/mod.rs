use oak_core::{GreenBuilder, IncrementalCache, Lexer, Parser, SourceText};
use oak_jasmin::{JasminLanguage, JasminLexer, JasminParser};

#[test]
fn parser_basic_class() {
    let source = SourceText::new(".class public HelloWorld");
    let language = JasminLanguage::default();
    let lexer = JasminLexer::new(&language);
    let parser = JasminParser::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);

    // 使用新的 API
    let lex_output = lexer.lex_incremental(&source, 0, cache);
    assert!(lex_output.result.is_ok());

    let mut pool2 = GreenBuilder::new(0);
    let cache2 = IncrementalCache::new(&mut pool2);
    let parse_output = parser.parse_incremental(&source, 0, cache2);
    assert!(parse_output.result.is_ok());
}

#[test]
fn parser_empty_input() {
    let source = SourceText::new("");
    let language = JasminLanguage::default();
    let parser = JasminParser::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let result = parser.parse_incremental(&source, 0, cache);
    assert!(result.result.is_ok());
}

#[test]
fn parser_class_with_method() {
    let source = SourceText::new(
        r#"
.class public HelloWorld
.method public static main([Ljava/lang/String;)V
    aload_0
    return
.end method
"#,
    );
    let language = JasminLanguage::default();
    let lexer = JasminLexer::new(&language);
    let parser = JasminParser::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let lex_output = lexer.lex_incremental(&source, 0, cache);
    assert!(lex_output.result.is_ok());

    let mut pool2 = GreenBuilder::new(0);
    let cache2 = IncrementalCache::new(&mut pool2);
    let parse_output = parser.parse_incremental(&source, 0, cache2);
    assert!(parse_output.result.is_ok());
}

#[test]
fn parser_class_with_field() {
    let source = SourceText::new(
        r#"
.class public HelloWorld
.field private static value I
"#,
    );
    let language = JasminLanguage::default();
    let lexer = JasminLexer::new(&language);
    let parser = JasminParser::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let lex_output = lexer.lex_incremental(&source, 0, cache);
    assert!(lex_output.result.is_ok());

    let mut pool2 = GreenBuilder::new(0);
    let cache2 = IncrementalCache::new(&mut pool2);
    let parse_output = parser.parse_incremental(&source, 0, cache2);
    assert!(parse_output.result.is_ok());
}

#[test]
fn parser_complex_class() {
    let source = SourceText::new(
        r#"
.class public final HelloWorld
.source "HelloWorld.java"
.field private value I

.method public <init>()V
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

.method public static main([Ljava/lang/String;)V
    ldc "Hello, World!"
    return
.end method
"#,
    );
    let language = JasminLanguage::default();
    let lexer = JasminLexer::new(&language);
    let parser = JasminParser::new(&language);

    let mut pool = GreenBuilder::new(0);
    let cache = IncrementalCache::new(&mut pool);
    let lex_output = lexer.lex_incremental(&source, 0, cache);
    assert!(lex_output.result.is_ok());

    let mut pool2 = GreenBuilder::new(0);
    let cache2 = IncrementalCache::new(&mut pool2);
    let parse_output = parser.parse_incremental(&source, 0, cache2);
    assert!(parse_output.result.is_ok());
}
