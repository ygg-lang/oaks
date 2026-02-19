use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Scala parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScalaElementType {
    /// Root node for Scala source files.
    SourceFile,

    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token (generic).
    Comment,
    /// Single-line comment token.
    LineComment,
    /// Multi-line block comment token.
    BlockComment,
    /// Error token for lexical errors.
    Error,
    /// End-of-file token.
    Eof,
    /// Error node for parse errors.
    ErrorNode,

    /// Identifier token for names and symbols.
    Identifier,
    /// Integer literal token.
    IntegerLiteral,
    /// Floating-point literal token.
    FloatLiteral,
    /// String literal token.
    StringLiteral,
    /// Character literal token.
    CharLiteral,
    /// Boolean literal token (true or false).
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

    /// Addition operator `+`.
    Plus,
    /// Subtraction operator `-`.
    Minus,
    /// Multiplication operator `*`.
    Star,
    /// Division operator `/`.
    Slash,
    /// Modulo operator `%`.
    Percent,
    /// Assignment operator `=`.
    Eq,
    /// Equality operator `==`.
    EqEq,
    /// Inequality operator `!=`.
    Ne,
    /// Less-than operator `<`.
    Lt,
    /// Less-than-or-equal operator `<=`.
    Le,
    /// Greater-than operator `>`.
    Gt,
    /// Greater-than-or-equal operator `>=`.
    Ge,
    /// Less-than-or-equal operator `<=`.
    LessEqual,
    /// Greater-than-or-equal operator `>=`.
    GreaterEqual,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    NotEqual,
    /// Bitwise AND operator `&`.
    And,
    /// Bitwise OR operator `|`.
    Or,
    /// Bitwise XOR operator `^`.
    Xor,
    /// Logical AND operator `&&`.
    AndAnd,
    /// Logical OR operator `||`.
    OrOr,
    /// Logical NOT operator `!`.
    Not,
    /// Bitwise NOT operator `~`.
    Tilde,
    /// Left shift operator `<<`.
    LShift,
    /// Right shift operator `>>`.
    RShift,
    /// Unsigned right shift operator `>>>`.
    URShift,
    /// Addition assignment operator `+=`.
    PlusEq,
    /// Subtraction assignment operator `-=`.
    MinusEq,
    /// Multiplication assignment operator `*=`.
    StarEq,
    /// Division assignment operator `/=`.
    SlashEq,
    /// Modulo assignment operator `%=`.
    PercentEq,
    /// Bitwise AND assignment operator `&=`.
    AndEq,
    /// Bitwise OR assignment operator `|=`.
    OrEq,
    /// Bitwise XOR assignment operator `^=`.
    XorEq,
    /// Left shift assignment operator `<<=`.
    LShiftEq,
    /// Right shift assignment operator `>>=`.
    RShiftEq,
    /// Unsigned right shift assignment operator `>>>=`.
    URShiftEq,
    /// Arrow operator `=>`.
    Arrow,
    /// Left arrow operator `<-`.
    LeftArrow,
    /// Colon operator `:`.
    Colon,
    /// Cons operator `::`.
    ColonColon,
    /// Semicolon separator `;`.
    Semicolon,
    /// Dot operator `.`.
    Dot,
    /// Comma separator `,`.
    Comma,
    /// Question mark operator `?`.
    Question,
    /// Annotation operator `@`.
    At,
    /// Hash operator `#`.
    Hash,

    /// Left parenthesis `(`.
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
}

