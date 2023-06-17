use alloc::{boxed::Box, string::String, vec::Vec};

/// C 语言抽象语法
#[derive(Debug, Clone, PartialEq)]
pub struct CRoot {
    pub translation_unit: TranslationUnit,
}

/// 翻译单元（C 程序的顶层结构）
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
    pub span: core::range::Range<usize>,
}

/// 外部声明
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDeclaration {
    /// 函数定义
    FunctionDefinition(FunctionDefinition),
    /// 声明
    Declaration(Declaration),
}

/// 函数定义
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Declarator,
    pub compound_statement: CompoundStatement,
    pub span: core::range::Range<usize>,
}

/// 声明
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub init_declarators: Vec<InitDeclarator>,
    pub span: core::range::Range<usize>,
}

/// 声明说明
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationSpecifier {
    /// 存储类说明符
    StorageClassSpecifier(StorageClassSpecifier),
    /// 类型说明
    TypeSpecifier(TypeSpecifier),
    /// 类型限定
    TypeQualifier(TypeQualifier),
    /// 函数说明
    FunctionSpecifier(FunctionSpecifier),
}

/// 存储类说明符
#[derive(Debug, Clone, PartialEq)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
}

/// 类型说明
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Complex,
    Imaginary,
    StructOrUnion(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(String),
}

/// 类型限定
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
}

/// 函数说明
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionSpecifier {
    Inline,
    Noreturn,
}

/// 结构体或联合体说明符
#[derive(Debug, Clone, PartialEq)]
pub struct StructOrUnionSpecifier {
    pub struct_or_union: StructOrUnion,
    pub identifier: Option<String>,
    pub struct_declarations: Option<Vec<StructDeclaration>>,
    pub span: core::range::Range<usize>,
}

/// 结构体或联合
#[derive(Debug, Clone, PartialEq)]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// 结构体声
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub struct_declarators: Vec<StructDeclarator>,
    pub span: core::range::Range<usize>,
}

/// 说明符限定符
#[derive(Debug, Clone, PartialEq)]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
}

/// 结构体声明符
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclarator {
    pub declarator: Option<Declarator>,
    pub constant_expression: Option<Expression>,
    pub span: core::range::Range<usize>,
}

/// 枚举说明
#[derive(Debug, Clone, PartialEq)]
pub struct EnumSpecifier {
    pub identifier: Option<String>,
    pub enumerators: Option<Vec<Enumerator>>,
    pub span: core::range::Range<usize>,
}

/// 枚举
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerator {
    pub identifier: String,
    pub constant_expression: Option<Expression>,
    pub span: core::range::Range<usize>,
}

/// 初始化声明符
#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
    pub span: core::range::Range<usize>,
}

/// 声明
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    pub pointer: Option<Pointer>,
    pub direct_declarator: DirectDeclarator,
    pub span: core::range::Range<usize>,
}

/// 指针
#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
    pub type_qualifiers: Vec<TypeQualifier>,
    pub pointer: Option<Box<Pointer>>,
    pub span: core::range::Range<usize>,
}

/// 直接声明
#[derive(Debug, Clone, PartialEq)]
pub enum DirectDeclarator {
    Identifier(String),
    Declarator(Box<Declarator>),
    Array {
        declarator: Box<DirectDeclarator>,
        assignment_expression: Option<Expression>,
    },
    Function {
        declarator: Box<DirectDeclarator>,
        parameter_type_list: Option<ParameterTypeList>,
        identifier_list: Option<Vec<String>>,
    },
}

/// 参数类型列表
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterTypeList {
    pub parameter_list: Vec<ParameterDeclaration>,
    pub variadic: bool,
    pub span: core::range::Range<usize>,
}

/// 参数声明
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Option<Declarator>,
    pub abstract_declarator: Option<AbstractDeclarator>,
    pub span: core::range::Range<usize>,
}

/// 抽象声明
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractDeclarator {
    pub pointer: Option<Pointer>,
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    pub span: core::range::Range<usize>,
}

/// 直接抽象声明
#[derive(Debug, Clone, PartialEq)]
pub enum DirectAbstractDeclarator {
    AbstractDeclarator(Box<AbstractDeclarator>),
    Array { declarator: Option<Box<DirectAbstractDeclarator>>, assignment_expression: Option<Box<Expression>> },
    Function { declarator: Option<Box<DirectAbstractDeclarator>>, parameter_type_list: Option<ParameterTypeList> },
}

