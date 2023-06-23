use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum IdlElementType {
    // Basic types
    Void,
    Boolean,
    Byte,
    Octet,
    Short,
    UnsignedShort,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
    Char,
    WChar,
    String,
    WString,
    Any,
    Object,
    ValueBase,

    // Composite types
    Struct,
    Union,
    Enum,
    Interface,
    Module,
    Exception,
    Typedef,
    Sequence,
    Array,
    Fixed,

    // Members
    Attribute,
    Operation,
    Const,
    ExceptionMember,

    // Declarations
    ModuleDeclaration,
    InterfaceDeclaration,
    StructDeclaration,
    UnionDeclaration,
    EnumDeclaration,
    TypedefDeclaration,
    ConstDeclaration,
    ExceptionDeclaration,

    // Misc
    SourceFile,
    Include,
    Pragma,
    Error,
    Eof,
}

impl ElementType for IdlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::ModuleDeclaration | Self::InterfaceDeclaration | Self::StructDeclaration | Self::UnionDeclaration | Self::EnumDeclaration | Self::TypedefDeclaration | Self::ConstDeclaration | Self::ExceptionDeclaration => {
                UniversalElementRole::Definition
            }
            Self::Module | Self::Interface | Self::Struct | Self::Union | Self::Enum => UniversalElementRole::Container,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::IdlTokenType> for IdlElementType {
    fn from(token: crate::lexer::token_type::IdlTokenType) -> Self {
        match token {
            crate::lexer::token_type::IdlTokenType::Void => Self::Void,
            crate::lexer::token_type::IdlTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::IdlTokenType::Byte => Self::Byte,
            crate::lexer::token_type::IdlTokenType::Octet => Self::Octet,
            crate::lexer::token_type::IdlTokenType::Short => Self::Short,
            crate::lexer::token_type::IdlTokenType::UnsignedShort => Self::UnsignedShort,
            crate::lexer::token_type::IdlTokenType::Long => Self::Long,
            crate::lexer::token_type::IdlTokenType::UnsignedLong => Self::UnsignedLong,
            crate::lexer::token_type::IdlTokenType::LongLong => Self::LongLong,
            crate::lexer::token_type::IdlTokenType::UnsignedLongLong => Self::UnsignedLongLong,
            crate::lexer::token_type::IdlTokenType::Float => Self::Float,
            crate::lexer::token_type::IdlTokenType::Double => Self::Double,
            crate::lexer::token_type::IdlTokenType::LongDouble => Self::LongDouble,
            crate::lexer::token_type::IdlTokenType::Char => Self::Char,
            crate::lexer::token_type::IdlTokenType::WChar => Self::WChar,
            crate::lexer::token_type::IdlTokenType::String => Self::String,
            crate::lexer::token_type::IdlTokenType::WString => Self::WString,
            crate::lexer::token_type::IdlTokenType::Any => Self::Any,
            crate::lexer::token_type::IdlTokenType::Object => Self::Object,
            crate::lexer::token_type::IdlTokenType::ValueBase => Self::ValueBase,
            crate::lexer::token_type::IdlTokenType::Struct => Self::Struct,
            crate::lexer::token_type::IdlTokenType::Union => Self::Union,
            crate::lexer::token_type::IdlTokenType::Enum => Self::Enum,
            crate::lexer::token_type::IdlTokenType::Interface => Self::Interface,
            crate::lexer::token_type::IdlTokenType::Module => Self::Module,
            crate::lexer::token_type::IdlTokenType::Exception => Self::Exception,
            crate::lexer::token_type::IdlTokenType::Typedef => Self::Typedef,
            crate::lexer::token_type::IdlTokenType::Sequence => Self::Sequence,
            crate::lexer::token_type::IdlTokenType::Array => Self::Array,
            crate::lexer::token_type::IdlTokenType::Fixed => Self::Fixed,
            crate::lexer::token_type::IdlTokenType::Const => Self::Const,
            crate::lexer::token_type::IdlTokenType::Attribute => Self::Attribute,
            crate::lexer::token_type::IdlTokenType::Error => Self::Error,
            crate::lexer::token_type::IdlTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
