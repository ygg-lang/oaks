use oak_core::{Token, TokenType, UniversalTokenRole};

/// Django token type alias.
pub type DjangoToken = Token<DjangoTokenType>;

/// Token types for the Django template lexer.
///
/// This enum represents all possible token types in Django templates,
/// including template tags, variables, filters, and HTML content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DjangoTokenType {
    // Basic tokens
    /// Identifier token.
    Identifier,
    /// Number literal token.
    Number,
    /// String literal token.
    String,
    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,

    // Django template delimiters
    /// Variable start delimiter `{{`.
    VariableStart,
    /// Variable end delimiter `}}`.
    VariableEnd,
    /// Tag start delimiter `{%`.
    TagStart,
    /// Tag end delimiter `%}`.
    TagEnd,
    /// Comment start delimiter `{#`.
    CommentStart,
    /// Comment end delimiter `#}`.
    CommentEnd,

    // Django tag keywords
    /// `if` keyword for conditional blocks.
    If,
    /// `elif` keyword for else-if branches.
    Elif,
    /// `else` keyword for else branches.
    Else,
    /// `endif` keyword to close if blocks.
    Endif,
    /// `for` keyword for loop blocks.
    For,
    /// `empty` keyword for empty loop fallback.
    Empty,
    /// `endfor` keyword to close for loops.
    Endfor,
    /// `block` keyword for template inheritance.
    Block,
    /// `endblock` keyword to close block sections.
    Endblock,
    /// `extends` keyword for template inheritance.
    Extends,
    /// `include` keyword for including other templates.
    Include,
    /// `load` keyword for loading template tags.
    Load,
    /// `with` keyword for context variables.
    With,
    /// `endwith` keyword to close with blocks.
    Endwith,
    /// `autoescape` keyword for auto-escaping.
    Autoescape,
    /// `endautoescape` keyword to close autoescape blocks.
    Endautoescape,
    /// `csrf_token` tag for CSRF protection.
    Csrf,
    /// `url` tag for URL resolution.
    Url,
    /// `static` tag for static file references.
    Static,
    /// `now` tag for current datetime.
    Now,
    /// `cycle` tag for cycling values.
    Cycle,
    /// `filter` tag for filter sections.
    Filter,
    /// `endfilter` keyword to close filter blocks.
    Endfilter,
    /// `spaceless` tag for removing whitespace.
    Spaceless,
    /// `endspaceless` keyword to close spaceless blocks.
    Endspaceless,
    /// `verbatim` tag for literal content.
    Verbatim,
    /// `endverbatim` keyword to close verbatim blocks.
    Endverbatim,
    /// `and` logical operator.
    And,
    /// `or` logical operator.
    Or,
    /// `not` logical operator.
    Not,
    /// `in` membership operator.
    In,

    // Symbols
    /// Dot operator `.`.
    Dot,
    /// Pipe operator `|` for filters.
    Pipe,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Assignment operator `=`.
    Equal,
    /// Equality operator `==`.
    EqualEqual,
    /// Inequality operator `!=`.
    NotEqual,
    /// Less than operator `<`.
    Less,
    /// Greater than operator `>`.
    Greater,
    /// Less than or equal operator `<=`.
    LessEqual,
    /// Greater than or equal operator `>=`.
    GreaterEqual,
    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiplication operator `*`.
    Star,
    /// Division operator `/`.
    Slash,
    /// Modulo operator `%`.
    Percent,

    // Brackets
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,

    // Special tokens
    /// HTML content outside Django tags.
    HtmlContent,
    /// End of file token.
    Eof,
    /// Error token for invalid input.
    Error,
}

impl DjangoTokenType {
    /// Returns `true` if this token is a Django keyword.
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

    /// Returns true if this token type is trivia (whitespace, newline, or comment).
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
