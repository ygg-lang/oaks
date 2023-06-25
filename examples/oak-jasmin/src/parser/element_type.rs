use oak_core::{ElementType, UniversalElementRole};

/// Jasmin element types for the green tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JasminElementType {
    /// Root node.
    Root,
    /// Class definition.
    Class,
    /// Method definition.
    Method,
    /// Field definition.
    Field,
    /// Instruction.
    Instruction,
    /// Identifier node.
    IdentifierNode,
    /// String node.
    StringNode,
    /// Number node.
    NumberNode,
    /// Error node.
    ErrorNode,

    /// `.class` keyword.
    ClassKw,
    /// `.version` keyword.
    VersionKw,
    /// `.method` keyword.
    MethodKw,
    /// `.field` keyword.
    FieldKw,
    /// `.string` keyword.
    StringKw,
    /// `.source` keyword.
    SourceFileKw,
    /// `.stack` keyword.
    StackKw,
    /// `.limit locals` keyword.
    LocalsKw,
    /// `.end` keyword.
    EndKw,
    /// `compiled` keyword.
    CompiledKw,
    /// `from` keyword.
    FromKw,
    /// `.inner` keyword.
    InnerClassKw,
    /// `.nest` keyword.
    NestMembersKw,
    /// `.bootstrap` keyword.
    BootstrapMethodKw,

    /// `public` access flag.
    Public,
    /// `private` access flag.
    Private,
    /// `protected` access flag.
    Protected,
    /// `static` access flag.
    Static,
    /// `super` access flag.
    Super,
    /// `final` access flag.
    Final,
    /// `abstract` access flag.
    Abstract,
    /// `synchronized` access flag.
    Synchronized,
    /// `native` access flag.
    Native,
    /// `synthetic` access flag.
    Synthetic,
    /// `deprecated` access flag.
    Deprecated,
    /// `varargs` access flag.
    Varargs,
    /// `bridge` access flag.
    Bridge,
    /// `enum` keyword.
    Enum,
    /// `annotation` keyword.
    Annotation,
    /// `strictfp` access flag.
    Strictfp,
    /// `interface` keyword.
    Interface,
    /// `volatile` access flag.
    Volatile,
    /// `transient` access flag.
    Transient,

    /// `aload_0` instruction.
    ALoad0,
    /// `aload_1` instruction.
    ALoad1,
    /// `aload_2` instruction.
    ALoad2,
    /// `aload_3` instruction.
    ALoad3,
    /// `iload_0` instruction.
    ILoad0,
    /// `iload_1` instruction.
    ILoad1,
    /// `iload_2` instruction.
    ILoad2,
    /// `iload_3` instruction.
    ILoad3,
    /// `ldc` instruction.
    Ldc,
    /// `ldc_w` instruction.
    LdcW,
    /// `ldc2_w` instruction.
    Ldc2W,
    /// `invokespecial` instruction.
    InvokeSpecial,
    /// `invokevirtual` instruction.
    InvokeVirtual,
    /// `invokestatic` instruction.
    InvokeStatic,
    /// `invokeinterface` instruction.
    InvokeInterface,
    /// `invokedynamic` instruction.
    InvokeDynamic,
    /// `getstatic` instruction.
    GetStatic,
    /// `putstatic` instruction.
    PutStatic,
    /// `getfield` instruction.
    GetField,
    /// `putfield` instruction.
    PutField,
    /// `return` instruction.
    Return,
    /// `ireturn` instruction.
    IReturn,
    /// `areturn` instruction.
    AReturn,
    /// `lreturn` instruction.
    LReturn,
    /// `freturn` instruction.
    FReturn,
    /// `dreturn` instruction.
    DReturn,
    /// `nop` instruction.
    Nop,
    /// `dup` instruction.
    Dup,
    /// `pop` instruction.
    Pop,
    /// `new` instruction.
    New,

    /// `{` token.
    LeftBrace,
    /// `}` token.
    RightBrace,
    /// `(` token.
    LeftParen,
    /// `)` token.
    RightParen,
    /// `[` token.
    LeftBracket,
    /// `]` token.
    RightBracket,
    /// `:` token.
    Colon,
    /// `;` token.
    Semicolon,
    /// `.` token.
    Dot,
    /// `,` token.
    Comma,
    /// `/` token.
    Slash,

    /// String literal token.
    StringLiteral,
    /// Number token.
    Number,
    /// Type descriptor token.
    TypeDescriptor,
    /// Identifier token.
    IdentifierToken,
    /// Whitespace token.
    Whitespace,
    /// Newline token.
    Newline,
    /// Comment token.
    Comment,
    /// EOF token.
    Eof,
    /// Error token.
    Error,
}

