use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Type alias for a Twig token.
pub type TwigToken = Token<TwigTokenType>;

/// Token types for the Twig lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TwigTokenType {
    /// The root node of the parse tree.
    Root,
    /// A document node.
    Document,
    /// A template node.
    Template,
    /// A block node (`{% block %}`).
    Block,
    /// A variable node (`{{ var }}`).
    Variable,
    /// A filter node (`| filter`).
    Filter,
    /// A function node (`func()`).
    Function,
    /// A tag node (`{% tag %}`).
    Tag,
    /// A comment node (`{# comment #}`).
    Comment,
    /// Plain text content.
    Text,
    /// An expression node.
    Expression,

    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A boolean literal.
    Boolean,
    /// A null literal.
    Null,
    /// An array literal.
    Array,
    /// An object literal.
    Object,
    /// An identifier.
    Identifier,
    /// An operator.
    Operator,
    /// An error node in the parse tree.
    ErrorNode,

    /// An opening brace (`{`).
    LeftBrace,
    /// A closing brace (`}`).
    RightBrace,
    /// An opening bracket (`[`).
    LeftBracket,
    /// A closing bracket (`]`).
    RightBracket,
    /// Double opening braces (`{{`).
    DoubleLeftBrace,
    /// Double closing braces (`}}`).
    DoubleRightBrace,
    /// Opening brace and percent (`{%`).
    LeftBracePercent,
    /// Percent and closing brace (`%}`).
    PercentRightBrace,
    /// An opening parenthesis (`(`).
    LeftParen,
    /// A closing parenthesis (`)`).
    RightParen,
    /// A pipe character (`|`).
    Pipe,
    /// A comma (`,`).
    Comma,
    /// A dot character (`.`).
    Dot,
    /// A colon character (`:`).
    Colon,
    /// A semicolon character (`;`).
    Semicolon,
    /// An equal sign (`=`).
    Eq,
    /// A plus sign (`+`).
    Plus,
    /// A minus sign (`-`).
    Minus,
    /// A star sign (`*`).
    Star,
    /// A slash sign (`/`).
    Slash,
    /// A percent sign (`%`).
    Percent,
    /// An exclamation mark (`!`).
    Bang,
    /// A question mark (`?`).
    Question,
    /// A less-than sign (`<`).
    Lt,
    /// A greater-than sign (`>`).
    Gt,
    /// An ampersand character (`&`).
    Amp,
    /// A caret character (`^`).
    Caret,
    /// A tilde character (`~`).
    Tilde,
    /// Whitespace characters.
    Whitespace,
    /// End of stream.
    Eof,
    /// An error token.
    Error,
}

impl oak_core::TokenType for TwigTokenType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TwigTokenType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl core::fmt::Display for TwigTokenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwigTokenType::Root => f.write_str("Root"),
            TwigTokenType::Document => f.write_str("Document"),
            TwigTokenType::Template => f.write_str("Template"),
            TwigTokenType::Block => f.write_str("Block"),
            TwigTokenType::Variable => f.write_str("Variable"),
            TwigTokenType::Filter => f.write_str("Filter"),
            TwigTokenType::Function => f.write_str("Function"),
            TwigTokenType::Tag => f.write_str("Tag"),
            TwigTokenType::Comment => f.write_str("Comment"),
            TwigTokenType::Text => f.write_str("Text"),
            TwigTokenType::Expression => f.write_str("Expression"),
            TwigTokenType::String => f.write_str("String"),
            TwigTokenType::Number => f.write_str("Number"),
            TwigTokenType::Boolean => f.write_str("Boolean"),
            TwigTokenType::Null => f.write_str("Null"),
            TwigTokenType::Array => f.write_str("Array"),
            TwigTokenType::Object => f.write_str("Object"),
            TwigTokenType::Identifier => f.write_str("Identifier"),
            TwigTokenType::Operator => f.write_str("Operator"),
            TwigTokenType::ErrorNode => f.write_str("ErrorNode"),
            TwigTokenType::LeftBrace => f.write_str("{"),
            TwigTokenType::RightBrace => f.write_str("}"),
            TwigTokenType::LeftBracket => f.write_str("["),
            TwigTokenType::RightBracket => f.write_str("]"),
            TwigTokenType::DoubleLeftBrace => f.write_str("{{"),
            TwigTokenType::DoubleRightBrace => f.write_str("}}"),
            TwigTokenType::LeftBracePercent => f.write_str("{%"),
            TwigTokenType::PercentRightBrace => f.write_str("%}"),
            TwigTokenType::LeftParen => f.write_str("("),
            TwigTokenType::RightParen => f.write_str(")"),
            TwigTokenType::Pipe => f.write_str("|"),
            TwigTokenType::Comma => f.write_str(","),
            TwigTokenType::Dot => f.write_str("."),
            TwigTokenType::Colon => f.write_str(":"),
            TwigTokenType::Semicolon => f.write_str(";"),
            TwigTokenType::Eq => f.write_str("="),
            TwigTokenType::Plus => f.write_str("+"),
            TwigTokenType::Minus => f.write_str("-"),
            TwigTokenType::Star => f.write_str("*"),
            TwigTokenType::Slash => f.write_str("/"),
            TwigTokenType::Percent => f.write_str("%"),
            TwigTokenType::Bang => f.write_str("!"),
            TwigTokenType::Question => f.write_str("?"),
            TwigTokenType::Lt => f.write_str("<"),
            TwigTokenType::Gt => f.write_str(">"),
            TwigTokenType::Amp => f.write_str("&"),
            TwigTokenType::Caret => f.write_str("^"),
            TwigTokenType::Tilde => f.write_str("~"),
            TwigTokenType::Whitespace => f.write_str("Whitespace"),
            TwigTokenType::Eof => f.write_str("EOF"),
            TwigTokenType::Error => f.write_str("Error"),
        }
    }
}
