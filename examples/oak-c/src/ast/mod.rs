/// C 语言抽象语法
#[derive(Debug, Clone, PartialEq)]
pub struct CRoot {
    pub translation_unit: TranslationUnit,
    pub span: core::range::Range<usize>,
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

impl ExternalDeclaration {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::FunctionDefinition(n) => n.span.clone(),
            Self::Declaration(n) => n.span.clone(),
        }
    }
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

impl DeclarationSpecifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::StorageClassSpecifier(n) => n.span(),
            Self::TypeSpecifier(n) => n.span(),
            Self::TypeQualifier(n) => n.span(),
            Self::FunctionSpecifier(n) => n.span(),
        }
    }
}

/// 存储类说明符
#[derive(Debug, Clone, PartialEq)]
pub enum StorageClassSpecifier {
    Typedef { span: core::range::Range<usize> },
    Extern { span: core::range::Range<usize> },
    Static { span: core::range::Range<usize> },
    Auto { span: core::range::Range<usize> },
    Register { span: core::range::Range<usize> },
}

impl StorageClassSpecifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Typedef { span } => span.clone(),
            Self::Extern { span } => span.clone(),
            Self::Static { span } => span.clone(),
            Self::Auto { span } => span.clone(),
            Self::Register { span } => span.clone(),
        }
    }
}

/// 类型说明
#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    Void { span: core::range::Range<usize> },
    Char { span: core::range::Range<usize> },
    Short { span: core::range::Range<usize> },
    Int { span: core::range::Range<usize> },
    Long { span: core::range::Range<usize> },
    Float { span: core::range::Range<usize> },
    Double { span: core::range::Range<usize> },
    Signed { span: core::range::Range<usize> },
    Unsigned { span: core::range::Range<usize> },
    Bool { span: core::range::Range<usize> },
    Complex { span: core::range::Range<usize> },
    Imaginary { span: core::range::Range<usize> },
    StructOrUnion(StructOrUnionSpecifier),
    Enum(EnumSpecifier),
    TypedefName(String, core::range::Range<usize>),
}

impl TypeSpecifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Void { span } => span.clone(),
            Self::Char { span } => span.clone(),
            Self::Short { span } => span.clone(),
            Self::Int { span } => span.clone(),
            Self::Long { span } => span.clone(),
            Self::Float { span } => span.clone(),
            Self::Double { span } => span.clone(),
            Self::Signed { span } => span.clone(),
            Self::Unsigned { span } => span.clone(),
            Self::Bool { span } => span.clone(),
            Self::Complex { span } => span.clone(),
            Self::Imaginary { span } => span.clone(),
            Self::StructOrUnion(n) => n.span.clone(),
            Self::Enum(n) => n.span.clone(),
            Self::TypedefName(_, span) => span.clone(),
        }
    }
}

/// 类型限定
#[derive(Debug, Clone, PartialEq)]
pub enum TypeQualifier {
    Const { span: core::range::Range<usize> },
    Restrict { span: core::range::Range<usize> },
    Volatile { span: core::range::Range<usize> },
}

impl TypeQualifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Const { span } => span.clone(),
            Self::Restrict { span } => span.clone(),
            Self::Volatile { span } => span.clone(),
        }
    }
}

/// 函数说明
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionSpecifier {
    Inline { span: core::range::Range<usize> },
    Noreturn { span: core::range::Range<usize> },
}

impl FunctionSpecifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Inline { span } => span.clone(),
            Self::Noreturn { span } => span.clone(),
        }
    }
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
    Struct { span: core::range::Range<usize> },
    Union { span: core::range::Range<usize> },
}

impl StructOrUnion {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Struct { span } => span.clone(),
            Self::Union { span } => span.clone(),
        }
    }
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

impl SpecifierQualifier {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::TypeSpecifier(n) => n.span(),
            Self::TypeQualifier(n) => n.span(),
        }
    }
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
    Identifier(String, core::range::Range<usize>),
    Declarator(Box<Declarator>),
    Array { declarator: Box<DirectDeclarator>, assignment_expression: Option<Expression>, span: core::range::Range<usize> },
    Function { declarator: Box<DirectDeclarator>, parameter_type_list: Option<ParameterTypeList>, identifier_list: Option<Vec<String>>, span: core::range::Range<usize> },
}

impl DirectDeclarator {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Identifier(_, span) => span.clone(),
            Self::Declarator(n) => n.span.clone(),
            Self::Array { span, .. } => span.clone(),
            Self::Function { span, .. } => span.clone(),
        }
    }
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
    Array { declarator: Option<Box<DirectAbstractDeclarator>>, assignment_expression: Option<Box<Expression>>, span: core::range::Range<usize> },
    Function { declarator: Option<Box<DirectAbstractDeclarator>>, parameter_type_list: Option<ParameterTypeList>, span: core::range::Range<usize> },
}

impl DirectAbstractDeclarator {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::AbstractDeclarator(n) => n.span.clone(),
            Self::Array { span, .. } => span.clone(),
            Self::Function { span, .. } => span.clone(),
        }
    }
}

/// 初始化器
#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    AssignmentExpression(Expression),
    InitializerList(Vec<Initializer>, core::range::Range<usize>),
}

