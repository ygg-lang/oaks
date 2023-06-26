use oak_asciidoc::{
    language::AsciidocLanguage,
    lexer::{AsciidocLexer, token_type::AsciidocTokenType},
};

#[test]
fn test_basic_lexing() {
    let language = AsciidocLanguage::default();
    let lexer = AsciidocLexer::new(&language);

    let content = "= Hello World\n\nThis is a paragraph.\n\n- List item 1\n- List item 2";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}

#[test]
fn test_code_block_lexing() {
    let language = AsciidocLanguage::default();
    let lexer = AsciidocLexer::new(&language);

    let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
    // 检查是否包含代码块和语言指定
    let has_code_block = tokens.iter().any(|t| t.kind == AsciidocTokenType::CodeBlock);
    let has_language = tokens.iter().any(|t| t.kind == AsciidocTokenType::CodeBlockLanguage);
    assert!(has_code_block);
    assert!(has_language);
}

#[test]
fn test_table_lexing() {
    let language = AsciidocLanguage::default();
    let lexer = AsciidocLexer::new(&language);

    let content = "|===\n| Column 1 | Column 2\n| Value 1 | Value 2\n|===";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
    // 检查是否包含表格相关标记
    let has_table = tokens.iter().any(|t| t.kind == AsciidocTokenType::Table);
    let has_table_cell = tokens.iter().any(|t| t.kind == AsciidocTokenType::TableCell);
    assert!(has_table);
    assert!(has_table_cell);
}

#[test]
fn test_inline_elements_lexing() {
    let language = AsciidocLanguage::default();
    let lexer = AsciidocLexer::new(&language);

    let content = "*Emphasis* **Strong** `Monospace`";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
    // 检查是否包含内联元素标记
    let has_emphasis = tokens.iter().any(|t| t.kind == AsciidocTokenType::Emphasis);
    let has_strong = tokens.iter().any(|t| t.kind == AsciidocTokenType::Strong);
    let has_monospace = tokens.iter().any(|t| t.kind == AsciidocTokenType::Monospace);
    assert!(has_emphasis);
    assert!(has_strong);
    assert!(has_monospace);
}

#[test]
fn test_error_handling() {
    let language = AsciidocLanguage::default();
    let lexer = AsciidocLexer::new(&language);

    let content = "Invalid content with unclosed ` backtick";
    let lex_output = lexer.lex_internal(content);

    assert!(lex_output.result.is_ok());
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty());
}
