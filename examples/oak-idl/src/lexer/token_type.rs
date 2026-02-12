use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token types for the IDL lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdlTokenType {
    /// Whitespace characters.
    Whitespace = 0,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,

    /// A string literal.
    StringLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// A character literal.
    CharLiteral,

    /// An identifier.
    Identifier,

    /// The `void` keyword.
    Void,
    /// The `boolean` keyword.
    Boolean,
    /// The `byte` keyword.
    Byte,
    /// The `octet` keyword.
    Octet,
    /// The `short` keyword.
    Short,
    /// The `unsigned short` keyword.
    UnsignedShort,
    /// The `long` keyword.
    Long,
    /// The `unsigned long` keyword.
    UnsignedLong,
    /// The `long long` keyword.
    LongLong,
    /// The `unsigned long long` keyword.
    UnsignedLongLong,
    /// The `float` keyword.
    Float,
    /// The `double` keyword.
    Double,
    /// The `long double` keyword.
    LongDouble,
    /// The `char` keyword.
    Char,
    /// The `wchar` keyword.
    WChar,
    /// The `string` keyword.
    String,
    /// The `wstring` keyword.
    WString,
    /// The `any` keyword.
    Any,
    /// The `Object` keyword.
    Object,
    /// The `ValueBase` keyword.
    ValueBase,

    /// The `struct` keyword.
    Struct,
    /// The `union` keyword.
    Union,
    /// The `enum` keyword.
    Enum,
    /// The `interface` keyword.
    Interface,
    /// The `module` keyword.
    Module,
    /// The `exception` keyword.
    Exception,
    /// The `typedef` keyword.
    Typedef,
    /// The `sequence` keyword.
    Sequence,
    /// The `array` keyword.
    Array,
    /// The `fixed` keyword.
    Fixed,

    /// The `const` keyword.
    Const,
    /// The `readonly` keyword.
    Readonly,
    /// The `attribute` keyword.
    Attribute,
    /// The `oneway` keyword.
    Oneway,
    /// The `in` keyword.
    In,
    /// The `out` keyword.
    Out,
    /// The `inout` keyword.
    Inout,
    /// The `raises` keyword.
    Raises,
    /// The `context` keyword.
    Context,
    /// The `local` keyword.
    Local,
    /// The `abstract` keyword.
    Abstract,
    /// The `custom` keyword.
    Custom,
    /// The `private` keyword.
    Private,
    /// The `public` keyword.
    Public,
    /// The `truncatable` keyword.
    Truncatable,
    /// The `supports` keyword.
    Supports,
    /// The `valuetype` keyword.
    ValueType,
    /// The `native` keyword.
    Native,
    /// The `factory` keyword.
    Factory,

    /// An `#include` directive.
    Include,
    /// A `#pragma` directive.
    Pragma,
    /// A `#define` directive.
    Define,
    /// An `#ifdef` directive.
    Ifdef,
    /// An `#ifndef` directive.
    Ifndef,
    /// An `#endif` directive.
    Endif,
    /// An `#else` directive.
    Else,
    /// An `#elif` directive.
    Elif,
    /// An `#undef` directive.
    Undef,
    /// A hash sign (`#`).
    Hash,

    /// An opening parenthesis (`(`).
    LeftParen,
    /// A closing parenthesis (`)`).
    RightParen,
    /// An opening brace (`{`).
    LeftBrace,
    /// A closing brace (`}`).
    RightBrace,
    /// An opening bracket (`[`).
    LeftBracket,
    /// A closing bracket (`]`).
    RightBracket,
    /// A left angle bracket (`<`).
    LeftAngle,
    /// A right angle bracket (`>`).
    RightAngle,
    /// A comma (`,`).
    Comma,
    /// A semicolon (`;`).
    Semicolon,
    /// A colon (`:`).
    Colon,
    /// A double colon (`::`).
    DoubleColon,
    /// A dot (`.`).
    Dot,
    /// An arrow (`->`).
    Arrow,

    /// An assignment operator (`=`).
    Assign,
    /// A plus sign (`+`).
    Plus,
    /// A minus sign (`-`).
    Minus,
    /// A multiplication operator (`*`).
    Multiply,
    /// A division operator (`/`).
    Divide,
    /// A modulo operator (`%`).
    Modulo,
    /// A bitwise AND operator (`&`).
    BitwiseAnd,
    /// A bitwise OR operator (`|`).
    BitwiseOr,
    /// A bitwise XOR operator (`^`).
    BitwiseXor,
    /// A bitwise NOT operator (`~`).
    BitwiseNot,
    /// A logical AND operator (`&&`).
    LogicalAnd,
    /// A logical OR operator (`||`).
    LogicalOr,
    /// A logical NOT operator (`!`).
    LogicalNot,
    /// A less-than operator (`<`).
    Less,
    /// A greater-than operator (`>`).
    Greater,
    /// A less-than-or-equal-to operator (`<=`).
    LessEqual,
    /// A greater-than-or-equal-to operator (`>=`).
    GreaterEqual,
    /// An equality operator (`==`).
    Equal,
    /// A not-equal operator (`!=`).
    NotEqual,
    /// A left shift operator (`<<`).
    LeftShift,
    /// A right shift operator (`>>`).
    RightShift,

    /// Source file node.
    SourceFile,
    /// An error token.
    Error,
    /// End of file.
    Eof,
}

impl TokenType for IdlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::CharLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Void
            | Self::Boolean
            | Self::Byte
            | Self::Octet
            | Self::Short
            | Self::UnsignedShort
            | Self::Long
            | Self::UnsignedLong
            | Self::LongLong
            | Self::UnsignedLongLong
            | Self::Float
            | Self::Double
            | Self::LongDouble
            | Self::Char
            | Self::WChar
            | Self::String
            | Self::WString
            | Self::Any
            | Self::Object
            | Self::ValueBase
            | Self::Struct
            | Self::Union
            | Self::Enum
            | Self::Interface
            | Self::Module
            | Self::Exception
            | Self::Typedef
            | Self::Sequence
            | Self::Array
            | Self::Fixed
            | Self::Const
            | Self::Readonly
            | Self::Attribute
            | Self::Oneway
            | Self::In
            | Self::Out
            | Self::Inout
            | Self::Raises
            | Self::Context
            | Self::Local
            | Self::Abstract
            | Self::Custom
            | Self::Private
            | Self::Public
            | Self::Truncatable
            | Self::Supports
            | Self::ValueType
            | Self::Native
            | Self::Factory => UniversalTokenRole::Keyword,
            Self::Include | Self::Pragma | Self::Define | Self::Ifdef | Self::Ifndef | Self::Endif | Self::Else | Self::Elif | Self::Undef | Self::Hash => UniversalTokenRole::Keyword,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftAngle
            | Self::RightAngle
            | Self::Comma
            | Self::Semicolon
            | Self::Colon
            | Self::DoubleColon
            | Self::Dot
            | Self::Arrow => UniversalTokenRole::Punctuation,
            Self::Assign
            | Self::Plus
            | Self::Minus
            | Self::Multiply
            | Self::Divide
            | Self::Modulo
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::BitwiseNot
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Equal
            | Self::NotEqual
            | Self::LeftShift
            | Self::RightShift => UniversalTokenRole::Operator,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

/// A token in the IDL language.
pub type IdlToken = Token<IdlTokenType>;
