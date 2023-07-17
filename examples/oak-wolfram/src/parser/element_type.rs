use oak_core::{ElementType, UniversalElementRole};
use std::fmt;

/// Element types for the Wolfram language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WolframElementType {
    /// Root node of the tree.
    Root,
    // Expressions
    /// A general expression.
    Expression,
    // Function call f[x]
    /// A function call, e.g., `f[x]`.
    Call,
    // Argument list [x, y]
    /// A list of arguments, e.g., `[x, y]`.
    Arguments,
    // List {a, b}
    /// A list, e.g., `{a, b}`.
    List,
    // Symbol/Identifier
    /// A symbol or identifier.
    Symbol,
    // Literals
    /// A literal value.
    Literal,
    // Binary expression x + y
    /// A binary expression, e.g., `x + y`.
    BinaryExpr,
    // Prefix expression !x
    /// A prefix expression, e.g., `!x`.
    PrefixExpr,
    // Postfix expression x!
    /// A postfix expression, e.g., `x!`.
    PostfixExpr,
    /// Part access, e.g., `list[[1]]`.
    Part,
    // Errors
    /// An error element.
    Error,
}

impl fmt::Display for WolframElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementType for WolframElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::WolframTokenType> for WolframElementType {
    fn from(token: crate::lexer::token_type::WolframTokenType) -> Self {
        match token {
            crate::lexer::token_type::WolframTokenType::Root => Self::Root,
            crate::lexer::token_type::WolframTokenType::Identifier => Self::Symbol,
            crate::lexer::token_type::WolframTokenType::Integer | crate::lexer::token_type::WolframTokenType::Real | crate::lexer::token_type::WolframTokenType::String => Self::Literal,
            _ => Self::Error,
        }
    }
}
