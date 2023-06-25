mod lexer;
mod parser;

#[test]
fn test_basic_compilation() {
    // Basic compilation test, ensuring all modules compile correctly.
    use oak_go::{GoLanguage, GoLexer, GoSyntaxKind};

    // Create language instance.
    let language = GoLanguage::default();

    // Create lexer.
    let _lexer = GoLexer::new(&language);

    // Test syntax kinds.
    let _kind = GoSyntaxKind::Package;

    println!("Basic compilation test passed")
}

#[test]
fn test_syntax_kinds() {
    use oak_go::GoSyntaxKind;

    // Test various syntax kinds.
    let _keywords = [GoSyntaxKind::Package, GoSyntaxKind::Import, GoSyntaxKind::Func, GoSyntaxKind::Var, GoSyntaxKind::Const, GoSyntaxKind::If, GoSyntaxKind::Else, GoSyntaxKind::For, GoSyntaxKind::Range, GoSyntaxKind::Return];

    let _literals = [GoSyntaxKind::BoolLiteral, GoSyntaxKind::NilLiteral, GoSyntaxKind::IntLiteral, GoSyntaxKind::FloatLiteral, GoSyntaxKind::StringLiteral, GoSyntaxKind::RuneLiteral, GoSyntaxKind::NumberLiteral];

    println!("Syntax kinds test passed")
}
