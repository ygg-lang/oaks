use oak_core::{ElementType, UniversalElementRole};

/// Element types for Clojure AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClojureElementType {
    /// Generic token element.
    Token,
    /// List element.
    List,
    /// Vector element.
    Vector,
    /// Map element.
    Map,
    /// Set element.
    Set,
    /// Anonymous function element.
    AnonFn,
    /// Quotation element.
    Quotation,
    /// Metadata element.
    Meta,
    /// Root element.
    Root,
    /// Source file element.
    SourceFile,
    /// Error element.
    Error,
}

impl ElementType for ClojureElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            Self::List | Self::Vector | Self::Map | Self::Set | Self::AnonFn | Self::Quotation | Self::Meta => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ClojureTokenType> for ClojureElementType {
    fn from(token: crate::lexer::token_type::ClojureTokenType) -> Self {
        use crate::lexer::token_type::ClojureTokenType as T;
        match token {
            T::Token => Self::Token,
            T::List => Self::List,
            T::Vector => Self::Vector,
            T::Map => Self::Map,
            T::Set => Self::Set,
            T::AnonFn => Self::AnonFn,
            T::Root => Self::Root,
            T::SourceFile => Self::SourceFile,
            T::Error => Self::Error,
            T::ListStart => Self::Token,
            T::ListEnd => Self::Token,
            T::VectorStart => Self::Token,
            T::VectorEnd => Self::Token,
            T::MapStart => Self::Token,
            T::MapEnd => Self::Token,
            T::SetStart => Self::Token,
            T::AnonFnStart => Self::Token,
            T::Quote => Self::Token,
            T::Unquote => Self::Token,
            T::UnquoteSplice => Self::Token,
            T::Meta => Self::Token,
            T::Whitespace => Self::Token,
            T::Comment => Self::Token,
            T::StringLiteral => Self::Token,
            T::CharacterLiteral => Self::Token,
            T::NumberLiteral => Self::Token,
            T::KeywordLiteral => Self::Token,
            T::Dispatch => Self::Token,
            T::RegexLiteral => Self::Token,
            T::Symbol => Self::Token,
        }
    }
}
