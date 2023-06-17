use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ClojureSyntaxKind {
    // Trivia
    Whitespace,
    Newline,
    Comment,

    // Literals
    StringLiteral,
    CharacterLiteral,
    NumberLiteral,
    BooleanLiteral,
    NilLiteral,
    KeywordLiteral,

    // Identifiers and symbols
    Symbol,
    Keyword,

    // Collections
    ListStart,   // (
    ListEnd,     // )
    VectorStart, // [
    VectorEnd,   // ]
    MapStart,    // {
    MapEnd,      // }
    SetStart,    // #{

    // Special forms
    Quote,         // '
    Unquote,       // ~
    UnquoteSplice, // ~@
    Deref,         // @
    Meta,          // ^
    Dispatch,      // #

    // Reader macros
    ReaderMacro,

    // Regex
    RegexLiteral,

    // Anonymous function
    AnonFnStart, // #(
    AnonFnArg,   // %1, %2, etc.

    // Composite nodes
    SourceFile,

    // Error handling
    Error,
    Eof,
}

impl SyntaxKind for ClojureSyntaxKind {
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
        matches!(
            self,
            Self::Whitespace
                | Self::Newline
                | Self::Comment
                | Self::StringLiteral
                | Self::CharacterLiteral
                | Self::NumberLiteral
                | Self::BooleanLiteral
                | Self::NilLiteral
                | Self::KeywordLiteral
                | Self::Symbol
                | Self::Keyword
                | Self::ListStart
                | Self::ListEnd
                | Self::VectorStart
                | Self::VectorEnd
                | Self::MapStart
                | Self::MapEnd
                | Self::SetStart
                | Self::Quote
                | Self::Unquote
                | Self::UnquoteSplice
                | Self::Deref
                | Self::Meta
                | Self::Dispatch
                | Self::ReaderMacro
                | Self::RegexLiteral
                | Self::AnonFnStart
                | Self::AnonFnArg
                | Self::Error
                | Self::Eof
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}
