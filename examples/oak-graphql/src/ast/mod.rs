use core::range::Range;

type SourceSpan = Range<usize>;

/// GraphQL 根节点
#[derive(Debug, Clone, PartialEq)]
pub struct GraphQLRoot {
    pub document: Document,
}

/// GraphQL 文档
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub definitions: Vec<Definition>,
    pub span: SourceSpan,
}

/// GraphQL 定义
#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
    Schema(SchemaDefinition),
    Type(TypeDefinition),
}

/// GraphQL 操作定义
#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinition {
    pub operation_type: OperationType,
    pub name: Option<String>,
    pub span: SourceSpan,
}

/// GraphQL 操作类型
#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

/// GraphQL 片段定义
#[derive(Debug, Clone, PartialEq)]
pub struct FragmentDefinition {
    pub name: String,
    pub span: SourceSpan,
}

/// GraphQL 模式定义
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaDefinition {
    pub span: SourceSpan,
}

/// GraphQL 类型定义
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefinition {
    pub name: String,
    pub span: SourceSpan,
}

/// C AST 根节点
#[derive(Debug, Clone, PartialEq)]
pub struct CAst {
    pub translation_unit: TranslationUnit,
}

/// 翻译单元
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit {
    pub external_declarations: Vec<ExternalDeclaration>,
    pub span: SourceSpan,
}

/// 外部声明
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration),
}

/// 函数定义
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Declarator,
    pub compound_statement: CompoundStatement,
    pub span: SourceSpan,
}

/// 声明
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub init_declarators: Vec<InitDeclarator>,
    pub span: SourceSpan,
}

/// 声明说明符
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationSpecifier {
    StorageClassSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
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

/// 类型说明符
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

/// 类型限定符
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
}

/// 函数说明符
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
    pub span: SourceSpan,
}

/// 结构体或联合体
#[derive(Debug, Clone, PartialEq)]
pub enum StructOrUnion {
    Struct,
    Union,
}

/// 结构体声明
#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub struct_declarators: Vec<StructDeclarator>,
    pub span: SourceSpan,
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
    pub span: SourceSpan,
}

/// 枚举说明符
#[derive(Debug, Clone, PartialEq)]
pub struct EnumSpecifier {
    pub identifier: Option<String>,
    pub enumerators: Option<Vec<Enumerator>>,
    pub span: SourceSpan,
}

/// 枚举器
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerator {
    pub identifier: String,
    pub constant_expression: Option<Expression>,
    pub span: SourceSpan,
}

/// 初始化声明符
#[derive(Debug, Clone, PartialEq)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>,
    pub span: SourceSpan,
}

/// 声明符
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    pub pointer: Option<Pointer>,
    pub direct_declarator: DirectDeclarator,
    pub span: SourceSpan,
}

/// 指针
#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
    pub type_qualifiers: Vec<TypeQualifier>,
    pub pointer: Option<Box<Pointer>>,
    pub span: SourceSpan,
}

/// 直接声明符
#[derive(Debug, Clone, PartialEq)]
pub enum DirectDeclarator {
    Identifier(String),
    Declarator(Box<Declarator>),
    Array { declarator: Box<DirectDeclarator>, assignment_expression: Option<Expression> },
    Function { declarator: Box<DirectDeclarator>, parameter_type_list: Option<ParameterTypeList>, identifier_list: Option<Vec<String>> },
}

/// 参数类型列表
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterTypeList {
    pub parameter_list: Vec<ParameterDeclaration>,
    pub variadic: bool,
    pub span: SourceSpan,
}

/// 参数声明
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Option<Declarator>,
    pub abstract_declarator: Option<AbstractDeclarator>,
    pub span: SourceSpan,
}

/// 抽象声明符
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractDeclarator {
    pub pointer: Option<Pointer>,
    pub direct_abstract_declarator: Option<Box<DirectAbstractDeclarator>>,
    pub span: SourceSpan,
}

/// 直接抽象声明符
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
    Labeled(LabeledStatement),
    Compound(CompoundStatement),
    Expression(ExpressionStatement),
    Selection(SelectionStatement),
    Iteration(IterationStatement),
    Jump(JumpStatement),
}

/// 标签语句
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
    pub span: SourceSpan,
}

/// 块项
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Statement),
}

/// 表达式语句
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Option<Expression>,
    pub span: SourceSpan,
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

/// 表达式
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: Box<ExpressionKind>,
    pub span: SourceSpan,
}

/// 表达式类型
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Identifier(String),
    Constant(Constant),
    StringLiteral(String),
    ArraySubscript {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
        is_pointer: bool, // true for ->, false for .
    },
    PostfixIncDec {
        operand: Box<Expression>,
        is_increment: bool,
    },
    PrefixIncDec {
        operand: Box<Expression>,
        is_increment: bool,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Cast {
        type_name: Box<TypeName>,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Conditional {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Assignment {
        left: Box<Expression>,
        operator: AssignmentOperator,
        right: Box<Expression>,
    },
    Comma {
        expressions: Vec<Expression>,
    },
}

/// 常量
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    Character(char),
}

/// 一元运算符
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

/// 二元运算符
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // 算术运算符
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // 位运算符
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    LeftShift,  // <<
    RightShift, // >>

    // 比较运算符
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // 逻辑运算符
    LogicalAnd, // &&
    LogicalOr,  // ||
}

/// 赋值运算符
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

/// 类型名
#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    pub span: SourceSpan,
}

impl CAst {
    pub fn new(translation_unit: TranslationUnit) -> Self {
        Self { translation_unit }
    }
}

impl TranslationUnit {
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: SourceSpan) -> Self {
        Self { external_declarations, span }
    }
}
