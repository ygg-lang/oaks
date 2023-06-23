use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type NimToken = Token<NimTokenType>;

impl NimTokenType {
    pub fn is_token(&self) -> bool {
        !self.is_element()
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::ProcDecl | Self::TypeDecl | Self::VarDecl | Self::ConstDecl | Self::LetDecl | Self::ImportDecl | Self::Comment | Self::ErrorNode)
    }

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

    pub fn is_punctuation(&self) -> bool {
        matches!(
            self,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Semicolon | Self::Colon | Self::Dot | Self::Question | Self::Exclamation | Self::Dollar | Self::Backtick
        )
    }
}

impl TokenType for NimTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NimTokenType {
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
