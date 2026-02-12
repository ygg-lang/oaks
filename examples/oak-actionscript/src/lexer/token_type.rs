use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the ActionScript language.
pub type ActionScriptToken = Token<ActionScriptTokenType>;

/// Token types for the ActionScript language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ActionScriptTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comments.
    Comment,
    /// An identifier.
    Identifier,
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A number literal.
    NumberLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// A null literal.
    NullLiteral,

    // Keywords
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

    // Types
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

    // Operators
    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
    Percent,
    /// `=` operator.
    Equal,
    /// `==` operator.
    EqualEqual,
    /// `===` operator.
    EqualEqualEqual,
    /// `!=` operator.
    NotEqual,
    /// `!==` operator.
    NotEqualEqual,
    /// `<` operator.
    LessThan,
    /// `<=` operator.
    LessEqual,
    /// `>` operator.
    GreaterThan,
    /// `>=` operator.
    GreaterEqual,
    /// `&&` operator.
    LogicalAnd,
    /// `||` operator.
    LogicalOr,
    /// `!` operator.
    LogicalNot,
    /// `&` operator.
    BitwiseAnd,
    /// `|` operator.
    BitwiseOr,
    /// `^` operator.
    BitwiseXor,
    /// `~` operator.
    BitwiseNot,
    /// `<<` operator.
    LeftShift,
    /// `>>` operator.
    RightShift,
    /// `>>>` operator.
    UnsignedRightShift,
    /// `++` operator.
    Increment,
    /// `--` operator.
    Decrement,
    /// `+=` operator.
    PlusAssign,
    /// `-=` operator.
    MinusAssign,
    /// `*=` operator.
    StarAssign,
    /// `/=` operator.
    SlashAssign,
    /// `%=` operator.
    PercentAssign,
    /// `<<=` operator.
    LeftShiftAssign,
    /// `>>=` operator.
    RightShiftAssign,
    /// `>>>=` operator.
    UnsignedRightShiftAssign,
    /// `&=` operator.
    BitwiseAndAssign,
    /// `|=` operator.
    BitwiseOrAssign,
    /// `^=` operator.
    BitwiseXorAssign,

    // Punctuation
    /// `?` punctuation.
    Question,
    /// `:` punctuation.
    Colon,
    /// `.` punctuation.
    Dot,
    /// `->` operator.
    Arrow,
    /// `(` punctuation.
    LeftParen,
    /// `)` punctuation.
    RightParen,
    /// `{` punctuation.
    LeftBrace,
    /// `}` punctuation.
    RightBrace,
    /// `[` punctuation.
    LeftBracket,
    /// `]` punctuation.
    RightBracket,
    /// `;` punctuation.
    Semicolon,
    /// `,` punctuation.
    Comma,
    /// `@` punctuation.
    At,
    /// `#` punctuation.
    Hash,
    /// `$` punctuation.
    Dollar,
    /// `&` punctuation.
    Ampersand,
    /// `\` punctuation.
    Backslash,
    /// `'` punctuation.
    Quote,
    /// `"` punctuation.
    DoubleQuote,
    /// `` ` `` punctuation.
    Backtick,

    /// End of file token.
    Eof,

    // Element types (used for elements)
    /// A program.
    Program,
    /// A block.
    Block,
    /// A variable.
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
    /// Root element.
    Root,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// An assignment.
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
    /// Error token.
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

impl TokenType for ActionScriptTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            t if t.is_keyword() => UniversalTokenRole::Keyword,
            t if t.is_operator() => UniversalTokenRole::Operator,
            t if t.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Identifier => UniversalTokenRole::Name,
            t if t.is_literal() => UniversalTokenRole::Literal,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ActionScriptTokenType {
    /// Returns true if the token type is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral | Self::True | Self::False | Self::Null)
    }

    /// Returns true if the token type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::As
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Delete
                | Self::Do
                | Self::Else
                | Self::Extends
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Instanceof
                | Self::Interface
                | Self::Internal
                | Self::Is
                | Self::Native
                | Self::New
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::Try
                | Self::Typeof
                | Self::Use
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Each
                | Self::Get
                | Self::Set
                | Self::Namespace
                | Self::Include
                | Self::Dynamic
                | Self::Final
                | Self::Override
        )
    }

    /// Returns true if the token type is an operator.
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Equal
                | Self::EqualEqual
                | Self::EqualEqualEqual
                | Self::NotEqual
                | Self::NotEqualEqual
                | Self::LessThan
                | Self::LessEqual
                | Self::GreaterThan
                | Self::GreaterEqual
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::LogicalNot
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::UnsignedRightShift
                | Self::PlusAssign
                | Self::MinusAssign
                | Self::StarAssign
                | Self::SlashAssign
                | Self::PercentAssign
                | Self::LeftShiftAssign
                | Self::RightShiftAssign
                | Self::UnsignedRightShiftAssign
                | Self::BitwiseAndAssign
                | Self::BitwiseOrAssign
                | Self::BitwiseXorAssign
                | Self::Question
        )
    }

    /// Returns true if the token type is a punctuation.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Dot | Self::Comma | Self::Colon | Self::Semicolon)
    }
}
