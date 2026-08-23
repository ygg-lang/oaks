mod lexer;
mod parser;

#[test]
fn test_basic_compilation() {
    // 基本的编译测试，确保所有模块都能正确编译
    use oak_go::{GoLanguage, GoLexer, GoTokenType};

    // 创建语言实例
    let language = GoLanguage::default();

    // 创建词法分析器
    let _lexer = GoLexer::new(&language);

    // 测试语法种类
    let _kind = GoTokenType::Package;

    println!("Basic compilation test passed")
}

#[test]
fn test_syntax_kinds() {
    use oak_go::GoTokenType;

    // 测试各种语法种类
    let _keywords = [GoTokenType::Package, GoTokenType::Import, GoTokenType::Func, GoTokenType::Var, GoTokenType::Const, GoTokenType::If, GoTokenType::Else, GoTokenType::For, GoTokenType::Range, GoTokenType::Return];

    let _literals = [GoTokenType::BoolLiteral, GoTokenType::NilLiteral, GoTokenType::IntLiteral, GoTokenType::FloatLiteral, GoTokenType::StringLiteral, GoTokenType::RuneLiteral, GoTokenType::NumberLiteral];

    println!("Syntax kinds test passed")
}
