/// Unified Sass syntax kinds (includes nodes and tokens)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SassTokenType {
    // Node kinds
    Root,
    Value,
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
    ObjectEntry,
    ArrayElement,
    ErrorNode,

    // Token kinds
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    Whitespace,
    Comment,
    Eof,
    Error,
}
