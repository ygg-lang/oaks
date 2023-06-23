use oak_core::{TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// CSS token type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CssTokenType {
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,
    /// Comment
    Comment,

    /// String literal
    StringLiteral,
    /// Number literal
    NumberLiteral,
    /// Color literal
    ColorLiteral,
    /// Url literal
    UrlLiteral,

    /// Identifier
    Identifier,
    /// Class name
    ClassName,
    /// Id selector
    IdSelector,
    /// Tag name
    TagName,
    /// Pseudo class
    PseudoClass,
    /// Pseudo element
    PseudoElement,
    /// Attribute name
    AttributeName,
    /// Property name
    PropertyName,
    /// Function name
    FunctionName,

    /// !important
    Important,
    /// inherit
    Inherit,
    /// initial
    Initial,
    /// unset
    Unset,
    /// auto
    Auto,
    /// none
    None,
    /// normal
    Normal,

    /// px
    Px,
    /// em
    Em,
    /// rem
    Rem,
    /// %
    Percent,
    /// vh
    Vh,
    /// vw
    Vw,
    /// pt
    Pt,
    /// cm
    Cm,
    /// mm
    Mm,
    /// in
    In,
    /// pc
    Pc,
    /// ex
    Ex,
    /// ch
    Ch,
    /// vmin
    Vmin,
    /// vmax
    Vmax,

    /// :
    Colon,
    /// ;
    Semicolon,
    /// ,
    Comma,
    /// .
    Dot,
    /// #
    Hash,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Star,
    /// /
    Slash,
    /// =
    Equals,
    /// ~
    Tilde,
    /// |
    Pipe,
    /// ^
    Caret,
    /// $
    Dollar,
    /// >
    GreaterThan,

    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// [
    LeftBracket,
    /// ]
    RightBracket,

    /// @import
    AtImport,
    /// @media
    AtMedia,
    /// @keyframes
    AtKeyframes,
    /// @font-face
    AtFontFace,
    /// @charset
    AtCharset,
    /// @namespace
    AtNamespace,
    /// @supports
    AtSupports,
    /// @page
    AtPage,
    /// @document
    AtDocument,
    /// Generic @-rule
    AtRule,

    /// End of file
    Eof,
    /// Error
    Error,
}

impl TokenType for CssTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::ColorLiteral | Self::UrlLiteral => UniversalTokenRole::Literal,
            Self::Identifier | Self::ClassName | Self::IdSelector | Self::TagName | Self::PseudoClass | Self::PseudoElement | Self::AttributeName | Self::PropertyName | Self::FunctionName => UniversalTokenRole::Name,
            Self::Important
            | Self::Inherit
            | Self::Initial
            | Self::Unset
            | Self::Auto
            | Self::None
            | Self::Normal
            | Self::AtImport
            | Self::AtMedia
            | Self::AtKeyframes
            | Self::AtFontFace
            | Self::AtCharset
            | Self::AtNamespace
            | Self::AtSupports
            | Self::AtPage
            | Self::AtDocument => UniversalTokenRole::Keyword,
            Self::Colon | Self::Semicolon | Self::Comma | Self::Dot | Self::Hash | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Equals | Self::Tilde | Self::Pipe | Self::Caret | Self::Dollar | Self::GreaterThan => {
                UniversalTokenRole::Operator
            }
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
