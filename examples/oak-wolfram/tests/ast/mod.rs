use oak_core::{Parser, source::SourceText, tree::TypedNode};
use oak_wolfram::{
    WolframLanguage, WolframParser,
    ast::{WolframBinaryExpr, WolframCall, WolframCallHead, WolframExpr, WolframPart, WolframPattern, WolframRoot},
    lexer::token_type::WolframTokenType,
};

fn with_root<R>(input: &str, f: impl FnOnce(&SourceText, WolframRoot<'_>) -> R) -> R {
    let source = SourceText::new(input.to_string());
    let language = WolframLanguage::default();
    let mut session = oak_core::ParseSession::<WolframLanguage>::default();
    let parser = WolframParser::new(&language);
    let output = parser.parse(&source, &[], &mut session);
    let green = output.result.expect("parse ok");
    let root = WolframRoot::from_green(green).expect("root cast");
    f(&source, root)
}

#[test]
fn ast_binary_lhs_rhs_operator() {
    with_root("a + b", |source, root| {
        let expr = root.expressions().next().expect("expr");
        let WolframExpr::Binary(bin) = expr else {
            panic!("expected Binary, got {expr:?}");
        };
        let lhs = bin.lhs().expect("lhs");
        let rhs = bin.rhs().expect("rhs");
        let op = bin.operator().expect("op");
        assert_eq!(lhs.text(source).trim(), "a");
        assert_eq!(rhs.text(source).trim(), "b");
        assert_eq!(op.kind(), WolframTokenType::Plus);
        assert!(WolframBinaryExpr::cast(bin.red()).is_some());
    });
}

#[test]
fn ast_call_head_and_arguments() {
    with_root("If[1, 2, 3]", |source, root| {
        let expr = root.expressions().next().expect("expr");
        let WolframExpr::Call(call) = expr else {
            panic!("expected Call, got {expr:?}");
        };
        match call.head().expect("head") {
            WolframCallHead::Token(tok) => {
                assert_eq!(tok.kind(), WolframTokenType::If);
                assert_eq!(tok.text(source).as_ref(), "If");
            }
            WolframCallHead::Expr(other) => panic!("expected token head, got {other:?}"),
        }
        let args: Vec<_> = call.arguments().collect();
        assert_eq!(args.len(), 3);
        assert_eq!(args[0].text(source).as_ref(), "1");
        assert!(WolframCall::cast(call.red()).is_some());
    });
}

#[test]
fn ast_part_indices() {
    with_root("{1, 2}[[1]]", |source, root| {
        let expr = root.expressions().next().expect("expr");
        let WolframExpr::Part(part) = expr else {
            panic!("expected Part, got {expr:?}");
        };
        let base = part.expression().expect("base");
        assert!(matches!(base, WolframExpr::List(_)));
        let indices: Vec<_> = part.indices().collect();
        assert_eq!(indices.len(), 1);
        assert_eq!(indices[0].text(source).as_ref(), "1");
        assert!(WolframPart::cast(part.red()).is_some());
    });
}

#[test]
fn ast_pattern_name_and_blank() {
    with_root("x_", |source, root| {
        let expr = root.expressions().next().expect("expr");
        let WolframExpr::Pattern(pat) = expr else {
            panic!("expected Pattern, got {expr:?}");
        };
        let name = pat.name().expect("name");
        assert_eq!(name.text(source).as_ref(), "x");
        let blank = pat.blank_token().expect("blank");
        assert_eq!(blank.kind(), WolframTokenType::Underscore);
        assert!(WolframPattern::cast(pat.red()).is_some());
    });
}

#[test]
fn ast_compound_via_binary_semicolon() {
    with_root("1; 2", |source, root| {
        let exprs: Vec<_> = root.expressions().collect();
        assert_eq!(exprs.len(), 1);
        let WolframExpr::Binary(bin) = exprs[0] else {
            panic!("expected Binary CompoundExpression, got {:?}", exprs[0]);
        };
        assert_eq!(bin.operator().expect("op").kind(), WolframTokenType::Semicolon);
        assert_eq!(bin.lhs().expect("lhs").text(source).as_ref(), "1");
        assert_eq!(bin.rhs().expect("rhs").text(source).as_ref(), "2");
    });
}
