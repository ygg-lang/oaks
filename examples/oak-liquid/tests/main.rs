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

#[test]
fn test_liquid_parser_comparison_operators() {
    let source = SourceText::new("{% if x == 1 %}equal{% endif %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for comparison operators");
}

#[test]
fn test_liquid_parser_logical_operators() {
    let source = SourceText::new("{% if x and y or not z %}result{% endif %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for logical operators");
}

#[test]
fn test_liquid_parser_property_access() {
    let source = SourceText::new("{{ user.name }}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for property access");
}

#[test]
fn test_liquid_parser_index_access() {
    let source = SourceText::new("{{ items[0] }}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for index access");
}

#[test]
fn test_liquid_parser_filter_with_args() {
    let source = SourceText::new("{{ value | filter: arg1, arg2 }}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for filter with colon arguments");
}

#[test]
fn test_liquid_parser_range_expression() {
    let source = SourceText::new("{% for i in (1..5) %}{{ i }}{% endfor %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for range expression");
}

#[test]
fn test_liquid_parser_assign() {
    let source = SourceText::new("{% assign x = 42 %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for assign tag");
}

#[test]
fn test_liquid_parser_capture() {
    let source = SourceText::new("{% capture var %}hello{% endcapture %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for capture tag");
}

#[test]
fn test_liquid_parser_case_when() {
    let source = SourceText::new("{% case x %}{% when \"a\" %}A{% endcase %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for case/when tag");
}

#[test]
fn test_liquid_parser_include() {
    let source = SourceText::new("{% include \"template\" %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for include tag");
}

#[test]
fn test_liquid_parser_render() {
    let source = SourceText::new("{% render \"template\" %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for render tag");
}

#[test]
fn test_liquid_parser_unless() {
    let source = SourceText::new("{% unless cond %}not true{% endunless %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for unless tag");
}

#[test]
fn test_liquid_parser_raw() {
    let source = SourceText::new("{% raw %}{{ this should not be parsed }}{% endraw %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for raw tag");
}

#[test]
fn test_liquid_parser_whitespace_control() {
    let source = SourceText::new("{%- if x -%}hello{%- endif -%}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for whitespace control markers");
}

#[test]
fn test_liquid_parser_for_with_params() {
    let source = SourceText::new("{% for item in items limit:5 offset:2 reversed %}{{ item }}{% endfor %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for for loop parameters");
}

#[test]
fn test_liquid_parser_break() {
    let source = SourceText::new("{% for item in items %}{% break %}{% endfor %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for break tag");
}

#[test]
fn test_liquid_parser_continue() {
    let source = SourceText::new("{% for item in items %}{% continue %}{% endfor %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for continue tag");
}

#[test]
fn test_liquid_parser_cycle() {
    let source = SourceText::new("{% cycle \"a\", \"b\" %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for cycle tag");
}

#[test]
fn test_liquid_parser_tablerow() {
    let source = SourceText::new("{% tablerow item in items %}{{ item }}{% endtablerow %}");
    let language = LiquidLanguage::default();
    let parser = LiquidParser::new(&language);
    let mut cache = ParseSession::default();

    let result = parser.parse(&source, &[], &mut cache);
    assert!(result.diagnostics.is_empty(), "Parser should not return errors for tablerow tag");
}
