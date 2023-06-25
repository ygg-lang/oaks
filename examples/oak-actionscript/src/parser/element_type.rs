use oak_core::{ElementType, UniversalElementRole};

/// Element types for ActionScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionScriptElementType {
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Character literal.
    CharLiteral,
    /// Number literal.
    NumberLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Null literal.
    NullLiteral,
    /// `as` keyword.
    As,
    /// `break` keyword.
    Break,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `class` keyword.
    Class,
    /// `const` keyword.
    Const,
    /// `continue` keyword.
    Continue,
    /// `default` keyword.
    Default,
    /// `delete` keyword.
    Delete,
    /// `do` keyword.
    Do,
    /// `else` keyword.
    Else,
    /// `extends` keyword.
    Extends,
    /// `false` keyword.
    False,
    /// `finally` keyword.
    Finally,
    /// `for` keyword.
    For,
    /// `function` keyword.
    Function,
    /// `if` keyword.
    If,
    /// `implements` keyword.
    Implements,
    /// `import` keyword.
    Import,
    /// `in` keyword.
    In,
    /// `instanceof` keyword.
    Instanceof,
    /// `interface` keyword.
    Interface,
    /// `internal` keyword.
    Internal,
    /// `is` keyword.
    Is,
    /// `native` keyword.
    Native,
    /// `new` keyword.
    New,
    /// `null` keyword.
    Null,
    /// `package` keyword.
    Package,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `public` keyword.
    Public,
    /// `return` keyword.
    Return,
    /// `static` keyword.
    Static,
    /// `super` keyword.
    Super,
    /// `switch` keyword.
    Switch,
    /// `this` keyword.
    This,
    /// `throw` keyword.
    Throw,
    /// `true` keyword.
    True,
    /// `try` keyword.
    Try,
    /// `typeof` keyword.
    Typeof,
    /// `use` keyword.
    Use,
    /// `var` keyword.
    Var,
    /// `void` keyword.
    Void,
    /// `while` keyword.
    While,
    /// `with` keyword.
    With,
    /// `each` keyword.
    Each,
    /// `get` keyword.
    Get,
    /// `set` keyword.
    Set,
    /// `namespace` keyword.
    Namespace,
    /// `include` keyword.
    Include,
    /// `dynamic` keyword.
    Dynamic,
    /// `final` keyword.
    Final,
    /// `override` keyword.
    Override,
    /// `Array` type.
    Array,
    /// `Boolean` type.
    Boolean,
    /// `Date` type.
    Date,
    /// `Number` type.
    Number,
    /// `Object` type.
    ObjectType,
    /// `RegExp` type.
    RegExp,
    /// `String` type.
    StringType,
    /// `uint` type.
    Uint,
    /// `Vector` type.
    Vector,
    /// `void` type.
    VoidType,
    /// `XML` type.
    Xml,
    /// `XMLList` type.
    XmlList,
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `%`.
    Percent,
    /// `=`.
    Equal,
    /// `==`.
    EqualEqual,
    /// `===`.
    EqualEqualEqual,
    /// `!=`.
    NotEqual,
    /// `!==`.
    NotEqualEqual,
    /// `<`.
    LessThan,
    /// `<=`.
    LessEqual,
    /// `>`.
    GreaterThan,
    /// `>=`.
    GreaterEqual,
    /// `&&`.
    LogicalAnd,
    /// `||`.
    LogicalOr,
    /// `!`.
    LogicalNot,
    /// `&`.
    BitwiseAnd,
    /// `|`.
    BitwiseOr,
    /// `^`.
    BitwiseXor,
    /// `~`.
    BitwiseNot,
    /// `<<`.
    LeftShift,
    /// `>>`.
    RightShift,
    /// `>>>`.
    UnsignedRightShift,
    /// `++`.
    Increment,
    /// `--`.
    Decrement,
    /// `+=`.
    PlusAssign,
    /// `-=`.
    MinusAssign,
    /// `*=`.
    StarAssign,
    /// `/=`.
    SlashAssign,
    /// `%=`.
    PercentAssign,
    /// `<<=`.
    LeftShiftAssign,
    /// `>>=`.
    RightShiftAssign,
    /// `>>>=`.
    UnsignedRightShiftAssign,
    /// `&=`.
    BitwiseAndAssign,
    /// `|=`.
    BitwiseOrAssign,
    /// `^=`.
    BitwiseXorAssign,
    /// `?`.
    Question,
    /// `:`.
    Colon,
    /// `.`.
    Dot,
    /// `->`.
    Arrow,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `;`.
    Semicolon,
    /// `,`.
    Comma,
    /// `@`.
    At,
    /// `#`.
    Hash,
    /// `$`.
    Dollar,
    /// `&`.
    Ampersand,
    /// `\`.
    Backslash,
    /// `'`.
    Quote,
    /// `"`.
    DoubleQuote,
    /// `` ` ``.
    Backtick,
    /// End of file.
    Eof,
    /// A program.
    Program,
    /// A block.
    Block,
    /// A variable declaration.
    Variable,
    /// A function call.
    FunctionCall,
    /// A method call.
    MethodCall,
    /// A property access.
    PropertyAccess,
    /// An array access.
    ArrayAccess,
    /// A parameter list.
    ParameterList,
    /// A use item.
    UseItem,
    /// A module item.
    ModuleItem,
    /// A struct item.
    StructItem,
    /// An enum item.
    EnumItem,
    /// A function type.
    FunctionType,
    /// The root node.
    Root,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// An assignment expression.
    Assignment,
    /// A conditional expression.
    ConditionalExpression,
    /// A binary expression.
    BinaryExpression,
    /// A unary expression.
    UnaryExpression,
    /// An if statement.
    IfStatement,
    /// A for statement.
    ForStatement,
    /// A while statement.
    WhileStatement,
    /// A do-while statement.
    DoWhileStatement,
    /// A switch statement.
    SwitchStatement,
    /// A try statement.
    TryStatement,
    /// A throw statement.
    ThrowStatement,
    /// A return statement.
    ReturnStatement,
    /// A break statement.
    BreakStatement,
    /// A continue statement.
    ContinueStatement,
    /// An error node.
    Error,
    /// A literal expression.
    LiteralExpression,
    /// An identifier expression.
    IdentifierExpression,
    /// A parenthesized expression.
    ParenthesizedExpression,
    /// A source file.
    SourceFile,
    /// A block expression.
    BlockExpression,
    /// A let statement.
    LetStatement,
    /// An if expression.
    IfExpression,
    /// A while expression.
    WhileExpression,
    /// A loop expression.
    LoopExpression,
    /// A for expression.
    ForExpression,
    /// A call expression.
    CallExpression,
    /// An index expression.
    IndexExpression,
    /// A field expression.
    FieldExpression,
}

impl ElementType for ActionScriptElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ActionScriptTokenType> for ActionScriptElementType {
    fn from(token: crate::lexer::token_type::ActionScriptTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
