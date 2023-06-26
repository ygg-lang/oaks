use oak_fsharp::parse;

#[test]
fn test_basic_syntax() {
    let code = r#"
        let add x y = x + y
        let result = add 5 3
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse basic F# code");
}

#[test]
fn test_modules() {
    let code = r#"
        module Math
        let add x y = x + y
        let multiply x y = x * y
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse modules");
}

#[test]
fn test_open_statements() {
    let code = r#"
        open System
        open System.Collections.Generic
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse open statements");
}

#[test]
fn test_namespaces() {
    let code = r#"
        namespace MyCompany.Project
        let add x y = x + y
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse namespaces");
}

#[test]
fn test_if_expressions() {
    let code = r#"
        let checkNumber n = 
            if n > 0 then "Positive"
            else "Negative"
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse if expressions");
}

#[test]
fn test_let_bindings() {
    let code = r#"
        let x = 10
        let y = 20
        let sum = x + y
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse let bindings");
}

#[test]
fn test_function_bindings() {
    let code = r#"
        let add x y = x + y
        let multiply x y = x * y
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse function bindings");
}

#[test]
fn test_simple_expressions() {
    let code = r#"
        let x = 10
        let y = 20
        let sum = x + y
        let product = x * y
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse simple expressions");
}

#[test]
fn test_literal_expressions() {
    let code = r#"
        let intLiteral = 123
        let floatLiteral = 123.45
        let stringLiteral = "Hello"
        let charLiteral = 'A'
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse literal expressions");
}

#[test]
fn test_simple_complex_code() {
    let code = r#"
        module Program
        open System
        
        let add x y = x + y
        let multiply x y = x * y
        
        let result = add 5 (multiply 3 2)
        printfn "Result: %d" result
    "#;

    let result = parse(code);
    assert!(result.is_ok(), "Failed to parse simple complex code");
}
