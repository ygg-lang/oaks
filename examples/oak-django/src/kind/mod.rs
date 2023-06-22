use oak_core::{Token, TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type DjangoToken = Token<DjangoSyntaxKind>;

/// Django 模板语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DjangoSyntaxKind {
    // 基本 kind
    Identifier,
    Number,
    String,
    Whitespace,
    Newline,

    // Django 模板标签
    VariableStart, // {{
    VariableEnd,   // }}
    TagStart,      // {%
    TagEnd,        // %}
    CommentStart,  // {#
    CommentEnd,    // #}

    // Django 标签关键字
    If,
    Elif,
    Else,
    Endif,
    For,
    Empty,
    Endfor,
    Block,
    Endblock,
    Extends,
    Include,
    Load,
    With,
    Endwith,
    Autoescape,
    Endautoescape,
    Csrf,
    Url,
    Static,
    Now,
    Cycle,
    Filter,
    Endfilter,
    Spaceless,
    Endspaceless,
    Verbatim,
    Endverbatim,

    // 过滤器和操作符
    Pipe,    // |
    Colon,   // :
    Dot,     // .
    Comma,   // ,
    Equal,   // =
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // 比较操作符
    EqualEqual,   // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // 逻辑操作符
    And,
    Or,
    Not,
    In,

    // 括号
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;

    // HTML 内容
    HtmlText,
    HtmlTag,

    // 注释
    Comment,

    // 节点类型
    Variable,
    Tag,

    // 特殊
    Root,
    Error,
    Eof,
}

impl DjangoSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Elif
                | Self::Else
                | Self::Endif
                | Self::For
                | Self::Empty
                | Self::Endfor
                | Self::Block
                | Self::Endblock
                | Self::Extends
                | Self::Include
                | Self::Load
                | Self::With
                | Self::Endwith
                | Self::Autoescape
                | Self::Endautoescape
                | Self::Csrf
                | Self::Url
                | Self::Static
                | Self::Now
                | Self::Cycle
                | Self::Filter
                | Self::Endfilter
                | Self::Spaceless
                | Self::Endspaceless
                | Self::Verbatim
                | Self::Endverbatim
                | Self::And
                | Self::Or
                | Self::Not
                | Self::In
        )
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for DjangoSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        if self.is_keyword() {
            return UniversalTokenRole::Keyword;
        }

        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String => UniversalTokenRole::Literal,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::VariableStart | Self::VariableEnd | Self::TagStart | Self::TagEnd | Self::CommentStart | Self::CommentEnd => UniversalTokenRole::Punctuation,
            Self::Pipe | Self::Colon | Self::Dot | Self::Comma | Self::Equal | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::EqualEqual | Self::NotEqual | Self::Less | Self::LessEqual | Self::Greater | Self::GreaterEqual => {
                UniversalTokenRole::Operator
            }
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Semicolon => UniversalTokenRole::Punctuation,
            Self::HtmlText | Self::HtmlTag => UniversalTokenRole::None,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl oak_core::ElementType for DjangoSyntaxKind {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => oak_core::UniversalElementRole::Root,
            Self::Variable | Self::Tag => oak_core::UniversalElementRole::Statement,
            Self::Error => oak_core::UniversalElementRole::Error,
            _ => oak_core::UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
