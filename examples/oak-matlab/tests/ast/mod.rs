use oak_core::{Builder, SourceText, parser::ParseSession};
use oak_matlab::{
    MatlabBuilder, MatlabLanguage,
    ast::{Expression, MatlabRoot, Statement},
    lexer::token_type::MatlabTokenType,
};

fn build(input: &str) -> MatlabRoot {
    let language = MatlabLanguage::default();
    let builder = MatlabBuilder::new(&language);
    let source = SourceText::new(input.to_string());
    let mut cache = ParseSession::<MatlabLanguage>::default();
    builder.build(&source, &[], &mut cache).result.expect("build ok")
}

#[test]
fn ast_binary_owned() {
    let root = build("a + b");
    assert_eq!(root.items.len(), 1);
    match &root.items[0] {
        Statement::Expr(Expression::Binary(bin)) => {
            assert_eq!(bin.operator, MatlabTokenType::Plus);
            match &bin.lhs {
                Expression::Symbol(id) => assert_eq!(id.name, "a"),
                other => panic!("expected symbol lhs, got {other:?}"),
            }
        }
        other => panic!("expected binary expr, got {other:?}"),
    }
}

#[test]
fn ast_and_infix_owned() {
    let root = build("1 & 0");
    match &root.items[0] {
        Statement::Expr(Expression::Binary(bin)) => {
            assert_eq!(bin.operator, MatlabTokenType::And);
        }
        other => panic!("expected And binary, got {other:?}"),
    }
}

#[test]
fn ast_call_owned() {
    let root = build("sin(x, y)");
    match &root.items[0] {
        Statement::Expr(Expression::Call { head, arguments, .. }) => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "sin"),
                other => panic!("expected sin head, got {other:?}"),
            }
            assert_eq!(arguments.len(), 2);
        }
        other => panic!("expected Call, got {other:?}"),
    }
}

#[test]
fn ast_if_owned() {
    let root = build("if 1, 2, else, 3, end");
    match &root.items[0] {
        Statement::If { condition, then_body, else_body, .. } => {
            assert!(matches!(condition, Expression::Literal { .. }));
            assert_eq!(then_body.len(), 1);
            assert_eq!(else_body.len(), 1);
        }
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn ast_array_owned() {
    let root = build("[1, 2; 3]");
    match &root.items[0] {
        Statement::Expr(Expression::Array { rows, .. }) => {
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].len(), 2);
            assert_eq!(rows[1].len(), 1);
        }
        other => panic!("expected Array, got {other:?}"),
    }
}

#[test]
fn ast_for_owned() {
    let root = build("for i=1:3, i, end");
    match &root.items[0] {
        Statement::For { header, body, .. } => {
            assert!(matches!(header, Expression::Binary(_)));
            assert_eq!(body.len(), 1);
        }
        other => panic!("expected For, got {other:?}"),
    }
}

#[test]
fn ast_try_owned() {
    let root = build("try, 2, catch, 3, end");
    match &root.items[0] {
        Statement::Try { body, catch_name, catch_body, .. } => {
            assert_eq!(body.len(), 1);
            assert!(catch_name.is_none());
            assert_eq!(catch_body.len(), 1);
        }
        other => panic!("expected Try, got {other:?}"),
    }
}
