use oak_core::{ElementType, UniversalElementRole};

/// Element types for the IDL parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum IdlElementType {
    /// The `void` type.
    Void,
    /// The `boolean` type.
    Boolean,
    /// The `byte` type.
    Byte,
    /// The `octet` type.
    Octet,
    /// The `short` type.
    Short,
    /// The `unsigned short` type.
    UnsignedShort,
    /// The `long` type.
    Long,
    /// The `unsigned long` type.
    UnsignedLong,
    /// The `long long` type.
    LongLong,
    /// The `unsigned long long` type.
    UnsignedLongLong,
    /// The `float` type.
    Float,
    /// The `double` type.
    Double,
    /// The `long double` type.
    LongDouble,
    /// The `char` type.
    Char,
    /// The `wchar` type.
    WChar,
    /// The `string` type.
    String,
    /// The `wstring` type.
    WString,
    /// The `any` type.
    Any,
    /// The `Object` type.
    Object,
    /// The `ValueBase` type.
    ValueBase,

    /// A struct type.
    Struct,
    /// A union type.
    Union,
    /// An enum type.
    Enum,
    /// An interface type.
    Interface,
    /// A module type.
    Module,
    /// An exception type.
    Exception,
    /// A typedef.
    Typedef,
    /// A sequence type.
    Sequence,
    /// An array type.
    Array,
    /// A fixed-point type.
    Fixed,

    /// An attribute member.
    Attribute,
    /// An operation member.
    Operation,
    /// A constant member.
    Const,
    /// An exception member.
    ExceptionMember,

    /// A module declaration.
    ModuleDeclaration,
    /// An interface declaration.
    InterfaceDeclaration,
    /// A struct declaration.
    StructDeclaration,
    /// A union declaration.
    UnionDeclaration,
    /// An enum declaration.
    EnumDeclaration,
    /// A typedef declaration.
    TypedefDeclaration,
    /// A constant declaration.
    ConstDeclaration,
    /// An exception declaration.
    ExceptionDeclaration,

    /// The entire source file.
    SourceFile,
    /// An include directive.
    Include,
    /// A pragma directive.
    Pragma,
    /// An error element.
    Error,
    /// End of file.
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
