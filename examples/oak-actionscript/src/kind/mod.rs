#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum ActionScriptSyntaxKind {
    // Basic tokens
    Whitespace,
    Newline,
    Comment,

    // Identifiers and literals
    Identifier,
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // ActionScript keywords
    As,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Delete,
    Do,
    Else,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Internal,
    Is,
    Native,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Use,
    Var,
    Void,
    While,
    With,

    // Contextual keywords
    Each,
    Get,
    Set,
    Namespace,
    Include,
    Dynamic,
    Final,
    Override,

    // Type keywords
    Array,
    Boolean,
    Date,
    Function_,
    Number,
    Object,
    RegExp,
    String_,
    Uint,
    Vector,
    Void_,
    Xml,
    XmlList,

    // Operators
    Plus,                     // +
    Minus,                    // -
    Star,                     // *
    Slash,                    // /
    Percent,                  // %
    Equal,                    // =
    EqualEqual,               // ==
    EqualEqualEqual,          // ===
    NotEqual,                 // !=
    NotEqualEqual,            // !==
    LessThan,                 // <
    LessEqual,                // <=
    GreaterThan,              // >
    GreaterEqual,             // >=
    LogicalAnd,               // &&
    LogicalOr,                // ||
    LogicalNot,               // !
    BitwiseAnd,               // &
    BitwiseOr,                // |
    BitwiseXor,               // ^
    BitwiseNot,               // ~
    LeftShift,                // <<
    RightShift,               // >>
    UnsignedRightShift,       // >>>
    Increment,                // ++
    Decrement,                // --
    PlusAssign,               // +=
    MinusAssign,              // -=
    StarAssign,               // *=
    SlashAssign,              // /=
    PercentAssign,            // %=
    LeftShiftAssign,          // <<=
    RightShiftAssign,         // >>=
    UnsignedRightShiftAssign, // >>>=
    BitwiseAndAssign,         // &=
    BitwiseOrAssign,          // |=
    BitwiseXorAssign,         // ^=
    Question,                 // ?
    Colon,                    // :

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Arrow,        // ->

    // Special characters
    At,          // @
    Hash,        // #
    Dollar,      // $
    Ampersand,   // &
    Backslash,   // \
    Quote,       // '
    DoubleQuote, // "
    Backtick,    // `

    // Composite nodes
    Program,
    Statement,
    Expression,
    Block,
    ClassDeclaration,
    InterfaceDeclaration,
    FunctionDeclaration,
    VariableDeclaration,
    ImportStatement,
    PackageDeclaration,
    NamespaceDeclaration,
    Assignment,
    FunctionCall,
    MethodCall,
    PropertyAccess,
    ArrayAccess,
    ConditionalExpression,
    BinaryExpression,
    UnaryExpression,
    IfStatement,
    ForStatement,
    WhileStatement,
    DoWhileStatement,
    SwitchStatement,
    TryStatement,
    ThrowStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,

    // Error and EOF
    Error,
    Eof,
}

impl oak_core::SyntaxKind for ActionScriptSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Program
                | Self::Statement
                | Self::Expression
                | Self::Block
                | Self::ClassDeclaration
                | Self::InterfaceDeclaration
                | Self::FunctionDeclaration
                | Self::VariableDeclaration
                | Self::ImportStatement
                | Self::PackageDeclaration
                | Self::NamespaceDeclaration
                | Self::Assignment
                | Self::FunctionCall
                | Self::MethodCall
                | Self::PropertyAccess
                | Self::ArrayAccess
                | Self::ConditionalExpression
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::IfStatement
                | Self::ForStatement
                | Self::WhileStatement
                | Self::DoWhileStatement
                | Self::SwitchStatement
                | Self::TryStatement
                | Self::ThrowStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Program
                | Self::Statement
                | Self::Expression
                | Self::Block
                | Self::ClassDeclaration
                | Self::InterfaceDeclaration
                | Self::FunctionDeclaration
                | Self::VariableDeclaration
                | Self::ImportStatement
                | Self::PackageDeclaration
                | Self::NamespaceDeclaration
                | Self::Assignment
                | Self::FunctionCall
                | Self::MethodCall
                | Self::PropertyAccess
                | Self::ArrayAccess
                | Self::ConditionalExpression
                | Self::BinaryExpression
                | Self::UnaryExpression
                | Self::IfStatement
                | Self::ForStatement
                | Self::WhileStatement
                | Self::DoWhileStatement
                | Self::SwitchStatement
                | Self::TryStatement
                | Self::ThrowStatement
                | Self::ReturnStatement
                | Self::BreakStatement
                | Self::ContinueStatement
        )
    }
}

pub type ActionScriptToken = oak_core::Token<ActionScriptSyntaxKind>;
