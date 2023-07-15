use oak_core::{Parser, source::SourceText};
use oak_matlab::{MatlabLanguage, MatlabParser};

fn parse_ok(input: &str) {
    let source = SourceText::new(input.to_string());
    let language = MatlabLanguage::default();
    let mut cache = oak_core::ParseSession::<MatlabLanguage>::default();
    let parser = MatlabParser::new(&language);
    let output = parser.parse(&source, &[], &mut cache);
    assert!(output.result.is_ok(), "parse failed for `{input}`: {:?}", output.result.err());
}

#[test]
fn test_parser_binary_expr() {
    parse_ok("1+2*3");
}

#[test]
fn test_parser_call() {
    parse_ok("f(x,y)");
}

#[test]
fn test_parser_array() {
    parse_ok("[1,2;3,4]");
}

#[test]
fn test_parser_prefix_postfix() {
    parse_ok("-a'");
}

#[test]
fn test_parser_grouped() {
    parse_ok("(a+b)*c");
}
