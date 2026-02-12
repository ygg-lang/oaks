use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the OCaml language.
pub type OCamlToken = Token<OCamlTokenType>;

impl TokenType for OCamlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::True | Self::False => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl OCamlTokenType {
    /// Checks if the token is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::And
                | Self::As
                | Self::Assert
                | Self::Begin
                | Self::Class
                | Self::Constraint
                | Self::Do
                | Self::Done
                | Self::Downto
                | Self::Else
                | Self::End
                | Self::Exception
                | Self::External
                | Self::False
                | Self::For
                | Self::Fun
                | Self::Function
                | Self::Functor
                | Self::If
                | Self::In
                | Self::Include
                | Self::Inherit
                | Self::Initializer
                | Self::Lazy
                | Self::Let
                | Self::Match
                | Self::Method
                | Self::Module
                | Self::Mutable
                | Self::New
                | Self::Object
                | Self::Of
                | Self::Open
                | Self::Or
                | Self::Private
                | Self::Rec
                | Self::Sig
                | Self::Struct
                | Self::Then
                | Self::To
                | Self::True
                | Self::Try
                | Self::Type
                | Self::Val
                | Self::Virtual
                | Self::When
                | Self::While
                | Self::With
        )
    }

    /// Checks if the token is an operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Equal
                | Self::EqualEqual
                | Self::NotEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::LeftArrow
                | Self::RightArrow
                | Self::OrOr
                | Self::AndAnd
                | Self::ColonColon
                | Self::Pipe
                | Self::Ampersand
                | Self::Bang
                | Self::Question
                | Self::Caret
                | Self::Tilde
                | Self::At
                | Self::Hash
                | Self::Dollar
                | Self::Backtick
        )
    }

    /// Checks if the token is a punctuation character.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::Colon | Self::Semicolon | Self::Comma | Self::Dot | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace)
    }
}

/// Token types for the OCaml language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OCamlTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// A comment.
    Comment,

    /// The `and` keyword.
    And,
    /// The `as` keyword.
    As,
    /// The `assert` keyword.
    Assert,
    /// The `begin` keyword.
    Begin,
    /// The `class` keyword.
    Class,
    /// The `constraint` keyword.
    Constraint,
    /// The `do` keyword.
    Do,
    /// The `done` keyword.
    Done,
    /// The `downto` keyword.
    Downto,
    /// The `else` keyword.
    Else,
    /// The `end` keyword.
    End,
    /// The `exception` keyword.
    Exception,
    /// The `external` keyword.
    External,
    /// The `false` keyword.
    False,
    /// The `for` keyword.
    For,
    /// The `fun` keyword.
    Fun,
    /// The `function` keyword.
    Function,
    /// The `functor` keyword.
    Functor,
    /// The `if` keyword.
    If,
    /// The `in` keyword.
    In,
    /// The `include` keyword.
    Include,
    /// The `inherit` keyword.
    Inherit,
    /// The `initializer` keyword.
    Initializer,
    /// The `lazy` keyword.
    Lazy,
    /// The `let` keyword.
    Let,
    /// The `match` keyword.
    Match,
    /// The `method` keyword.
    Method,
    /// The `module` keyword.
    Module,
    /// The `mutable` keyword.
    Mutable,
    /// The `new` keyword.
    New,
    /// The `object` keyword.
    Object,
    /// The `of` keyword.
    Of,
    /// The `open` keyword.
    Open,
    /// The `or` keyword.
    Or,
    /// The `private` keyword.
    Private,
    /// The `rec` keyword.
    Rec,
    /// The `sig` keyword.
    Sig,
    /// The `struct` keyword.
    Struct,
    /// The `then` keyword.
    Then,
    /// The `to` keyword.
    To,
    /// The `true` keyword.
    True,
    /// The `try` keyword.
    Try,
    /// The `type` keyword.
    Type,
    /// The `val` keyword.
    Val,
    /// The `virtual` keyword.
    Virtual,
    /// The `when` keyword.
    When,
    /// The `while` keyword.
    While,
    /// The `with` keyword.
    With,

    /// An identifier.
    Identifier,
    /// An integer literal.
    IntegerLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,

    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `=` operator.
    Equal,
    /// The `==` operator.
    EqualEqual,
    /// The `!=` operator.
    NotEqual,
    /// The `<` operator.
    Less,
    /// The `>` operator.
    Greater,
    /// The `<=` operator.
    LessEqual,
    /// The `>=` operator.
    GreaterEqual,
    /// The `<-` operator.
    LeftArrow,
    /// The `->` operator.
    RightArrow,
    /// The `||` operator.
    OrOr,
    /// The `&&` operator.
    AndAnd,
    /// The `::` operator.
    ColonColon,
    /// The `|` operator.
    Pipe,
    /// The `&` operator.
    Ampersand,
    /// The `!` operator.
    Bang,
    /// The `?` operator.
    Question,
    /// The `:` punctuation.
    Colon,
    /// The `;` punctuation.
    Semicolon,
    /// The `,` punctuation.
    Comma,
    /// The `.` punctuation.
    Dot,
    /// The `^` operator.
    Caret,
    /// The `~` operator.
    Tilde,
    /// The `@` operator.
    At,
    /// The `#` operator.
    Hash,
    /// The `$` operator.
    Dollar,
    /// The `` ` `` operator.
    Backtick,

    /// The `-.` operator.
    MinusDot,
    /// The `(` operator.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,

    /// Root node of the OCaml AST.
    Root,
    /// A module definition.
    ModuleDef,
    /// A let binding.
    LetBinding,
    /// A match expression.
    MatchExpr,
    /// A function definition.
    FunctionDef,
    /// A type definition.
    TypeDefinition,
    /// An expression.
    Expression,

    /// A function call.
    CallExpr,
    /// A literal expression.
    LiteralExpr,
    /// An identifier expression.
    IdentifierExpr,
    /// A binary expression.
    BinaryExpr,
    /// A unary expression.
    UnaryExpr,

    /// An error token.
    Error,
    /// End of stream.
    Eof,
}
