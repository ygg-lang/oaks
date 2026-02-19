use oak_core::{ParseSession, parser::Parser};
use oak_rst::{
    language::RstLanguage,
    parser::{RstParser, element_type::RstElementType as ET},
};

#[test]
fn test_basic_parsing() {
    let language = RstLanguage::default();
    let parser = RstParser::new(&language);

    let content = "Hello World\n==========\n\nThis is a paragraph.\n\n- List item 1\n- List item 2";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_strong_emphasis_parsing() {
    let language = RstLanguage::default();
    let parser = RstParser::new(&language);

    let content = "This is **strong** text.";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_enumerated_list_parsing() {
    let language = RstLanguage::default();
    let parser = RstParser::new(&language);

    let content = "1. First item\n2. Second item";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}

#[test]
fn test_code_block_parsing() {
    let language = RstLanguage::default();
    let parser = RstParser::new(&language);

    let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let mut session = ParseSession::new(0);
    let parse_result = parser.parse(content, &[], &mut session);

    assert!(parse_result.result.is_ok());
    let root = parse_result.result.unwrap();
    assert!(root.children().len() > 0);
}
