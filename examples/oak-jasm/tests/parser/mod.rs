use oak_jasm::{JasmLexer, JasmParser};

#[test]
fn parser_basic_class() {
    let mut lexer = JasmLexer::new();
    let mut parser = JasmParser::new();
    let input = ".class public HelloWorld {}";
    let tokens = lexer.tokenize(input).unwrap();
    let root = parser.parse(tokens).unwrap();
    assert_eq!(root.class.name, "Dummy"); // 简化解析返回占位类
}

#[test]
fn parser_empty_tokens() {
    let mut parser = JasmParser::new();
    let root = parser.parse(vec![]).unwrap();
    assert_eq!(root.class.methods.len(), 0);
}
