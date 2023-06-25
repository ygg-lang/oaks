use oak_core::{ElementType, UniversalElementRole};

/// Represents an element type in a Haskell source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HaskellElementType {
    /// Whitespace characters.
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment.
    Comment,
    /// 'case' keyword.
    Case,
    /// 'class' keyword.
    Class,
    /// 'data' keyword.
    Data,
    /// 'default' keyword.
    Default,
    /// 'deriving' keyword.
    Deriving,
    /// 'do' keyword.
    Do,
    /// 'else' keyword.
    Else,
    /// 'foreign' keyword.
    Foreign,
    /// 'if' keyword.
    If,
    /// 'import' keyword.
    Import,
    /// 'in' keyword.
    In,
    /// 'infix' keyword.
    Infix,
    /// 'infixl' keyword.
    Infixl,
    /// 'infixr' keyword.
    Infixr,
    /// 'instance' keyword.
    Instance,
    /// 'let' keyword.
    Let,
    /// 'module' keyword.
    Module,
    /// 'newtype' keyword.
    Newtype,
    /// 'of' keyword.
    Of,
    /// 'then' keyword.
    Then,
    /// 'type' keyword.
    Type,
    /// 'where' keyword.
    Where,
    /// Underscore character (_).
    Underscore,
    /// 'as' keyword.
    As,
    /// 'qualified' keyword.
    Qualified,
    /// 'hiding' keyword.
    Hiding,
    /// An identifier.
    Identifier,
    /// A constructor identifier.
    Constructor,
    /// A numeric literal.
    Number,
    /// An integer literal.
    Integer,
    /// A floating-point literal.
    Float,
    /// A string literal.
    String,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    Char,
    /// A character literal.
    CharLiteral,
    /// Plus operator (+).
    Plus,
    /// Minus operator (-).
    Minus,
    /// Multiplication operator (*).
    Star,
    /// Division operator (/).
    Slash,
    /// Modulo operator (%).
    Percent,
    /// Assignment operator (=).
    Assign,
    /// Equality operator (==).
    Equal,
    /// Inequality operator (/=).
    NotEqual,
    /// Less than operator (<).
    Less,
    /// Greater than operator (>).
    Greater,
    /// Less than or equal to operator (<=).
    LessEqual,
    /// Greater than or equal to operator (>=).
    GreaterEqual,
    /// Logical AND operator (&&).
    And,
    /// Logical OR operator (||).
    Or,
    /// Function arrow operator (->).
    Arrow,
    /// Left arrow operator (<-).
    LeftArrow,
    /// Double arrow operator (=>).
    DoubleArrow,
    /// Pipe character (|).
    Pipe,
    /// Ampersand character (&).
    Ampersand,
    /// Bang operator (!).
    Bang,
    /// Exclamation mark (!).
    Exclamation,
    /// Question mark (?).
    Question,
    /// Colon character (:).
    Colon,
    /// Double colon character (::).
    DoubleColon,
    /// Semicolon character (;).
    Semicolon,
    /// Comma character (,).
    Comma,
    /// Dot character (.).
    Dot,
    /// Double dot character (..).
    DoubleDot,
    /// Range operator (..).
    DotDot,
    /// Dollar sign ($).
    Dollar,
    /// At sign (@).
    At,
    /// Tilde character (~).
    Tilde,
    /// Backslash character (\).
    Backslash,
    /// Append operator (++).
    Append,
    /// Left parenthesis (().
    LeftParen,
    /// Right parenthesis ()).
    RightParen,
    /// Left bracket ([).
    LeftBracket,
    /// Right bracket (]).
    RightBracket,
    /// Left brace ({).
    LeftBrace,
    /// Right brace (}).
    RightBrace,
    /// Single quote (').
    Quote,
    /// Backquote (`).
    Backquote,
    /// Backtick (`).
    Backtick,
    /// A function definition.
    Function,
    /// A data declaration.
    DataDeclaration,
    /// A module declaration.
    ModuleDeclaration,
    /// An import declaration.
    ImportDeclaration,
    /// A type alias declaration.
    TypeAliasDeclaration,
    /// A type signature.
    TypeSignature,
    /// A function equation.
    Equation,
    /// A pattern.
    Pattern,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A prefix expression.
    PrefixExpression,
    /// An infix expression.
    InfixExpression,
    /// A function application.
    ApplicationExpression,
    /// A lambda expression.
    LambdaExpression,
    /// A let expression.
    LetExpression,
    /// A case expression.
    CaseExpression,
    /// A case arm.
    CaseArm,
    /// A type expression.
    TypeExpr,
    /// Root node of the AST.
    Root,
    /// Error node.
    Error,
    /// End of file marker.
    Eof,
}

impl HaskellElementType {
    /// Returns true if the element type is a Haskell keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Case
                | Self::Class
                | Self::Data
                | Self::Default
                | Self::Deriving
                | Self::Do
                | Self::Else
                | Self::Foreign
                | Self::If
                | Self::Import
                | Self::In
                | Self::Infix
                | Self::Infixl
                | Self::Infixr
                | Self::Instance
                | Self::Let
                | Self::Module
                | Self::Newtype
                | Self::Of
                | Self::Then
                | Self::Type
                | Self::Where
                | Self::As
                | Self::Qualified
                | Self::Hiding
        )
    }
}

impl oak_core::TokenType for HaskellElementType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = oak_core::UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            Self::Identifier | Self::Constructor => oak_core::UniversalTokenRole::Name,
            Self::Number | Self::Integer | Self::Float | Self::String | Self::StringLiteral | Self::Char | Self::CharLiteral => oak_core::UniversalTokenRole::Literal,
            _ if self.is_keyword() => oak_core::UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Assign
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::And
            | Self::Or
            | Self::Arrow
            | Self::LeftArrow
            | Self::DoubleArrow
            | Self::Pipe
            | Self::Ampersand
            | Self::Bang
            | Self::Exclamation
            | Self::Question
            | Self::Colon
            | Self::DoubleColon
            | Self::Dollar
            | Self::At
            | Self::Tilde
            | Self::Backslash
            | Self::Append => oak_core::UniversalTokenRole::Operator,
            Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::DoubleDot
            | Self::DotDot
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftBrace
            | Self::RightBrace
            | Self::Underscore
            | Self::Quote
            | Self::Backquote
            | Self::Backtick => oak_core::UniversalTokenRole::Punctuation,
            Self::Eof => oak_core::UniversalTokenRole::Eof,
            _ => oak_core::UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for HaskellElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::HaskellTokenType> for HaskellElementType {
    fn from(token: crate::lexer::token_type::HaskellTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
