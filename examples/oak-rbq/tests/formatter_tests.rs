#![cfg(feature = "lsp")]
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_rbq::{RbqFormatter, RbqLanguage, RbqParser};

#[test]
fn test_rbq_formatter_basic() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let formatter = RbqFormatter::new(&config);

    let input = "struct User { id: i32 }";
    let source = SourceText::new(input);

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    println!("AST Structure: {:#?}", red);

    let formatted = formatter.format(&red, source.text());
    println!("Basic Formatted:\n'{}'", formatted);

    // Check if the formatted output is reasonable
    // It should have correct spacing and braces
    assert!(formatted.contains("struct User {"));
    assert!(formatted.contains("id: i32;"))
}

#[test]
fn test_rbq_formatter_enum() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let formatter = RbqFormatter::new(&config);

    let input = "enum Status { Active = 1; Inactive = 0 }";
    let source = SourceText::new(input);

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    println!("Enum AST Structure: {:#?}", red);

    let formatted = formatter.format(&red, source.text());
    println!("Enum Formatted:\n'{}'", formatted);

    assert!(formatted.contains("enum Status {"));
    assert!(formatted.contains("Active = 1;"));
    assert!(formatted.contains("Inactive = 0;"))
}

#[test]
fn test_rbq_formatter_namespace() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let formatter = RbqFormatter::new(&config);

    let input = "namespace App { struct User {} }";
    let source = SourceText::new(input);

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);
    println!("Namespace AST Structure: {:#?}", red);

    let formatted = formatter.format(&red, source.text());
    println!("Namespace Formatted:\n'{}'", formatted);

    assert!(formatted.contains("namespace App {"));
    assert!(formatted.contains("struct User {"))
}

#[test]
fn test_rbq_formatter_complex_enum() {
    let config = RbqLanguage::default();
    let parser = RbqParser::new(&config);
    let formatter = RbqFormatter::new(&config);

    let input = "enum LongStatus { VeryLongMemberName1 = 1; VeryLongMemberName2 = 2; VeryLongMemberName3 = 3; VeryLongMemberName4 = 4; VeryLongMemberName5 = 5 }";
    let source = SourceText::new(input);

    let mut session = ParseSession::<RbqLanguage>::default();
    let output = parser.parse(&source, &[], &mut session);
    assert!(output.result.is_ok());

    let green = output.result.unwrap();
    let red = oak_core::tree::RedNode::new(green, 0);

    let formatted = formatter.format(&red, source.text());
    println!("Complex Enum Formatted:\n'{}'", formatted);

    assert!(formatted.contains("\n    VeryLongMemberName1"))
}
