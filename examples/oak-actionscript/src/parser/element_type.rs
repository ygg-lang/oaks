use crate::lexer::ActionScriptTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// ActionScript 语法树元素的类型别名
pub type ActionScriptElement<'a> = Arc<GreenNode<'a, ActionScriptElementType>>;

/// ActionScript 语法树中所有可能的元素类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionScriptElementType {
    /// Root node of the program
    Program,
    /// Root node
    Root,
    /// Generic statement node
    Statement,
    /// Generic expression node
    Expression,
    /// Block of statements
    Block,
    /// Class declaration
    Class,
    /// Interface declaration
    Interface,
    /// Function declaration
    Function,
    /// Variable declaration
    Variable,
    /// Import statement
    Import,
    /// Package declaration
    Package,
    /// Namespace declaration
    Namespace,
    /// Assignment expression
    Assignment,
    /// Function call
    FunctionCall,
    /// Method call
    MethodCall,
    /// Property access
    PropertyAccess,
    /// Array access
    ArrayAccess,
    /// Conditional expression (ternary)
    ConditionalExpression,
    /// Binary expression
    BinaryExpression,
    /// Unary expression
    UnaryExpression,
    /// If statement
    IfStatement,
    /// For statement
    ForStatement,
    /// While statement
    WhileStatement,
    /// Do-while statement
    DoWhileStatement,
    /// Switch statement
    SwitchStatement,
    /// Try statement
    TryStatement,
    /// Throw statement
    ThrowStatement,
    /// Return statement
    ReturnStatement,
    /// Break statement
    BreakStatement,
    /// Continue statement
    ContinueStatement,

    /// Error token
    Error,

    /// Identifier
    Identifier,
    /// Literal
    LiteralExpression,
    /// Identifier expression
    IdentifierExpression,
    /// Parenthesized expression
    ParenthesizedExpression,
    /// Source file
    SourceFile,
    /// Parameter list
    ParameterList,
    /// Block expression
    BlockExpression,
    /// Use item
    UseItem,
    /// Module item
    ModuleItem,
    /// Struct item
    StructItem,
    /// Enum item
    EnumItem,
    /// Let statement
    LetStatement,
    /// If expression
    IfExpression,
    /// While expression
    WhileExpression,
    /// Loop expression
    LoopExpression,
    /// For expression
    ForExpression,
    /// Call expression
    CallExpression,
    /// Index expression
    IndexExpression,
    /// Field expression
    FieldExpression,
}

impl ElementType for ActionScriptElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Program | Self::Root | Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        use UniversalElementRole::*;
        match self {
            Self::Program | Self::Root | Self::SourceFile => Root,
            Self::Class | Self::Interface | Self::Function | Self::Variable | Self::Package | Self::Namespace | Self::ModuleItem | Self::StructItem | Self::EnumItem => Definition,
            Self::Block | Self::BlockExpression | Self::ParameterList | Self::ParenthesizedExpression => Container,
            Self::Statement | Self::Import | Self::UseItem | Self::LetStatement | Self::ReturnStatement | Self::BreakStatement | Self::ContinueStatement | Self::ThrowStatement => Statement,
            Self::Expression
            | Self::Assignment
            | Self::BinaryExpression
            | Self::UnaryExpression
            | Self::ConditionalExpression
            | Self::IfExpression
            | Self::WhileExpression
            | Self::LoopExpression
            | Self::ForExpression
            | Self::IdentifierExpression
            | Self::LiteralExpression
            | Self::IndexExpression
            | Self::FieldExpression
            | Self::PropertyAccess
            | Self::ArrayAccess => Expression,
            Self::FunctionCall | Self::MethodCall | Self::CallExpression => Call,
            Self::Identifier => Reference,
            Self::Error => Error,
            _ => None,
        }
    }
}

impl From<ActionScriptTokenType> for ActionScriptElementType {
    fn from(token_type: ActionScriptTokenType) -> Self {
        match token_type {
            ActionScriptTokenType::Identifier => Self::Identifier,
            ActionScriptTokenType::StringLiteral | ActionScriptTokenType::NumberLiteral | ActionScriptTokenType::BooleanLiteral | ActionScriptTokenType::NullLiteral => Self::LiteralExpression,
            _ => Self::Error,
        }
    }
}
