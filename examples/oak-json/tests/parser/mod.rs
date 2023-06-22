use oak_core::{GreenNode, GreenTree, OakErrorKind, SourceText, TextEdit, parser::ParseSession};
use oak_json::{kind::JsonSyntaxKind, language::JsonLanguage, lexer::JsonLexer, parser::JsonParser};
use oak_testing::parsing::ParserTester;
use std::path::PathBuf;

#[test]
fn run_compliance_tests() -> Result<(), oak_core::OakError> {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests/compliance");

    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);

    let tester = ParserTester::new(root).with_extension("json").with_timeout(std::time::Duration::from_secs(5));

    // 运行所有测试
    // 注意：如果是 invalid 目录下的文件，ParserTester 会预期解析失败或生成 Error 节点
    tester.run_tests(&parser)
}

fn fingerprint(node: &GreenNode<JsonLanguage>, out: &mut Vec<(JsonSyntaxKind, usize, usize)>) {
    out.push((node.kind, node.text_len as usize, node.children.len()));
    for child in node.children {
        match child {
            GreenTree::Node(n) => fingerprint(n, out),
            GreenTree::Leaf(l) => out.push((l.kind, l.length as usize, 0)),
        }
    }
}

#[test]
fn parse_trailing_comma_error_offset_standard() {
    let text = "{\"a\":1,}";
    let source = SourceText::new(text);
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let lexer = JsonLexer::new(&binding);
    let mut cache = ParseSession::<JsonLanguage>::default();

    let out = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
    let err = out.result.err().or_else(|| out.diagnostics.last().cloned()).expect("expected trailing comma error");
    match err.kind() {
        OakErrorKind::TrailingCommaNotAllowed { offset, .. } => {
            assert_eq!(*offset, text.find('}').unwrap());
        }
        OakErrorKind::SyntaxError { message, offset, .. } => {
            assert!(message.contains("Trailing comma not allowed"));
            assert_eq!(*offset, text.find('}').unwrap());
        }
        _ => panic!("unexpected error kind: {:?}", err.kind()),
    }
}

#[test]
fn parse_unexpected_token_on_bare_key() {
    let text = "{a: 1}";
    let source = SourceText::new(text);
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let lexer = JsonLexer::new(&binding);
    let mut cache = ParseSession::<JsonLanguage>::default();

    let out = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
    let err = out.result.err().or_else(|| out.diagnostics.last().cloned()).expect("expected error for bare key");
    match err.kind() {
        OakErrorKind::UnexpectedCharacter { offset, .. } => assert_eq!(*offset, 1),
        OakErrorKind::SyntaxError { offset, .. } => assert_eq!(*offset, 1),
        OakErrorKind::ExpectedToken { offset, .. } => assert_eq!(*offset, 1),
        OakErrorKind::UnexpectedToken { offset, .. } => assert_eq!(*offset, 1),
        _ => panic!("unexpected error kind: {:?}", err.kind()),
    }
}

#[test]
fn parse_unexpected_eof_in_object() {
    let text = "{\"a\":";
    let source = SourceText::new(text);
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let lexer = JsonLexer::new(&binding);
    let mut cache = ParseSession::<JsonLanguage>::default();

    let out = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
    let err = out.result.err().or_else(|| out.diagnostics.last().cloned()).expect("expected parse error");
    assert!(matches!(err.kind(), OakErrorKind::UnexpectedEof { .. }));
}

#[test]
fn incremental_multi_edits_matches_full_parse_shape() {
    let binding = JsonLanguage::standard();
    let parser = JsonParser::new(&binding);
    let lexer = JsonLexer::new(&binding);
    let mut cache = ParseSession::<JsonLanguage>::default();

    let original = "{\"a\": 1, \"b\": 2, \"c\": 3}";
    let source1 = SourceText::new(original);
    let out1 = oak_core::parser::parse(&parser, &lexer, &source1, &[], &mut cache);
    assert!(out1.result.is_ok());
    assert!(out1.diagnostics.is_empty());

    let a_pos = original.find("\"a\": 1").unwrap();
    let b_pos = original.find("\"b\": 2").unwrap();

    let mut edited = original.to_string();
    edited.replace_range(b_pos + "\"b\": ".len()..b_pos + "\"b\": 2".len(), "20");
    edited.replace_range(a_pos + "\"a\": ".len()..a_pos + "\"a\": 1".len(), "10");

    let source2 = SourceText::new(edited.as_str());
    let edit1 = TextEdit { span: (a_pos + "\"a\": ".len()..a_pos + "\"a\": 1".len()).into(), text: "10".to_string().into() };
    let edit2 = TextEdit { span: (b_pos + "\"b\": ".len()..b_pos + "\"b\": 2".len()).into(), text: "20".to_string().into() };
    let edits = vec![edit1, edit2];

    let out2 = oak_core::parser::parse(&parser, &lexer, &source2, &edits, &mut cache);
    assert!(out2.result.is_ok());

    let mut cache_full = ParseSession::<JsonLanguage>::default();
    let out3 = oak_core::parser::parse(&parser, &lexer, &source2, &[], &mut cache_full);

    let mut a = Vec::new();
    let mut b = Vec::new();
    let inc_root = out2.result.unwrap();
    let full_root = out3.result.unwrap();

    fingerprint(inc_root, &mut a);
    fingerprint(full_root, &mut b);

    assert_eq!(a, b);
}
