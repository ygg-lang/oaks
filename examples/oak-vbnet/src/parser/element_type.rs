use oak_core::{ElementType, UniversalElementRole};

/// VB.NET element types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VbNetElementType {
    /// Root node
    Root,
    /// Expression
    Expression,
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,

    /// Identifier
    Identifier,
    /// Integer literal
    IntegerLiteral,
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Character literal
    CharLiteral,
    /// Boolean literal
    BooleanLiteral,
    /// Date literal
    DateLiteral,
    /// Nothing literal
    NothingLiteral,

    /// Namespace declaration
    Namespace,
    /// Imports directive
    Imports,
    /// Class declaration
    Class,
    /// Interface declaration
    Interface,
    /// Structure declaration
    Structure,
    /// Enum declaration
    Enum,
    /// Module declaration
    Module,
    /// Delegate declaration
    Delegate,
    /// Event declaration
    Event,
    /// Function declaration
    Function,
    /// Subroutine declaration
    Sub,
    /// Property declaration
    Property,
    /// Variable declaration
    Variable,

    /// If statement
    If,
    /// For loop
    For,
    /// For Each loop
    ForEach,
    /// While loop
    While,
    /// Do While loop
    DoWhile,
    /// Select Case statement
    SelectCase,
    /// With statement
    With,
    /// Try statement
    Try,
    /// Catch clause
    Catch,
    /// Finally block
    Finally,
    /// Dim statement
    Dim,
    /// Const statement
    Const,
    /// Return statement
    Return,
    /// Exit statement
    Exit,
    /// Continue statement
    Continue,
    /// Statement
    Statement,
    /// Throw statement
    Throw,

    /// Binary expression
    BinaryExpression,
    /// Unary expression
    UnaryExpression,
    /// Assignment expression
    AssignmentExpression,
    /// Method call
    MethodCall,
    /// Member access
    MemberAccess,
    /// Element access
    ElementAccess,
    /// New expression
    NewExpression,
    /// Array expression
    ArrayExpression,
    /// Tuple expression
    TupleExpression,
    /// Parenthesized expression
    ParenthesizedExpression,
    /// TypeOf expression
    TypeOfExpression,
    /// Is expression
    IsExpression,
    /// Like expression
    LikeExpression,
    /// If expression
    IfExpression,
    /// Lambda expression
    LambdaExpression,
    /// XML literal
    XmlLiteral,

    /// Error
    Error,
    /// End of file
    Eof,
}

impl VbNetElementType {
    /// Checks if it is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Namespace
                | Self::Imports
                | Self::Class
                | Self::Interface
                | Self::Structure
                | Self::Enum
                | Self::Module
                | Self::Delegate
                | Self::Event
                | Self::Function
                | Self::Sub
                | Self::Property
                | Self::Dim
                | Self::Const
                | Self::If
                | Self::For
                | Self::ForEach
                | Self::While
                | Self::DoWhile
                | Self::SelectCase
                | Self::With
                | Self::Try
                | Self::Catch
                | Self::Finally
                | Self::Return
                | Self::Exit
                | Self::Continue
                | Self::Statement
                | Self::Throw
        )
    }
}

impl ElementType for VbNetElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VbNetTokenType> for VbNetElementType {
    fn from(token: crate::lexer::token_type::VbNetTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
