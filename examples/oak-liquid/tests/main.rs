use oak_core::{
    parser::{ParseSession, Parser},
    source::SourceText,
};
use oak_liquid::{LiquidLanguage, LiquidParser};

#[test]
fn test_liquid_parser_basic() {
    let source = SourceText::new("Hello {{ name }}!");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);

    if !result.diagnostics.is_empty() {
        println!("Diagnostics:");
        for diag in &result.diagnostics {
            println!("{:?}", diag);
        }
    }

    assert!(result.diagnostics.is_empty(), "Parser should not return errors");

    println!("Parse successful!");
    println!("Tree: {:?}", result.result);
}

#[test]
fn test_liquid_parser_if_statement() {
    let source = SourceText::new("{% if age > 18 %}You are an adult{% else %}You are a minor{% endif %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("If statement parse successful!");
}

#[test]
fn test_liquid_parser_for_loop() {
    let source = SourceText::new("{% for item in items %}{{ item }}{% endfor %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("For loop parse successful!");
}

#[test]
fn test_liquid_parser_filter() {
    let source = SourceText::new("{{ name | upper }}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Filter parse successful!");
}

#[test]
fn test_liquid_parser_block() {
    let source = SourceText::new("{% block content %}{% endblock %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Block parse successful!");
}

#[test]
fn test_liquid_parser_macro() {
    let source = SourceText::new("{% macro hello(name) %}Hello {{ name }}{% endmacro %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Macro parse successful!");
}
