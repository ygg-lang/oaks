use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::Colon | Self::Semicolon | Self::Comma | Self::Dot | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OCamlTokenType {
    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 关键字
    And,
    As,
    Assert,
    Begin,
    Class,
    Constraint,
    Do,
    Done,
    Downto,
    Else,
    End,
    Exception,
    External,
    False,
    For,
    Fun,
    Function,
    Functor,
    If,
    In,
    Include,
    Inherit,
    Initializer,
    Lazy,
    Let,
    Match,
    Method,
    Module,
    Mutable,
    New,
    Object,
    Of,
    Open,
    Or,
    Private,
    Rec,
    Sig,
    Struct,
    Then,
    To,
    True,
    Try,
    Type,
    Val,
    Virtual,
    When,
    While,
    With,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LeftArrow,
    RightArrow,
    OrOr,
    AndAnd,
    ColonColon,
    Pipe,
    Ampersand,
    Bang,
    Question,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Caret,
    Tilde,
    At,
    Hash,
    Dollar,
    Backtick,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // 特殊

    // Element kinds
    Root,
    ModuleDef,
    LetBinding,
    MatchExpr,
    FunctionDef,
    TypeDefinition,
    Expression,

    // 错误和结束
    Error,
    Eof,
}
