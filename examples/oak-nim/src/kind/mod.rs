use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NimSyntaxKind {
    // Whitespace and comments
    Whitespace,
    Newline,
    CommentToken,

    // Keywords
    AddrKeyword,
    AndKeyword,
    AsKeyword,
    AsmKeyword,
    BindKeyword,
    BlockKeyword,
    BreakKeyword,
    CaseKeyword,
    CastKeyword,
    ConceptKeyword,
    ConstKeyword,
    ContinueKeyword,
    ConverterKeyword,
    DeferKeyword,
    DiscardKeyword,
    DistinctKeyword,
    DivKeyword,
    DoKeyword,
    ElifKeyword,
    ElseKeyword,
    EndKeyword,
    EnumKeyword,
    ExceptKeyword,
    ExportKeyword,
    FinallyKeyword,
    ForKeyword,
    FromKeyword,
    FuncKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    IncludeKeyword,
    InterfaceKeyword,
    IsKeyword,
    IteratorKeyword,
    LetKeyword,
    MacroKeyword,
    MethodKeyword,
    MixinKeyword,
    ModKeyword,
    NilKeyword,
    NotKeyword,
    NotnilKeyword,
    ObjectKeyword,
    OfKeyword,
    OrKeyword,
    OutKeyword,
    ProcKeyword,
    PtrKeyword,
    RaiseKeyword,
    RefKeyword,
    ReturnKeyword,
    ShlKeyword,
    ShrKeyword,
    StaticKeyword,
    TemplateKeyword,
    TryKeyword,
    TupleKeyword,
    TypeKeyword,
    UsingKeyword,
    VarKeyword,
    WhenKeyword,
    WhileKeyword,
    XorKeyword,
    YieldKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    DotDot,
    Arrow,
    At,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Question,
    Exclamation,
    Dollar,
    Backtick,

    // Literals
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    // Identifiers
    Identifier,

    // Special
    Root,
    ProcDecl,
    VarDecl,
    LetDecl,
    ConstDecl,
    TypeDecl,
    IfStmt,
    WhileStmt,
    ForStmt,
    CaseStmt,
    BlockStmt,
    Expression,
    Literal,
    Comment,
    ImportDecl,
    ErrorNode,
    Error,
    Eof,
}

impl NimSyntaxKind {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }
}

impl TokenType for NimSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken | Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken | Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for NimSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::ErrorNode | Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::ProcDecl | Self::TypeDecl | Self::VarDecl => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
