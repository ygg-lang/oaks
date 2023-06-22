mod lexer;
mod parser;

#[test]
fn test_basic_compilation() {
    // 基本的编译测试，确保所有模块都能正确编译
    use oak_go::{GoLanguage, GoLexer, GoSyntaxKind};

    // 创建语言实例
    let language = GoLanguage::default();

    // 创建词法分析器
    let _lexer = GoLexer::new(&language);

    // 测试语法种类
    let _kind = GoSyntaxKind::Package;

    println!("Basic compilation test passed");
}

#[test]
fn test_syntax_kinds() {
    use oak_go::GoSyntaxKind;

    // 测试各种语法种类
    let _keywords = [GoSyntaxKind::Package, GoSyntaxKind::Import, GoSyntaxKind::Func, GoSyntaxKind::Var, GoSyntaxKind::Const, GoSyntaxKind::If, GoSyntaxKind::Else, GoSyntaxKind::For, GoSyntaxKind::Range, GoSyntaxKind::Return];

    let _literals = [GoSyntaxKind::BoolLiteral, GoSyntaxKind::NilLiteral, GoSyntaxKind::IntLiteral, GoSyntaxKind::FloatLiteral, GoSyntaxKind::StringLiteral, GoSyntaxKind::RuneLiteral, GoSyntaxKind::NumberLiteral];

    println!("Syntax kinds test passed");
}
