use crate::lexer::token_type::VerilogKind;
use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VerilogElementType = VerilogKind;

impl ElementType for VerilogKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Module => UniversalElementRole::Root,
            _ => UniversalElementRole::Value,
        }
    }
}
