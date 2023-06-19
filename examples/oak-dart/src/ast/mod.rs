use core::range::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone)]
pub struct DartRoot {
    pub items: Vec<Item>,
}

/// 顶层项：类、函数、变量声明等
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub name: Identifier,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub span: Range<usize>,
}
