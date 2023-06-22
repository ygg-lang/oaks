use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type ElixirToken = Token<ElixirSyntaxKind>;

/// Represents all possible syntax kinds in the Elixir programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElixirSyntaxKind {
    /// Root node of the syntax tree
    Root,
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline character
    Newline,
    /// Comment
    Comment,
    /// Identifier (variable names, function names, etc.)
    Identifier,
    /// Atom literal
    Atom,
    /// Variable name
    Variable,
    /// Number literal
    Number,
    /// Float literal
    Float,
    /// String literal
    String,
    /// Character literal
    Character,
    /// Sigil literal
    Sigil,

    /// after keyword
    After,
    /// and keyword
    And,
    /// case keyword
    Case,
    /// catch keyword
    Catch,
    /// cond keyword
    Cond,
    /// def keyword
    Def,
    /// defp keyword (private function)
    Defp,
    /// defmodule keyword
    Defmodule,
    /// defstruct keyword
    Defstruct,
    /// defprotocol keyword
    Defprotocol,
    /// defimpl keyword
    Defimpl,
    /// defmacro keyword
    Defmacro,
    /// defmacrop keyword (private macro)
    Defmacrop,
    /// do keyword
    Do,
    /// else keyword
    Else,
    /// elsif keyword
    Elsif,
    /// end keyword
    End,
    /// false keyword
    False,
    /// fn keyword
    Fn,
    /// if keyword
    If,
    /// in keyword
    In,
    /// not keyword
    Not,
    /// or keyword
    Or,
    /// receive keyword
    Receive,
    /// rescue keyword
    Rescue,
    /// true keyword
    True,
    /// try keyword
    Try,
    /// unless keyword
    Unless,
    /// when keyword
    When,
    /// with keyword
    With,

    /// plus operator (+)
    Plus,
    /// minus operator (-)
    Minus,
    /// multiplication operator (*)
    Star,
    /// division operator (/)
    Slash,
    /// assignment operator (=)
    Equal,
    /// equality operator (==)
    EqualEqual,
    /// inequality operator (!=)
    NotEqual,
    /// strict equality operator (===)
    EqualEqualEqual,
    /// strict inequality operator (!==)
    NotEqualEqual,
    /// less than operator (<)
    Less,
    /// greater than operator (>)
    Greater,
    /// less than or equal operator (<=)
    LessEqual,
    /// greater than or equal operator (>=)
    GreaterEqual,
    /// concatenation operator (++)
    PlusPlus,
    /// subtraction operator (--)
    MinusMinus,
    /// exponentiation operator (**)
    StarStar,
    /// exclamation mark (!)
    Exclamation,
    /// question mark (?)
    Question,
    /// ampersand (&)
    Ampersand,
    /// at symbol (@)
    At,
    /// caret (^)
    Caret,
    /// tilde (~)
    Tilde,
    /// left shift operator (<<)
    LeftShift,
    /// right shift operator (>>)
    RightShift,
    /// match operator (=~)
    MatchOp,
    /// pipe right operator (|>)
    PipeRight,

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Dot,          // .
    Colon,        // :
    Arrow,        // ->
    Pipe,         // |
    PipePipe,     // ||
    Hash,         // #

    // 特殊
    Error,
    Eof,

    // 语法节点类型 (非终结符)
    SourceFile,
    Module,
    Function,
    ParameterList,
    Parameter,
    BlockExpression,
    LetStatement,
    ExpressionStatement,
    IdentifierExpression,
    LiteralExpression,
    BooleanLiteral,
    ParenthesizedExpression,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    FieldExpression,
    IndexExpression,
    IfExpression,
    MatchExpression,
    LoopExpression,
    WhileExpression,
    ForExpression,
    BreakExpression,
    ContinueExpression,
    ReturnExpression,
    StructExpression,
    TupleExpression,
    ArrayExpression,
    RangeExpression,
    ClosureExpression,
    AsyncBlockExpression,
    UnsafeBlockExpression,
    TryExpression,
    AwaitExpression,
    MacroCall,
    Path,
    PathSegment,
    GenericArgs,
    TypePath,
    TupleType,
    ArrayType,
    SliceType,
    ReferenceType,
    PointerType,
    FunctionType,
    TraitObjectType,
    ImplTraitType,
    InferredType,
    NeverType,
    Pattern,
    IdentifierPattern,
    WildcardPattern,
    TuplePattern,
    StructPattern,
    TupleStructPattern,
    SlicePattern,
    ReferencePattern,
    LiteralPattern,
    RangePattern,
    OrPattern,
    RestPattern,
    StructDeclaration,
    EnumDeclaration,
    UnionDeclaration,
    TraitDeclaration,
    ImplDeclaration,
    ModuleDeclaration,
    UseDeclaration,
    ConstDeclaration,
    StaticDeclaration,
    TypeAliasDeclaration,
    ExternBlock,
    ExternFunction,
    Attribute,
    Visibility,
    GenericParams,
    GenericParam,
    TypeParam,
    ConstParam,
    LifetimeParam,
    WhereClause,
    WherePredicate,
    ReturnType,
    FieldList,
    Field,
    Variant,
    VariantList,
    AssociatedItem,
    TraitItem,
    ImplItem,
}

