use oak_asciidoc::{
    language::AsciidocLanguage,
    parser::{AsciidocParser, element_type::AsciidocElementType},
};
use oak_core::{ParseSession, parser::Parser};

#[test]
fn test_basic_parsing() {
    let language = AsciidocLanguage::default();
    let parser = AsciidocParser::new(&language);

    let content = "= Hello World\n\nThis is a paragraph.\n\n- List item 1\n- List item 2";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_code_block_parsing() {
    let language = AsciidocLanguage::default();
    let parser = AsciidocParser::new(&language);

    let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_table_parsing() {
    let language = AsciidocLanguage::default();
    let parser = AsciidocParser::new(&language);

    let content = "|===\n| Column 1 | Column 2\n| Value 1 | Value 2\n|===";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_inline_elements_parsing() {
    let language = AsciidocLanguage::default();
    let parser = AsciidocParser::new(&language);

    let content = "*Emphasis* **Strong** `Monospace`";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_complex_document_parsing() {
    let language = AsciidocLanguage::default();
    let parser = AsciidocParser::new(&language);

    let content = "= Test Document\n\nThis is a paragraph with *emphasis* and **strong** text.\n\n== Section 1\n\n- List item 1\n- List item 2\n\n[source,rust]
----
fn main() {
    println!(\"Hello!\");
}
----\n\n|===\n| Name | Value\n| Foo | Bar\n|===";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}
