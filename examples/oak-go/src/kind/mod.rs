use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Go 语法节点类型
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GoSyntaxKind {
    // 非终端节点
    SourceFile,
    PackageClause,
    ImportDeclaration,
    ImportSpec,
    FunctionDeclaration,
    ParameterList,
    ParameterDecl,
    Block,
    VariableDeclaration,
    VariableSpec,
    ConstDeclaration,
    ConstSpec,
    TypeDeclaration,
    TypeSpec,
    StructType,
    FieldDeclList,
    FieldDecl,
    InterfaceType,
    MethodSpecList,
    MethodSpec,
    ExpressionList,
    AssignmentStatement,
    ShortVarDecl,
    ReturnStatement,
    IfStatement,
    ForStatement,
    SwitchStatement,
    ExprCaseClause,
    TypeSwitchStatement,
    TypeCaseClause,
    SelectStatement,
    CommClause,
    GoStatement,
    DeferStatement,
    CallExpression,
    IndexExpression,
    SelectorExpression,
    SliceExpression,
    TypeAssertion,
    UnaryExpression,
    BinaryExpression,
    LiteralValue,
    ElementList,
    KeyedElement,

    // 字面
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    RuneLiteral,
    BoolLiteral,

    // 标识
    Identifier,

    // 关键
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    For,
    Func,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,

    // 内置类型
    Bool,
    Byte,
    Complex64,
    Complex128,
    ErrorType,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Rune,
    String,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uintptr,

    // 特殊字面
    NilLiteral,
    NumberLiteral,
    CharLiteral,

    // 操作
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    LeftShift,      // <<
    RightShift,     // >>
    AmpersandCaret, // &^

    PlusAssign,           // +=
    MinusAssign,          // -=
    StarAssign,           // *=
    SlashAssign,          // /=
    PercentAssign,        // %=
    AmpersandAssign,      // &=
    PipeAssign,           // |=
    CaretAssign,          // ^=
    XorAssign,            // ^= (别名)
    LeftShiftAssign,      // <<=
    RightShiftAssign,     // >>=
    AmpersandCaretAssign, // &^=
    AndAssign,            // &=
    OrAssign,             // |=
    AndNotAssign,         // &^=
    AndNot,               // &^

    LogicalAnd, // &&
    LogicalOr,  // ||
    And,        // && (别名)
    Or,         // || (别名)
    Arrow,      // <-
    LeftArrow,  // <- (别名)
    Increment,  // ++
    Decrement,  // --

    Equal,      // ==
    Less,       // <
    Greater,    // >
    Assign,     // =
    LogicalNot, // !
    Not,        // ! (别名)

    NotEqual,     // !=
    LessEqual,    // <=
    GreaterEqual, // >=
    ColonAssign,  // :=
    Define,       // := (别名)
    Ellipsis,     // ...

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Period,       // .
    Dot,          // . (别名)
    Semicolon,    // ;
    Colon,        // :

    // 空白和注
    Whitespace,
    Comment,

    // 特殊
    Eof,
    Error,
}

impl GoSyntaxKind {
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Break
                | Self::Case
                | Self::Chan
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Defer
                | Self::Else
                | Self::Fallthrough
                | Self::For
                | Self::Func
                | Self::Go
                | Self::Goto
                | Self::If
                | Self::Import
                | Self::Interface
                | Self::Map
                | Self::Package
                | Self::Range
                | Self::Return
                | Self::Select
                | Self::Struct
                | Self::Switch
                | Self::Type
                | Self::Var
        )
    }
}