impl Initializer {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::AssignmentExpression(n) => n.span.clone(),
            Self::InitializerList(_, span) => span.clone(),
        }
    }
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

impl Statement {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Labeled(n) => n.span(),
            Self::Compound(n) => n.span.clone(),
            Self::Expression(n) => n.span.clone(),
            Self::Selection(n) => n.span(),
            Self::Iteration(n) => n.span(),
            Self::Jump(n) => n.span(),
        }
    }
}

/// 标号语句
#[derive(Debug, Clone, PartialEq)]
pub enum LabeledStatement {
    Label { identifier: String, statement: Box<Statement>, span: core::range::Range<usize> },
    Case { constant_expression: Expression, statement: Box<Statement>, span: core::range::Range<usize> },
    Default { statement: Box<Statement>, span: core::range::Range<usize> },
}

impl LabeledStatement {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Label { span, .. } => span.clone(),
            Self::Case { span, .. } => span.clone(),
            Self::Default { span, .. } => span.clone(),
        }
    }
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

impl BlockItem {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Declaration(n) => n.span.clone(),
            Self::Statement(n) => n.span(),
        }
    }
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
    If { condition: Expression, then_statement: Box<Statement>, else_statement: Option<Box<Statement>>, span: core::range::Range<usize> },
    Switch { expression: Expression, statement: Box<Statement>, span: core::range::Range<usize> },
}

impl SelectionStatement {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::If { span, .. } => span.clone(),
            Self::Switch { span, .. } => span.clone(),
        }
    }
}

/// 迭代语句
#[derive(Debug, Clone, PartialEq)]
pub enum IterationStatement {
    While { condition: Expression, statement: Box<Statement>, span: core::range::Range<usize> },
    DoWhile { statement: Box<Statement>, condition: Expression, span: core::range::Range<usize> },
    For { init: Option<Expression>, condition: Option<Expression>, update: Option<Expression>, statement: Box<Statement>, span: core::range::Range<usize> },
}

impl IterationStatement {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::While { span, .. } => span.clone(),
            Self::DoWhile { span, .. } => span.clone(),
            Self::For { span, .. } => span.clone(),
        }
    }
}

/// 跳转语句
#[derive(Debug, Clone, PartialEq)]
pub enum JumpStatement {
    Goto(String, core::range::Range<usize>),
    Continue(core::range::Range<usize>),
    Break(core::range::Range<usize>),
    Return(Option<Expression>, core::range::Range<usize>),
}

impl JumpStatement {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Goto(_, span) => span.clone(),
            Self::Continue(span) => span.clone(),
            Self::Break(span) => span.clone(),
            Self::Return(_, span) => span.clone(),
        }
    }
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
    Identifier(String, core::range::Range<usize>),
    /// 常量
    Constant(Constant, core::range::Range<usize>),
    /// 字符串字面量
    StringLiteral(String, core::range::Range<usize>),
    /// 数组下标
    ArraySubscript { array: Box<Expression>, index: Box<Expression>, span: core::range::Range<usize> },
    /// 函数调用
    FunctionCall { function: Box<Expression>, arguments: Vec<Expression>, span: core::range::Range<usize> },
    /// 成员访问
    MemberAccess {
        object: Box<Expression>,
        member: String,
        is_pointer: bool, // true for ->, false for .
        span: core::range::Range<usize>,
    },
    /// 后缀递增/递减
    PostfixIncDec { operand: Box<Expression>, is_increment: bool, span: core::range::Range<usize> },
    /// 前缀递增/递减
    PrefixIncDec { operand: Box<Expression>, is_increment: bool, span: core::range::Range<usize> },
    /// 一元操作符
    Unary { operator: UnaryOperator, operand: Box<Expression>, span: core::range::Range<usize> },
    /// 类型转换
    Cast { type_name: Box<TypeName>, expression: Box<Expression>, span: core::range::Range<usize> },
    /// 二元操作
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>, span: core::range::Range<usize> },
    /// 条件表达
    Conditional { condition: Box<Expression>, then_expr: Box<Expression>, else_expr: Box<Expression>, span: core::range::Range<usize> },
    /// 赋值表达式
    Assignment { left: Box<Expression>, operator: AssignmentOperator, right: Box<Expression>, span: core::range::Range<usize> },
    /// 逗号表达
    Comma { expressions: Vec<Expression>, span: core::range::Range<usize> },
}

impl ExpressionKind {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Identifier(_, span) => span.clone(),
            Self::Constant(_, span) => span.clone(),
            Self::StringLiteral(_, span) => span.clone(),
            Self::ArraySubscript { span, .. } => span.clone(),
            Self::FunctionCall { span, .. } => span.clone(),
            Self::MemberAccess { span, .. } => span.clone(),
            Self::PostfixIncDec { span, .. } => span.clone(),
            Self::PrefixIncDec { span, .. } => span.clone(),
            Self::Unary { span, .. } => span.clone(),
            Self::Cast { span, .. } => span.clone(),
            Self::Binary { span, .. } => span.clone(),
            Self::Conditional { span, .. } => span.clone(),
            Self::Assignment { span, .. } => span.clone(),
            Self::Comma { span, .. } => span.clone(),
        }
    }
}

