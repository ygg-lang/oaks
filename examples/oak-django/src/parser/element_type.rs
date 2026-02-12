use oak_core::{ElementType, UniversalElementRole};

/// Element types for Django templates.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DjangoElementType {
    /// The root element.
    Root,
    /// An identifier.
    Identifier,
    /// A variable expression: `{{ ... }}`.
    Variable,
    /// A tag: `{% ... %}`.
    Tag,
    /// A comment: `{# ... #}`.
    Comment,
    /// Plain HTML content.
    HtmlContent,

    /// `if` keyword.
    If,
    /// `elif` keyword.
    Elif,
    /// `else` keyword.
    Else,
    /// `endif` keyword.
    Endif,
    /// `for` keyword.
    For,
    /// `empty` keyword.
    Empty,
    /// `endfor` keyword.
    Endfor,
    /// `block` keyword.
    Block,
    /// `endblock` keyword.
    Endblock,
    /// `extends` keyword.
    Extends,
    /// `include` keyword.
    Include,
    /// `load` keyword.
    Load,
    /// `with` keyword.
    With,
    /// `endwith` keyword.
    Endwith,
    /// `autoescape` keyword.
    Autoescape,
    /// `endautoescape` keyword.
    Endautoescape,
    /// `csrf_token` keyword.
    Csrf,
    /// `url` keyword.
    Url,
    /// `static` keyword.
    Static,
    /// `now` keyword.
    Now,
    /// `cycle` keyword.
    Cycle,
    /// `filter` keyword.
    Filter,
    /// `endfilter` keyword.
    Endfilter,
    /// `spaceless` keyword.
    Spaceless,
    /// `endspaceless` keyword.
    Endspaceless,
    /// `verbatim` keyword.
    Verbatim,
    /// `endverbatim` keyword.
    Endverbatim,
    /// `and` operator.
    And,
    /// `or` operator.
    Or,
    /// `not` operator.
    Not,
    /// `in` operator.
    In,

    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `;`.
    Semicolon,
    /// `.`.
    Dot,
    /// `,`.
    Comma,
    /// `=`.
    Equal,

    // Grammar constructs
    IfStatement,
    ForStatement,
    BlockStatement,
    FilterExpression,
    BinaryExpression,
    Literal,

    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Error token.
    Error,
}

impl DjangoElementType {
    /// Returns true if the element type is a keyword.
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

    /// Returns true if the element type is trivia (whitespace, newline, or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl ElementType for DjangoElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Variable => UniversalElementRole::Expression,
            Self::Tag => UniversalElementRole::Statement,
            Self::Comment => UniversalElementRole::Documentation,
            Self::HtmlContent => UniversalElementRole::Embedded,
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
            | Self::In => UniversalElementRole::Statement,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DjangoTokenType> for DjangoElementType {
    fn from(token: crate::lexer::token_type::DjangoTokenType) -> Self {
        use crate::lexer::token_type::DjangoTokenType as T;
        match token {
            T::If => Self::If,
            T::Elif => Self::Elif,
            T::Else => Self::Else,
            T::Endif => Self::Endif,
            T::For => Self::For,
            T::Empty => Self::Empty,
            T::Endfor => Self::Endfor,
            T::Block => Self::Block,
            T::Endblock => Self::Endblock,
            T::Extends => Self::Extends,
            T::Include => Self::Include,
            T::Load => Self::Load,
            T::With => Self::With,
            T::Endwith => Self::Endwith,
            T::Autoescape => Self::Autoescape,
            T::Endautoescape => Self::Endautoescape,
            T::Csrf => Self::Csrf,
            T::Url => Self::Url,
            T::Static => Self::Static,
            T::Now => Self::Now,
            T::Cycle => Self::Cycle,
            T::Filter => Self::Filter,
            T::Endfilter => Self::Endfilter,
            T::Spaceless => Self::Spaceless,
            T::Endspaceless => Self::Endspaceless,
            T::Verbatim => Self::Verbatim,
            T::Endverbatim => Self::Endverbatim,
            T::And => Self::And,
            T::Or => Self::Or,
            T::Not => Self::Not,
            T::In => Self::In,
            T::Whitespace => Self::Whitespace,
            T::Newline => Self::Newline,
            T::Comment => Self::Comment,
            T::HtmlContent => Self::HtmlContent,
            _ => Self::Error,
        }
    }
}
