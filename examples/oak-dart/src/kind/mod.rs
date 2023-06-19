use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type DartToken = Token<DartSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DartSyntaxKind {
    // 基础
    Whitespace,
    Newline,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    DoubleLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,

    // 关键字
    Abstract,
    As,
    Assert,
    Async,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Covariant,
    Default,
    Deferred,
    Do,
    Dynamic,
    Else,
    Enum,
    Export,
    Extends,
    Extension,
    External,
    Factory,
    False,
    Final,
    Finally,
    For,
    Function,
    Get,
    Hide,
    If,
    Implements,
    Import,
    In,
    Interface,
    Is,
    Late,
    Library,
    Mixin,
    New,
    Null,
    On,
    Operator,
    Part,
    Required,
    Rethrow,
    Return,
    Set,
    Show,
    Static,
    Super,
    Switch,
    Sync,
    This,
    Throw,
    True,
    Try,
    Typedef,
    Var,
    Void,
    While,
    With,
    Yield,

    // 操作符
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Percent,               // %
    TildeSlash,            // ~/
    Equal,                 // =
    EqualEqual,            // ==
    BangEqual,             // !=
    Less,                  // <
    Greater,               // >
    LessEqual,             // <=
    GreaterEqual,          // >=
    LeftShift,             // <<
    RightShift,            // >>
    Ampersand,             // &
    Pipe,                  // |
    Caret,                 // ^
    Tilde,                 // ~
    Bang,                  // !
    AmpersandAmpersand,    // &&
    PipePipe,              // ||
    Question,              // ?
    QuestionQuestion,      // ??
    PlusPlus,              // ++
    MinusMinus,            // --
    PlusEqual,             // +=
    MinusEqual,            // -=
    StarEqual,             // *=
    SlashEqual,            // /=
    PercentEqual,          // %=
    TildeSlashEqual,       // ~/=
    LeftShiftEqual,        // <<=
    RightShiftEqual,       // >>=
    AmpersandEqual,        // &=
    PipeEqual,             // |=
    CaretEqual,            // ^=
    QuestionQuestionEqual, // ??=
    Arrow,                 // =>
    Dot,                   // .
    DotDot,                // ..
    DotDotDot,             // ...
    QuestionDot,           // ?.

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Semicolon,    // ;
    Comma,        // ,
    Colon,        // :
    At,           // @
    Hash,         // #

    // 注释
    LineComment,
    BlockComment,
    DocComment,

    // 错误处理
    Error,

    // EOF
    Eof,
}

impl SyntaxKind for DartSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            Self::Whitespace
                | Self::Newline
                | Self::Identifier
                | Self::IntegerLiteral
                | Self::DoubleLiteral
                | Self::StringLiteral
                | Self::BooleanLiteral
                | Self::NullLiteral
                | Self::Abstract
                | Self::As
                | Self::Assert
                | Self::Async
                | Self::Await
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Covariant
                | Self::Default
                | Self::Deferred
                | Self::Do
                | Self::Dynamic
                | Self::Else
                | Self::Enum
                | Self::Export
                | Self::Extends
                | Self::Extension
                | Self::External
                | Self::Factory
                | Self::False
                | Self::Final
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::Get
                | Self::Hide
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Interface
                | Self::Is
                | Self::Late
                | Self::Library
                | Self::Mixin
                | Self::New
                | Self::Null
                | Self::On
                | Self::Operator
                | Self::Part
                | Self::Required
                | Self::Rethrow
                | Self::Return
                | Self::Set
                | Self::Show
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::Sync
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typedef
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Yield
                | Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::TildeSlash
                | Self::Equal
                | Self::EqualEqual
                | Self::BangEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::LeftShift
                | Self::RightShift
                | Self::Ampersand
                | Self::Pipe
                | Self::Caret
                | Self::Tilde
                | Self::Bang
                | Self::AmpersandAmpersand
                | Self::PipePipe
                | Self::Question
                | Self::QuestionQuestion
                | Self::PlusPlus
                | Self::MinusMinus
                | Self::PlusEqual
                | Self::MinusEqual
                | Self::StarEqual
                | Self::SlashEqual
                | Self::PercentEqual
                | Self::TildeSlashEqual
                | Self::LeftShiftEqual
                | Self::RightShiftEqual
                | Self::AmpersandEqual
                | Self::PipeEqual
                | Self::CaretEqual
                | Self::QuestionQuestionEqual
                | Self::Arrow
                | Self::Dot
                | Self::DotDot
                | Self::DotDotDot
                | Self::QuestionDot
                | Self::LeftParen
                | Self::RightParen
                | Self::LeftBracket
                | Self::RightBracket
                | Self::LeftBrace
                | Self::RightBrace
                | Self::Semicolon
                | Self::Comma
                | Self::Colon
                | Self::At
                | Self::Hash
                | Self::LineComment
                | Self::BlockComment
                | Self::DocComment
                | Self::Error
                | Self::Eof
        )
    }

    fn is_element_type(&self) -> bool {
        !self.is_token_type()
    }
}
