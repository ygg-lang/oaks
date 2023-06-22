use oak_core::{Language, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Vue Syntax Kind (Tokens and Elements)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VueSyntaxKind {
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
    At,         // @

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
    SelfClosingEnd,     // />
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

impl TokenType for VueSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn role(&self) -> Self::Role {
        use UniversalTokenRole::*;
        match self {
            Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::LeftParen
            | Self::RightParen
            | Self::Comma
            | Self::Colon
            | Self::Semicolon
            | Self::Dot
            | Self::At
            | Self::Hash
            | Self::InterpolationStart
            | Self::InterpolationEnd
            | Self::SelfClosingEnd
            | Self::LtSlash => Punctuation,

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
            | Self::Bang
            | Self::Amp
            | Self::Pipe
            | Self::Question
            | Self::Arrow
            | Self::And
            | Self::Or
            | Self::PlusPlus
            | Self::MinusMinus => Operator,

            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => Literal,

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
            | Self::True
            | Self::False
            | Self::Null => Keyword,

            Self::Identifier => Name,
            Self::Text => Literal,
            Self::Whitespace => Whitespace,
            Self::Comment => Comment,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl oak_core::ElementType for VueSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::Program)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Root | Self::Program => Root,
            Self::Element => Container,
            Self::Tag | Self::CloseTag | Self::Attribute => Detail,
            Self::AttributeName | Self::AttributeValue => Name,
            Self::TextNode | Self::CommentNode | Self::Interpolation => Value,

            Self::Expression | Self::BinaryExpr | Self::UnaryExpr | Self::CallExpr | Self::MemberExpr | Self::ArrayExpr | Self::ObjectExpr | Self::ArrowFunction | Self::ConditionalExpr | Self::TemplateLiteral => Value,

            Self::VariableDecl
            | Self::VariableDeclarator
            | Self::ImportStmt
            | Self::ImportSpecifier
            | Self::ExportStmt
            | Self::FunctionDecl
            | Self::BlockStmt
            | Self::ExpressionStmt
            | Self::ReturnStmt
            | Self::IfStmt
            | Self::WhileStmt
            | Self::ForStmt
            | Self::BreakStmt
            | Self::ContinueStmt => Statement,

            Self::TseElement | Self::TseAttribute => Detail,

            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VueLanguage;

impl Language for VueLanguage {
    const NAME: &'static str = "vue";
    type TokenType = VueSyntaxKind;
    type ElementType = VueSyntaxKind;
    type TypedRoot = ();
}
