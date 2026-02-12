use oak_core::{Token, TokenType, UniversalTokenRole};

/// MSIL token.
pub type MsilToken = Token<MsilTokenType>;

impl TokenType for MsilTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::CommentToken => UniversalTokenRole::Comment,
            Self::IdentifierToken => UniversalTokenRole::Name,
            Self::NumberToken => UniversalTokenRole::Literal,
            Self::StringToken => UniversalTokenRole::Literal,
            Self::AssemblyKeyword | Self::ExternKeyword | Self::ModuleKeyword | Self::ClassKeyword | Self::MethodKeyword | Self::PublicKeyword | Self::PrivateKeyword | Self::StaticKeyword | Self::Keyword => UniversalTokenRole::Keyword,
            Self::LeftBrace | Self::RightBrace | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Dot | Self::Colon | Self::Semicolon | Self::Comma | Self::Equal | Self::Slash => UniversalTokenRole::Operator,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error | Self::ErrorNode => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// MSIL token type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MsilTokenType {
    /// Root node.
    Root,
    /// Assembly.
    Assembly,
    /// External assembly.
    AssemblyExtern,
    /// Module.
    Module,
    /// Class.
    Class,
    /// Method.
    Method,
    /// Instruction.
    Instruction,
    /// Label.
    Label,
    /// Directive.
    Directive,
    /// Type.
    Type,
    /// Identifier.
    Identifier,
    /// Number.
    Number,
    /// String.
    String,
    /// Comment.
    Comment,
    /// Error node.
    ErrorNode,

    /// .assembly keyword
    AssemblyKeyword,
    /// extern keyword
    ExternKeyword,
    /// .module keyword
    ModuleKeyword,
    /// .class keyword
    ClassKeyword,
    /// .method keyword
    MethodKeyword,
    /// public keyword
    PublicKeyword,
    /// private keyword
    PrivateKeyword,
    /// static keyword
    StaticKeyword,
    /// Other keywords
    Keyword,

    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,
    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Dot (.)
    Dot,
    /// Colon (:)
    Colon,
    /// Semicolon (;)
    Semicolon,
    /// Comma (,)
    Comma,
    /// Equal (=)
    Equal,
    /// Slash (/)
    Slash,

    /// Identifier unit
    IdentifierToken,
    /// Number unit
    NumberToken,
    /// String unit
    StringToken,

    /// Whitespace
    Whitespace,
    /// Comment unit
    CommentToken,
    /// End of file
    Eof,
    /// Error unit
    Error,
}
