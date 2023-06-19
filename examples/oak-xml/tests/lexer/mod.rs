#![feature(new_range_api)]

use oak_core::{helpers::LexerTester, source::Source};
use oak_xml::{XmlLanguage, XmlLexer};
use std::{path::Path, time::Duration};

#[test]
fn test_xml_lexer() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    // don't use `xml` here to avoid confusion with XML source files
    let test_runner = LexerTester::new(here.join("tests/lexer")).with_extension("xml").with_timeout(Duration::from_secs(5));
    match test_runner.run_tests::<XmlLanguage, _>(lexer) {
        Ok(()) => println!("XML lexer tests passed!"),
        Err(e) => panic!("XML lexer tests failed: {}", e),
    }
}

#[test]
fn test_peek_behavior() {
    use oak_core::{SourceText, lexer::LexerState};
    use oak_xml::XmlLanguage;

    let source = SourceText::new("<tag>");
    let mut state = LexerState::<&SourceText, XmlLanguage>::new(&source);

    println!("初始状态:");
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());

    println!("\n前进 1 个字符后:");
    state.advance(1);
    println!("位置: {}", state.get_position());
    println!("current(): {:?}", state.current());
    println!("peek(): {:?}", state.peek());
}

#[test]
fn test_xml_comment_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("<!-- This is a comment -->");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}

#[test]
fn test_xml_tag_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("<tag attr=\"value\">content</tag>");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}

#[test]
fn test_xml_cdata_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("<![CDATA[Some data]]>");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}

#[test]
fn test_xml_processing_instruction_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("<?xml version=\"1.0\"?>");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}

#[test]
fn test_xml_doctype_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("<!DOCTYPE html>");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}

#[test]
fn test_xml_entity_reference_parsing() {
    use oak_core::{Lexer, SourceText};
    use oak_xml::{XmlLanguage, XmlLexer};

    let language = Box::leak(Box::new(XmlLanguage::default()));
    let lexer = XmlLexer::new(language);
    let source = SourceText::new("&amp; &lt; &gt; &#123; &#x1A;");
    let result = lexer.lex(&source);

    println!("Result: {:?}", result);
    let tokens = result.result.expect("词法分析应该成功");
    assert!(!tokens.is_empty());
}
