use oak_core::{Token, TokenType, UniversalTokenRole};

/// Represents a token in a Protobuf file.
pub type ProtobufToken = Token<ProtobufTokenType>;

impl TokenType for ProtobufTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Assign | Self::Semicolon | Self::Comma | Self::Dot | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::LeftAngle | Self::RightAngle => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::Keyword, // Most variants are keywords or types
        }
    }
}

/// Token types for the Protobuf language.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtobufTokenType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    /// Comment.
    Comment,

    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Boolean literal.
    BooleanLiteral,

    /// Identifier.
    Identifier,

    /// `syntax` keyword.
    Syntax,
    /// `package` keyword.
    Package,
    /// `import` keyword.
    Import,
    /// `option` keyword.
    Option,
    /// `message` keyword.
    Message,
    /// `enum` keyword.
    Enum,
    /// `service` keyword.
    Service,
    /// `rpc` keyword.
    Rpc,
    /// `returns` keyword.
    Returns,
    /// `stream` keyword.
    Stream,
    /// `repeated` keyword.
    Repeated,
    /// `optional` keyword.
    Optional,
    /// `required` keyword.
    Required,
    /// `oneof` keyword.
    Oneof,
    /// `map` keyword.
    Map,
    /// `reserved` keyword.
    Reserved,
    /// `extensions` keyword.
    Extensions,
    /// `extend` keyword.
    Extend,
    /// `group` keyword.
    Group,
    /// `public` keyword.
    Public,
    /// `weak` keyword.
    Weak,

    /// `double` type.
    Double,
    /// `float` type.
    Float,
    /// `int32` type.
    Int32,
    /// `int64` type.
    Int64,
    /// `uint32` type.
    Uint32,
    /// `uint64` type.
    Uint64,
    /// `sint32` type.
    Sint32,
    /// `sint64` type.
    Sint64,
    /// `fixed32` type.
    Fixed32,
    /// `fixed64` type.
    Fixed64,
    /// `sfixed32` type.
    Sfixed32,
    /// `sfixed64` type.
    Sfixed64,
    /// `bool` type.
    Bool,
    /// `string` type.
    String,
    /// `bytes` type.
    Bytes,

    /// Assignment operator `=`.
    Assign,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left angle bracket `<`.
    LeftAngle,
    /// Right angle bracket `>`.
    RightAngle,

    /// Error token.
    Error,
    /// End of stream.
    Eof,

    // Element types
    /// The root element.
    Root,
    /// A syntax definition.
    SyntaxDef,
    /// A package definition.
    PackageDef,
    /// An import definition.
    ImportDef,
    /// An option definition.
    OptionDef,
    /// A message definition.
    MessageDef,
    /// An enum definition.
    EnumDef,
    /// A service definition.
    ServiceDef,
    /// An RPC definition.
    RpcDef,
    /// A field definition.
    FieldDef,
    /// An enum field definition.
    EnumFieldDef,
    /// A oneof definition.
    OneofDef,
    /// A map field definition.
    MapFieldDef,
    /// A reserved definition.
    ReservedDef,
    /// An extensions definition.
    ExtensionsDef,
}
