use oak_rst::{
    language::RstLanguage,
    lexer::{RstLexer, token_type::RstTokenType},
};

#[test]
fn test_basic_lexing() {
    let language = RstLanguage::default();
    let lexer = RstLexer::new(&language);

    let content = "Hello World\n==========\n\nThis is a paragraph.\n\n- List item 1\n- List item 2";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_strong_emphasis() {
    let language = RstLanguage::default();
    let lexer = RstLexer::new(&language);

    let content = "This is **strong** text and __also strong__ text.";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有 Strong 标记
    let has_strong = tokens.iter().any(|token| token.kind == RstTokenType::Strong);
    assert!(has_strong);
}

#[test]
fn test_enumerated_list() {
    let language = RstLanguage::default();
    let lexer = RstLexer::new(&language);

    let content = "1. First item";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有 EnumeratedListMarker 标记
    let has_enumerated_list = tokens.iter().any(|token| token.kind == RstTokenType::EnumeratedListMarker);
    assert!(has_enumerated_list);
}

#[test]
fn test_code_block() {
    let language = RstLanguage::default();
    let lexer = RstLexer::new(&language);

    let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());

    // 检查是否有 CodeBlock 标记
    let has_code_block = tokens.iter().any(|token| token.kind == RstTokenType::CodeBlock);
    assert!(has_code_block);
}
