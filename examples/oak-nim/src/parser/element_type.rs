use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Nim language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum NimElementType {
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
    /// When statement (compile-time).
    WhenStmt,
    /// Static statement (compile-time).
    StaticStmt,
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

impl NimElementType {
    /// Returns true if the element type is a token.
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    /// Returns true if the element type is a non-terminal element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }
}

impl ElementType for NimElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::ProcDecl | Self::VarDecl | Self::LetDecl | Self::ConstDecl | Self::TypeDecl => UniversalElementRole::Definition,
            Self::ImportDecl => UniversalElementRole::Metadata,
            Self::IfStmt | Self::WhileStmt | Self::ForStmt | Self::CaseStmt | Self::BlockStmt => UniversalElementRole::Statement,
            Self::Expression => UniversalElementRole::Expression,
            Self::Comment => UniversalElementRole::Documentation,
            Self::ErrorNode | Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::NimTokenType> for NimElementType {
    fn from(token: crate::lexer::token_type::NimTokenType) -> Self {
                match token {
            crate::lexer::token_type::NimTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::NimTokenType::Newline => Self::Newline,
            crate::lexer::token_type::NimTokenType::CommentToken => Self::CommentToken,
            crate::lexer::token_type::NimTokenType::AddrKeyword => Self::AddrKeyword,
            crate::lexer::token_type::NimTokenType::AndKeyword => Self::AndKeyword,
            crate::lexer::token_type::NimTokenType::AsKeyword => Self::AsKeyword,
            crate::lexer::token_type::NimTokenType::AsmKeyword => Self::AsmKeyword,
            crate::lexer::token_type::NimTokenType::BindKeyword => Self::BindKeyword,
            crate::lexer::token_type::NimTokenType::BlockKeyword => Self::BlockKeyword,
            crate::lexer::token_type::NimTokenType::BreakKeyword => Self::BreakKeyword,
            crate::lexer::token_type::NimTokenType::CaseKeyword => Self::CaseKeyword,
            crate::lexer::token_type::NimTokenType::CastKeyword => Self::CastKeyword,
            crate::lexer::token_type::NimTokenType::ConceptKeyword => Self::ConceptKeyword,
            crate::lexer::token_type::NimTokenType::ConstKeyword => Self::ConstKeyword,
            crate::lexer::token_type::NimTokenType::ContinueKeyword => Self::ContinueKeyword,
            crate::lexer::token_type::NimTokenType::ConverterKeyword => Self::ConverterKeyword,
            crate::lexer::token_type::NimTokenType::DeferKeyword => Self::DeferKeyword,
            crate::lexer::token_type::NimTokenType::DiscardKeyword => Self::DiscardKeyword,
            crate::lexer::token_type::NimTokenType::DistinctKeyword => Self::DistinctKeyword,
            crate::lexer::token_type::NimTokenType::DivKeyword => Self::DivKeyword,
            crate::lexer::token_type::NimTokenType::DoKeyword => Self::DoKeyword,
            crate::lexer::token_type::NimTokenType::ElifKeyword => Self::ElifKeyword,
            crate::lexer::token_type::NimTokenType::ElseKeyword => Self::ElseKeyword,
            crate::lexer::token_type::NimTokenType::EndKeyword => Self::EndKeyword,
            crate::lexer::token_type::NimTokenType::EnumKeyword => Self::EnumKeyword,
            crate::lexer::token_type::NimTokenType::ExceptKeyword => Self::ExceptKeyword,
            crate::lexer::token_type::NimTokenType::ExportKeyword => Self::ExportKeyword,
            crate::lexer::token_type::NimTokenType::FinallyKeyword => Self::FinallyKeyword,
            crate::lexer::token_type::NimTokenType::ForKeyword => Self::ForKeyword,
            crate::lexer::token_type::NimTokenType::FromKeyword => Self::FromKeyword,
            crate::lexer::token_type::NimTokenType::FuncKeyword => Self::FuncKeyword,
            crate::lexer::token_type::NimTokenType::IfKeyword => Self::IfKeyword,
            crate::lexer::token_type::NimTokenType::ImportKeyword => Self::ImportKeyword,
            crate::lexer::token_type::NimTokenType::InKeyword => Self::InKeyword,
            crate::lexer::token_type::NimTokenType::IncludeKeyword => Self::IncludeKeyword,
            crate::lexer::token_type::NimTokenType::InterfaceKeyword => Self::InterfaceKeyword,
            crate::lexer::token_type::NimTokenType::IsKeyword => Self::IsKeyword,
            crate::lexer::token_type::NimTokenType::IteratorKeyword => Self::IteratorKeyword,
            crate::lexer::token_type::NimTokenType::LetKeyword => Self::LetKeyword,
            crate::lexer::token_type::NimTokenType::MacroKeyword => Self::MacroKeyword,
            crate::lexer::token_type::NimTokenType::MethodKeyword => Self::MethodKeyword,
            crate::lexer::token_type::NimTokenType::MixinKeyword => Self::MixinKeyword,
            crate::lexer::token_type::NimTokenType::ModKeyword => Self::ModKeyword,
            crate::lexer::token_type::NimTokenType::NilKeyword => Self::NilKeyword,
            crate::lexer::token_type::NimTokenType::NotKeyword => Self::NotKeyword,
            crate::lexer::token_type::NimTokenType::NotnilKeyword => Self::NotnilKeyword,
            crate::lexer::token_type::NimTokenType::ObjectKeyword => Self::ObjectKeyword,
            crate::lexer::token_type::NimTokenType::OfKeyword => Self::OfKeyword,
            crate::lexer::token_type::NimTokenType::OrKeyword => Self::OrKeyword,
            crate::lexer::token_type::NimTokenType::OutKeyword => Self::OutKeyword,
            crate::lexer::token_type::NimTokenType::ProcKeyword => Self::ProcKeyword,
            crate::lexer::token_type::NimTokenType::PtrKeyword => Self::PtrKeyword,
            crate::lexer::token_type::NimTokenType::RaiseKeyword => Self::RaiseKeyword,
            crate::lexer::token_type::NimTokenType::RefKeyword => Self::RefKeyword,
            crate::lexer::token_type::NimTokenType::ReturnKeyword => Self::ReturnKeyword,
            crate::lexer::token_type::NimTokenType::ShlKeyword => Self::ShlKeyword,
            crate::lexer::token_type::NimTokenType::ShrKeyword => Self::ShrKeyword,
            crate::lexer::token_type::NimTokenType::StaticKeyword => Self::StaticKeyword,
            crate::lexer::token_type::NimTokenType::TemplateKeyword => Self::TemplateKeyword,
            crate::lexer::token_type::NimTokenType::TryKeyword => Self::TryKeyword,
            crate::lexer::token_type::NimTokenType::TupleKeyword => Self::TupleKeyword,
            crate::lexer::token_type::NimTokenType::TypeKeyword => Self::TypeKeyword,
            crate::lexer::token_type::NimTokenType::UsingKeyword => Self::UsingKeyword,
            crate::lexer::token_type::NimTokenType::VarKeyword => Self::VarKeyword,
            crate::lexer::token_type::NimTokenType::WhenKeyword => Self::WhenKeyword,
            crate::lexer::token_type::NimTokenType::WhileKeyword => Self::WhileKeyword,
            crate::lexer::token_type::NimTokenType::XorKeyword => Self::XorKeyword,
            crate::lexer::token_type::NimTokenType::YieldKeyword => Self::YieldKeyword,
            crate::lexer::token_type::NimTokenType::Plus => Self::Plus,
            crate::lexer::token_type::NimTokenType::Minus => Self::Minus,
            crate::lexer::token_type::NimTokenType::Star => Self::Star,
            crate::lexer::token_type::NimTokenType::Slash => Self::Slash,
            crate::lexer::token_type::NimTokenType::Percent => Self::Percent,
            crate::lexer::token_type::NimTokenType::Equal => Self::Equal,
            crate::lexer::token_type::NimTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::NimTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::NimTokenType::Less => Self::Less,
            crate::lexer::token_type::NimTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::NimTokenType::Greater => Self::Greater,
            crate::lexer::token_type::NimTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::NimTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::NimTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::NimTokenType::Caret => Self::Caret,
            crate::lexer::token_type::NimTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::NimTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::NimTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::NimTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::NimTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::NimTokenType::At => Self::At,
            crate::lexer::token_type::NimTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::NimTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::NimTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::NimTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::NimTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::NimTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::NimTokenType::Comma => Self::Comma,
            crate::lexer::token_type::NimTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::NimTokenType::Colon => Self::Colon,
            crate::lexer::token_type::NimTokenType::Dot => Self::Dot,
            crate::lexer::token_type::NimTokenType::Question => Self::Question,
            crate::lexer::token_type::NimTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::NimTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::NimTokenType::Backtick => Self::Backtick,
            crate::lexer::token_type::NimTokenType::IntLiteral => Self::IntLiteral,
            crate::lexer::token_type::NimTokenType::FloatLiteral => Self::FloatLiteral,
            crate::lexer::token_type::NimTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::NimTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::NimTokenType::BoolLiteral => Self::BoolLiteral,
            crate::lexer::token_type::NimTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::NimTokenType::Root => Self::Root,
            crate::lexer::token_type::NimTokenType::ProcDecl => Self::ProcDecl,
            crate::lexer::token_type::NimTokenType::VarDecl => Self::VarDecl,
            crate::lexer::token_type::NimTokenType::LetDecl => Self::LetDecl,
            crate::lexer::token_type::NimTokenType::ConstDecl => Self::ConstDecl,
            crate::lexer::token_type::NimTokenType::TypeDecl => Self::TypeDecl,
            crate::lexer::token_type::NimTokenType::IfStmt => Self::IfStmt,
            crate::lexer::token_type::NimTokenType::WhileStmt => Self::WhileStmt,
            crate::lexer::token_type::NimTokenType::ForStmt => Self::ForStmt,
            crate::lexer::token_type::NimTokenType::CaseStmt => Self::CaseStmt,
            crate::lexer::token_type::NimTokenType::BlockStmt => Self::BlockStmt,
            crate::lexer::token_type::NimTokenType::Expression => Self::Expression,
            crate::lexer::token_type::NimTokenType::Literal => Self::Literal,
            crate::lexer::token_type::NimTokenType::Comment => Self::Comment,
            crate::lexer::token_type::NimTokenType::ImportDecl => Self::ImportDecl,
            crate::lexer::token_type::NimTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::NimTokenType::Error => Self::Error,
            crate::lexer::token_type::NimTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
