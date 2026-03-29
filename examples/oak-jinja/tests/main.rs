use oak_core::{
    parser::{ParseSession, Parser},
    source::SourceText,
};
use oak_jinja::{JinjaLanguage, JinjaParser};

#[test]
fn test_jinja_parser_basic() {
    let source = SourceText::new("Hello {{ name }}!");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
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
fn test_jinja_parser_if_statement() {
    let source = SourceText::new("{% if age > 18 %}You are an adult{% else %}You are a minor{% endif %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("If statement parse successful!");
}

#[test]
fn test_jinja_parser_for_loop() {
    let source = SourceText::new("{% for item in items %}{{ item }}{% endfor %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("For loop parse successful!");
}

#[test]
fn test_jinja_parser_filter() {
    let source = SourceText::new("{{ name | upper }}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Filter parse successful!");
}

#[test]
fn test_jinja_parser_block() {
    let source = SourceText::new("{% block content %}{% endblock %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Block parse successful!");
}

#[test]
fn test_jinja_parser_macro() {
    let source = SourceText::new("{% macro hello(name) %}Hello {{ name }}{% endmacro %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors");
    println!("Macro parse successful!");
}

#[test]
fn test_jinja_parser_comparison_operators() {
    let source = SourceText::new("{% if x == 1 %}equal{% endif %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for comparison operators");
}

#[test]
fn test_jinja_parser_logical_operators() {
    let source = SourceText::new("{% if x and y or not z %}result{% endif %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for logical operators");
}

#[test]
fn test_jinja_parser_property_access() {
    let source = SourceText::new("{{ user.name }}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for property access");
}

#[test]
fn test_jinja_parser_index_access() {
    let source = SourceText::new("{{ items[0] }}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for index access");
}

#[test]
fn test_jinja_parser_filter_with_args() {
    let source = SourceText::new("{{ value | filter(arg) }}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for filter with arguments");
}

#[test]
fn test_jinja_parser_extends() {
    let source = SourceText::new("{% extends \"base.html\" %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for extends tag");
}

#[test]
fn test_jinja_parser_include() {
    let source = SourceText::new("{% include \"partial.html\" %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for include tag");
}

#[test]
fn test_jinja_parser_set() {
    let source = SourceText::new("{% set x = 42 %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for set tag");
}

#[test]
fn test_jinja_parser_from_import() {
    let source = SourceText::new("{% from \"macros.html\" import foo %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for from-import tag");
}

#[test]
fn test_jinja_parser_whitespace_control() {
    let source = SourceText::new("{%- if x -%}hello{%- endif -%}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for whitespace control markers");
}

#[test]
fn test_jinja_parser_for_else() {
    let source = SourceText::new("{% for item in items %}{{ item }}{% else %}No items{% endfor %}");
    let language = JinjaLanguage::default();
    let parser = JinjaParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for for/else");
}
