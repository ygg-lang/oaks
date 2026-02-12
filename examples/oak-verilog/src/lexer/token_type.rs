use oak_core::{TokenType, UniversalTokenRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerilogKind {
    /// Whitespace token.
    Whitespace,
    /// Comment token.
    Comment,
    /// String literal.
    String,
    /// Number literal.
    Number,
    /// Identifier.
    Identifier,

    // Keywords
    /// 'module' keyword.
    ModuleKw,
    /// 'endmodule' keyword.
    EndmoduleKw,
    /// 'wire' keyword.
    WireKw,
    /// 'reg' keyword.
    RegKw,
    /// 'input' keyword.
    InputKw,
    /// 'output' keyword.
    OutputKw,
    /// 'always' keyword.
    AlwaysKw,
    /// 'begin' keyword.
    BeginKw,
    /// 'end' keyword.
    EndKw,
    /// 'if' keyword.
    IfKw,
    /// 'else' keyword.
    ElseKw,
    /// 'assign' keyword.
    AssignKw,
    /// 'posedge' keyword.
    PosedgeKw,
    /// 'negedge' keyword.
    NegedgeKw,
    /// 'case' keyword.
    CaseKw,
    /// 'endcase' keyword.
    EndcaseKw,
    /// 'default' keyword.
    DefaultKw,
    /// 'initial' keyword.
    InitialKw,
    /// 'inout' keyword.
    InoutKw,
    /// 'parameter' keyword.
    ParameterKw,

    // Operators
    /// '==' operator.
    EqualEqual,
    /// '!=' operator.
    NotEqual,
    /// '<=' operator.
    LessEqual,
    /// '>=' operator.
    GreaterEqual,
    /// '<<' operator.
    LeftShift,
    /// '>>' operator.
    RightShift,
    /// '&&' operator.
    AndAnd,
    /// '||' operator.
    OrOr,
    /// '+' operator.
    Plus,
    /// '-' operator.
    Minus,
    /// '*' operator.
    Star,
    /// '/' operator.
    Slash,
    /// '%' operator.
    Percent,
    /// '=' operator.
    Equal,
    /// '!' operator.
    Bang,
    /// '<' operator.
    Less,
    /// '>' operator.
    Greater,
    /// '&' operator.
    Ampersand,
    /// '|' operator.
    Pipe,
    /// '^' operator.
    Caret,
    /// '~' operator.
    Tilde,

    // Punctuation
    /// '(' punctuation.
    LeftParen,
    /// ')' punctuation.
    RightParen,
    /// '[' punctuation.
    LeftBracket,
    /// ']' punctuation.
    RightBracket,
    /// '{' punctuation.
    LeftBrace,
    /// '}' punctuation.
    RightBrace,
    /// ';' punctuation.
    Semicolon,
    /// ',' punctuation.
    Comma,
    /// '.' punctuation.
    Dot,
    /// ':' punctuation.
    Colon,
    /// '#' punctuation.
    Hash,
    /// '@' punctuation.
    At,
    /// '?' punctuation.
    Question,

    // Elements
    /// Root element.
    Root,
    /// Module element.
    Module,
    /// Port list element.
    PortList,
    /// Port element.
    Port,
    /// Module item element.
    ModuleItem,
    /// Assign element.
    Assign,
    /// Declaration element.
    Declaration,
    /// Always block element.
    Always,
    /// Initial block element.
    Initial,
    /// Block element.
    Block,
    /// Expression element.
    Expression,
    /// Statement element.
    Statement,

    // Internal
    /// Error token.
    Error,
    /// End of file token.
    Eof,
}

/// Verilog token type.
pub type VerilogTokenType = VerilogKind;

impl TokenType for VerilogKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::String | Self::Number => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::ModuleKw
            | Self::EndmoduleKw
            | Self::WireKw
            | Self::RegKw
            | Self::InputKw
            | Self::OutputKw
            | Self::AlwaysKw
            | Self::BeginKw
            | Self::EndKw
            | Self::IfKw
            | Self::ElseKw
            | Self::AssignKw
            | Self::PosedgeKw
            | Self::NegedgeKw
            | Self::CaseKw
            | Self::EndcaseKw
            | Self::DefaultKw
            | Self::InitialKw
            | Self::InoutKw
            | Self::ParameterKw => UniversalTokenRole::Keyword,
            Self::EqualEqual
            | Self::NotEqual
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::LeftShift
            | Self::RightShift
            | Self::AndAnd
            | Self::OrOr
            | Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equal
            | Self::Bang
            | Self::Less
            | Self::Greater
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Hash | Self::At | Self::Question => {
                UniversalTokenRole::Punctuation
            }
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
            _ => UniversalTokenRole::None,
        }
    }
}