impl ElementType for ScalaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ScalaTokenType> for ScalaElementType {
    fn from(token: crate::lexer::token_type::ScalaTokenType) -> Self {
        use crate::lexer::token_type::ScalaTokenType as T;
        match token {
            T::SourceFile => ScalaElementType::SourceFile,
            T::Whitespace => ScalaElementType::Whitespace,
            T::Newline => ScalaElementType::Newline,
            T::Comment => ScalaElementType::Comment,
            T::LineComment => ScalaElementType::LineComment,
            T::BlockComment => ScalaElementType::BlockComment,
            T::Error => ScalaElementType::Error,
            T::Eof => ScalaElementType::Eof,
            T::ErrorNode => ScalaElementType::ErrorNode,
            T::Identifier => ScalaElementType::Identifier,
            T::IntegerLiteral => ScalaElementType::IntegerLiteral,
            T::FloatLiteral => ScalaElementType::FloatLiteral,
            T::StringLiteral => ScalaElementType::StringLiteral,
            T::CharLiteral => ScalaElementType::CharLiteral,
            T::BooleanLiteral => ScalaElementType::BooleanLiteral,
            T::Abstract => ScalaElementType::Abstract,
            T::Case => ScalaElementType::Case,
            T::Catch => ScalaElementType::Catch,
            T::Class => ScalaElementType::Class,
            T::Def => ScalaElementType::Def,
            T::Do => ScalaElementType::Do,
            T::Else => ScalaElementType::Else,
            T::Extends => ScalaElementType::Extends,
            T::False => ScalaElementType::False,
            T::Final => ScalaElementType::Final,
            T::Finally => ScalaElementType::Finally,
            T::For => ScalaElementType::For,
            T::ForSome => ScalaElementType::ForSome,
            T::If => ScalaElementType::If,
            T::Implicit => ScalaElementType::Implicit,
            T::Import => ScalaElementType::Import,
            T::Lazy => ScalaElementType::Lazy,
            T::Match => ScalaElementType::Match,
            T::New => ScalaElementType::New,
            T::Null => ScalaElementType::Null,
            T::Object => ScalaElementType::Object,
            T::Override => ScalaElementType::Override,
            T::Package => ScalaElementType::Package,
            T::Private => ScalaElementType::Private,
            T::Protected => ScalaElementType::Protected,
            T::Return => ScalaElementType::Return,
            T::Sealed => ScalaElementType::Sealed,
            T::Super => ScalaElementType::Super,
            T::This => ScalaElementType::This,
            T::Throw => ScalaElementType::Throw,
            T::Trait => ScalaElementType::Trait,
            T::Try => ScalaElementType::Try,
            T::True => ScalaElementType::True,
            T::Type => ScalaElementType::Type,
            T::Val => ScalaElementType::Val,
            T::Var => ScalaElementType::Var,
            T::While => ScalaElementType::While,
            T::With => ScalaElementType::With,
            T::Yield => ScalaElementType::Yield,
            T::Plus => ScalaElementType::Plus,
            T::Minus => ScalaElementType::Minus,
            T::Star => ScalaElementType::Star,
            T::Slash => ScalaElementType::Slash,
            T::Percent => ScalaElementType::Percent,
            T::Eq => ScalaElementType::Eq,
            T::EqEq => ScalaElementType::EqEq,
            T::Ne => ScalaElementType::Ne,
            T::Lt => ScalaElementType::Lt,
            T::Le => ScalaElementType::Le,
            T::Gt => ScalaElementType::Gt,
            T::Ge => ScalaElementType::Ge,
            T::LessEqual => ScalaElementType::LessEqual,
            T::GreaterEqual => ScalaElementType::GreaterEqual,
            T::EqualEqual => ScalaElementType::EqualEqual,
            T::NotEqual => ScalaElementType::NotEqual,
            T::And => ScalaElementType::And,
            T::Or => ScalaElementType::Or,
            T::Xor => ScalaElementType::Xor,
            T::AndAnd => ScalaElementType::AndAnd,
            T::OrOr => ScalaElementType::OrOr,
            T::Not => ScalaElementType::Not,
            T::Tilde => ScalaElementType::Tilde,
            T::LShift => ScalaElementType::LShift,
            T::RShift => ScalaElementType::RShift,
            T::URShift => ScalaElementType::URShift,
            T::PlusEq => ScalaElementType::PlusEq,
            T::MinusEq => ScalaElementType::MinusEq,
            T::StarEq => ScalaElementType::StarEq,
            T::SlashEq => ScalaElementType::SlashEq,
            T::PercentEq => ScalaElementType::PercentEq,
            T::AndEq => ScalaElementType::AndEq,
            T::OrEq => ScalaElementType::OrEq,
            T::XorEq => ScalaElementType::XorEq,
            T::LShiftEq => ScalaElementType::LShiftEq,
            T::RShiftEq => ScalaElementType::RShiftEq,
            T::URShiftEq => ScalaElementType::URShiftEq,
            T::Arrow => ScalaElementType::Arrow,
            T::LeftArrow => ScalaElementType::LeftArrow,
            T::Colon => ScalaElementType::Colon,
            T::ColonColon => ScalaElementType::ColonColon,
            T::Semicolon => ScalaElementType::Semicolon,
            T::Dot => ScalaElementType::Dot,
            T::Comma => ScalaElementType::Comma,
            T::Question => ScalaElementType::Question,
            T::At => ScalaElementType::At,
            T::Hash => ScalaElementType::Hash,
            T::LeftParen => ScalaElementType::LeftParen,
            T::RightParen => ScalaElementType::RightParen,
            T::LeftBracket => ScalaElementType::LeftBracket,
            T::RightBracket => ScalaElementType::RightBracket,
            T::LeftBrace => ScalaElementType::LeftBrace,
            T::RightBrace => ScalaElementType::RightBrace,
        }
    }
}
