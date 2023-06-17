use oak_core::{SourceText, lexer::Lexer};
use oak_wat::{WatLanguage, WatLexer, WatSyntaxKind};

#[test]
fn test_wat_lexer_basic_tokens() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(module (func $add (param i32 i32) (result i32)))");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Check for expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&WatSyntaxKind::LeftParen));
    assert!(token_kinds.contains(&WatSyntaxKind::RightParen));
    assert!(token_kinds.contains(&WatSyntaxKind::ModuleKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::FuncKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ParamKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ResultKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::I32));
    assert!(token_kinds.contains(&WatSyntaxKind::Identifier));
}

#[test]
fn test_wat_lexer_module() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(module)");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&WatSyntaxKind::LeftParen));
    assert!(token_kinds.contains(&WatSyntaxKind::ModuleKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::RightParen));
}

#[test]
fn test_wat_lexer_function() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(func $test (param $x i32) (param $y i64) (result f32))");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&WatSyntaxKind::FuncKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ParamKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ResultKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::I32));
    assert!(token_kinds.contains(&WatSyntaxKind::I64));
    assert!(token_kinds.contains(&WatSyntaxKind::F32));

    let identifier_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::Identifier).collect();
    assert_eq!(identifier_tokens.len(), 3); // $test, $x, $y
}

#[test]
fn test_wat_lexer_memory() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(memory $mem 1)");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&WatSyntaxKind::MemoryKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::Identifier)); // $mem
    assert!(token_kinds.contains(&WatSyntaxKind::Number)); // 1
}

#[test]
fn test_wat_lexer_export_import() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new(
        r#"(export "add" (func $add))
(import "env" "memory" (memory 1))"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&WatSyntaxKind::ExportKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ImportKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::String));
    assert!(token_kinds.contains(&WatSyntaxKind::FuncKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::MemoryKeyword));
}

#[test]
fn test_wat_lexer_types() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(param i32) (param i64) (param f32) (param f64)");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();

    assert!(token_kinds.contains(&WatSyntaxKind::I32));
    assert!(token_kinds.contains(&WatSyntaxKind::I64));
    assert!(token_kinds.contains(&WatSyntaxKind::F32));
    assert!(token_kinds.contains(&WatSyntaxKind::F64));

    let param_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::ParamKeyword).collect();
    assert_eq!(param_tokens.len(), 4);
}

#[test]
fn test_wat_lexer_strings() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new(
        r#"(export "function_name" (func $fn))
(import "module" "function" (func))"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let string_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::String).collect();

    assert_eq!(string_tokens.len(), 3); // "function_name", "module", "function"
}

#[test]
fn test_wat_lexer_numbers() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(memory 1) (i32.const 42) (f32.const 3.14) (i64.const -100)");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let number_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::Number).collect();

    assert_eq!(number_tokens.len(), 4); // 1, 42, 3.14, -100
}

#[test]
fn test_wat_lexer_identifiers() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("(func $add (param $x i32) (param $y i32) (local $temp i32))");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let identifier_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::Identifier).collect();

    assert_eq!(identifier_tokens.len(), 4); // $add, $x, $y, $temp
}

#[test]
fn test_wat_lexer_comments() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new(
        r#";; This is a line comment
(module ;; End of line comment
  (func $test) ;; Another comment
  (; This is a block comment ;)
)"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let comment_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::Comment).collect();

    assert_eq!(comment_tokens.len(), 4);
}

#[test]
fn test_wat_lexer_whitespace() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new("  (module  \n  (func  $test)  \n)  ");

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    let whitespace_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == WatSyntaxKind::Whitespace).collect();

    assert!(!whitespace_tokens.is_empty());
}

#[test]
fn test_wat_lexer_complex_example() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new(
        r#";; A simple WebAssembly module
(module
  ;; Import memory from the environment
  (import "env" "memory" (memory 1))
  
  ;; Export a function
  (export "add" (func $add))
  
  ;; Define the add function
  (func $add (param $x i32) (param $y i32) (result i32)
    (local $temp i32)
    local.get $x
    local.get $y
    i32.add
  )
)"#,
    );

    let result = lexer.tokenize_source(&source);
    assert!(result.result.is_ok());

    let tokens = result.result.unwrap();
    assert!(!tokens.is_empty());

    // Verify we have all expected kind types
    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
    assert!(token_kinds.contains(&WatSyntaxKind::Comment));
    assert!(token_kinds.contains(&WatSyntaxKind::LeftParen));
    assert!(token_kinds.contains(&WatSyntaxKind::RightParen));
    assert!(token_kinds.contains(&WatSyntaxKind::ModuleKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ImportKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ExportKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::FuncKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ParamKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::ResultKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::LocalKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::MemoryKeyword));
    assert!(token_kinds.contains(&WatSyntaxKind::I32));
    assert!(token_kinds.contains(&WatSyntaxKind::String));
    assert!(token_kinds.contains(&WatSyntaxKind::Number));
    assert!(token_kinds.contains(&WatSyntaxKind::Identifier));
}

#[test]
fn test_wat_lexer_error_handling() {
    let config = WatLanguage::default();
    let lexer = WatLexer::new(&config);
    let source = SourceText::new(r#"(module "unterminated string)"#);

    let result = lexer.tokenize_source(&source);
    // Should have diagnostics for unterminated string
    assert!(!result.diagnostics.is_empty());
}
