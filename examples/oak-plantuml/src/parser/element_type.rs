use oak_core::{ElementType, UniversalElementRole};

/// Element types for the PlantUML language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PlantUmlElementType {
    /// The root of a PlantUML document.
    Root,
    /// A class definition.
    Class,
    /// An interface definition.
    Interface,
    /// A relationship between elements.
    Relation,
    /// @startuml directive.
    StartUml,
    /// @enduml directive.
    EndUml,
    /// A comment.
    Comment,
    /// An identifier.
    Id,
    /// A label or description.
    Label,
    /// Error or unknown element.
    Error,
}

impl ElementType for PlantUmlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::PlantUmlTokenType> for PlantUmlElementType {
    fn from(token: crate::lexer::token_type::PlantUmlTokenType) -> Self {
        match token {
            crate::lexer::token_type::PlantUmlTokenType::Whitespace => Self::Error,
            crate::lexer::token_type::PlantUmlTokenType::Newline => Self::Error,
            crate::lexer::token_type::PlantUmlTokenType::Comment => Self::Comment,
            crate::lexer::token_type::PlantUmlTokenType::StartUml => Self::StartUml,
            crate::lexer::token_type::PlantUmlTokenType::EndUml => Self::EndUml,
            crate::lexer::token_type::PlantUmlTokenType::Class => Self::Class,
            crate::lexer::token_type::PlantUmlTokenType::Interface => Self::Interface,
            crate::lexer::token_type::PlantUmlTokenType::Id => Self::Id,
            crate::lexer::token_type::PlantUmlTokenType::Label => Self::Label,
            crate::lexer::token_type::PlantUmlTokenType::Error => Self::Error,
        }
    }
}
