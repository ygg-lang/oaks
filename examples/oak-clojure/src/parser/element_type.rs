use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClojureElementType {
    Token,
    List,
    Vector,
    Map,
    Set,
    AnonFn,
    Root,
    SourceFile,
    Error,
}

impl ElementType for ClojureElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ClojureTokenType> for ClojureElementType {
    fn from(token: crate::lexer::token_type::ClojureTokenType) -> Self {
                match token {
            crate::lexer::token_type::ClojureTokenType::Token => Self::Token,
            crate::lexer::token_type::ClojureTokenType::List => Self::List,
            crate::lexer::token_type::ClojureTokenType::Vector => Self::Vector,
            crate::lexer::token_type::ClojureTokenType::Map => Self::Map,
            crate::lexer::token_type::ClojureTokenType::Set => Self::Set,
            crate::lexer::token_type::ClojureTokenType::AnonFn => Self::AnonFn,
            crate::lexer::token_type::ClojureTokenType::Root => Self::Root,
            crate::lexer::token_type::ClojureTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::ClojureTokenType::Error => Self::Error,
            crate::lexer::token_type::ClojureTokenType::ListStart => Self::Token,
            crate::lexer::token_type::ClojureTokenType::ListEnd => Self::Token,
            crate::lexer::token_type::ClojureTokenType::VectorStart => Self::Token,
            crate::lexer::token_type::ClojureTokenType::VectorEnd => Self::Token,
            crate::lexer::token_type::ClojureTokenType::MapStart => Self::Token,
            crate::lexer::token_type::ClojureTokenType::MapEnd => Self::Token,
            crate::lexer::token_type::ClojureTokenType::SetStart => Self::Token,
            crate::lexer::token_type::ClojureTokenType::AnonFnStart => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Quote => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Unquote => Self::Token,
            crate::lexer::token_type::ClojureTokenType::UnquoteSplice => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Meta => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Whitespace => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Comment => Self::Token,
            crate::lexer::token_type::ClojureTokenType::StringLiteral => Self::Token,
            crate::lexer::token_type::ClojureTokenType::CharacterLiteral => Self::Token,
            crate::lexer::token_type::ClojureTokenType::NumberLiteral => Self::Token,
            crate::lexer::token_type::ClojureTokenType::KeywordLiteral => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Dispatch => Self::Token,
            crate::lexer::token_type::ClojureTokenType::RegexLiteral => Self::Token,
            crate::lexer::token_type::ClojureTokenType::Symbol => Self::Token,
            _ => Self::Error,
        }
    }
}
