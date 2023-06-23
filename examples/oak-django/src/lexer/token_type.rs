use oak_core::{Token, TokenRole, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DjangoToken = Token<DjangoTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DjangoTokenType {
    // 基本 kind
    Identifier,
    Number,
    String,
    Whitespace,
    Newline,
    Comment,

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
    And,
    Or,
    Not,
    In,

    // 符号
    Dot,
    Pipe,
    Colon,
    Comma,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // 括号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,

    // 其他
    HtmlContent,
    Eof,
    Error,
}

impl DjangoTokenType {
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

impl TokenType for DjangoTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        self.is_trivia()
    }

    fn role(&self) -> Self::Role {
        match self {
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
            | Self::In => UniversalTokenRole::Keyword,

            Self::Dot
            | Self::Pipe
            | Self::Colon
            | Self::Comma
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon => UniversalTokenRole::Punctuation,

            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String => UniversalTokenRole::Literal,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
