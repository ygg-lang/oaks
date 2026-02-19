use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Scala lexer output.
pub type ScalaToken = Token<ScalaTokenType>;

/// Token types for the Scala lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScalaTokenType {
    /// Root node for Scala source files.
    SourceFile,

    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// Line comment token.
    LineComment,
    /// Block comment token.
    BlockComment,
    /// Error token.
    Error,
    /// End of file token.
    Eof,
    /// Error node token.
    ErrorNode,

    /// Identifier token.
    Identifier,
    /// Integer literal token.
    IntegerLiteral,
    /// Float literal token.
    FloatLiteral,
    /// String literal token.
    StringLiteral,
    /// Character literal token.
    CharLiteral,
    /// Boolean literal token.
    BooleanLiteral,

    /// `abstract` keyword.
    Abstract,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `class` keyword.
    Class,
    /// `def` keyword.
    Def,
    /// `do` keyword.
    Do,
    /// `else` keyword.
    Else,
    /// `extends` keyword.
    Extends,
    /// `false` keyword.
    False,
    /// `final` keyword.
    Final,
    /// `finally` keyword.
    Finally,
    /// `for` keyword.
    For,
    /// `forSome` keyword.
    ForSome,
    /// `if` keyword.
    If,
    /// `implicit` keyword.
    Implicit,
    /// `import` keyword.
    Import,
    /// `lazy` keyword.
    Lazy,
    /// `match` keyword.
    Match,
    /// `new` keyword.
    New,
    /// `null` keyword.
    Null,
    /// `object` keyword.
    Object,
    /// `override` keyword.
    Override,
    /// `package` keyword.
    Package,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `return` keyword.
    Return,
    /// `sealed` keyword.
    Sealed,
    /// `super` keyword.
    Super,
    /// `this` keyword.
    This,
    /// `throw` keyword.
    Throw,
    /// `trait` keyword.
    Trait,
    /// `try` keyword.
    Try,
    /// `true` keyword.
    True,
    /// `type` keyword.
    Type,
    /// `val` keyword.
    Val,
    /// `var` keyword.
    Var,
    /// `while` keyword.
    While,
    /// `with` keyword.
    With,
    /// `yield` keyword.
    Yield,

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
    /// `=` operator.
    Eq,
    /// `==` operator.
    EqEq,
    /// `!=` operator.
    Ne,
    /// `<` operator.
    Lt,
    /// `<=` operator.
    Le,
    /// `>` operator.
    Gt,
    /// `>=` operator.
    Ge,
    /// `<=` operator.
    LessEqual,
    /// `>=` operator.
    GreaterEqual,
    /// `==` operator.
    EqualEqual,
    /// `!=` operator.
    NotEqual,
    /// `&` operator.
    And,
    /// `|` operator.
    Or,
    /// `^` operator.
    Xor,
    /// `&&` operator.
    AndAnd,
    /// `||` operator.
    OrOr,
    /// `!` operator.
    Not,
    /// `~` operator.
    Tilde,
    /// `<<` operator.
    LShift,
    /// `>>` operator.
    RShift,
    /// `>>>` operator.
    URShift,
    /// `+=` operator.
    PlusEq,
    /// `-=` operator.
    MinusEq,
    /// `*=` operator.
    StarEq,
    /// `/=` operator.
    SlashEq,
    /// `%=` operator.
    PercentEq,
    /// `&=` operator.
    AndEq,
    /// `|=` operator.
    OrEq,
    /// `^=` operator.
    XorEq,
    /// `<<=` operator.
    LShiftEq,
    /// `>>=` operator.
    RShiftEq,
    /// `>>>=` operator.
    URShiftEq,
    /// `=>` operator.
    Arrow,
    /// `<-` operator.
    LeftArrow,
    /// `:` operator.
    Colon,
    /// `::` operator.
    ColonColon,
    /// `;` operator.
    Semicolon,
    /// `.` operator.
    Dot,
    /// `,` operator.
    Comma,
    /// `?` operator.
    Question,
    /// `@` operator.
    At,
    /// `#` operator.
    Hash,

    /// `(` separator.
    LeftParen,
    /// `)` separator.
    RightParen,
    /// `[` separator.
    LeftBracket,
    /// `]` separator.
    RightBracket,
    /// `{` separator.
    LeftBrace,
    /// `}` separator.
    RightBrace,
}

impl TokenType for ScalaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
