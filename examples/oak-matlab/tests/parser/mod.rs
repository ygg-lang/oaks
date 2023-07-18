use oak_core::{Parser, source::SourceText};
use oak_matlab::parser::element_type::MatlabElementType;
use oak_matlab::{MatlabLanguage, MatlabParser};

fn count_kind(node: &oak_core::tree::GreenNode<'_, oak_matlab::MatlabLanguage>, kind: MatlabElementType) -> usize {
    use oak_core::tree::GreenTree;
    let mut n = if node.kind == kind { 1 } else { 0 };
    for child in node.children {
        if let GreenTree::Node(c) = child {
            n += count_kind(c, kind);
        }
    }
    n
}

fn assert_has_kind(input: &str, kind: MatlabElementType) {
    let source = SourceText::new(input.to_string());
    let language = MatlabLanguage::default();
    let mut cache = oak_core::ParseSession::<MatlabLanguage>::default();
    let parser = MatlabParser::new(&language);
    let output = parser.parse(&source, &[], &mut cache);
    let root = output.result.expect(&format!("parse failed for `{input}`"));
    assert!(count_kind(root, kind) >= 1, "expected {kind:?} in `{input}`, got {root:?}");
}

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

#[test]
fn test_parser_array_index() {
    assert_has_kind("[1, 2, 3](1:2)", MatlabElementType::Call);
}

#[test]
fn test_parser_if_else_end() {
    assert_has_kind("if 1, 2, else, 3, end", MatlabElementType::IfStmt);
}

#[test]
fn test_parser_while_end() {
    assert_has_kind("while 0, 1, end", MatlabElementType::WhileStmt);
}

#[test]
fn test_parser_for_end() {
    assert_has_kind("for i=1:3, i, end", MatlabElementType::ForStmt);
}