impl TokenType for GoSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntLiteral | Self::FloatLiteral | Self::StringLiteral | Self::RuneLiteral | Self::BoolLiteral | Self::NilLiteral | Self::NumberLiteral | Self::CharLiteral => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::LeftShift
            | Self::RightShift
            | Self::AmpersandCaret
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::AmpersandAssign
            | Self::PipeAssign
            | Self::CaretAssign
            | Self::XorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::AmpersandCaretAssign
            | Self::AndAssign
            | Self::OrAssign
            | Self::AndNotAssign
            | Self::AndNot
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::And
            | Self::Or
            | Self::Arrow
            | Self::LeftArrow
            | Self::Increment
            | Self::Decrement
            | Self::Equal
            | Self::Less
            | Self::Greater
            | Self::Assign
            | Self::LogicalNot
            | Self::Not
            | Self::NotEqual
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::ColonAssign
            | Self::Define => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Comma | Self::Period | Self::Dot | Self::Semicolon | Self::Colon | Self::Ellipsis => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

use core::fmt;

impl fmt::Debug for GoSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::SourceFile => "SourceFile",
            Self::PackageClause => "PackageClause",
            Self::ImportDeclaration => "ImportDeclaration",
            Self::ImportSpec => "ImportSpec",
            Self::FunctionDeclaration => "FunctionDeclaration",
            Self::ParameterList => "ParameterList",
            Self::ParameterDecl => "ParameterDecl",
            Self::Block => "Block",
            Self::VariableDeclaration => "VariableDeclaration",
            Self::VariableSpec => "VariableSpec",
            Self::ConstDeclaration => "ConstDeclaration",
            Self::ConstSpec => "ConstSpec",
            Self::TypeDeclaration => "TypeDeclaration",
            Self::TypeSpec => "TypeSpec",
            Self::StructType => "StructType",
            Self::FieldDeclList => "FieldDeclList",
            Self::FieldDecl => "FieldDecl",
            Self::InterfaceType => "InterfaceType",
            Self::MethodSpecList => "MethodSpecList",
            Self::MethodSpec => "MethodSpec",
            Self::ExpressionList => "ExpressionList",
            Self::AssignmentStatement => "AssignmentStatement",
            Self::ShortVarDecl => "ShortVarDecl",
            Self::ReturnStatement => "ReturnStatement",
            Self::IfStatement => "IfStatement",
            Self::ForStatement => "ForStatement",
            Self::SwitchStatement => "SwitchStatement",
            Self::ExprCaseClause => "ExprCaseClause",
            Self::TypeSwitchStatement => "TypeSwitchStatement",
            Self::TypeCaseClause => "TypeCaseClause",
            Self::SelectStatement => "SelectStatement",
            Self::CommClause => "CommClause",
            Self::GoStatement => "GoStatement",
            Self::DeferStatement => "DeferStatement",
            Self::CallExpression => "CallExpression",
            Self::IndexExpression => "IndexExpression",
            Self::SelectorExpression => "SelectorExpression",
            Self::SliceExpression => "SliceExpression",
            Self::TypeAssertion => "TypeAssertion",
            Self::UnaryExpression => "UnaryExpression",
            Self::BinaryExpression => "BinaryExpression",
            Self::LiteralValue => "LiteralValue",
            Self::ElementList => "ElementList",
            Self::KeyedElement => "KeyedElement",
            Self::IntLiteral => "IntLiteral",
            Self::FloatLiteral => "FloatLiteral",
            Self::StringLiteral => "StringLiteral",
            Self::RuneLiteral => "RuneLiteral",
            Self::BoolLiteral => "BoolLiteral",
            Self::NilLiteral => "NilLiteral",
            Self::Identifier => "Identifier",
            Self::Package => "Package",
            Self::Import => "Import",
            Self::Func => "Func",
            Self::Var => "Var",
            Self::Const => "Const",
            Self::Type => "Type",
            Self::Struct => "Struct",
            Self::Interface => "Interface",
            Self::Map => "Map",
            Self::Chan => "Chan",
            Self::If => "If",
            Self::Else => "Else",
            Self::For => "For",
            Self::Range => "Range",
            Self::Switch => "Switch",
            Self::Case => "Case",
            Self::Default => "Default",
            Self::Break => "Break",
            Self::Continue => "Continue",
            Self::Return => "Return",
            Self::Go => "Go",
            Self::Defer => "Defer",
            Self::Select => "Select",
            Self::Fallthrough => "Fallthrough",
            Self::Goto => "Goto",
            Self::LeftParen => "LeftParen",
            Self::RightParen => "RightParen",
            Self::LeftBrace => "LeftBrace",
            Self::RightBrace => "RightBrace",
            Self::LeftBracket => "LeftBracket",
            Self::RightBracket => "RightBracket",
            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Percent => "Percent",
            Self::Ampersand => "Ampersand",
            Self::Pipe => "Pipe",
            Self::Caret => "Caret",
            Self::LeftShift => "LeftShift",
            Self::RightShift => "RightShift",
            Self::AndNot => "AndNot",
            Self::PlusAssign => "PlusAssign",
            Self::MinusAssign => "MinusAssign",
            Self::StarAssign => "StarAssign",
            Self::SlashAssign => "SlashAssign",
            Self::PercentAssign => "PercentAssign",
            Self::AmpersandAssign => "AmpersandAssign",
            Self::PipeAssign => "PipeAssign",
            Self::CaretAssign => "CaretAssign",
            Self::LeftShiftAssign => "LeftShiftAssign",
            Self::RightShiftAssign => "RightShiftAssign",
            Self::XorAssign => "XorAssign",
            Self::AndAssign => "AndAssign",
            Self::OrAssign => "OrAssign",
            Self::AndNotAssign => "AndNotAssign",
            Self::LogicalAnd => "LogicalAnd",
            Self::LogicalOr => "LogicalOr",
            Self::And => "And",
            Self::Or => "Or",
            Self::Arrow => "Arrow",
            Self::LeftArrow => "LeftArrow",
            Self::Increment => "Increment",
            Self::Decrement => "Decrement",
            Self::Equal => "Equal",
            Self::Less => "Less",
            Self::Greater => "Greater",
            Self::Assign => "Assign",
            Self::LogicalNot => "LogicalNot",
            Self::Not => "Not",
            Self::NotEqual => "NotEqual",
            Self::LessEqual => "LessEqual",
            Self::GreaterEqual => "GreaterEqual",
            Self::ColonAssign => "ColonAssign",
            Self::Define => "Define",
            Self::Comma => "Comma",
            Self::Period => "Period",
            Self::Dot => "Dot",
            Self::Semicolon => "Semicolon",
            Self::Colon => "Colon",
            Self::Ellipsis => "Ellipsis",
            Self::AmpersandCaret => "AmpersandCaret",
            Self::AmpersandCaretAssign => "AmpersandCaretAssign",
            Self::Bool => "Bool",
            Self::Byte => "Byte",
            Self::Complex64 => "Complex64",
            Self::Complex128 => "Complex128",
            Self::ErrorType => "ErrorType",
            Self::Float32 => "Float32",
            Self::Float64 => "Float64",
            Self::Int => "Int",
            Self::Int8 => "Int8",
            Self::Int16 => "Int16",
            Self::Int32 => "Int32",
            Self::Int64 => "Int64",
            Self::Rune => "Rune",
            Self::String => "String",
            Self::Uint => "Uint",
            Self::Uint8 => "Uint8",
            Self::Uint16 => "Uint16",
            Self::Uint32 => "Uint32",
            Self::Uint64 => "Uint64",
            Self::Uintptr => "Uintptr",
            Self::NumberLiteral => "NumberLiteral",
            Self::CharLiteral => "CharLiteral",
            Self::Whitespace => "Whitespace",
            Self::Comment => "Comment",
            Self::Eof => "Eof",
            Self::Error => "Error",
        };
        write!(f, "{}", name)
    }
}

impl ElementType for GoSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::FunctionDeclaration | Self::VariableDeclaration | Self::ConstDeclaration | Self::TypeDeclaration => UniversalElementRole::Definition,
            Self::IfStatement | Self::ForStatement | Self::SwitchStatement | Self::ReturnStatement => UniversalElementRole::Statement,
            Self::CallExpression | Self::BinaryExpression | Self::UnaryExpression => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }
}
