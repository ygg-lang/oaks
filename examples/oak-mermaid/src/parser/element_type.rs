use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Mermaid AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum MermaidElementType {
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Comment.
    Comment,

    /// `graph` keyword.
    Graph,
    /// Direction (e.g., LR, TD).
    Direction,
    /// Identifier.
    Id,
    /// Node label.
    Label,
    /// Connection arrow (e.g., `-->`).
    Arrow,

    /// Lexing or parsing error.
    Error,

    /// Root node of the diagram.
    Root,
    /// Node in the diagram.
    Node,
    /// Edge between nodes.
    Edge,

    /// End of stream.
    Eof,
}

impl MermaidElementType {
    /// Returns `true` if this element type represents a token.
    pub fn is_token(&self) -> bool {
        (*self as u8) <= (Self::Eof as u8) && !self.is_element()
    }

    /// Returns `true` if this element type represents a composite element.
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Root | Self::Node | Self::Edge)
    }
}

impl ElementType for MermaidElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::MermaidTokenType> for MermaidElementType {
    fn from(token: crate::lexer::token_type::MermaidTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
