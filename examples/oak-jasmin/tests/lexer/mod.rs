use oak_jasmin::{JasminLanguage, JasminLexer};

#[test]
fn lexer_empty_input() {
    let _lang = JasminLanguage::standard();
    let mut lexer = JasminLexer::new();
    let tokens = lexer.tokenize("").unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn lexer_basic_tokens() {
    let mut lexer = JasminLexer::new();
    let tokens = lexer.tokenize("hello world").unwrap();
    assert_eq!(tokens.len(), 4); // hello + whitespace + world + EOF
}

#[test]
fn lexer_jasmin_keywords() {
    let mut lexer = JasminLexer::new();
    let input = ".class public static final";
    let tokens = lexer.tokenize(input).unwrap();

    // 验证关键字被正确识别
    assert_eq!(tokens.len(), 8); // .class + ws + public + ws + static + ws + final + EOF

    // 检查具体的 kind 类型
    use oak_jasmin::JasminSyntaxKind;
    assert_eq!(tokens[0].kind, JasminSyntaxKind::ClassKw);
    assert_eq!(tokens[2].kind, JasminSyntaxKind::Public);
    assert_eq!(tokens[4].kind, JasminSyntaxKind::Static);
    assert_eq!(tokens[6].kind, JasminSyntaxKind::Final);
}

#[test]
fn lexer_jasmin_instructions() {
    let mut lexer = JasminLexer::new();
    let input = "aload_0 return ldc invokespecial";
    let tokens = lexer.tokenize(input).unwrap();

    use oak_jasmin::JasminSyntaxKind;
    assert_eq!(tokens[0].kind, JasminSyntaxKind::ALoad0);
    assert_eq!(tokens[2].kind, JasminSyntaxKind::Return);
    assert_eq!(tokens[4].kind, JasminSyntaxKind::Ldc);
    assert_eq!(tokens[6].kind, JasminSyntaxKind::InvokeSpecial);
}

#[test]
fn lexer_string_literals_and_comments() {
    let mut lexer = JasminLexer::new();
    let input = r#""Hello World" ; This is a comment"#;
    let tokens = lexer.tokenize(input).unwrap();

    use oak_jasmin::JasminSyntaxKind;
    assert_eq!(tokens[0].kind, JasminSyntaxKind::StringLiteral);
    assert_eq!(tokens[0].text, r#""Hello World""#);
    assert_eq!(tokens[2].kind, JasminSyntaxKind::Comment);
    assert_eq!(tokens[2].text, "; This is a comment");
}

#[test]
fn lexer_complex_jasmin_code() {
    let mut lexer = JasminLexer::new();
    let input = r#"
.class public HelloWorld
.method public static main([Ljava/lang/String;)V
    ldc "Hello, World!"
    return
.end method
"#;
    let tokens = lexer.tokenize(input).unwrap();

    // 验证包含了所有预期的 kind 类型
    use oak_jasmin::JasminSyntaxKind;
    let mut has_class_kw = false;
    let mut has_method_kw = false;
    let mut has_public = false;
    let mut has_static = false;
    let mut has_string_literal = false;
    let mut has_ldc = false;
    let mut has_return = false;

    for token in &tokens {
        match token.kind {
            JasminSyntaxKind::ClassKw => has_class_kw = true,
            JasminSyntaxKind::MethodKw => has_method_kw = true,
            JasminSyntaxKind::Public => has_public = true,
            JasminSyntaxKind::Static => has_static = true,
            JasminSyntaxKind::StringLiteral => has_string_literal = true,
            JasminSyntaxKind::Ldc => has_ldc = true,
            JasminSyntaxKind::Return => has_return = true,
            _ => {}
        }
    }

    assert!(has_class_kw, "Should recognize .class keyword");
    assert!(has_method_kw, "Should recognize .method keyword");
    assert!(has_public, "Should recognize public modifier");
    assert!(has_static, "Should recognize static modifier");
    assert!(has_string_literal, "Should recognize string literal");
    assert!(has_ldc, "Should recognize ldc instruction");
    assert!(has_return, "Should recognize return instruction");
}
