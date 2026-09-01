//! Tests for the JASM parser and formatter.

use oak_core::{Parser, Source, SourceText};
use oak_jasm::{JasmLanguage, JasmLexer, JasmParser};

/// Test parsing a simple JASM file with basic structure.
#[test]
fn test_basic_parsing() {
    let source = SourceText::new(
        r#"
.source Test.java
.super java/lang/Object

.class public Test
.version 65:0

.field public static value:I

.method public static main:"([Ljava/lang/String;)V"
    .limit stack 2
    .limit locals 1
    aload_0
    invokestatic Method java/lang/System.out:"Ljava/io/PrintStream;"
    ldc "Hello, World!"
    invokevirtual Method java/io/PrintStream.println:"(Ljava/lang/String;)V"
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test parsing JASM file with extends/implements syntax.
#[test]
fn test_extends_implements() {
    let source = SourceText::new(
        r#"
.class public Test extends java/lang/Object implements java/io/Serializable, java/lang/Cloneable
.version 65:0

.method public static main:"([Ljava/lang/String;)V"
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test parsing JASM file with annotations.
#[test]
fn test_annotations() {
    let source = SourceText::new(
        r#"
@java/lang/Deprecated
.class public Test
.version 65:0

@java/lang/Override
.method public void test()V"
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test parsing JASM file with exception handlers.
#[test]
fn test_exception_handlers() {
    let source = SourceText::new(
        r#"
.class public Test
.version 65:0

.method public static main:"([Ljava/lang/String;)V"
    .limit stack 1
    .limit locals 1
    try_start:
    new java/lang/Exception
    dup
    invokespecial Method java/lang/Exception."<init>":"()V"
    athrow
    try_end:
    .catch java/lang/Exception from try_start to try_end using catch_block
    catch_block:
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test parsing JASM file with attributes.
#[test]
fn test_attributes() {
    let source = SourceText::new(
        r#"
.class public Test
.version 65:0

.attribute SourceFile "Test.java"
.attribute InnerClasses

.method public static main:"([Ljava/lang/String;)V"
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test parsing JASM file with stack map frames.
#[test]
fn test_stack_map_frames() {
    let source = SourceText::new(
        r#"
.class public Test
.version 65:0

.method public static main:"([Ljava/lang/String;)V"
    .limit stack 1
    .limit locals 1
    .stackmap same_locals_1_stack_item {
        int
    }
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());
}

/// Test error recovery mechanism.
#[test]
fn test_error_recovery() {
    let source = SourceText::new(
        r#"
.class public Test
.version 65:0

// Missing method keyword
public static main:"([Ljava/lang/String;)V"
    return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    // Should not panic, even with syntax errors
    assert!(result.result.is_ok());
}

/// Test code formatter.
#[test]
fn test_formatter() {
    let source = SourceText::new(
        r#"
.class public Test
.version 65:0

.method public static main:"([Ljava/lang/String;)V"
return
.end method
"#,
    );

    let language = JasmLanguage::default();
    let lexer = JasmLexer::new(&language);
    let parser = JasmParser::new(&language);
    let mut cache = oak_core::parser::ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.result.is_ok());

    let _green = result.result.unwrap();
    #[cfg(feature = "oak-pretty-print")]
    {
        use oak_core::Builder;
        use oak_jasm::{JasmBuilder, JasmFormatter};
        let builder = JasmBuilder::new(&language);
        let mut build_cache = oak_core::parser::ParseSession::default();
        let typed = builder.build(&source, &[], &mut build_cache);
        let root = typed.result.expect("jasm typed root");
        let formatter = JasmFormatter::new();
        let formatted = formatter.format(&root);
        assert!(!formatted.is_empty());

        let pretty_formatted = formatter.format_pretty(&root);
        assert!(!pretty_formatted.is_empty());
    }
}
