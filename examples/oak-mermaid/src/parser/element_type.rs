use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum MermaidElementType {
    // 基础标记 (与 MermaidTokenType 保持一致)
    Whitespace,
    Newline,
    Comment,

    Graph,
    Direction,
    Id,
    Label,
    Arrow,

    Error,

    // 文档结构 (Element)
    Root,
    Node,
    Edge,

    // EOF
    Eof,
}

impl MermaidElementType {
    pub fn is_token(&self) -> bool {
        (*self as u8) <= (Self::Eof as u8) && !self.is_element()
    }

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
