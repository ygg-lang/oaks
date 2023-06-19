mod lexer;

#[test]
fn test_basic_compilation() {
    // 基本的编译测试，确保所有模块都能正确编译
    use oak_go::{GoLangLanguage, GoLangSyntaxKind, GoLexer};

    // 创建语言实例
    let language = GoLangLanguage {};

    // 创建词法分析器
    let lexer = GoLexer::new(&language);

    // 测试语法种类
    let _kind = GoLangSyntaxKind::Package;

    println!("Basic compilation test passed");
}

#[test]
fn test_syntax_kinds() {
    use oak_go::GoLangSyntaxKind;

    // 测试各种语法种类
    let _keywords = [
        GoLangSyntaxKind::Package,
        GoLangSyntaxKind::Import,
        GoLangSyntaxKind::Func,
        GoLangSyntaxKind::Var,
        GoLangSyntaxKind::Const,
        GoLangSyntaxKind::If,
        GoLangSyntaxKind::Else,
        GoLangSyntaxKind::For,
        GoLangSyntaxKind::Range,
        GoLangSyntaxKind::Return,
    ];

    let _literals = [
        GoLangSyntaxKind::BoolLiteral,
        GoLangSyntaxKind::NilLiteral,
        GoLangSyntaxKind::IntLiteral,
        GoLangSyntaxKind::FloatLiteral,
        GoLangSyntaxKind::StringLiteral,
        GoLangSyntaxKind::RuneLiteral,
        GoLangSyntaxKind::NumberLiteral,
    ];

    println!("Syntax kinds test passed");
}
