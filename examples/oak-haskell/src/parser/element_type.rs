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
                match token {
            crate::lexer::token_type::HaskellTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::HaskellTokenType::Newline => Self::Newline,
            crate::lexer::token_type::HaskellTokenType::Comment => Self::Comment,
            crate::lexer::token_type::HaskellTokenType::Case => Self::Case,
            crate::lexer::token_type::HaskellTokenType::Class => Self::Class,
            crate::lexer::token_type::HaskellTokenType::Data => Self::Data,
            crate::lexer::token_type::HaskellTokenType::Default => Self::Default,
            crate::lexer::token_type::HaskellTokenType::Deriving => Self::Deriving,
            crate::lexer::token_type::HaskellTokenType::Do => Self::Do,
            crate::lexer::token_type::HaskellTokenType::Else => Self::Else,
            crate::lexer::token_type::HaskellTokenType::Foreign => Self::Foreign,
            crate::lexer::token_type::HaskellTokenType::If => Self::If,
            crate::lexer::token_type::HaskellTokenType::Import => Self::Import,
            crate::lexer::token_type::HaskellTokenType::In => Self::In,
            crate::lexer::token_type::HaskellTokenType::Infix => Self::Infix,
            crate::lexer::token_type::HaskellTokenType::Infixl => Self::Infixl,
            crate::lexer::token_type::HaskellTokenType::Infixr => Self::Infixr,
            crate::lexer::token_type::HaskellTokenType::Instance => Self::Instance,
            crate::lexer::token_type::HaskellTokenType::Let => Self::Let,
            crate::lexer::token_type::HaskellTokenType::Module => Self::Module,
            crate::lexer::token_type::HaskellTokenType::Newtype => Self::Newtype,
            crate::lexer::token_type::HaskellTokenType::Of => Self::Of,
            crate::lexer::token_type::HaskellTokenType::Then => Self::Then,
            crate::lexer::token_type::HaskellTokenType::Type => Self::Type,
            crate::lexer::token_type::HaskellTokenType::Where => Self::Where,
            crate::lexer::token_type::HaskellTokenType::Underscore => Self::Underscore,
            crate::lexer::token_type::HaskellTokenType::As => Self::As,
            crate::lexer::token_type::HaskellTokenType::Qualified => Self::Qualified,
            crate::lexer::token_type::HaskellTokenType::Hiding => Self::Hiding,
            crate::lexer::token_type::HaskellTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::HaskellTokenType::Constructor => Self::Constructor,
            crate::lexer::token_type::HaskellTokenType::Number => Self::Number,
            crate::lexer::token_type::HaskellTokenType::Integer => Self::Integer,
            crate::lexer::token_type::HaskellTokenType::Float => Self::Float,
            crate::lexer::token_type::HaskellTokenType::String => Self::String,
            crate::lexer::token_type::HaskellTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::HaskellTokenType::Char => Self::Char,
            crate::lexer::token_type::HaskellTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::HaskellTokenType::Plus => Self::Plus,
            crate::lexer::token_type::HaskellTokenType::Minus => Self::Minus,
            crate::lexer::token_type::HaskellTokenType::Star => Self::Star,
            crate::lexer::token_type::HaskellTokenType::Slash => Self::Slash,
            crate::lexer::token_type::HaskellTokenType::Percent => Self::Percent,
            crate::lexer::token_type::HaskellTokenType::Assign => Self::Assign,
            crate::lexer::token_type::HaskellTokenType::Equal => Self::Equal,
            crate::lexer::token_type::HaskellTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::HaskellTokenType::Less => Self::Less,
            crate::lexer::token_type::HaskellTokenType::Greater => Self::Greater,
            crate::lexer::token_type::HaskellTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::HaskellTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::HaskellTokenType::And => Self::And,
            crate::lexer::token_type::HaskellTokenType::Or => Self::Or,
            crate::lexer::token_type::HaskellTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::HaskellTokenType::LeftArrow => Self::LeftArrow,
            crate::lexer::token_type::HaskellTokenType::DoubleArrow => Self::DoubleArrow,
            crate::lexer::token_type::HaskellTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::HaskellTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::HaskellTokenType::Bang => Self::Bang,
            crate::lexer::token_type::HaskellTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::HaskellTokenType::Question => Self::Question,
            crate::lexer::token_type::HaskellTokenType::Colon => Self::Colon,
            crate::lexer::token_type::HaskellTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::HaskellTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::HaskellTokenType::Comma => Self::Comma,
            crate::lexer::token_type::HaskellTokenType::Dot => Self::Dot,
            crate::lexer::token_type::HaskellTokenType::DoubleDot => Self::DoubleDot,
            crate::lexer::token_type::HaskellTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::HaskellTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::HaskellTokenType::At => Self::At,
            crate::lexer::token_type::HaskellTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::HaskellTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::HaskellTokenType::Append => Self::Append,
            crate::lexer::token_type::HaskellTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::HaskellTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::HaskellTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::HaskellTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::HaskellTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::HaskellTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::HaskellTokenType::Quote => Self::Quote,
            crate::lexer::token_type::HaskellTokenType::Backquote => Self::Backquote,
            crate::lexer::token_type::HaskellTokenType::Backtick => Self::Backtick,
            crate::lexer::token_type::HaskellTokenType::Function => Self::Function,
            crate::lexer::token_type::HaskellTokenType::DataDeclaration => Self::DataDeclaration,
            crate::lexer::token_type::HaskellTokenType::ModuleDeclaration => Self::ModuleDeclaration,
            crate::lexer::token_type::HaskellTokenType::Root => Self::Root,
            crate::lexer::token_type::HaskellTokenType::Error => Self::Error,
            crate::lexer::token_type::HaskellTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
