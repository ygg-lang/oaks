use oak_core::{ElementType, UniversalElementRole};

/// Vue element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VueElementType {
    /// Root node.
    Root,
    /// Program node.
    Program,
    /// Template element.
    TemplateElement,
    /// Element node.
    Element,
    /// Tag node.
    Tag,
    /// Closing tag node.
    CloseTag,
    /// Doctype node.
    DocType,
    /// Attribute node.
    Attribute,
    /// Attribute name.
    AttributeName,
    /// Attribute value.
    AttributeValue,
    /// Directive node.
    Directive,
    /// Modifier node.
    Modifier,
    /// Interpolation node.
    Interpolation,
    /// Text node.
    TextNode,
    /// Comment node.
    CommentNode,
    /// Identifier node.
    Identifier,

    // JS/Expression Elements
    /// Expression node.
    Expression,
    /// Literal node.
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
    /// Pattern node.
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

    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Error node.
    Error,
}

impl ElementType for VueElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Program | Self::TemplateElement => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::Tag | Self::CloseTag => UniversalElementRole::Name,
            Self::Attribute => UniversalElementRole::Attribute,
            Self::Interpolation => UniversalElementRole::Expression,
            Self::TextNode => UniversalElementRole::Value,
            Self::CommentNode | Self::Comment => UniversalElementRole::None,
            Self::Identifier => UniversalElementRole::Name,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VueTokenType> for VueElementType {
    fn from(token: crate::lexer::token_type::VueTokenType) -> Self {
        use crate::lexer::token_type::VueTokenType as T;
        match token {
            T::Root => Self::Root,
            T::Program => Self::Program,
            T::TemplateElement => Self::TemplateElement,
            T::Element => Self::Element,
            T::Tag => Self::Tag,
            T::CloseTag => Self::CloseTag,
            T::DocType => Self::DocType,
            T::Attribute => Self::Attribute,
            T::AttributeName => Self::AttributeName,
            T::AttributeValue => Self::AttributeValue,
            T::Directive => Self::Directive,
            T::Modifier => Self::Modifier,
            T::Interpolation => Self::Interpolation,
            T::TextNode => Self::TextNode,
            T::CommentNode => Self::CommentNode,
            T::Identifier => Self::Identifier,
            T::Expression => Self::Expression,
            T::Literal => Self::Literal,
            T::BinaryExpr => Self::BinaryExpr,
            T::UnaryExpr => Self::UnaryExpr,
            T::CallExpr => Self::CallExpr,
            T::MemberExpr => Self::MemberExpr,
            T::ArrayExpr => Self::ArrayExpr,
            T::ObjectExpr => Self::ObjectExpr,
            T::ObjectProperty => Self::ObjectProperty,
            T::ArrowFunction => Self::ArrowFunction,
            T::ConditionalExpr => Self::ConditionalExpr,
            T::TemplateLiteral => Self::TemplateLiteral,
            T::ForExpr => Self::ForExpr,
            T::ForInExpr => Self::ForInExpr,
            T::ForOfExpr => Self::ForOfExpr,
            T::Pattern => Self::Pattern,
            T::ImportStmt => Self::ImportStmt,
            T::ImportSpecifier => Self::ImportSpecifier,
            T::ExportStmt => Self::ExportStmt,
            T::VariableDecl => Self::VariableDecl,
            T::VariableDeclarator => Self::VariableDeclarator,
            T::FunctionDecl => Self::FunctionDecl,
            T::ExpressionStmt => Self::ExpressionStmt,
            T::ReturnStmt => Self::ReturnStmt,
            T::IfStmt => Self::IfStmt,
            T::WhileStmt => Self::WhileStmt,
            T::ForStmt => Self::ForStmt,
            T::BlockStmt => Self::BlockStmt,
            T::BreakStmt => Self::BreakStmt,
            T::ContinueStmt => Self::ContinueStmt,
            T::TseElement => Self::TseElement,
            T::TseAttribute => Self::TseAttribute,
            T::Whitespace => Self::Whitespace,
            T::Comment => Self::Comment,
            T::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
