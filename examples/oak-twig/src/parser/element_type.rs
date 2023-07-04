use oak_core::{ElementType, Parser, UniversalElementRole};

/// Element types for the Twig parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Element types for Twig syntax nodes.
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
                match token {
            crate::lexer::token_type::TwigTokenType::Root => Self::Root,
            crate::lexer::token_type::TwigTokenType::Document => Self::Document,
            crate::lexer::token_type::TwigTokenType::Template => Self::Template,
            crate::lexer::token_type::TwigTokenType::Block => Self::Block,
            crate::lexer::token_type::TwigTokenType::Variable => Self::Variable,
            crate::lexer::token_type::TwigTokenType::Filter => Self::Filter,
            crate::lexer::token_type::TwigTokenType::Function => Self::Function,
            crate::lexer::token_type::TwigTokenType::Tag => Self::Tag,
            crate::lexer::token_type::TwigTokenType::Comment => Self::Comment,
            crate::lexer::token_type::TwigTokenType::Text => Self::Text,
            crate::lexer::token_type::TwigTokenType::Expression => Self::Expression,
            crate::lexer::token_type::TwigTokenType::String => Self::String,
            crate::lexer::token_type::TwigTokenType::Number => Self::Number,
            crate::lexer::token_type::TwigTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::TwigTokenType::Null => Self::Null,
            crate::lexer::token_type::TwigTokenType::Array => Self::Array,
            crate::lexer::token_type::TwigTokenType::Object => Self::Object,
            crate::lexer::token_type::TwigTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::TwigTokenType::Operator => Self::Operator,
            crate::lexer::token_type::TwigTokenType::ErrorNode => Self::ErrorNode,
            crate::lexer::token_type::TwigTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::TwigTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::TwigTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::TwigTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::TwigTokenType::DoubleLeftBrace => Self::DoubleLeftBrace,
            crate::lexer::token_type::TwigTokenType::DoubleRightBrace => Self::DoubleRightBrace,
            crate::lexer::token_type::TwigTokenType::LeftBracePercent => Self::LeftBracePercent,
            crate::lexer::token_type::TwigTokenType::PercentRightBrace => Self::PercentRightBrace,
            crate::lexer::token_type::TwigTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::TwigTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::TwigTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::TwigTokenType::Comma => Self::Comma,
            crate::lexer::token_type::TwigTokenType::Dot => Self::Dot,
            crate::lexer::token_type::TwigTokenType::Colon => Self::Colon,
            crate::lexer::token_type::TwigTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::TwigTokenType::Eq => Self::Eq,
            crate::lexer::token_type::TwigTokenType::Plus => Self::Plus,
            crate::lexer::token_type::TwigTokenType::Minus => Self::Minus,
            crate::lexer::token_type::TwigTokenType::Star => Self::Star,
            crate::lexer::token_type::TwigTokenType::Slash => Self::Slash,
            crate::lexer::token_type::TwigTokenType::Percent => Self::Percent,
            crate::lexer::token_type::TwigTokenType::Bang => Self::Bang,
            crate::lexer::token_type::TwigTokenType::Question => Self::Question,
            crate::lexer::token_type::TwigTokenType::Lt => Self::Lt,
            crate::lexer::token_type::TwigTokenType::Gt => Self::Gt,
            crate::lexer::token_type::TwigTokenType::Amp => Self::Amp,
            crate::lexer::token_type::TwigTokenType::Caret => Self::Caret,
            crate::lexer::token_type::TwigTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::TwigTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::TwigTokenType::Eof => Self::Eof,
            crate::lexer::token_type::TwigTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
