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
                match token {
            crate::lexer::token_type::ActionScriptTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::ActionScriptTokenType::Newline => Self::Newline,
            crate::lexer::token_type::ActionScriptTokenType::Comment => Self::Comment,
            crate::lexer::token_type::ActionScriptTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::ActionScriptTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::ActionScriptTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::ActionScriptTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::ActionScriptTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::ActionScriptTokenType::NullLiteral => Self::NullLiteral,
            crate::lexer::token_type::ActionScriptTokenType::As => Self::As,
            crate::lexer::token_type::ActionScriptTokenType::Break => Self::Break,
            crate::lexer::token_type::ActionScriptTokenType::Case => Self::Case,
            crate::lexer::token_type::ActionScriptTokenType::Catch => Self::Catch,
            crate::lexer::token_type::ActionScriptTokenType::Class => Self::Class,
            crate::lexer::token_type::ActionScriptTokenType::Const => Self::Const,
            crate::lexer::token_type::ActionScriptTokenType::Continue => Self::Continue,
            crate::lexer::token_type::ActionScriptTokenType::Default => Self::Default,
            crate::lexer::token_type::ActionScriptTokenType::Delete => Self::Delete,
            crate::lexer::token_type::ActionScriptTokenType::Do => Self::Do,
            crate::lexer::token_type::ActionScriptTokenType::Else => Self::Else,
            crate::lexer::token_type::ActionScriptTokenType::Extends => Self::Extends,
            crate::lexer::token_type::ActionScriptTokenType::False => Self::False,
            crate::lexer::token_type::ActionScriptTokenType::Finally => Self::Finally,
            crate::lexer::token_type::ActionScriptTokenType::For => Self::For,
            crate::lexer::token_type::ActionScriptTokenType::Function => Self::Function,
            crate::lexer::token_type::ActionScriptTokenType::If => Self::If,
            crate::lexer::token_type::ActionScriptTokenType::Implements => Self::Implements,
            crate::lexer::token_type::ActionScriptTokenType::Import => Self::Import,
            crate::lexer::token_type::ActionScriptTokenType::In => Self::In,
            crate::lexer::token_type::ActionScriptTokenType::Instanceof => Self::Instanceof,
            crate::lexer::token_type::ActionScriptTokenType::Interface => Self::Interface,
            crate::lexer::token_type::ActionScriptTokenType::Internal => Self::Internal,
            crate::lexer::token_type::ActionScriptTokenType::Is => Self::Is,
            crate::lexer::token_type::ActionScriptTokenType::Native => Self::Native,
            crate::lexer::token_type::ActionScriptTokenType::New => Self::New,
            crate::lexer::token_type::ActionScriptTokenType::Null => Self::Null,
            crate::lexer::token_type::ActionScriptTokenType::Package => Self::Package,
            crate::lexer::token_type::ActionScriptTokenType::Private => Self::Private,
            crate::lexer::token_type::ActionScriptTokenType::Protected => Self::Protected,
            crate::lexer::token_type::ActionScriptTokenType::Public => Self::Public,
            crate::lexer::token_type::ActionScriptTokenType::Return => Self::Return,
            crate::lexer::token_type::ActionScriptTokenType::Static => Self::Static,
            crate::lexer::token_type::ActionScriptTokenType::Super => Self::Super,
            crate::lexer::token_type::ActionScriptTokenType::Switch => Self::Switch,
            crate::lexer::token_type::ActionScriptTokenType::This => Self::This,
            crate::lexer::token_type::ActionScriptTokenType::Throw => Self::Throw,
            crate::lexer::token_type::ActionScriptTokenType::True => Self::True,
            crate::lexer::token_type::ActionScriptTokenType::Try => Self::Try,
            crate::lexer::token_type::ActionScriptTokenType::Typeof => Self::Typeof,
            crate::lexer::token_type::ActionScriptTokenType::Use => Self::Use,
            crate::lexer::token_type::ActionScriptTokenType::Var => Self::Var,
            crate::lexer::token_type::ActionScriptTokenType::Void => Self::Void,
            crate::lexer::token_type::ActionScriptTokenType::While => Self::While,
            crate::lexer::token_type::ActionScriptTokenType::With => Self::With,
            crate::lexer::token_type::ActionScriptTokenType::Each => Self::Each,
            crate::lexer::token_type::ActionScriptTokenType::Get => Self::Get,
            crate::lexer::token_type::ActionScriptTokenType::Set => Self::Set,
            crate::lexer::token_type::ActionScriptTokenType::Namespace => Self::Namespace,
            crate::lexer::token_type::ActionScriptTokenType::Include => Self::Include,
            crate::lexer::token_type::ActionScriptTokenType::Dynamic => Self::Dynamic,
            crate::lexer::token_type::ActionScriptTokenType::Final => Self::Final,
            crate::lexer::token_type::ActionScriptTokenType::Override => Self::Override,
            crate::lexer::token_type::ActionScriptTokenType::Array => Self::Array,
            crate::lexer::token_type::ActionScriptTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::ActionScriptTokenType::Date => Self::Date,
            crate::lexer::token_type::ActionScriptTokenType::Number => Self::Number,
            crate::lexer::token_type::ActionScriptTokenType::ObjectType => Self::ObjectType,
            crate::lexer::token_type::ActionScriptTokenType::RegExp => Self::RegExp,
            crate::lexer::token_type::ActionScriptTokenType::StringType => Self::StringType,
            crate::lexer::token_type::ActionScriptTokenType::Uint => Self::Uint,
            crate::lexer::token_type::ActionScriptTokenType::Vector => Self::Vector,
            crate::lexer::token_type::ActionScriptTokenType::VoidType => Self::VoidType,
            crate::lexer::token_type::ActionScriptTokenType::Xml => Self::Xml,
            crate::lexer::token_type::ActionScriptTokenType::XmlList => Self::XmlList,
            crate::lexer::token_type::ActionScriptTokenType::Plus => Self::Plus,
            crate::lexer::token_type::ActionScriptTokenType::Minus => Self::Minus,
            crate::lexer::token_type::ActionScriptTokenType::Star => Self::Star,
            crate::lexer::token_type::ActionScriptTokenType::Slash => Self::Slash,
            crate::lexer::token_type::ActionScriptTokenType::Percent => Self::Percent,
            crate::lexer::token_type::ActionScriptTokenType::Equal => Self::Equal,
            crate::lexer::token_type::ActionScriptTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::ActionScriptTokenType::EqualEqualEqual => Self::EqualEqualEqual,
            crate::lexer::token_type::ActionScriptTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::ActionScriptTokenType::NotEqualEqual => Self::NotEqualEqual,
            crate::lexer::token_type::ActionScriptTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::ActionScriptTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::ActionScriptTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::ActionScriptTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::ActionScriptTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::ActionScriptTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::ActionScriptTokenType::LogicalNot => Self::LogicalNot,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::ActionScriptTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::ActionScriptTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::ActionScriptTokenType::UnsignedRightShift => Self::UnsignedRightShift,
            crate::lexer::token_type::ActionScriptTokenType::Increment => Self::Increment,
            crate::lexer::token_type::ActionScriptTokenType::Decrement => Self::Decrement,
            crate::lexer::token_type::ActionScriptTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::ActionScriptTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::ActionScriptTokenType::StarAssign => Self::StarAssign,
            crate::lexer::token_type::ActionScriptTokenType::SlashAssign => Self::SlashAssign,
            crate::lexer::token_type::ActionScriptTokenType::PercentAssign => Self::PercentAssign,
            crate::lexer::token_type::ActionScriptTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::ActionScriptTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::ActionScriptTokenType::UnsignedRightShiftAssign => Self::UnsignedRightShiftAssign,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseAndAssign => Self::BitwiseAndAssign,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseOrAssign => Self::BitwiseOrAssign,
            crate::lexer::token_type::ActionScriptTokenType::BitwiseXorAssign => Self::BitwiseXorAssign,
            crate::lexer::token_type::ActionScriptTokenType::Question => Self::Question,
            crate::lexer::token_type::ActionScriptTokenType::Colon => Self::Colon,
            crate::lexer::token_type::ActionScriptTokenType::Dot => Self::Dot,
            crate::lexer::token_type::ActionScriptTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::ActionScriptTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::ActionScriptTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::ActionScriptTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::ActionScriptTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::ActionScriptTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::ActionScriptTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::ActionScriptTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::ActionScriptTokenType::Comma => Self::Comma,
            crate::lexer::token_type::ActionScriptTokenType::At => Self::At,
            crate::lexer::token_type::ActionScriptTokenType::Hash => Self::Hash,
            crate::lexer::token_type::ActionScriptTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::ActionScriptTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::ActionScriptTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::ActionScriptTokenType::Quote => Self::Quote,
            crate::lexer::token_type::ActionScriptTokenType::DoubleQuote => Self::DoubleQuote,
            crate::lexer::token_type::ActionScriptTokenType::Backtick => Self::Backtick,
            crate::lexer::token_type::ActionScriptTokenType::Eof => Self::Eof,
            crate::lexer::token_type::ActionScriptTokenType::Program => Self::Program,
            crate::lexer::token_type::ActionScriptTokenType::Block => Self::Block,
            crate::lexer::token_type::ActionScriptTokenType::Variable => Self::Variable,
            crate::lexer::token_type::ActionScriptTokenType::FunctionCall => Self::FunctionCall,
            crate::lexer::token_type::ActionScriptTokenType::MethodCall => Self::MethodCall,
            crate::lexer::token_type::ActionScriptTokenType::PropertyAccess => Self::PropertyAccess,
            crate::lexer::token_type::ActionScriptTokenType::ArrayAccess => Self::ArrayAccess,
            crate::lexer::token_type::ActionScriptTokenType::ParameterList => Self::ParameterList,
            crate::lexer::token_type::ActionScriptTokenType::UseItem => Self::UseItem,
            crate::lexer::token_type::ActionScriptTokenType::ModuleItem => Self::ModuleItem,
            crate::lexer::token_type::ActionScriptTokenType::StructItem => Self::StructItem,
            crate::lexer::token_type::ActionScriptTokenType::EnumItem => Self::EnumItem,
            crate::lexer::token_type::ActionScriptTokenType::FunctionType => Self::FunctionType,
            crate::lexer::token_type::ActionScriptTokenType::Root => Self::Root,
            crate::lexer::token_type::ActionScriptTokenType::Statement => Self::Statement,
            crate::lexer::token_type::ActionScriptTokenType::Expression => Self::Expression,
            crate::lexer::token_type::ActionScriptTokenType::Assignment => Self::Assignment,
            crate::lexer::token_type::ActionScriptTokenType::ConditionalExpression => Self::ConditionalExpression,
            crate::lexer::token_type::ActionScriptTokenType::BinaryExpression => Self::BinaryExpression,
            crate::lexer::token_type::ActionScriptTokenType::UnaryExpression => Self::UnaryExpression,
            crate::lexer::token_type::ActionScriptTokenType::IfStatement => Self::IfStatement,
            crate::lexer::token_type::ActionScriptTokenType::ForStatement => Self::ForStatement,
            crate::lexer::token_type::ActionScriptTokenType::WhileStatement => Self::WhileStatement,
            crate::lexer::token_type::ActionScriptTokenType::DoWhileStatement => Self::DoWhileStatement,
            crate::lexer::token_type::ActionScriptTokenType::SwitchStatement => Self::SwitchStatement,
            crate::lexer::token_type::ActionScriptTokenType::TryStatement => Self::TryStatement,
            crate::lexer::token_type::ActionScriptTokenType::ThrowStatement => Self::ThrowStatement,
            crate::lexer::token_type::ActionScriptTokenType::ReturnStatement => Self::ReturnStatement,
            crate::lexer::token_type::ActionScriptTokenType::BreakStatement => Self::BreakStatement,
            crate::lexer::token_type::ActionScriptTokenType::ContinueStatement => Self::ContinueStatement,
            crate::lexer::token_type::ActionScriptTokenType::Error => Self::Error,
            crate::lexer::token_type::ActionScriptTokenType::LiteralExpression => Self::LiteralExpression,
            crate::lexer::token_type::ActionScriptTokenType::IdentifierExpression => Self::IdentifierExpression,
            crate::lexer::token_type::ActionScriptTokenType::ParenthesizedExpression => Self::ParenthesizedExpression,
            crate::lexer::token_type::ActionScriptTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::ActionScriptTokenType::BlockExpression => Self::BlockExpression,
            crate::lexer::token_type::ActionScriptTokenType::LetStatement => Self::LetStatement,
            crate::lexer::token_type::ActionScriptTokenType::IfExpression => Self::IfExpression,
            crate::lexer::token_type::ActionScriptTokenType::WhileExpression => Self::WhileExpression,
            crate::lexer::token_type::ActionScriptTokenType::LoopExpression => Self::LoopExpression,
            crate::lexer::token_type::ActionScriptTokenType::ForExpression => Self::ForExpression,
            crate::lexer::token_type::ActionScriptTokenType::CallExpression => Self::CallExpression,
            crate::lexer::token_type::ActionScriptTokenType::IndexExpression => Self::IndexExpression,
            crate::lexer::token_type::ActionScriptTokenType::FieldExpression => Self::FieldExpression,
            _ => Self::Error,
        }
    }
}
