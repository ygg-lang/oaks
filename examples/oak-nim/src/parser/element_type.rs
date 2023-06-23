use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum NimElementType {
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

impl NimElementType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }
}

impl ElementType for NimElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::NimTokenType> for NimElementType {
    fn from(token: crate::lexer::token_type::NimTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
