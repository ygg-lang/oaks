use oak_core::{Token, TokenType, UniversalTokenRole};

pub type JinjaToken = Token<JinjaTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JinjaTokenType {
    // Basic kinds
    Identifier,
    Number,
    String,
    Whitespace,
    Newline,
    Comment,

    // Jinja2 template tags
    VariableStart, // {{
    VariableEnd,   // }}
    TagStart,      // {%
    TagEnd,        // %}
    CommentStart,  // {#
    CommentEnd,    // #}

    // Jinja2 tag keywords
    If,
    Elif,
    Else,
    Endif,
    For,
    Endfor,
    Block,
    Endblock,
    Extends,
    Include,
    Import,
    From,
    Macro,
    Endmacro,
    Call,
    Endcall,
    Filter,
    Endfilter,
    Set,
    Endset,
    With,
    Endwith,
    Autoescape,
    Endautoescape,
    Do,
    And,
    Or,
    Not,
    In,
    Is,
    Recursive,
    Scoped,

    // Symbols
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
    DoubleStar, // **
    Slash,
    DoubleSlash, // //
    Percent,
    Tilde, // ~ (String concatenation)

    // Brackets
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // Others
    HtmlContent,
    Eof,
    Error,
}

impl JinjaTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::If
                | Self::Elif
                | Self::Else
                | Self::Endif
                | Self::For
                | Self::Endfor
                | Self::Block
                | Self::Endblock
                | Self::Extends
                | Self::Include
                | Self::Import
                | Self::From
                | Self::Macro
                | Self::Endmacro
                | Self::Call
                | Self::Endcall
                | Self::Filter
                | Self::Endfilter
                | Self::Set
                | Self::Endset
                | Self::With
                | Self::Endwith
                | Self::Autoescape
                | Self::Endautoescape
                | Self::Do
                | Self::And
                | Self::Or
                | Self::Not
                | Self::In
                | Self::Is
                | Self::Recursive
                | Self::Scoped
        )
    }
}

impl TokenType for JinjaTokenType {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::VariableStart | Self::VariableEnd | Self::TagStart | Self::TagEnd | Self::CommentStart | Self::CommentEnd => UniversalTokenRole::Punctuation,
            Self::Dot
            | Self::Pipe
            | Self::Colon
            | Self::Comma
            | Self::Equal
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::DoubleStar
            | Self::Slash
            | Self::DoubleSlash
            | Self::Percent
            | Self::Tilde => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::HtmlContent => UniversalTokenRole::Name,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::Error,
        }
    }
}