/// 初始化器
#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    AssignmentExpression(Expression),
    InitializerList(Vec<Initializer>),
}

/// 语句
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// 标号语句
    Labeled(LabeledStatement),
    /// 复合语句
    Compound(CompoundStatement),
    /// 表达式语
    Expression(ExpressionStatement),
    /// 选择语句
    Selection(SelectionStatement),
    /// 迭代语句
    Iteration(IterationStatement),
    /// 跳转语句
    Jump(JumpStatement),
}

/// 标号语句
#[derive(Debug, Clone, PartialEq)]
pub enum LabeledStatement {
    Label { identifier: String, statement: Box<Statement> },
    Case { constant_expression: Expression, statement: Box<Statement> },
    Default { statement: Box<Statement> },
}

/// 复合语句
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundStatement {
    pub block_items: Vec<BlockItem>,
    pub span: core::range::Range<usize>,
}

/// 块项
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

/// 表达式语
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Option<Expression>,
    pub span: core::range::Range<usize>,
}

/// 选择语句
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStatement {
    If { condition: Expression, then_statement: Box<Statement>, else_statement: Option<Box<Statement>> },
    Switch { expression: Expression, statement: Box<Statement> },
}

/// 迭代语句
#[derive(Debug, Clone, PartialEq)]
pub enum IterationStatement {
    While { condition: Expression, statement: Box<Statement> },
    DoWhile { statement: Box<Statement>, condition: Expression },
    For { init: Option<Expression>, condition: Option<Expression>, update: Option<Expression>, statement: Box<Statement> },
}

/// 跳转语句
#[derive(Debug, Clone, PartialEq)]
pub enum JumpStatement {
    Goto(String),
    Continue,
    Break,
    Return(Option<Expression>),
}

/// 表达
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: Box<ExpressionKind>,
    pub span: core::range::Range<usize>,
}

/// 表达式种
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    /// 标识
    Identifier(String),
    /// 常量
    Constant(Constant),
    /// 字符串字面量
    StringLiteral(String),
    /// 数组下标
    ArraySubscript { array: Box<Expression>, index: Box<Expression> },
    /// 函数调用
    FunctionCall { function: Box<Expression>, arguments: Vec<Expression> },
    /// 成员访问
    MemberAccess {
        object: Box<Expression>,
        member: String,
        is_pointer: bool, // true for ->, false for .
    },
    /// 后缀递增/递减
    PostfixIncDec { operand: Box<Expression>, is_increment: bool },
    /// 前缀递增/递减
    PrefixIncDec { operand: Box<Expression>, is_increment: bool },
    /// 一元操作符
    Unary { operator: UnaryOperator, operand: Box<Expression> },
    /// 类型转换
    Cast { type_name: Box<TypeName>, expression: Box<Expression> },
    /// 二元操作
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    /// 条件表达
    Conditional { condition: Box<Expression>, then_expr: Box<Expression>, else_expr: Box<Expression> },
    /// 赋值表达式
    Assignment { left: Box<Expression>, operator: AssignmentOperator, right: Box<Expression> },
    /// 逗号表达
    Comma { expressions: Vec<Expression> },
}

/// 常量
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    Character(char),
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    AddressOf,   // &
    Dereference, // *
    Plus,        // +
    Minus,       // -
    BitwiseNot,  // ~
    LogicalNot,  // !
    Sizeof,      // sizeof
}

/// 二元操作
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // 算术操作
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // 位操作符
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    LeftShift,  // <<
    RightShift, // >>

    // 比较操作
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // 逻辑操作
    LogicalAnd, // &&
    LogicalOr,  // ||
}

/// 赋值操作符
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign,           // =
    AddAssign,        // +=
    SubAssign,        // -=
    MulAssign,        // *=
    DivAssign,        // /=
    ModAssign,        // %=
    AndAssign,        // &=
    OrAssign,         // |=
    XorAssign,        // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=
}

/// 类型
#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    pub span: core::range::Range<usize>,
}

impl CRoot {
    /// 创建新的 AST
    pub fn new(translation_unit: TranslationUnit) -> Self {
        Self { translation_unit }
    }
}

impl TranslationUnit {
    /// 创建新的翻译单元
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: core::range::Range<usize>) -> Self {
        Self { external_declarations, span }
    }
}
