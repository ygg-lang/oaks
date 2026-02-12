use super::*;

/// Expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// Expression kind.
    pub kind: Box<ExpressionKind>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Expression kind.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    /// Identifier.
    Identifier(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Constant (literal).
    Constant(Constant, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// String literal.
    StringLiteral(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Array subscript `array[index]`.
    ArraySubscript {
        /// The array expression.
        array: Box<Expression>,
        /// The index expression.
        index: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Function call `function(arguments)`.
    FunctionCall {
        /// The function expression.
        function: Box<Expression>,
        /// List of arguments.
        arguments: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Member access `object.member` or `object->member`.
    MemberAccess {
        /// The object expression.
        object: Box<Expression>,
        /// Member name.
        member: String,
        /// Whether it's a pointer access (`->`).
        is_pointer: bool,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Postfix increment or decrement (`++`, `--`).
    PostfixIncDec {
        /// The operand.
        operand: Box<Expression>,
        /// Whether it's an increment.
        is_increment: bool,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Prefix increment or decrement (`++`, `--`).
    PrefixIncDec {
        /// The operand.
        operand: Box<Expression>,
        /// Whether it's an increment.
        is_increment: bool,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Unary operation.
    Unary {
        /// Unary operator.
        operator: UnaryOperator,
        /// The operand.
        operand: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Type cast `(type_name) expression`.
    Cast {
        /// Target type.
        type_name: Box<TypeName>,
        /// The expression to cast.
        expression: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Binary operation.
    Binary {
        /// Left operand.
        left: Box<Expression>,
        /// Binary operator.
        operator: BinaryOperator,
        /// Right operand.
        right: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Conditional expression `condition ? then_expr : else_expr`.
    Conditional {
        /// Condition.
        condition: Box<Expression>,
        /// Then expression.
        then_expr: Box<Expression>,
        /// Else expression.
        else_expr: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Assignment expression.
    Assignment {
        /// Left side of assignment.
        left: Box<Expression>,
        /// Assignment operator.
        operator: AssignmentOperator,
        /// Right side of assignment.
        right: Box<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// Comma expression `expr1, expr2, ...`.
    Comma {
        /// List of expressions.
        expressions: Vec<Expression>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
}

impl ExpressionKind {
    /// Returns the source span of the expression kind.
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

/// Constant (literal).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    /// Integer constant.
    Integer(i64, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Floating-point constant.
    Float(f64, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// Character constant.
    Character(char, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
}

impl Constant {
    /// Returns the source span of the constant.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Integer(_, span) => span.clone(),
            Self::Float(_, span) => span.clone(),
            Self::Character(_, span) => span.clone(),
        }
    }
}

/// Unary operator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `&`
    AddressOf,
    /// `*`
    Indirection,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `~`
    BitNot,
    /// `!`
    LogicalNot,
    /// `sizeof`
    Sizeof,
}

/// Binary operator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `%`
    Modulo,
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `&`
    BitAnd,
    /// `^`
    BitXor,
    /// `|`
    BitOr,
    /// `&&`
    LogicalAnd,
    /// `||`
    LogicalOr,
}

/// Assignment operator.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    /// `=`
    Assign,
    /// `*=`
    MulAssign,
    /// `/=`
    DivAssign,
    /// `%=`
    ModAssign,
    /// `+=`
    AddAssign,
    /// `-=`
    SubAssign,
    /// `<<=`
    ShlAssign,
    /// `>>=`
    ShrAssign,
    /// `&=`
    AndAssign,
    /// `^=`
    XorAssign,
    /// `|=`
    OrAssign,
}

/// Type name.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    /// List of specifiers and qualifiers.
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    /// Optional abstract declarator.
    pub abstract_declarator: Option<Box<AbstractDeclarator>>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

impl TypeName {
    /// Returns the source span of the type name.
    pub fn span(&self) -> core::range::Range<usize> {
        self.span.clone()
    }
}
