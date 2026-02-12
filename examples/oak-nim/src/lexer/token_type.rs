use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for the Nim language.
pub type NimToken = Token<NimTokenType>;

impl NimTokenType {
    /// Returns true if the token type is a token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Returns true if the token type is a non-terminal element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }

    /// Returns true if the token type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::AddrKeyword
                | Self::AndKeyword
                | Self::AsKeyword
                | Self::AsmKeyword
                | Self::BindKeyword
                | Self::BlockKeyword
                | Self::BreakKeyword
                | Self::CaseKeyword
                | Self::CastKeyword
                | Self::ConceptKeyword
                | Self::ConstKeyword
                | Self::ContinueKeyword
                | Self::ConverterKeyword
                | Self::DeferKeyword
                | Self::DiscardKeyword
                | Self::DistinctKeyword
                | Self::DivKeyword
                | Self::DoKeyword
                | Self::ElifKeyword
                | Self::ElseKeyword
                | Self::EndKeyword
                | Self::EnumKeyword
                | Self::ExceptKeyword
                | Self::ExportKeyword
                | Self::FinallyKeyword
                | Self::ForKeyword
                | Self::FromKeyword
                | Self::FuncKeyword
                | Self::IfKeyword
                | Self::ImportKeyword
                | Self::InKeyword
                | Self::IncludeKeyword
                | Self::InterfaceKeyword
                | Self::IsKeyword
                | Self::IteratorKeyword
                | Self::LetKeyword
                | Self::MacroKeyword
                | Self::MethodKeyword
                | Self::MixinKeyword
                | Self::ModKeyword
                | Self::NilKeyword
                | Self::NotKeyword
                | Self::NotnilKeyword
                | Self::ObjectKeyword
                | Self::OfKeyword
                | Self::OrKeyword
                | Self::OutKeyword
                | Self::ProcKeyword
                | Self::PtrKeyword
                | Self::RaiseKeyword
                | Self::RefKeyword
                | Self::ReturnKeyword
                | Self::ShlKeyword
                | Self::ShrKeyword
                | Self::StaticKeyword
                | Self::TemplateKeyword
                | Self::TryKeyword
                | Self::TupleKeyword
                | Self::TypeKeyword
                | Self::UsingKeyword
                | Self::VarKeyword
                | Self::WhenKeyword
                | Self::WhileKeyword
                | Self::XorKeyword
                | Self::YieldKeyword
        )
    }

    /// Returns true if the token type is an operator.
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
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::Ampersand
                | Self::Pipe
                | Self::Caret
                | Self::Tilde
                | Self::LeftShift
                | Self::RightShift
                | Self::DotDot
                | Self::Arrow
                | Self::At
        )
    }

    /// Returns true if the token type is punctuation.
    pub fn is_punctuation(&self) -> bool {
        matches!(
            self,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Semicolon | Self::Colon | Self::Dot | Self::Question | Self::Exclamation | Self::Dollar | Self::Backtick
        )
    }
}

impl TokenType for NimTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::CommentToken)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken | Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Nim language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NimTokenType {
    // Whitespace and comments
    /// Whitespace characters.
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment token.
    CommentToken,

