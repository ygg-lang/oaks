use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum YamlSyntaxKind {
    // Trivia
    Whitespace,
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // Identifiers
    Identifier,

    // Operators and punctuation
    Colon,       // :
    Dash,        // -
    Pipe,        // |
    GreaterThan, // >
    Question,    // ?
    Ampersand,   // &
    Asterisk,    // *
    Exclamation, // !

    // Brackets
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // Special
    Anchor, // &anchor
    Alias,  // *alias
    Tag,    // !tag

    // Document markers
    DocumentStart, // ---
    DocumentEnd,   // ...

    // Newlines and indentation
    Newline,

    // Error and EOF
    Error,
    Eof,
}

impl SyntaxKind for YamlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
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
