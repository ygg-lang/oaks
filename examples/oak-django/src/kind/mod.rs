use oak_core::SyntaxKind;
use serde::Serialize;

/// Django 模板语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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

impl SyntaxKind for DjangoSyntaxKind {
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
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