    // Keywords
    /// The `addr` keyword.
    AddrKeyword,
    /// The `and` keyword.
    AndKeyword,
    /// The `as` keyword.
    AsKeyword,
    /// The `asm` keyword.
    AsmKeyword,
    /// The `bind` keyword.
    BindKeyword,
    /// The `block` keyword.
    BlockKeyword,
    /// The `break` keyword.
    BreakKeyword,
    /// The `case` keyword.
    CaseKeyword,
    /// The `cast` keyword.
    CastKeyword,
    /// The `concept` keyword.
    ConceptKeyword,
    /// The `const` keyword.
    ConstKeyword,
    /// The `continue` keyword.
    ContinueKeyword,
    /// The `converter` keyword.
    ConverterKeyword,
    /// The `defer` keyword.
    DeferKeyword,
    /// The `discard` keyword.
    DiscardKeyword,
    /// The `distinct` keyword.
    DistinctKeyword,
    /// The `div` keyword.
    DivKeyword,
    /// The `do` keyword.
    DoKeyword,
    /// The `elif` keyword.
    ElifKeyword,
    /// The `else` keyword.
    ElseKeyword,
    /// The `end` keyword.
    EndKeyword,
    /// The `enum` keyword.
    EnumKeyword,
    /// The `except` keyword.
    ExceptKeyword,
    /// The `export` keyword.
    ExportKeyword,
    /// The `finally` keyword.
    FinallyKeyword,
    /// The `for` keyword.
    ForKeyword,
    /// The `from` keyword.
    FromKeyword,
    /// The `func` keyword.
    FuncKeyword,
    /// The `if` keyword.
    IfKeyword,
    /// The `import` keyword.
    ImportKeyword,
    /// The `in` keyword.
    InKeyword,
    /// The `include` keyword.
    IncludeKeyword,
    /// The `interface` keyword.
    InterfaceKeyword,
    /// The `is` keyword.
    IsKeyword,
    /// The `iterator` keyword.
    IteratorKeyword,
    /// The `let` keyword.
    LetKeyword,
    /// The `macro` keyword.
    MacroKeyword,
    /// The `method` keyword.
    MethodKeyword,
    /// The `mixin` keyword.
    MixinKeyword,
    /// The `mod` keyword.
    ModKeyword,
    /// The `nil` keyword.
    NilKeyword,
    /// The `not` keyword.
    NotKeyword,
    /// The `notnil` keyword.
    NotnilKeyword,
    /// The `object` keyword.
    ObjectKeyword,
    /// The `of` keyword.
    OfKeyword,
    /// The `or` keyword.
    OrKeyword,
    /// The `out` keyword.
    OutKeyword,
    /// The `proc` keyword.
    ProcKeyword,
    /// The `ptr` keyword.
    PtrKeyword,
    /// The `raise` keyword.
    RaiseKeyword,
    /// The `ref` keyword.
    RefKeyword,
    /// The `return` keyword.
    ReturnKeyword,
    /// The `shl` keyword.
    ShlKeyword,
    /// The `shr` keyword.
    ShrKeyword,
    /// The `static` keyword.
    StaticKeyword,
    /// The `template` keyword.
    TemplateKeyword,
    /// The `try` keyword.
    TryKeyword,
    /// The `tuple` keyword.
    TupleKeyword,
    /// The `type` keyword.
    TypeKeyword,
    /// The `using` keyword.
    UsingKeyword,
    /// The `var` keyword.
    VarKeyword,
    /// The `when` keyword.
    WhenKeyword,
    /// The `while` keyword.
    WhileKeyword,
    /// The `xor` keyword.
    XorKeyword,
    /// The `yield` keyword.
    YieldKeyword,

    // Operators
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
    /// The `<=` operator.
    LessEqual,
    /// The `>` operator.
    Greater,
    /// The `>=` operator.
    GreaterEqual,
    /// The `&` operator.
    Ampersand,
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `~` operator.
    Tilde,
    /// The `<<` operator.
    LeftShift,
    /// The `>>` operator.
    RightShift,
    /// The `..` operator.
    DotDot,
    /// The `->` operator.
    Arrow,
    /// The `@` operator.
    At,

    // Punctuation
    /// An opening parenthesis `(`.
    LeftParen,
    /// A closing parenthesis `)`.
    RightParen,
    /// An opening bracket `[`.
    LeftBracket,
    /// A closing bracket `]`.
    RightBracket,
    /// An opening brace `{`.
    LeftBrace,
    /// A closing brace `}`.
    RightBrace,
    /// A comma `,`.
    Comma,
    /// A semicolon `;`.
    Semicolon,
    /// A colon `:`.
    Colon,
    /// A dot `.`.
    Dot,
    /// A question mark `?`.
    Question,
    /// An exclamation mark `!`.
    Exclamation,
    /// A dollar sign `$`.
    Dollar,
    /// A backtick `` ` ``.
    Backtick,

    // Literals
    /// An integer literal.
    IntLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A boolean literal.
    BoolLiteral,

    // Identifiers
    /// An identifier.
    Identifier,

    // Special
    /// Root node of the AST.
    Root,
    /// Procedure declaration.
    ProcDecl,
    /// Variable declaration.
    VarDecl,
    /// Let declaration.
    LetDecl,
    /// Constant declaration.
    ConstDecl,
    /// Type declaration.
    TypeDecl,
    /// If statement.
    IfStmt,
    /// While statement.
    WhileStmt,
    /// For statement.
    ForStmt,
    /// Case statement.
    CaseStmt,
    /// Block statement.
    BlockStmt,
    /// An expression.
    Expression,
    /// A literal.
    Literal,
    /// A comment.
    Comment,
    /// Import declaration.
    ImportDecl,
    /// An error node in the AST.
    ErrorNode,
    /// An error element.
    Error,
    /// End of stream.
    Eof,
}
