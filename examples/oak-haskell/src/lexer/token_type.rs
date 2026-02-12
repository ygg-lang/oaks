use oak_core::Token;

/// A token in the Haskell language.
pub type HaskellToken = Token<HaskellTokenType>;

/// Token types for the Haskell language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HaskellTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comments.
    Comment,

    // Keywords
    /// `case` keyword.
    Case,
    /// `class` keyword.
    Class,
    /// `data` keyword.
    Data,
    /// `default` keyword.
    Default,
    /// `deriving` keyword.
    Deriving,
    /// `do` keyword.
    Do,
    /// `else` keyword.
    Else,
    /// `foreign` keyword.
    Foreign,
    /// `if` keyword.
    If,
    /// `import` keyword.
    Import,
    /// `in` keyword.
    In,
    /// `infix` keyword.
    Infix,
    /// `infixl` keyword.
    Infixl,
    /// `infixr` keyword.
    Infixr,
    /// `instance` keyword.
    Instance,
    /// `let` keyword.
    Let,
    /// `module` keyword.
    Module,
    /// `newtype` keyword.
    Newtype,
    /// `of` keyword.
    Of,
    /// `then` keyword.
    Then,
    /// `type` keyword.
    Type,
    /// `where` keyword.
    Where,
    /// `_` keyword.
    Underscore,
    /// `as` keyword.
    As,
    /// `qualified` keyword.
    Qualified,
    /// `hiding` keyword.
    Hiding,

    // Identifiers and Literals
    /// An identifier.
    Identifier,
    /// A constructor identifier.
    Constructor,
    /// A number literal.
    Number,
    /// An integer literal.
    Integer,
    /// A float literal.
    Float,
    /// A string literal.
    String,
    /// A string literal.
    StringLiteral,
    /// A char literal.
    Char,
    /// A char literal.
    CharLiteral,

    // Operators and Punctuation
    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
    Percent,
    /// `=` operator (assignment).
    Assign,
    /// `==` operator.
    Equal,
    /// `/=` operator.
    NotEqual,
    /// `<` operator.
    Less,
    /// `>` operator.
    Greater,
    /// `<=` operator.
    LessEqual,
    /// `>=` operator.
    GreaterEqual,
    /// `&&` operator.
    And,
    /// `||` operator.
    Or,
    /// `->` operator.
    Arrow,
    /// `<-` operator.
    LeftArrow,
    /// `=>` operator.
    DoubleArrow,
    /// `|` operator.
    Pipe,
    /// `&` operator.
    Ampersand,
    /// `!` operator.
    Bang,
    /// `!` operator.
    Exclamation,
    /// `?` operator.
    Question,
    /// `:` operator.
    Colon,
    /// `::` operator.
    DoubleColon,
    /// `;` punctuation.
    Semicolon,
    /// `,` punctuation.
    Comma,
    /// `.` operator.
    Dot,
    /// `..` operator.
    DoubleDot,
    /// `..` operator.
    DotDot,
    /// `$` operator.
    Dollar,
    /// `@` operator.
    At,
    /// `~` operator.
    Tilde,
    /// `\` operator.
    Backslash,
    /// `++` operator.
    Append,
    /// `(` punctuation.
    LeftParen,
    /// `)` punctuation.
    RightParen,
    /// `[` punctuation.
    LeftBracket,
    /// `]` punctuation.
    RightBracket,
    /// `{` punctuation.
    LeftBrace,
    /// `}` punctuation.
    RightBrace,
    /// `'` punctuation.
    Quote,
    /// `` ` `` punctuation.
    Backquote,
    /// `` ` `` punctuation.
    Backtick,

    // Elements
    /// A function.
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
    /// Root element.
    Root,

    /// Error token.
    Error,
    /// End of file token.
    Eof,
}

impl HaskellTokenType {
    /// Returns true if the token type is a keyword.
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

impl oak_core::TokenType for HaskellTokenType {
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
