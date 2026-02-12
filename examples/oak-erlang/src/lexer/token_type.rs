use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for an Erlang token.
pub type ErlangToken = Token<ErlangTokenType>;

impl ErlangTokenType {
    /// Returns `true` if the token type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Andalso
                | Self::Band
                | Self::Begin
                | Self::Bnot
                | Self::Bor
                | Self::Bsl
                | Self::Bsr
                | Self::Bxor
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Div
                | Self::End
                | Self::Fun
                | Self::If
                | Self::Let
                | Self::Not
                | Self::Of
                | Self::Or
                | Self::Orelse
                | Self::Query
                | Self::Receive
                | Self::Rem
                | Self::Try
                | Self::When
                | Self::Xor
        )
    }

    /// Returns `true` if the token type is an operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Equal
                | Self::EqualEqual
                | Self::SlashEqual
                | Self::EqualColonEqual
                | Self::EqualSlashEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::PlusPlus
                | Self::MinusMinus
                | Self::Exclamation
                | Self::Question
        )
    }

    /// Returns `true` if the token type is a punctuation.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon | Self::Dot | Self::Colon | Self::Arrow | Self::Pipe | Self::PipePipe | Self::Hash)
    }
}

impl TokenType for ErlangTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Eof => UniversalTokenRole::Eof,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier | Self::Atom | Self::Variable => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Character => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Erlang language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ErlangTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,

    /// Identifier.
    Identifier,
    /// Atom.
    Atom,
    /// Variable.
    Variable,
    /// Number.
    Number,
    /// String.
    String,
    /// Character literal.
    Character,

    /// `after` keyword.
    After,
    /// `and` keyword.
    And,
    /// `andalso` keyword.
    Andalso,
    /// `band` keyword.
    Band,
    /// `begin` keyword.
    Begin,
    /// `bnot` keyword.
    Bnot,
    /// `bor` keyword.
    Bor,
    /// `bsl` keyword.
    Bsl,
    /// `bsr` keyword.
    Bsr,
    /// `bxor` keyword.
    Bxor,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `cond` keyword.
    Cond,
    /// `div` keyword.
    Div,
    /// `end` keyword.
    End,
    /// `fun` keyword.
    Fun,
    /// `if` keyword.
    If,
    /// `let` keyword.
    Let,
    /// `not` keyword.
    Not,
    /// `of` keyword.
    Of,
    /// `or` keyword.
    Or,
    /// `orelse` keyword.
    Orelse,
    /// `query` keyword.
    Query,
    /// `receive` keyword.
    Receive,
    /// `rem` keyword.
    Rem,
    /// `try` keyword.
    Try,
    /// `when` keyword.
    When,
    /// `xor` keyword.
    Xor,

    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Equal `=`.
    Equal,
    /// Double equal `==`.
    EqualEqual,
    /// Slash equal `/=`.
    SlashEqual,
    /// Equal colon equal `=:=`.
    EqualColonEqual,
    /// Equal slash equal `=/=`.
    EqualSlashEqual,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal to `=<`.
    LessEqual,
    /// Greater than or equal to `>=`.
    GreaterEqual,
    /// Plus plus `++`.
    PlusPlus,
    /// Minus minus `--`.
    MinusMinus,
    /// Exclamation `!`.
    Exclamation,
    /// Question mark `?`.
    Question,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Comma `,`.
    Comma,
    /// Semicolon `;`.
    Semicolon,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Arrow `->`.
    Arrow,
    /// Pipe `|`.
    Pipe,
    /// Double pipe `||`.
    PipePipe,
    /// Hash `#`.
    Hash,

    /// Root node.
    Root,
    /// Item node.
    Item,
    /// Module attribute.
    Module,
    /// Export attribute.
    Export,
    /// Other attribute.
    Attribute,
    /// Function definition.
    Function,
    /// Function clause.
    FunctionClause,
    /// Pattern.
    Pattern,
    /// Record pattern.
    RecordPattern,
    /// Statement.
    Statement,
    /// Expression.
    Expr,
    /// Binary expression.
    BinaryExpr,
    /// Call expression.
    CallExpr,
    /// Fun expression.
    FunExpr,
    /// Case expression.
    CaseExpr,
    /// Case clause.
    CaseClause,
    /// If expression.
    IfExpr,
    /// If clause.
    IfClause,
    /// Try expression.
    TryExpr,
    /// Catch clause.
    CatchClause,
    /// Receive expression.
    ReceiveExpr,
    /// Receive clause.
    ReceiveClause,
    /// Record expression.
    RecordExpr,

    /// Lexing error.
    Error,
    /// End of file.
    Eof,
}
