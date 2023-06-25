use crate::lexer::token_type::VerilogKind;
use oak_core::{ElementType, UniversalElementRole};

/// Verilog element type.
pub type VerilogElementType = VerilogKind;

impl ElementType for VerilogKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module => UniversalElementRole::Value,
            Self::PortList => UniversalElementRole::Value,
            Self::Port => UniversalElementRole::Value,
            Self::ModuleItem => UniversalElementRole::Value,
            Self::Assign => UniversalElementRole::Value,
            Self::Declaration => UniversalElementRole::Value,
            Self::Always => UniversalElementRole::Value,
            Self::Initial => UniversalElementRole::Value,
            Self::Block => UniversalElementRole::Value,
            Self::Expression => UniversalElementRole::Value,
            Self::Statement => UniversalElementRole::Value,
            _ => UniversalElementRole::Value,
        }
    }
}