/// 常量
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Integer(i64, core::range::Range<usize>),
    Float(f64, core::range::Range<usize>),
    Character(char, core::range::Range<usize>),
}

impl Constant {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Integer(_, span) => span.clone(),
            Self::Float(_, span) => span.clone(),
            Self::Character(_, span) => span.clone(),
        }
    }
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    AddressOf { span: core::range::Range<usize> },   // &
    Dereference { span: core::range::Range<usize> }, // *
    Plus { span: core::range::Range<usize> },        // +
    Minus { span: core::range::Range<usize> },       // -
    BitwiseNot { span: core::range::Range<usize> },  // ~
    LogicalNot { span: core::range::Range<usize> },  // !
    Sizeof { span: core::range::Range<usize> },      // sizeof
}

impl UnaryOperator {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::AddressOf { span } => span.clone(),
            Self::Dereference { span } => span.clone(),
            Self::Plus { span } => span.clone(),
            Self::Minus { span } => span.clone(),
            Self::BitwiseNot { span } => span.clone(),
            Self::LogicalNot { span } => span.clone(),
            Self::Sizeof { span } => span.clone(),
        }
    }
}

/// 二元操作
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // 算术操作
    Add { span: core::range::Range<usize> },      // +
    Subtract { span: core::range::Range<usize> }, // -
    Multiply { span: core::range::Range<usize> }, // *
    Divide { span: core::range::Range<usize> },   // /
    Modulo { span: core::range::Range<usize> },   // %

    // 位操作符
    BitwiseAnd { span: core::range::Range<usize> }, // &
    BitwiseOr { span: core::range::Range<usize> },  // |
    BitwiseXor { span: core::range::Range<usize> }, // ^
    LeftShift { span: core::range::Range<usize> },  // <<
    RightShift { span: core::range::Range<usize> }, // >>

    // 比较操作
    Equal { span: core::range::Range<usize> },        // ==
    NotEqual { span: core::range::Range<usize> },     // !=
    Less { span: core::range::Range<usize> },         // <
    Greater { span: core::range::Range<usize> },      // >
    LessEqual { span: core::range::Range<usize> },    // <=
    GreaterEqual { span: core::range::Range<usize> }, // >=

    // 逻辑操作
    LogicalAnd { span: core::range::Range<usize> }, // &&
    LogicalOr { span: core::range::Range<usize> },  // ||
}

impl BinaryOperator {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Add { span } => span.clone(),
            Self::Subtract { span } => span.clone(),
            Self::Multiply { span } => span.clone(),
            Self::Divide { span } => span.clone(),
            Self::Modulo { span } => span.clone(),
            Self::BitwiseAnd { span } => span.clone(),
            Self::BitwiseOr { span } => span.clone(),
            Self::BitwiseXor { span } => span.clone(),
            Self::LeftShift { span } => span.clone(),
            Self::RightShift { span } => span.clone(),
            Self::Equal { span } => span.clone(),
            Self::NotEqual { span } => span.clone(),
            Self::Less { span } => span.clone(),
            Self::Greater { span } => span.clone(),
            Self::LessEqual { span } => span.clone(),
            Self::GreaterEqual { span } => span.clone(),
            Self::LogicalAnd { span } => span.clone(),
            Self::LogicalOr { span } => span.clone(),
        }
    }
}

/// 赋值操作符
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Assign { span: core::range::Range<usize> },           // =
    AddAssign { span: core::range::Range<usize> },        // +=
    SubAssign { span: core::range::Range<usize> },        // -=
    MulAssign { span: core::range::Range<usize> },        // *=
    DivAssign { span: core::range::Range<usize> },        // /=
    ModAssign { span: core::range::Range<usize> },        // %=
    AndAssign { span: core::range::Range<usize> },        // &=
    OrAssign { span: core::range::Range<usize> },         // |=
    XorAssign { span: core::range::Range<usize> },        // ^=
    LeftShiftAssign { span: core::range::Range<usize> },  // <<=
    RightShiftAssign { span: core::range::Range<usize> }, // >>=
}

impl AssignmentOperator {
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Assign { span } => span.clone(),
            Self::AddAssign { span } => span.clone(),
            Self::SubAssign { span } => span.clone(),
            Self::MulAssign { span } => span.clone(),
            Self::DivAssign { span } => span.clone(),
            Self::ModAssign { span } => span.clone(),
            Self::AndAssign { span } => span.clone(),
            Self::OrAssign { span } => span.clone(),
            Self::XorAssign { span } => span.clone(),
            Self::LeftShiftAssign { span } => span.clone(),
            Self::RightShiftAssign { span } => span.clone(),
        }
    }
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
    pub fn new(translation_unit: TranslationUnit, span: core::range::Range<usize>) -> Self {
        Self { translation_unit, span }
    }
}

impl TranslationUnit {
    /// 创建新的翻译单元
    pub fn new(external_declarations: Vec<ExternalDeclaration>, span: core::range::Range<usize>) -> Self {
        Self { external_declarations, span }
    }
}