impl TokenType for ElixirSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier | Self::Variable | Self::Atom => UniversalTokenRole::Name,
            Self::Number | Self::Float | Self::String | Self::Character | Self::Sigil => UniversalTokenRole::Literal,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Equal
            | Self::EqualEqual
            | Self::NotEqual
            | Self::EqualEqualEqual
            | Self::NotEqualEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::StarStar
            | Self::Exclamation
            | Self::Question
            | Self::Ampersand
            | Self::At
            | Self::Caret
            | Self::Tilde
            | Self::LeftShift
            | Self::RightShift
            | Self::MatchOp
            | Self::PipeRight => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon | Self::Dot | Self::Colon | Self::Arrow | Self::Pipe | Self::PipePipe | Self::Hash => {
                UniversalTokenRole::Punctuation
            }
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for ElixirSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Module
            | Self::Function
            | Self::StructDeclaration
            | Self::EnumDeclaration
            | Self::UnionDeclaration
            | Self::TraitDeclaration
            | Self::ImplDeclaration
            | Self::ModuleDeclaration
            | Self::UseDeclaration
            | Self::ConstDeclaration
            | Self::StaticDeclaration
            | Self::TypeAliasDeclaration => UniversalElementRole::Definition,
            Self::BlockExpression | Self::AsyncBlockExpression | Self::UnsafeBlockExpression => UniversalElementRole::Container,
            Self::LetStatement | Self::ExpressionStatement => UniversalElementRole::Statement,
            Self::CallExpression | Self::MacroCall => UniversalElementRole::Call,
            Self::IdentifierExpression => UniversalElementRole::Reference,
            Self::LiteralExpression | Self::BooleanLiteral => UniversalElementRole::Value,
            Self::IfExpression | Self::MatchExpression | Self::LoopExpression | Self::WhileExpression | Self::ForExpression | Self::BreakExpression | Self::ContinueExpression | Self::ReturnExpression | Self::TryExpression | Self::AwaitExpression => {
                UniversalElementRole::Expression
            }
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}

impl ElixirSyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Def
                | Self::Defp
                | Self::Defmodule
                | Self::Defstruct
                | Self::Defprotocol
                | Self::Defimpl
                | Self::Defmacro
                | Self::Defmacrop
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::False
                | Self::Fn
                | Self::If
                | Self::In
                | Self::Not
                | Self::Or
                | Self::Receive
                | Self::Rescue
                | Self::True
                | Self::Try
                | Self::Unless
                | Self::When
                | Self::With
        )
    }
}
