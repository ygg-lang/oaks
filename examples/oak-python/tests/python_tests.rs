use oak_core::{Builder, OakError, parser::session::ParseSession, source::SourceText};
use oak_python::{PythonBuilder, PythonLanguage};

#[test]
fn test_python_parsing() -> Result<(), OakError> {
    let code = SourceText::new(
        "
def hello(x):
    print(x)
",
    );
    let config = PythonLanguage::default();
    let builder = PythonBuilder::new(config);
    let mut session = ParseSession::<PythonLanguage>::default();

    // Test building the typed AST
    let output = builder.build(&code, &[], &mut session);
    assert!(output.result.is_ok());
    Ok(())
}

#[test]
fn test_python_ast_structure() -> Result<(), OakError> {
    let code = SourceText::new(
        "
def add(a, b):
    return a + b

x = 10
y = 20
z = add(x, y)
",
    );
    let config = PythonLanguage::default();
    let builder = PythonBuilder::new(config);
    let mut session = ParseSession::<PythonLanguage>::default();

    let output = builder.build(&code, &[], &mut session);
    let root = output.result.expect("Failed to build AST");

    // Check if we have the expected number of statements
    // 1. FunctionDef (add)
    // 2. Assignment (x = 10)
    // 3. Assignment (y = 20)
    // 4. Assignment (z = add(x, y))

    let stmts = &root.program.statements;
    for (i, stmt) in stmts.iter().enumerate() {
        println!("Stmt {}: {:?}", i, stmt);
    }
    assert!(stmts.len() >= 4, "Expected at least 4 statements, got {}", stmts.len());

    match &stmts[0] {
        oak_python::ast::Statement::FunctionDef { name, parameters, .. } => {
            assert_eq!(name, "add");
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0].name, "a");
            assert_eq!(parameters[1].name, "b");
        }
        _ => panic!("First statement should be FunctionDef"),
    }

    match &stmts[1] {
        oak_python::ast::Statement::Assignment { target, value } => {
            match target {
                oak_python::ast::Expression::Name(name) => assert_eq!(name, "x"),
                _ => panic!("Expected target to be Name"),
            }
            match value {
                oak_python::ast::Expression::Literal(oak_python::ast::Literal::Integer(val)) => assert_eq!(*val, 10),
                _ => panic!("Expected value to be Integer literal"),
            }
        }
        _ => panic!("Second statement should be Assignment"),
    }
    Ok(())
}
