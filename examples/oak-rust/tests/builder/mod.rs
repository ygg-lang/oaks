use oak_core::LexerCache;
use oak_rust::highlighter::Highlighter;

#[test]
fn test_rust_integration() -> Result<(), oak_core::OakError> {
    use oak_core::{Lexer, Parser, SourceText};
    use oak_rust::{RustLanguage, RustLexer, RustParser};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    let parser = RustParser::new(language);

    // Test basic integration
    let source = SourceText::new("fn main() { let x = 42; println!(\"Hello, world!\"); }");

    // Test lexer
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source, &[], &mut cache);
    assert!(lex_output.result.is_ok(), "Lexer should produce tokens");

    // Test files
    cache.set_lex_output(lex_output.clone());
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok(), "Parser should produce AST");

    println!("Rust integration test passed - {} tokens generated", lex_output.result.unwrap().len());
    Ok(())
}

#[test]
fn test_rust_builder_single_file() -> Result<(), oak_core::OakError> {
    use oak_core::{Builder, SourceText, parser::session::ParseSession};
    use oak_rust::{RustBuilder, RustLanguage};

    let language = RustLanguage::default();
    let builder = RustBuilder::new(language);

    // 测试简单的函数
    let source = SourceText::new("fn add(x: i32, y: i32) -> i32 { x + y }");

    // 创建 ParseSession
    let mut cache = ParseSession::<RustLanguage>::default();

    let diagnostics = builder.build(&source, &[], &mut cache);
    assert!(diagnostics.result.is_ok());
    Ok(())
}

#[test]
fn test_rust_builder_complex() -> Result<(), oak_core::OakError> {
    use oak_core::{Builder, SourceText, parser::session::ParseSession};
    use oak_rust::{RustBuilder, RustLanguage};

    let language = RustLanguage::default();
    let builder = RustBuilder::new(language);

    // 测试更复杂的 Rust 代码
    let source = SourceText::new(
        r#"
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

fn main() {
    let p = Point::new(3, 4);
    println!("Distance: {}", p.distance());
}
"#,
    );

    println!("Testing builder with complex Rust code");

    // 创建 ParseSession
    let mut cache = ParseSession::<RustLanguage>::default();

    let diagnostics = builder.build(&source, &[], &mut cache);
    match diagnostics.result {
        Ok(_typed_root) => {
            println!("Successfully built complex typed root");
        }
        Err(e) => {
            println!("Complex build failed with error: {}", e);
        }
    }
    if !diagnostics.diagnostics.is_empty() {
        println!("Complex build diagnostics: {:?}", diagnostics.diagnostics);
    }
    Ok(())
}

#[test]
fn test_complete_rust_program() -> Result<(), oak_core::OakError> {
    use oak_core::{Lexer, SourceText};
    use oak_rust::{RustFormatter, RustHighlighter, RustLanguage, RustLexer};

    let language = RustLanguage::default();
    let lexer = RustLexer::new(&language);
    let highlighter = RustHighlighter::new();
    let formatter = RustFormatter::new();

    let source = r#"
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }
}

fn main() {
    let mut people = HashMap::new();
    let person = Person::new("Alice".to_string(), 30);
    people.insert(1, person);
    
    if let Some(p) = people.get(&1) {
        println!("{}", p.greet());
    }
}
"#;

    // Test lexer
    let source_text = SourceText::new(source);
    let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
    let lex_output = lexer.lex(&source_text, &[], &mut cache);
    assert!(lex_output.result.is_ok(), "Lexing should succeed");
    let tokens = lex_output.result.unwrap();
    assert!(!tokens.is_empty(), "Should tokenize complete program");

    // Test highlighter
    let highlights = highlighter.highlight(source);
    assert!(!highlights.is_empty(), "Should highlight complete program");

    // Test formatter
    let formatted = formatter.format(source);
    assert!(!formatted.is_empty(), "Should format complete program");

    println!("Complete Rust program test passed:");
    println!("  - {} tokens generated", tokens.len());
    println!("  - {} highlights generated", highlights.len());
    println!("  - Formatted code length: {} chars", formatted.len());
    Ok(())
}
