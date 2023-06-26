mod lexer;
mod parser;

#[test]
fn test_basic_compilation() {
    // Basic compilation test, ensuring all modules compile correctly.
    use oak_go::{GoLanguage, GoLexer, GoTokenType};

    // Create language instance.
    let language = GoLanguage::default();

    // Create lexer.
    let _lexer = GoLexer::new(&language);

    // Test token types.
    let _kind = GoTokenType::Package;

    println!("Basic compilation test passed")
}

#[test]
fn test_token_types() {
    use oak_go::GoTokenType;

    // Test various token types.
    let _keywords = [GoTokenType::Package, GoTokenType::Import, GoTokenType::Func, GoTokenType::Var, GoTokenType::Const, GoTokenType::If, GoTokenType::Else, GoTokenType::For, GoTokenType::Range, GoTokenType::Return];

    let _literals = [GoTokenType::BoolLiteral, GoTokenType::NilLiteral, GoTokenType::IntLiteral, GoTokenType::FloatLiteral, GoTokenType::StringLiteral, GoTokenType::RuneLiteral, GoTokenType::NumberLiteral];

    println!("Syntax kinds test passed")
}
