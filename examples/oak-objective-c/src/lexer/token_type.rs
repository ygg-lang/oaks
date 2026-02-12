use oak_core::{Token, TokenType, UniversalTokenRole};

impl TokenType for ObjectiveCTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        match self {
            Self::Whitespace | Self::Newline | Self::CommentToken => true,
            _ => false,
        }
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentToken => UniversalTokenRole::Comment,
            Self::IntegerLiteral | Self::FloatLiteral | Self::String | Self::Character => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::InterfaceKeyword
            | Self::ImplementationKeyword
            | Self::EndKeyword
            | Self::PropertyKeyword
            | Self::SynthesizeKeyword
            | Self::DynamicKeyword
            | Self::ProtocolKeyword
            | Self::CategoryKeyword
            | Self::ImportKeyword
            | Self::IncludeKeyword
            | Self::IfKeyword
            | Self::ElseKeyword
            | Self::ForKeyword
            | Self::WhileKeyword
            | Self::DoKeyword
            | Self::SwitchKeyword
            | Self::CaseKeyword
            | Self::DefaultKeyword
            | Self::BreakKeyword
            | Self::ContinueKeyword
            | Self::ReturnKeyword
            | Self::VoidKeyword
            | Self::IntKeyword
            | Self::FloatKeyword
            | Self::DoubleKeyword
            | Self::CharKeyword
            | Self::BoolKeyword
            | Self::IdKeyword
            | Self::SelfKeyword
            | Self::SuperKeyword
            | Self::NilKeyword
            | Self::YesKeyword
            | Self::NoKeyword => UniversalTokenRole::Keyword,
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Equal | Self::EqualEqual | Self::NotEqual | Self::Less | Self::Greater | Self::LessEqual | Self::GreaterEqual | Self::And | Self::Or | Self::Not | Self::Question => {
                UniversalTokenRole::Operator
            }
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::At => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Objective-C tokens.
pub type ObjectiveCToken = Token<ObjectiveCTokenType>;

/// Objective-C token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectiveCTokenType {
    /// Root node.
    Root,
    /// Interface declaration.
    InterfaceDeclaration,
    /// Implementation declaration.
    ImplementationDeclaration,
    /// Method declaration.
    MethodDeclaration,
    /// Property declaration.
    PropertyDeclaration,
    /// Protocol declaration.
    ProtocolDeclaration,
    /// Category declaration.
    CategoryDeclaration,
    /// Class extension.
    ClassExtension,

    /// @interface keyword.
    InterfaceKeyword,
    /// @implementation keyword.
    ImplementationKeyword,
    /// @end keyword.
    EndKeyword,
    /// @property keyword.
    PropertyKeyword,
    /// @synthesize keyword.
    SynthesizeKeyword,
    /// @dynamic keyword.
    DynamicKeyword,
    /// @protocol keyword.
    ProtocolKeyword,
    /// Category keyword.
    CategoryKeyword,
    /// #import keyword.
    ImportKeyword,
    /// #include keyword.
    IncludeKeyword,
    /// if keyword.
    IfKeyword,
    /// else keyword.
    ElseKeyword,
    /// for keyword.
    ForKeyword,
    /// while keyword.
    WhileKeyword,
    /// do keyword.
    DoKeyword,
    /// switch keyword.
    SwitchKeyword,
    /// case keyword.
    CaseKeyword,
    /// default keyword.
    DefaultKeyword,
    /// break keyword.
    BreakKeyword,
    /// continue keyword.
    ContinueKeyword,
    /// return keyword.
    ReturnKeyword,
    /// void keyword.
    VoidKeyword,
    /// int keyword.
    IntKeyword,
    /// float keyword.
    FloatKeyword,
    /// double keyword.
    DoubleKeyword,
    /// char keyword.
    CharKeyword,
    /// BOOL keyword.
    BoolKeyword,
    /// id keyword.
    IdKeyword,
    /// self keyword.
    SelfKeyword,
    /// super keyword.
    SuperKeyword,
    /// nil keyword.
    NilKeyword,
    /// YES keyword.
    YesKeyword,
    /// NO keyword.
    NoKeyword,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Star `*`.
    Star,
    /// Slash `/`.
    Slash,
    /// Percent `%`.
    Percent,
    /// Equal `=`.
    Equal,
    /// Double equal `==`.
    EqualEqual,
    /// Not equal `!=`.
    NotEqual,
    /// Less than `<`.
    Less,
    /// Greater than `>`.
    Greater,
    /// Less than or equal to `<=`.
    LessEqual,
    /// Greater than or equal to `>=`.
    GreaterEqual,
    /// Logical AND `&&`.
    And,
    /// Logical OR `||`.
    Or,
    /// Logical NOT `!`.
    Not,
    /// Question mark `?`.
    Question,
    /// At symbol `@`.
    At,

    /// Identifier.
    Identifier,
    /// Integer literal.
    IntegerLiteral,
    /// Float literal.
    FloatLiteral,
    /// String literal.
    String,
    /// Character literal.
    Character,

    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    CommentToken,
    /// Error token.
    Error,
    /// End of stream.
    Eof,
}
