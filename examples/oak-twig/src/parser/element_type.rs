use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the Twig parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TwigElementType {
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

    /// An if statement node (`{% if %}`).
    IfStatement,
    /// A for statement node (`{% for %}`).
    ForStatement,
    /// A macro definition node (`{% macro %}`).
    MacroDefinition,

    /// A generic literal.
    Literal,

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
    /// An error element.
    Error,
}

impl oak_core::TokenType for TwigElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TwigElementType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl core::fmt::Display for TwigElementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwigElementType::Root => f.write_str("Root"),
            TwigElementType::Document => f.write_str("Document"),
            TwigElementType::Template => f.write_str("Template"),
            TwigElementType::Block => f.write_str("Block"),
            TwigElementType::Variable => f.write_str("Variable"),
            TwigElementType::Filter => f.write_str("Filter"),
            TwigElementType::Function => f.write_str("Function"),
            TwigElementType::Tag => f.write_str("Tag"),
            TwigElementType::Comment => f.write_str("Comment"),
            TwigElementType::Text => f.write_str("Text"),
            TwigElementType::Expression => f.write_str("Expression"),
            TwigElementType::IfStatement => f.write_str("IfStatement"),
            TwigElementType::ForStatement => f.write_str("ForStatement"),
            TwigElementType::MacroDefinition => f.write_str("MacroDefinition"),
            TwigElementType::Literal => f.write_str("Literal"),
            TwigElementType::String => f.write_str("String"),
            TwigElementType::Number => f.write_str("Number"),
            TwigElementType::Boolean => f.write_str("Boolean"),
            TwigElementType::Null => f.write_str("Null"),
            TwigElementType::Array => f.write_str("Array"),
            TwigElementType::Object => f.write_str("Object"),
            TwigElementType::Identifier => f.write_str("Identifier"),
            TwigElementType::Operator => f.write_str("Operator"),
            TwigElementType::ErrorNode => f.write_str("ErrorNode"),
            TwigElementType::LeftBrace => f.write_str("{"),
            TwigElementType::RightBrace => f.write_str("}"),
            TwigElementType::LeftBracket => f.write_str("["),
            TwigElementType::RightBracket => f.write_str("]"),
            TwigElementType::DoubleLeftBrace => f.write_str("{{"),
            TwigElementType::DoubleRightBrace => f.write_str("}}"),
            TwigElementType::LeftBracePercent => f.write_str("{%"),
            TwigElementType::PercentRightBrace => f.write_str("%}"),
            TwigElementType::LeftParen => f.write_str("("),
            TwigElementType::RightParen => f.write_str(")"),
            TwigElementType::Pipe => f.write_str("|"),
            TwigElementType::Comma => f.write_str(","),
            TwigElementType::Dot => f.write_str("."),
            TwigElementType::Colon => f.write_str(":"),
            TwigElementType::Semicolon => f.write_str(";"),
            TwigElementType::Eq => f.write_str("="),
            TwigElementType::Plus => f.write_str("+"),
            TwigElementType::Minus => f.write_str("-"),
            TwigElementType::Star => f.write_str("*"),
            TwigElementType::Slash => f.write_str("/"),
            TwigElementType::Percent => f.write_str("%"),
            TwigElementType::Bang => f.write_str("!"),
            TwigElementType::Question => f.write_str("?"),
            TwigElementType::Lt => f.write_str("<"),
            TwigElementType::Gt => f.write_str(">"),
            TwigElementType::Amp => f.write_str("&"),
            TwigElementType::Caret => f.write_str("^"),
            TwigElementType::Tilde => f.write_str("~"),
            TwigElementType::Whitespace => f.write_str("Whitespace"),
            TwigElementType::Eof => f.write_str("EOF"),
            TwigElementType::Error => f.write_str("Error"),
        }
    }
}

impl ElementType for TwigElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::TwigTokenType> for TwigElementType {
    fn from(token: crate::lexer::token_type::TwigTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