impl ElementType for JasminElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::JasminTokenType> for JasminElementType {
    fn from(token: crate::lexer::token_type::JasminTokenType) -> Self {
        match token {
            crate::lexer::token_type::JasminTokenType::Root => JasminElementType::Root,
            crate::lexer::token_type::JasminTokenType::Class => JasminElementType::Class,
            crate::lexer::token_type::JasminTokenType::Method => JasminElementType::Method,
            crate::lexer::token_type::JasminTokenType::Field => JasminElementType::Field,
            crate::lexer::token_type::JasminTokenType::Instruction => JasminElementType::Instruction,
            crate::lexer::token_type::JasminTokenType::IdentifierNode => JasminElementType::IdentifierNode,
            crate::lexer::token_type::JasminTokenType::StringNode => JasminElementType::StringNode,
            crate::lexer::token_type::JasminTokenType::NumberNode => JasminElementType::NumberNode,
            crate::lexer::token_type::JasminTokenType::ErrorNode => JasminElementType::ErrorNode,

            crate::lexer::token_type::JasminTokenType::ClassKw => JasminElementType::ClassKw,
            crate::lexer::token_type::JasminTokenType::VersionKw => JasminElementType::VersionKw,
            crate::lexer::token_type::JasminTokenType::MethodKw => JasminElementType::MethodKw,
            crate::lexer::token_type::JasminTokenType::FieldKw => JasminElementType::FieldKw,
            crate::lexer::token_type::JasminTokenType::StringKw => JasminElementType::StringKw,
            crate::lexer::token_type::JasminTokenType::SourceFileKw => JasminElementType::SourceFileKw,
            crate::lexer::token_type::JasminTokenType::StackKw => JasminElementType::StackKw,
            crate::lexer::token_type::JasminTokenType::LocalsKw => JasminElementType::LocalsKw,
            crate::lexer::token_type::JasminTokenType::EndKw => JasminElementType::EndKw,
            crate::lexer::token_type::JasminTokenType::CompiledKw => JasminElementType::CompiledKw,
            crate::lexer::token_type::JasminTokenType::FromKw => JasminElementType::FromKw,
            crate::lexer::token_type::JasminTokenType::InnerClassKw => JasminElementType::InnerClassKw,
            crate::lexer::token_type::JasminTokenType::NestMembersKw => JasminElementType::NestMembersKw,
            crate::lexer::token_type::JasminTokenType::BootstrapMethodKw => JasminElementType::BootstrapMethodKw,

            crate::lexer::token_type::JasminTokenType::Public => JasminElementType::Public,
            crate::lexer::token_type::JasminTokenType::Private => JasminElementType::Private,
            crate::lexer::token_type::JasminTokenType::Protected => JasminElementType::Protected,
            crate::lexer::token_type::JasminTokenType::Static => JasminElementType::Static,
            crate::lexer::token_type::JasminTokenType::Super => JasminElementType::Super,
            crate::lexer::token_type::JasminTokenType::Final => JasminElementType::Final,
            crate::lexer::token_type::JasminTokenType::Abstract => JasminElementType::Abstract,
            crate::lexer::token_type::JasminTokenType::Synchronized => JasminElementType::Synchronized,
            crate::lexer::token_type::JasminTokenType::Native => JasminElementType::Native,
            crate::lexer::token_type::JasminTokenType::Synthetic => JasminElementType::Synthetic,
            crate::lexer::token_type::JasminTokenType::Deprecated => JasminElementType::Deprecated,
            crate::lexer::token_type::JasminTokenType::Varargs => JasminElementType::Varargs,
            crate::lexer::token_type::JasminTokenType::Bridge => JasminElementType::Bridge,
            crate::lexer::token_type::JasminTokenType::Enum => JasminElementType::Enum,
            crate::lexer::token_type::JasminTokenType::Annotation => JasminElementType::Annotation,
            crate::lexer::token_type::JasminTokenType::Strictfp => JasminElementType::Strictfp,
            crate::lexer::token_type::JasminTokenType::Interface => JasminElementType::Interface,
            crate::lexer::token_type::JasminTokenType::Volatile => JasminElementType::Volatile,
            crate::lexer::token_type::JasminTokenType::Transient => JasminElementType::Transient,

            crate::lexer::token_type::JasminTokenType::ALoad0 => JasminElementType::ALoad0,
            crate::lexer::token_type::JasminTokenType::ALoad1 => JasminElementType::ALoad1,
            crate::lexer::token_type::JasminTokenType::ALoad2 => JasminElementType::ALoad2,
            crate::lexer::token_type::JasminTokenType::ALoad3 => JasminElementType::ALoad3,
            crate::lexer::token_type::JasminTokenType::ILoad0 => JasminElementType::ILoad0,
            crate::lexer::token_type::JasminTokenType::ILoad1 => JasminElementType::ILoad1,
            crate::lexer::token_type::JasminTokenType::ILoad2 => JasminElementType::ILoad2,
            crate::lexer::token_type::JasminTokenType::ILoad3 => JasminElementType::ILoad3,
            crate::lexer::token_type::JasminTokenType::Ldc => JasminElementType::Ldc,
            crate::lexer::token_type::JasminTokenType::LdcW => JasminElementType::LdcW,
            crate::lexer::token_type::JasminTokenType::Ldc2W => JasminElementType::Ldc2W,
            crate::lexer::token_type::JasminTokenType::InvokeSpecial => JasminElementType::InvokeSpecial,
            crate::lexer::token_type::JasminTokenType::InvokeVirtual => JasminElementType::InvokeVirtual,
            crate::lexer::token_type::JasminTokenType::InvokeStatic => JasminElementType::InvokeStatic,
            crate::lexer::token_type::JasminTokenType::InvokeInterface => JasminElementType::InvokeInterface,
            crate::lexer::token_type::JasminTokenType::InvokeDynamic => JasminElementType::InvokeDynamic,
            crate::lexer::token_type::JasminTokenType::GetStatic => JasminElementType::GetStatic,
            crate::lexer::token_type::JasminTokenType::PutStatic => JasminElementType::PutStatic,
            crate::lexer::token_type::JasminTokenType::GetField => JasminElementType::GetField,
            crate::lexer::token_type::JasminTokenType::PutField => JasminElementType::PutField,
            crate::lexer::token_type::JasminTokenType::Return => JasminElementType::Return,
            crate::lexer::token_type::JasminTokenType::IReturn => JasminElementType::IReturn,
            crate::lexer::token_type::JasminTokenType::AReturn => JasminElementType::AReturn,
            crate::lexer::token_type::JasminTokenType::LReturn => JasminElementType::LReturn,
            crate::lexer::token_type::JasminTokenType::FReturn => JasminElementType::FReturn,
            crate::lexer::token_type::JasminTokenType::DReturn => JasminElementType::DReturn,
            crate::lexer::token_type::JasminTokenType::Nop => JasminElementType::Nop,
            crate::lexer::token_type::JasminTokenType::Dup => JasminElementType::Dup,
            crate::lexer::token_type::JasminTokenType::Pop => JasminElementType::Pop,
            crate::lexer::token_type::JasminTokenType::New => JasminElementType::New,

            crate::lexer::token_type::JasminTokenType::LeftBrace => JasminElementType::LeftBrace,
            crate::lexer::token_type::JasminTokenType::RightBrace => JasminElementType::RightBrace,
            crate::lexer::token_type::JasminTokenType::LeftParen => JasminElementType::LeftParen,
            crate::lexer::token_type::JasminTokenType::RightParen => JasminElementType::RightParen,
            crate::lexer::token_type::JasminTokenType::LeftBracket => JasminElementType::LeftBracket,
            crate::lexer::token_type::JasminTokenType::RightBracket => JasminElementType::RightBracket,
            crate::lexer::token_type::JasminTokenType::Colon => JasminElementType::Colon,
            crate::lexer::token_type::JasminTokenType::Semicolon => JasminElementType::Semicolon,
            crate::lexer::token_type::JasminTokenType::Dot => JasminElementType::Dot,
            crate::lexer::token_type::JasminTokenType::Comma => JasminElementType::Comma,
            crate::lexer::token_type::JasminTokenType::Slash => JasminElementType::Slash,

            crate::lexer::token_type::JasminTokenType::StringLiteral => JasminElementType::StringLiteral,
            crate::lexer::token_type::JasminTokenType::Number => JasminElementType::Number,
            crate::lexer::token_type::JasminTokenType::TypeDescriptor => JasminElementType::TypeDescriptor,
            crate::lexer::token_type::JasminTokenType::IdentifierToken => JasminElementType::IdentifierToken,
            crate::lexer::token_type::JasminTokenType::Whitespace => JasminElementType::Whitespace,
            crate::lexer::token_type::JasminTokenType::Newline => JasminElementType::Newline,
            crate::lexer::token_type::JasminTokenType::Comment => JasminElementType::Comment,
            crate::lexer::token_type::JasminTokenType::Eof => JasminElementType::Eof,
            crate::lexer::token_type::JasminTokenType::Error => JasminElementType::Error,
        }
    }
}
