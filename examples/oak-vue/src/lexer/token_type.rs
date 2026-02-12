use oak_core::{Token, TokenType, UniversalTokenRole};

/// Vue token.
pub type VueToken = Token<VueTokenType>;

/// Vue token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VueTokenType {
    // --- Lexical Tokens ---
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,

    // Keywords
    /// `import` keyword.
    Import, // import
    /// `export` keyword.
    Export, // export
    /// `default` keyword.
    Default, // default
    /// `from` keyword.
    From, // from
    /// `as` keyword.
    As, // as
    /// `const` keyword.
    Const, // const
    /// `let` keyword.
    Let, // let
    /// `var` keyword.
    Var, // var
    /// `function` keyword.
    Function, // function
    /// `if` keyword.
    If, // if
    /// `else` keyword.
    Else, // else
    /// `while` keyword.
    While, // while
    /// `for` keyword.
    For, // for
    /// `return` keyword.
    Return, // return
    /// `break` keyword.
    Break, // break
    /// `continue` keyword.
    Continue, // continue
    /// `switch` keyword.
    Switch, // switch
    /// `try` keyword.
    Try, // try
    /// `throw` keyword.
    Throw, // throw
    /// `in` keyword.
    In, // in
    /// `of` keyword.
    Of, // of
    /// `true` keyword.
    True, // true
    /// `false` keyword.
    False, // false
    /// `null` keyword.
    Null, // null

    // Literals & Identifiers
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,
    /// Plain text in template.
    Text, // Plain text in template

    // Operators
    /// Plus `+`.
    Plus, // +
    /// Minus `-`.
    Minus, // -
    /// Star `*`.
    Star, // *
    /// Slash `/`.
    Slash, // /
    /// Percent `%`.
    Percent, // %
    /// Equals `=`.
    Eq, // =
    /// Equal `==`.
    EqEq, // ==
    /// Strict equal `===`.
    EqEqEq, // ===
    /// Not equal `!=`.
    NotEq, // !=
    /// Strict not equal `!==`.
    NotEqEq, // !==
    /// Less than `<`.
    Lt, // <
    /// Greater than `>`.
    Gt, // >
    /// Less than or equal `<=`.
    LtEq, // <=
    /// Greater than or equal `>=`.
    GtEq, // >=
    /// Logical and `&&`.
    And, // &&
    /// Logical or `||`.
    Or, // ||
    /// Logical not `!`.
    Bang, // !
    /// Increment `++`.
    PlusPlus, // ++
    /// Decrement `--`.
    MinusMinus, // --
    /// Dot `.`.
    Dot, // .
    /// Arrow `=>`.
    Arrow, // =>
    /// Colon `:`.
    Colon, // :
    /// Comma `,`.
    Comma, // ,
    /// Semicolon `;`.
    Semicolon, // ;
    /// Question mark `?`.
    Question, // ?
    /// `&` symbol.
    Amp, // &
    /// `|` symbol.
    Pipe, // |
    /// `#` symbol.
    Hash, // #
    /// `@` symbol.
    At, // @

    // Delimiters
    /// Left parenthesis `(`.
    LeftParen, // (
    /// Right parenthesis `)`.
    RightParen, // )
    /// Left brace `{`.
    LeftBrace, // {
    /// Right brace `}`.
    RightBrace, // }
    /// Left bracket `[`.
    LeftBracket, // [
    /// Right bracket `]`.
    RightBracket, // ]

    // Multi-character Tokens
    /// Interpolation start `{{`.
    InterpolationStart, // {{
    /// Interpolation end `}}`.
    InterpolationEnd, // }}
    /// Self-closing tag end `/>`.
    SlashGt, // />
    /// Closing tag start `</`.
    LtSlash, // </

    // Special Markers
    /// `<script` start tag.
    ScriptStart, // <script
    /// `<style` start tag.
    StyleStart, // <style
    /// `<template` start tag.
    TemplateStart, // <template
    /// `<!` doctype start.
    DocTypeStart, // <!

    /// End of file.
    Eof,
    /// Error.
    Error,

    // --- Structural Elements ---
    /// Root node.
    Root,
    /// Directive.
    Directive,
    /// Modifier.
    Modifier,
    /// Template element.
    TemplateElement,
    /// Program.
    Program,
    /// Element.
    Element,
    /// Tag.
    Tag, // <tag ... >
    /// Closing tag.
    CloseTag, // </tag>
    /// Doctype.
    DocType, // <!DOCTYPE ...>
    /// Attribute.
    Attribute,
    /// Attribute name.
    AttributeName,
    /// Attribute value.
    AttributeValue,
    /// Interpolation.
    Interpolation,
    /// Text node.
    TextNode,
    /// Comment node.
    CommentNode,

    // JS/Expression Elements
    /// Expression.
    Expression,
    /// Literal.
    Literal,
    /// Binary expression.
    BinaryExpr,
    /// Unary expression.
    UnaryExpr,
    /// Call expression.
    CallExpr,
    /// Member expression.
    MemberExpr,
    /// Array expression.
    ArrayExpr,
    /// Object expression.
    ObjectExpr,
    /// Object property.
    ObjectProperty,
    /// Arrow function.
    ArrowFunction,
    /// Conditional expression.
    ConditionalExpr,
    /// Template string literal.
    TemplateLiteral,
    /// For expression.
    ForExpr,
    /// For-in expression.
    ForInExpr,
    /// For-of expression.
    ForOfExpr,
    /// Pattern.
    Pattern,

    // Statements & Declarations
    /// Import statement.
    ImportStmt,
    /// Import specifier.
    ImportSpecifier,
    /// Export statement.
    ExportStmt,
    /// Variable declaration.
    VariableDecl,
    /// Variable declarator.
    VariableDeclarator,
    /// Function declaration.
    FunctionDecl,
    /// Expression statement.
    ExpressionStmt,
    /// Return statement.
    ReturnStmt,
    /// If statement.
    IfStmt,
    /// While statement.
    WhileStmt,
    /// For statement.
    ForStmt,
    /// Block statement.
    BlockStmt,
    /// Break statement.
    BreakStmt,
    /// Continue statement.
    ContinueStmt,
    /// TSE element.
    TseElement,
    /// TSE attribute.
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
            Self::Import | Self::Export | Self::Default | Self::Const | Self::Let | Self::Var | Self::Function | Self::If | Self::Else | Self::While | Self::For | Self::Return | Self::True | Self::False | Self::Null => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral => UniversalTokenRole::Literal,
            Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Text => UniversalTokenRole::Literal,

            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Eq | Self::Dot | Self::Colon | Self::Comma | Self::Semicolon | Self::Hash | Self::At | Self::Question | Self::Bang | Self::Amp | Self::Pipe | Self::Percent | Self::Lt | Self::Gt => {
                UniversalTokenRole::Operator
            }

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
