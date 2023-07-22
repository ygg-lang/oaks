use oak_core::{Builder, SourceText, parser::ParseSession};
use oak_wolfram::{
    WolframBuilder, WolframLanguage,
    ast::{Expression, WolframRoot},
    lexer::token_type::WolframTokenType,
};

fn build(input: &str) -> WolframRoot {
    let language = WolframLanguage::default();
    let builder = WolframBuilder::new(&language);
    let source = SourceText::new(input.to_string());
    let mut cache = ParseSession::<WolframLanguage>::default();
    builder.build(&source, &[], &mut cache).result.expect("build ok")
}

#[test]
fn ast_binary_owned() {
    let root = build("a + b");
    assert_eq!(root.expressions.len(), 1);
    match &root.expressions[0] {
        Expression::Binary(bin) => {
            assert_eq!(bin.operator, WolframTokenType::Plus);
            match &bin.lhs {
                Expression::Symbol(id) => assert_eq!(id.name, "a"),
                other => panic!("expected symbol lhs, got {other:?}"),
            }
            match &bin.rhs {
                Expression::Symbol(id) => assert_eq!(id.name, "b"),
                other => panic!("expected symbol rhs, got {other:?}"),
            }
        }
        other => panic!("expected Binary, got {other:?}"),
    }
}

#[test]
fn ast_call_owned() {
    let root = build("If[1, 2, 3]");
    match &root.expressions[0] {
        Expression::Call { head, arguments, .. } => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "If"),
                other => panic!("expected If head, got {other:?}"),
            }
            assert_eq!(arguments.len(), 3);
        }
        other => panic!("expected Call, got {other:?}"),
    }
}

#[test]
fn ast_part_owned() {
    let root = build("{1, 2}[[1]]");
    match &root.expressions[0] {
        Expression::Part { expression, indices, .. } => {
            assert!(matches!(expression.as_ref(), Expression::List { .. }));
            assert_eq!(indices.len(), 1);
        }
        other => panic!("expected Part, got {other:?}"),
    }
}

#[test]
fn ast_pattern_owned() {
    let root = build("x_");
    match &root.expressions[0] {
        Expression::Pattern { name, blank, .. } => {
            match name.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "x"),
                other => panic!("expected symbol name, got {other:?}"),
            }
            assert_eq!(*blank, WolframTokenType::Underscore);
        }
        other => panic!("expected Pattern, got {other:?}"),
    }
}

#[test]
fn ast_compound_semicolon_owned() {
    let root = build("1; 2");
    assert_eq!(root.expressions.len(), 1);
    match &root.expressions[0] {
        Expression::Binary(bin) => {
            assert_eq!(bin.operator, WolframTokenType::Semicolon);
        }
        other => panic!("expected Binary semicolon, got {other:?}"),
    }
}

#[test]
fn ast_full_form_binary_and_part() {
    let root = build("a + b").full_form();
    match &root.expressions[0] {
        Expression::Call { head, arguments, .. } => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "Plus"),
                other => panic!("expected Plus, got {other:?}"),
            }
            assert_eq!(arguments.len(), 2);
        }
        other => panic!("expected Call Plus, got {other:?}"),
    }

    let part = build("{1}[[1]]").full_form();
    match &part.expressions[0] {
        Expression::Call { head, arguments, .. } => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "Part"),
                other => panic!("expected Part, got {other:?}"),
            }
            assert_eq!(arguments.len(), 2);
            assert!(matches!(arguments[0], Expression::List { .. }));
        }
        other => panic!("expected Call Part, got {other:?}"),
    }
}

#[test]
fn ast_full_form_pattern_and_if_call() {
    let root = build("x_").full_form();
    match &root.expressions[0] {
        Expression::Call { head, arguments, .. } => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "Pattern"),
                other => panic!("expected Pattern, got {other:?}"),
            }
            assert_eq!(arguments.len(), 2);
            match &arguments[1] {
                Expression::Call { head, arguments: blank_args, .. } => {
                    match head.as_ref() {
                        Expression::Symbol(id) => assert_eq!(id.name, "Blank"),
                        other => panic!("expected Blank, got {other:?}"),
                    }
                    assert!(blank_args.is_empty());
                }
                other => panic!("expected Blank call, got {other:?}"),
            }
        }
        other => panic!("expected Pattern call, got {other:?}"),
    }

    // Control flow stays a Call with symbol head — full_form does not invent IfStmt.
    let iff = build("If[1, 2]").full_form();
    match &iff.expressions[0] {
        Expression::Call { head, arguments, .. } => {
            match head.as_ref() {
                Expression::Symbol(id) => assert_eq!(id.name, "If"),
                other => panic!("expected If, got {other:?}"),
            }
            assert_eq!(arguments.len(), 2);
        }
        other => panic!("expected If call, got {other:?}"),
    }
}
