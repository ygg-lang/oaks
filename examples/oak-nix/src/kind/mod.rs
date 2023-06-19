use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NixSyntaxKind {
    // 基础 kind
    Whitespace,
    Newline,
    Comment,
    String,
    Number,
    Boolean,
    True,
    False,
    Null,
    Identifier,

    // 关键
    Let,
    In,
    If,
    Then,
    Else,
    With,
    Inherit,
    Rec,
    Import,
    Assert,
    Or,
    And,
    Not,

    // 操作
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    Concatenation, // ++
    Update,        // //
    Implication,   // ->
    Equal,         // ==
    NotEqual,      // !=
    Less,          // <
    Greater,       // >
    LessEqual,     // <=
    GreaterEqual,  // >=
    LogicalAnd,    // &&
    LogicalOr,     // ||
    Assign,        // =
    Question,      // ?

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    At,           // @
    Dollar,       // $
    Hash,         // #

    // 特殊
    Error,
    Eof,
}

impl SyntaxKind for NixSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error)
    }
}
