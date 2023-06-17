use oak_jasmin::{JasminLexer, JasminParser};

#[test]
fn parser_basic_class() {
    let mut lexer = JasminLexer::new();
    let mut parser = JasminParser::new();
    let input = ".class public HelloWorld";
    let tokens = lexer.tokenize(input).unwrap();
    let root = parser.parse(tokens).unwrap();

    assert_eq!(root.class.name, "HelloWorld");
    assert!(root.class.modifiers.contains(&"public".to_string()));
}

#[test]
fn parser_empty_tokens() {
    let mut parser = JasminParser::new();
    let tokens = vec![];
    let result = parser.parse(tokens);
    assert!(result.is_ok());
}

#[test]
fn parser_class_with_method() {
    let mut lexer = JasminLexer::new();
    let mut parser = JasminParser::new();
    let input = r#"
.class public HelloWorld
.method public static main([Ljava/lang/String;)V
    aload_0
    return
.end method
"#;
    let tokens = lexer.tokenize(input).unwrap();
    let root = parser.parse(tokens).unwrap();

    assert_eq!(root.class.name, "HelloWorld");
    assert_eq!(root.class.methods.len(), 1);

    let method = &root.class.methods[0];
    assert_eq!(method.modifiers.len(), 2);
    assert!(method.modifiers.contains(&"public".to_string()));
    assert!(method.modifiers.contains(&"static".to_string()));
    assert_eq!(method.name_and_descriptor, "main([Ljava/lang/String;)V");
    assert_eq!(method.instructions.len(), 2);
}

#[test]
fn parser_class_with_field() {
    let mut lexer = JasminLexer::new();
    let mut parser = JasminParser::new();
    let input = r#"
.class public HelloWorld
.field private static value I
"#;
    let tokens = lexer.tokenize(input).unwrap();
    let root = parser.parse(tokens).unwrap();

    assert_eq!(root.class.name, "HelloWorld");
    assert_eq!(root.class.fields.len(), 1);

    let field = &root.class.fields[0];
    assert_eq!(field.modifiers.len(), 2);
    assert!(field.modifiers.contains(&"private".to_string()));
    assert!(field.modifiers.contains(&"static".to_string()));
    assert_eq!(field.name_and_descriptor, "value I");
}

#[test]
fn parser_complex_class() {
    let mut lexer = JasminLexer::new();
    let mut parser = JasminParser::new();
    let input = r#"
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
"#;
    let tokens = lexer.tokenize(input).unwrap();
    let root = parser.parse(tokens).unwrap();

    assert_eq!(root.class.name, "HelloWorld");
    assert_eq!(root.class.modifiers.len(), 2);
    assert!(root.class.modifiers.contains(&"public".to_string()));
    assert!(root.class.modifiers.contains(&"final".to_string()));
    assert_eq!(root.class.source_file, Some("\"HelloWorld.java\"".to_string()));

    assert_eq!(root.class.fields.len(), 1);
    assert_eq!(root.class.methods.len(), 2);

    // 验证构造方
    let init_method = &root.class.methods[0];
    assert_eq!(init_method.modifiers, vec!["public"]);
    assert_eq!(init_method.name_and_descriptor, "<init>()V");
    assert_eq!(init_method.instructions.len(), 3);

    // 验证 main 方法
    let main_method = &root.class.methods[1];
    assert_eq!(main_method.modifiers, vec!["public", "static"]);
    assert_eq!(main_method.name_and_descriptor, "main([Ljava/lang/String;)V");
    assert_eq!(main_method.instructions.len(), 2);
}
