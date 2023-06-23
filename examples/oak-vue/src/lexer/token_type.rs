use oak_core::{Language, LanguageCategory, Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type VueToken = Token<VueTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VueTokenType {
    // --- Lexical Tokens ---
    Whitespace,
    Comment,

    // Keywords
    Import,   // import
    Export,   // export
    Default,  // default
    From,     // from
    As,       // as
    Const,    // const
    Let,      // let
    Var,      // var
    Function, // function
    If,       // if
    Else,     // else
    While,    // while
    For,      // for
    Return,   // return
    Break,    // break
    Continue, // continue
    Switch,   // switch
    Try,      // try
    Throw,    // throw
    In,       // in
    Of,       // of
    True,     // true
    False,    // false
    Null,     // null

    // Literals & Identifiers
    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    Text, // Plain text in template

    // Operators
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Eq,         // =
    EqEq,       // ==
    EqEqEq,     // ===
    NotEq,      // !=
    NotEqEq,    // !==
    Lt,         // <
    Gt,         // >
    LtEq,       // <=
    GtEq,       // >=
    And,        // &&
    Or,         // ||
    Bang,       // !
    PlusPlus,   // ++
    MinusMinus, // --
    Dot,        // .
    Arrow,      // =>
    Colon,      // :
    Comma,      // ,
    Semicolon,  // ;
    Question,   // ?
    Amp,        // &
    Pipe,       // |
    Hash,       // #
    At,         // ↯

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Multi-character Tokens
    InterpolationStart, // {{
    InterpolationEnd,   // }}
    SlashGt,            // />
    LtSlash,            // </

    // Special Markers
    ScriptStart,   // <script
    StyleStart,    // <style
    TemplateStart, // <template
    DocTypeStart,  // <!

    Eof,
    Error,

    // --- Structural Elements ---
    Root,
    Directive,
    Modifier,
    TemplateElement,
    Program,
    Element,
    Tag,      // <tag ... >
    CloseTag, // </tag>
    DocType,  // <!DOCTYPE ...>
    Attribute,
    AttributeName,
    AttributeValue,
    Interpolation,
    TextNode,
    CommentNode,

    // JS/Expression Elements
    Expression,
    Literal,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    MemberExpr,
    ArrayExpr,
    ObjectExpr,
    ObjectProperty,
    ArrowFunction,
    ConditionalExpr,
    TemplateLiteral,
    ForExpr,
    ForInExpr,
    ForOfExpr,
    Pattern,

    // Statements & Declarations
    ImportStmt,
    ImportSpecifier,
    ExportStmt,
    VariableDecl,
    VariableDeclarator,
    FunctionDecl,
    ExpressionStmt,
    ReturnStmt,
    IfStmt,
    WhileStmt,
    ForStmt,
    BlockStmt,
    BreakStmt,
    ContinueStmt,
    TseElement,
    TseAttribute,
}

impl TokenType for VueTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Import
            | Self::Export
            | Self::Default
            | Self::From
            | Self::As
            | Self::Const
            | Self::Let
            | Self::Var
            | Self::Function
            | Self::If
            | Self::Else
            | Self::While
            | Self::For
            | Self::Return
            | Self::Break
            | Self::Continue
            | Self::Switch
            | Self::Try
            | Self::Throw
            | Self::In
            | Self::Of
            | Self::True
            | Self::False
            | Self::Null => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::Text => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Eq
            | Self::EqEq
            | Self::EqEqEq
            | Self::NotEq
            | Self::NotEqEq
            | Self::Lt
            | Self::Gt
            | Self::LtEq
            | Self::GtEq
            | Self::And
            | Self::Or
            | Self::Bang
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::Dot
            | Self::Arrow
            | Self::Colon
            | Self::Comma
            | Self::Semicolon
            | Self::Question
            | Self::Amp
            | Self::Pipe
            | Self::Hash
            | Self::At => UniversalTokenRole::Operator,

            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket => UniversalTokenRole::Punctuation,

            Self::InterpolationStart | Self::InterpolationEnd | Self::SlashGt | Self::LtSlash => UniversalTokenRole::Punctuation,

            Self::ScriptStart | Self::StyleStart | Self::TemplateStart | Self::DocTypeStart => UniversalTokenRole::Keyword,

            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VueLanguage {}

impl Default for VueLanguage {
    fn default() -> Self {
        Self {}
    }
}

impl VueLanguage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Language for VueLanguage {
    const NAME: &'static str = "vue";
    const CATEGORY: LanguageCategory = LanguageCategory::Markup;
    type TokenType = VueTokenType;
    type ElementType = crate::parser::element_type::VueElementType;
    type TypedRoot = ();
}
