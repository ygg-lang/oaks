extern crate alloc;

use alloc::{boxed::Box, string::String, vec::Vec};
use oak_core::SourceLocation;

/// C# 语言抽象语法树
#[derive(Debug, Clone, PartialEq)]
pub struct CSharpAst {
    pub compilation_unit: CompilationUnit,
}

/// 编译单元（C# 程序的顶层结构）
#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    pub using_directives: Vec<UsingDirective>,
    pub namespace_declarations: Vec<NamespaceDeclaration>,
    pub type_declarations: Vec<TypeDeclaration>,
    pub span: SourceSpan,
}

/// 源码位置范围
pub type SourceSpan = SourceLocation;

/// Using 指令
#[derive(Debug, Clone, PartialEq)]
pub struct UsingDirective {
    pub namespace: String,
    pub span: SourceSpan,
}

/// 命名空间声明
#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    pub name: String,
    pub members: Vec<NamespaceMember>,
    pub span: SourceSpan,
}

/// 命名空间成员
#[derive(Debug, Clone, PartialEq)]
pub enum NamespaceMember {
    TypeDeclaration(TypeDeclaration),
    NamespaceDeclaration(NamespaceDeclaration),
}

/// 类型声明
#[derive(Debug, Clone, PartialEq)]
pub enum TypeDeclaration {
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
}

/// 类声明
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub base_types: Vec<String>,
    pub members: Vec<ClassMember>,
    pub span: SourceSpan,
}

/// 接口声明
#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub base_interfaces: Vec<String>,
    pub members: Vec<InterfaceMember>,
    pub span: SourceSpan,
}

/// 结构体声明
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub interfaces: Vec<String>,
    pub members: Vec<StructMember>,
    pub span: SourceSpan,
}

/// 枚举声明
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub base_type: Option<String>,
    pub members: Vec<EnumMember>,
    pub span: SourceSpan,
}

/// 修饰符
#[derive(Debug, Clone, PartialEq)]
pub enum Modifier {
    Public,
    Private,
    Protected,
    Internal,
    Static,
    Virtual,
    Override,
    Abstract,
    Sealed,
    Readonly,
    Const,
}

/// 类成员
#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Constructor(ConstructorDeclaration),
}

/// 接口成员
#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceMember {
    Method(MethodSignature),
    Property(PropertySignature),
}

/// 结构体成员
#[derive(Debug, Clone, PartialEq)]
pub enum StructMember {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Property(PropertyDeclaration),
    Constructor(ConstructorDeclaration),
}

/// 枚举成员
#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<Expression>,
    pub span: SourceSpan,
}

/// 字段声明
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDeclaration {
    pub modifiers: Vec<Modifier>,
    pub type_name: String,
    pub name: String,
    pub initializer: Option<Expression>,
    pub span: SourceSpan,
}

/// 方法声明
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDeclaration {
    pub modifiers: Vec<Modifier>,
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Option<Block>,
    pub span: SourceSpan,
}

/// 方法签名（用于接口）
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub span: SourceSpan,
}

/// 属性声明
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclaration {
    pub modifiers: Vec<Modifier>,
    pub type_name: String,
    pub name: String,
    pub accessors: Vec<Accessor>,
    pub span: SourceSpan,
}

/// 属性签名（用于接口）
#[derive(Debug, Clone, PartialEq)]
pub struct PropertySignature {
    pub type_name: String,
    pub name: String,
    pub accessors: Vec<AccessorSignature>,
    pub span: SourceSpan,
}

/// 构造函数声明
#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
    pub span: SourceSpan,
}

/// 参数
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub type_name: String,
    pub name: String,
    pub default_value: Option<Expression>,
    pub span: SourceSpan,
}

/// 访问器
#[derive(Debug, Clone, PartialEq)]
pub enum Accessor {
    Get(Option<Block>),
    Set(Option<Block>),
}

/// 访问器签名（用于接口）
#[derive(Debug, Clone, PartialEq)]
pub enum AccessorSignature {
    Get,
    Set,
}

/// 代码块
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: SourceSpan,
}

/// 语句
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(ExpressionStatement),
    Variable(VariableDeclaration),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Block(Block),
}

/// 表达式语句
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: SourceSpan,
}

/// 变量声明
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub type_name: String,
    pub name: String,
    pub initializer: Option<Expression>,
    pub span: SourceSpan,
}

/// If 语句
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_statement: Box<Statement>,
    pub else_statement: Option<Box<Statement>>,
    pub span: SourceSpan,
}

/// While 语句
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub span: SourceSpan,
}

/// For 语句
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub initializer: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub body: Box<Statement>,
    pub span: SourceSpan,
}

/// Return 语句
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
    pub span: SourceSpan,
}

/// 表达式
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: SourceSpan,
}

/// 表达式类型
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Identifier(String),
    Literal(Literal),
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Unary { operator: UnaryOperator, operand: Box<Expression> },
    Call { function: Box<Expression>, arguments: Vec<Expression> },
    MemberAccess { object: Box<Expression>, member: String },
    Assignment { left: Box<Expression>, right: Box<Expression> },
}

/// 字面量
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Character(char),
    Boolean(bool),
    Null,
}

/// 二元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}

impl CSharpAst {
    pub fn new(compilation_unit: CompilationUnit) -> Self {
        Self { compilation_unit }
    }
}

impl CompilationUnit {
    pub fn new(
        using_directives: Vec<UsingDirective>,
        namespace_declarations: Vec<NamespaceDeclaration>,
        type_declarations: Vec<TypeDeclaration>,
        span: SourceSpan,
    ) -> Self {
        Self { using_directives, namespace_declarations, type_declarations, span }
    }
}
