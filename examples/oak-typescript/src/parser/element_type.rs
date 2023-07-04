use oak_core::{ElementType, UniversalElementRole};

/// Element types for TypeScript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum TypeScriptElementType {
    /// Named imports.
    NamedImports,
    /// A decorator.
    Decorator,
    /// An arrow function.
    ArrowFunction,
    /// A predefined type.
    PredefinedType,
    /// `abstract` keyword.
    Abstract,
    /// `any` keyword.
    Any,
    /// `as` keyword.
    As,
    /// `asserts` keyword.
    Asserts,
    /// `async` keyword.
    Async,
    /// `await` keyword.
    Await,
    /// `boolean` keyword.
    Boolean,
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
    /// `constructor` keyword.
    Constructor,
    /// `continue` keyword.
    Continue,
    /// `debugger` keyword.
    Debugger,
    /// `declare` keyword.
    Declare,
    /// `default` keyword.
    Default,
    /// `delete` keyword.
    Delete,
    /// `do` keyword.
    Do,
    /// `else` keyword.
    Else,
    /// `enum` keyword.
    Enum,
    /// `export` keyword.
    Export,
    /// `extends` keyword.
    Extends,
    /// `false` keyword.
    False,
    /// `finally` keyword.
    Finally,
    /// `for` keyword.
    For,
    /// `from` keyword.
    From,
    /// `function` keyword.
    Function,
    /// `get` keyword.
    Get,
    /// `global` keyword.
    Global,
    /// `if` keyword.
    If,
    /// `implements` keyword.
    Implements,
    /// `import` keyword.
    Import,
    /// `in` keyword.
    In,
    /// `infer` keyword.
    Infer,
    /// `instanceof` keyword.
    Instanceof,
    /// `interface` keyword.
    Interface,
    /// `is` keyword.
    Is,
    /// `keyof` keyword.
    Keyof,
    /// `let` keyword.
    Let,
    /// `namespace` keyword.
    Namespace,
    /// `never` keyword.
    Never,
    /// `new` keyword.
    New,
    /// `null` keyword.
    Null,
    /// `number` keyword.
    Number,
    /// `object` keyword.
    Object,
    /// `of` keyword.
    Of,
    /// `override` keyword.
    Override,
    /// `package` keyword.
    Package,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `public` keyword.
    Public,
    /// `readonly` keyword.
    Readonly,
    /// `require` keyword.
    Require,
    /// `return` keyword.
    Return,
    /// `set` keyword.
    Set,
    /// `static` keyword.
    Static,
    /// `string` keyword.
    String,
    /// `super` keyword.
    Super,
    /// `switch` keyword.
    Switch,
    /// `symbol` keyword.
    Symbol,
    /// `this` keyword.
    This,
    /// `throw` keyword.
    Throw,
    /// `true` keyword.
    True,
    /// `try` keyword.
    Try,
    /// `type` keyword.
    Type,
    /// `typeof` keyword.
    Typeof,
    /// `undefined` keyword.
    Undefined,
    /// `unique` keyword.
    Unique,
    /// `unknown` keyword.
    Unknown,
    /// `var` keyword.
    Var,
    /// `void` keyword.
    Void,
    /// `while` keyword.
    While,
    /// `with` keyword.
    With,
    /// `yield` keyword.
    Yield,
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
    /// `**`.
    StarStar,
    /// `?`.
    Question,
    /// `...`.
    DotDotDot,
    /// `<`.
    Less,
    /// `>`.
    Greater,
    /// `<=`.
    LessEqual,
    /// `>=`.
    GreaterEqual,
    /// `==`.
    EqualEqual,
    /// `!=`.
    NotEqual,
    /// `===`.
    EqualEqualEqual,
    /// `!==`.
    NotEqualEqual,
    /// `&&`.
    AndAnd,
    /// `||`.
    OrOr,
    /// `!`.
    Exclamation,
    /// `~`.
    Tilde,
    /// `&`.
    Ampersand,
    /// `|`.
    Bar,
    /// `^`.
    Caret,
    /// `<<`.
    LessLess,
    /// `>>`.
    GreaterGreater,
    /// `>>>`.
    GreaterGreaterGreater,
    /// `??`.
    QuestionQuestion,
    /// `?.`.
    QuestionDot,
    /// `=`.
    Equal,
    /// `+=`.
    PlusEqual,
    /// `-=`.
    MinusEqual,
    /// `*=`.
    StarEqual,
    /// `/=`.
    SlashEqual,
    /// `%=`.
    PercentEqual,
    /// `**=`.
    StarStarEqual,
    /// `&=`.
    AmpersandEqual,
    /// `|=`.
    BarEqual,
    /// `^=`.
    CaretEqual,
    /// `<<=`.
    LessLessEqual,
    /// `>>=`.
    GreaterGreaterEqual,
    /// `>>>=`.
    GreaterGreaterGreaterEqual,
    /// `??=`.
    QuestionQuestionEqual,
    /// `++`.
    PlusPlus,
    /// `--`.
    MinusMinus,
    /// `(`.
    OpenParen,
    /// `)`.
    CloseParen,
    /// `[`.
    OpenBracket,
    /// `]`.
    CloseBracket,
    /// `{`.
    OpenBrace,
    /// `}`.
    CloseBrace,
    /// `,`.
    Comma,
    /// `.`.
    Dot,
    /// `;`.
    Semicolon,
    /// `:`.
    Colon,
    /// `@`.
    At,
    /// `=>`.
    EqualsGreater,
    /// String literal.
    StringLiteral,
    /// Numeric literal.
    NumericLiteral,
    /// BigInt literal.
    BigIntLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// Template string.
    TemplateString,
    /// Regular expression literal.
    RegexLiteral,
    /// Identifier name.
    IdentifierName,
    /// Line comment `//`.
    LineComment,
    /// Block comment `/* */`.
    BlockComment,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// End of stream.
    Eof,
    /// Root node.
    Root,
    /// Source file.
    SourceFile,
    /// Module.
    Module,
    /// Variable declaration.
    VariableDeclaration,
    /// Function declaration.
    FunctionDeclaration,
    /// Class declaration.
    ClassDeclaration,
    /// Interface declaration.
    InterfaceDeclaration,
    /// Type alias declaration.
    TypeAliasDeclaration,
    /// Enum declaration.
    EnumDeclaration,
    /// Namespace declaration.
    NamespaceDeclaration,
    /// Class body.
    ClassBody,
    /// Import declaration.
    ImportDeclaration,
    /// Export declaration.
    ExportDeclaration,
    /// Import clause.
    ImportClause,
    /// Import specifier.
    ImportSpecifier,
    /// Namespace import.
    NamespaceImport,
    /// Named exports.
    NamedExports,
    /// Export specifier.
    ExportSpecifier,
    /// Parameter.
    Parameter,
    /// Call argument.
    CallArgument,
    /// Property declaration.
    PropertyDeclaration,
    /// Method declaration.
    MethodDeclaration,
    /// Constructor declaration.
    ConstructorDeclaration,
    /// Property assignment.
    PropertyAssignment,
    /// Shorthand property assignment.
    ShorthandPropertyAssignment,
    /// Spread element.
    SpreadElement,
    /// Error token.
    Error,
    /// JSX element.
    JsxElement,
    /// JSX self-closing element.
    JsxSelfClosingElement,
    /// JSX opening element.
    JsxOpeningElement,
    /// JSX closing element.
    JsxClosingElement,
    /// JSX fragment.
    JsxFragment,
    /// JSX opening fragment.
    JsxOpeningFragment,
    /// JSX closing fragment.
    JsxClosingFragment,
    /// JSX attribute.
    JsxAttribute,
    /// JSX attributes.
    JsxAttributes,
    /// JSX expression container.
    JsxExpressionContainer,
    /// JSX spread attribute.
    JsxSpreadAttribute,
    /// JSX text.
    JsxText,
    /// Binary expression.
    BinaryExpression,
    /// Unary expression.
    UnaryExpression,
    /// Conditional expression `a ? b : c`.
    ConditionalExpression,
    /// Call expression `f()`.
    CallExpression,
    /// New expression `new C()`.
    NewExpression,
    /// Member expression `a.b` or `a[b]`.
    MemberExpression,
    /// Array expression `[a, b]`.
    ArrayExpression,
    /// Object expression `{a: b}`.
    ObjectExpression,
    /// Function expression `function() {}`.
    FunctionExpression,
    /// Template expression `` `...` ``.
    TemplateExpression,
    /// Tagged template expression `f` `...` ``.
    TaggedTemplateExpression,
    /// As expression `a as T`.
    AsExpression,
    /// Type assertion expression `<T>a`.
    TypeAssertionExpression,
    /// Non-null expression `a!`.
    NonNullExpression,
    /// Update expression `++a` or `a--`.
    UpdateExpression,
    /// Expression statement.
    ExpressionStatement,
    /// Block statement.
    BlockStatement,
    /// If statement.
    IfStatement,
    /// While statement.
    WhileStatement,
    /// For statement.
    ForStatement,
    /// For-in statement.
    ForInStatement,
    /// For-of statement.
    ForOfStatement,
    /// Do-while statement.
    DoWhileStatement,
    /// Switch statement.
    SwitchStatement,
    /// Case clause.
    CaseClause,
    /// Default clause.
    DefaultClause,
    /// Try statement.
    TryStatement,
    /// Catch clause.
    CatchClause,
    /// Finally clause.
    FinallyClause,
    /// Throw statement.
    ThrowStatement,
    /// Return statement.
    ReturnStatement,
    /// Break statement.
    BreakStatement,
    /// Continue statement.
    ContinueStatement,
    /// Debugger statement.
    DebuggerStatement,
    /// With statement.
    WithStatement,
    /// Binding pattern.
    BindingPattern,
    /// Array binding pattern.
    ArrayBindingPattern,
    /// Object binding pattern.
    ObjectBindingPattern,
    /// Binding element.
    BindingElement,
    /// Type reference.
    TypeReference,
    /// Type literal.
    TypeLiteral,
    /// Function type.
    FunctionType,
    /// Constructor type.
    ConstructorType,
    /// Array type.
    ArrayType,
    /// Tuple type.
    TupleType,
    /// Union type.
    UnionType,
    /// Intersection type.
    IntersectionType,
    /// Conditional type.
    ConditionalType,
    /// Mapped type.
    MappedType,
    /// Indexed access type.
    IndexedAccessType,
    /// Property signature.
    PropertySignature,
    /// Method signature.
    MethodSignature,
    /// Literal type.
    LiteralType,
    /// Type query.
    TypeQuery,
    /// Type predicate.
    TypePredicate,
    /// Type annotation.
    TypeAnnotation,
    /// Type parameter.
    TypeParameter,
    /// Heritage clause.
    HeritageClause,
    /// Enum member.
    EnumMember,
}

impl TypeScriptElementType {
    /// Returns the element type for the given keyword.
    pub fn from_keyword(text: &str) -> Option<Self> {
        match text {
            "abstract" => Some(Self::Abstract),
            "any" => Some(Self::Any),
            "as" => Some(Self::As),
            "asserts" => Some(Self::Asserts),
            "async" => Some(Self::Async),
            "await" => Some(Self::Await),
            "boolean" => Some(Self::Boolean),
            "break" => Some(Self::Break),
            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "constructor" => Some(Self::Constructor),
            "continue" => Some(Self::Continue),
            "debugger" => Some(Self::Debugger),
            "declare" => Some(Self::Declare),
            "default" => Some(Self::Default),
            "delete" => Some(Self::Delete),
            "do" => Some(Self::Do),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "export" => Some(Self::Export),
            "extends" => Some(Self::Extends),
            "false" => Some(Self::False),
            "finally" => Some(Self::Finally),
            "for" => Some(Self::For),
            "from" => Some(Self::From),
            "function" => Some(Self::Function),
            "get" => Some(Self::Get),
            "global" => Some(Self::Global),
            "if" => Some(Self::If),
            "implements" => Some(Self::Implements),
            "import" => Some(Self::Import),
            "in" => Some(Self::In),
            "infer" => Some(Self::Infer),
            "instanceof" => Some(Self::Instanceof),
            "interface" => Some(Self::Interface),
            "is" => Some(Self::Is),
            "keyof" => Some(Self::Keyof),
            "let" => Some(Self::Let),
            "namespace" => Some(Self::Namespace),
            "never" => Some(Self::Never),
            "new" => Some(Self::New),
            "null" => Some(Self::Null),
            "number" => Some(Self::Number),
            "object" => Some(Self::Object),
            "of" => Some(Self::Of),
            "override" => Some(Self::Override),
            "package" => Some(Self::Package),
            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),
            "readonly" => Some(Self::Readonly),
            "require" => Some(Self::Require),
            "return" => Some(Self::Return),
            "set" => Some(Self::Set),
            "static" => Some(Self::Static),
            "string" => Some(Self::String),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),
            "symbol" => Some(Self::Symbol),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "true" => Some(Self::True),
            "try" => Some(Self::Try),
            "type" => Some(Self::Type),
            "typeof" => Some(Self::Typeof),
            "undefined" => Some(Self::Undefined),
            "unique" => Some(Self::Unique),
            "unknown" => Some(Self::Unknown),
            "var" => Some(Self::Var),
            "void" => Some(Self::Void),
            "while" => Some(Self::While),
            "with" => Some(Self::With),
            "yield" => Some(Self::Yield),
            _ => None,
        }
    }
}

impl ElementType for TypeScriptElementType {
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

impl From<crate::lexer::token_type::TypeScriptTokenType> for TypeScriptElementType {
    fn from(token: crate::lexer::token_type::TypeScriptTokenType) -> Self {
                match token {
            crate::lexer::token_type::TypeScriptTokenType::NamedImports => Self::NamedImports,
            crate::lexer::token_type::TypeScriptTokenType::Decorator => Self::Decorator,
            crate::lexer::token_type::TypeScriptTokenType::ArrowFunction => Self::ArrowFunction,
            crate::lexer::token_type::TypeScriptTokenType::PredefinedType => Self::PredefinedType,
            crate::lexer::token_type::TypeScriptTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::TypeScriptTokenType::Any => Self::Any,
            crate::lexer::token_type::TypeScriptTokenType::As => Self::As,
            crate::lexer::token_type::TypeScriptTokenType::Asserts => Self::Asserts,
            crate::lexer::token_type::TypeScriptTokenType::Async => Self::Async,
            crate::lexer::token_type::TypeScriptTokenType::Await => Self::Await,
            crate::lexer::token_type::TypeScriptTokenType::Boolean => Self::Boolean,
            crate::lexer::token_type::TypeScriptTokenType::Break => Self::Break,
            crate::lexer::token_type::TypeScriptTokenType::Case => Self::Case,
            crate::lexer::token_type::TypeScriptTokenType::Catch => Self::Catch,
            crate::lexer::token_type::TypeScriptTokenType::Class => Self::Class,
            crate::lexer::token_type::TypeScriptTokenType::Const => Self::Const,
            crate::lexer::token_type::TypeScriptTokenType::Constructor => Self::Constructor,
            crate::lexer::token_type::TypeScriptTokenType::Continue => Self::Continue,
            crate::lexer::token_type::TypeScriptTokenType::Debugger => Self::Debugger,
            crate::lexer::token_type::TypeScriptTokenType::Declare => Self::Declare,
            crate::lexer::token_type::TypeScriptTokenType::Default => Self::Default,
            crate::lexer::token_type::TypeScriptTokenType::Delete => Self::Delete,
            crate::lexer::token_type::TypeScriptTokenType::Do => Self::Do,
            crate::lexer::token_type::TypeScriptTokenType::Else => Self::Else,
            crate::lexer::token_type::TypeScriptTokenType::Enum => Self::Enum,
            crate::lexer::token_type::TypeScriptTokenType::Export => Self::Export,
            crate::lexer::token_type::TypeScriptTokenType::Extends => Self::Extends,
            crate::lexer::token_type::TypeScriptTokenType::False => Self::False,
            crate::lexer::token_type::TypeScriptTokenType::Finally => Self::Finally,
            crate::lexer::token_type::TypeScriptTokenType::For => Self::For,
            crate::lexer::token_type::TypeScriptTokenType::From => Self::From,
            crate::lexer::token_type::TypeScriptTokenType::Function => Self::Function,
            crate::lexer::token_type::TypeScriptTokenType::Get => Self::Get,
            crate::lexer::token_type::TypeScriptTokenType::Global => Self::Global,
            crate::lexer::token_type::TypeScriptTokenType::If => Self::If,
            crate::lexer::token_type::TypeScriptTokenType::Implements => Self::Implements,
            crate::lexer::token_type::TypeScriptTokenType::Import => Self::Import,
            crate::lexer::token_type::TypeScriptTokenType::In => Self::In,
            crate::lexer::token_type::TypeScriptTokenType::Infer => Self::Infer,
            crate::lexer::token_type::TypeScriptTokenType::Instanceof => Self::Instanceof,
            crate::lexer::token_type::TypeScriptTokenType::Interface => Self::Interface,
            crate::lexer::token_type::TypeScriptTokenType::Is => Self::Is,
            crate::lexer::token_type::TypeScriptTokenType::Keyof => Self::Keyof,
            crate::lexer::token_type::TypeScriptTokenType::Let => Self::Let,
            crate::lexer::token_type::TypeScriptTokenType::Namespace => Self::Namespace,
            crate::lexer::token_type::TypeScriptTokenType::Never => Self::Never,
            crate::lexer::token_type::TypeScriptTokenType::New => Self::New,
            crate::lexer::token_type::TypeScriptTokenType::Null => Self::Null,
            crate::lexer::token_type::TypeScriptTokenType::Number => Self::Number,
            crate::lexer::token_type::TypeScriptTokenType::Object => Self::Object,
            crate::lexer::token_type::TypeScriptTokenType::Of => Self::Of,
            crate::lexer::token_type::TypeScriptTokenType::Override => Self::Override,
            crate::lexer::token_type::TypeScriptTokenType::Package => Self::Package,
            crate::lexer::token_type::TypeScriptTokenType::Private => Self::Private,
            crate::lexer::token_type::TypeScriptTokenType::Protected => Self::Protected,
            crate::lexer::token_type::TypeScriptTokenType::Public => Self::Public,
            crate::lexer::token_type::TypeScriptTokenType::Readonly => Self::Readonly,
            crate::lexer::token_type::TypeScriptTokenType::Require => Self::Require,
            crate::lexer::token_type::TypeScriptTokenType::Return => Self::Return,
            crate::lexer::token_type::TypeScriptTokenType::Set => Self::Set,
            crate::lexer::token_type::TypeScriptTokenType::Static => Self::Static,
            crate::lexer::token_type::TypeScriptTokenType::String => Self::String,
            crate::lexer::token_type::TypeScriptTokenType::Super => Self::Super,
            crate::lexer::token_type::TypeScriptTokenType::Switch => Self::Switch,
            crate::lexer::token_type::TypeScriptTokenType::Symbol => Self::Symbol,
            crate::lexer::token_type::TypeScriptTokenType::This => Self::This,
            crate::lexer::token_type::TypeScriptTokenType::Throw => Self::Throw,
            crate::lexer::token_type::TypeScriptTokenType::True => Self::True,
            crate::lexer::token_type::TypeScriptTokenType::Try => Self::Try,
            crate::lexer::token_type::TypeScriptTokenType::Type => Self::Type,
            crate::lexer::token_type::TypeScriptTokenType::Typeof => Self::Typeof,
            crate::lexer::token_type::TypeScriptTokenType::Undefined => Self::Undefined,
            crate::lexer::token_type::TypeScriptTokenType::Unique => Self::Unique,
            crate::lexer::token_type::TypeScriptTokenType::Unknown => Self::Unknown,
            crate::lexer::token_type::TypeScriptTokenType::Var => Self::Var,
            crate::lexer::token_type::TypeScriptTokenType::Void => Self::Void,
            crate::lexer::token_type::TypeScriptTokenType::While => Self::While,
            crate::lexer::token_type::TypeScriptTokenType::With => Self::With,
            crate::lexer::token_type::TypeScriptTokenType::Yield => Self::Yield,
            crate::lexer::token_type::TypeScriptTokenType::Plus => Self::Plus,
            crate::lexer::token_type::TypeScriptTokenType::Minus => Self::Minus,
            crate::lexer::token_type::TypeScriptTokenType::Star => Self::Star,
            crate::lexer::token_type::TypeScriptTokenType::Slash => Self::Slash,
            crate::lexer::token_type::TypeScriptTokenType::Percent => Self::Percent,
            crate::lexer::token_type::TypeScriptTokenType::StarStar => Self::StarStar,
            crate::lexer::token_type::TypeScriptTokenType::Question => Self::Question,
            crate::lexer::token_type::TypeScriptTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::TypeScriptTokenType::Less => Self::Less,
            crate::lexer::token_type::TypeScriptTokenType::Greater => Self::Greater,
            crate::lexer::token_type::TypeScriptTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::TypeScriptTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::TypeScriptTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::TypeScriptTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::TypeScriptTokenType::EqualEqualEqual => Self::EqualEqualEqual,
            crate::lexer::token_type::TypeScriptTokenType::NotEqualEqual => Self::NotEqualEqual,
            crate::lexer::token_type::TypeScriptTokenType::AmpersandAmpersand => Self::AmpersandAmpersand,
            crate::lexer::token_type::TypeScriptTokenType::PipePipe => Self::PipePipe,
            crate::lexer::token_type::TypeScriptTokenType::Exclamation => Self::Exclamation,
            crate::lexer::token_type::TypeScriptTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::TypeScriptTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::TypeScriptTokenType::Caret => Self::Caret,
            crate::lexer::token_type::TypeScriptTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::TypeScriptTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::TypeScriptTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::TypeScriptTokenType::UnsignedRightShift => Self::UnsignedRightShift,
            crate::lexer::token_type::TypeScriptTokenType::Equal => Self::Equal,
            crate::lexer::token_type::TypeScriptTokenType::PlusEqual => Self::PlusEqual,
            crate::lexer::token_type::TypeScriptTokenType::MinusEqual => Self::MinusEqual,
            crate::lexer::token_type::TypeScriptTokenType::StarEqual => Self::StarEqual,
            crate::lexer::token_type::TypeScriptTokenType::SlashEqual => Self::SlashEqual,
            crate::lexer::token_type::TypeScriptTokenType::PercentEqual => Self::PercentEqual,
            crate::lexer::token_type::TypeScriptTokenType::StarStarEqual => Self::StarStarEqual,
            crate::lexer::token_type::TypeScriptTokenType::LeftShiftEqual => Self::LeftShiftEqual,
            crate::lexer::token_type::TypeScriptTokenType::RightShiftEqual => Self::RightShiftEqual,
            crate::lexer::token_type::TypeScriptTokenType::UnsignedRightShiftEqual => Self::UnsignedRightShiftEqual,
            crate::lexer::token_type::TypeScriptTokenType::AmpersandEqual => Self::AmpersandEqual,
            crate::lexer::token_type::TypeScriptTokenType::PipeEqual => Self::PipeEqual,
            crate::lexer::token_type::TypeScriptTokenType::CaretEqual => Self::CaretEqual,
            crate::lexer::token_type::TypeScriptTokenType::AmpersandAmpersandEqual => Self::AmpersandAmpersandEqual,
            crate::lexer::token_type::TypeScriptTokenType::PipePipeEqual => Self::PipePipeEqual,
            crate::lexer::token_type::TypeScriptTokenType::QuestionQuestionEqual => Self::QuestionQuestionEqual,
            crate::lexer::token_type::TypeScriptTokenType::PlusPlus => Self::PlusPlus,
            crate::lexer::token_type::TypeScriptTokenType::MinusMinus => Self::MinusMinus,
            crate::lexer::token_type::TypeScriptTokenType::QuestionQuestion => Self::QuestionQuestion,
            crate::lexer::token_type::TypeScriptTokenType::QuestionDot => Self::QuestionDot,
            crate::lexer::token_type::TypeScriptTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::TypeScriptTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::TypeScriptTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::TypeScriptTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::TypeScriptTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::TypeScriptTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::TypeScriptTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::TypeScriptTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::TypeScriptTokenType::Comma => Self::Comma,
            crate::lexer::token_type::TypeScriptTokenType::Dot => Self::Dot,
            crate::lexer::token_type::TypeScriptTokenType::Colon => Self::Colon,
            crate::lexer::token_type::TypeScriptTokenType::At => Self::At,
            crate::lexer::token_type::TypeScriptTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::TypeScriptTokenType::NumericLiteral => Self::NumericLiteral,
            crate::lexer::token_type::TypeScriptTokenType::BigIntLiteral => Self::BigIntLiteral,
            crate::lexer::token_type::TypeScriptTokenType::BooleanLiteral => Self::BooleanLiteral,
            crate::lexer::token_type::TypeScriptTokenType::TemplateString => Self::TemplateString,
            crate::lexer::token_type::TypeScriptTokenType::RegexLiteral => Self::RegexLiteral,
            crate::lexer::token_type::TypeScriptTokenType::IdentifierName => Self::IdentifierName,
            crate::lexer::token_type::TypeScriptTokenType::LineComment => Self::LineComment,
            crate::lexer::token_type::TypeScriptTokenType::BlockComment => Self::BlockComment,
            crate::lexer::token_type::TypeScriptTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::TypeScriptTokenType::Newline => Self::Newline,
            crate::lexer::token_type::TypeScriptTokenType::Eof => Self::Eof,
            crate::lexer::token_type::TypeScriptTokenType::Root => Self::Root,
            crate::lexer::token_type::TypeScriptTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::TypeScriptTokenType::Module => Self::Module,
            crate::lexer::token_type::TypeScriptTokenType::VariableDeclaration => Self::VariableDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::FunctionDeclaration => Self::FunctionDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::ClassDeclaration => Self::ClassDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::InterfaceDeclaration => Self::InterfaceDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::TypeAliasDeclaration => Self::TypeAliasDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::EnumDeclaration => Self::EnumDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::NamespaceDeclaration => Self::NamespaceDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::ClassBody => Self::ClassBody,
            crate::lexer::token_type::TypeScriptTokenType::ImportDeclaration => Self::ImportDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::ExportDeclaration => Self::ExportDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::ImportClause => Self::ImportClause,
            crate::lexer::token_type::TypeScriptTokenType::ImportSpecifier => Self::ImportSpecifier,
            crate::lexer::token_type::TypeScriptTokenType::Parameter => Self::Parameter,
            crate::lexer::token_type::TypeScriptTokenType::CallArgument => Self::CallArgument,
            crate::lexer::token_type::TypeScriptTokenType::PropertyDeclaration => Self::PropertyDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::MethodDeclaration => Self::MethodDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::ConstructorDeclaration => Self::ConstructorDeclaration,
            crate::lexer::token_type::TypeScriptTokenType::PropertyAssignment => Self::PropertyAssignment,
            crate::lexer::token_type::TypeScriptTokenType::ShorthandPropertyAssignment => Self::ShorthandPropertyAssignment,
            crate::lexer::token_type::TypeScriptTokenType::SpreadElement => Self::SpreadElement,
            crate::lexer::token_type::TypeScriptTokenType::Error => Self::Error,
            crate::lexer::token_type::TypeScriptTokenType::JsxElement => Self::JsxElement,
            crate::lexer::token_type::TypeScriptTokenType::JsxSelfClosingElement => Self::JsxSelfClosingElement,
            crate::lexer::token_type::TypeScriptTokenType::JsxOpeningElement => Self::JsxOpeningElement,
            crate::lexer::token_type::TypeScriptTokenType::JsxClosingElement => Self::JsxClosingElement,
            crate::lexer::token_type::TypeScriptTokenType::JsxFragment => Self::JsxFragment,
            crate::lexer::token_type::TypeScriptTokenType::JsxOpeningFragment => Self::JsxOpeningFragment,
            crate::lexer::token_type::TypeScriptTokenType::JsxClosingFragment => Self::JsxClosingFragment,
            crate::lexer::token_type::TypeScriptTokenType::JsxAttribute => Self::JsxAttribute,
            crate::lexer::token_type::TypeScriptTokenType::JsxAttributes => Self::JsxAttributes,
            crate::lexer::token_type::TypeScriptTokenType::JsxExpressionContainer => Self::JsxExpressionContainer,
            crate::lexer::token_type::TypeScriptTokenType::JsxSpreadAttribute => Self::JsxSpreadAttribute,
            crate::lexer::token_type::TypeScriptTokenType::JsxText => Self::JsxText,
            crate::lexer::token_type::TypeScriptTokenType::BinaryExpression => Self::BinaryExpression,
            crate::lexer::token_type::TypeScriptTokenType::UnaryExpression => Self::UnaryExpression,
            crate::lexer::token_type::TypeScriptTokenType::ConditionalExpression => Self::ConditionalExpression,
            crate::lexer::token_type::TypeScriptTokenType::CallExpression => Self::CallExpression,
            crate::lexer::token_type::TypeScriptTokenType::NewExpression => Self::NewExpression,
            crate::lexer::token_type::TypeScriptTokenType::MemberExpression => Self::MemberExpression,
            crate::lexer::token_type::TypeScriptTokenType::ArrayExpression => Self::ArrayExpression,
            crate::lexer::token_type::TypeScriptTokenType::ObjectExpression => Self::ObjectExpression,
            crate::lexer::token_type::TypeScriptTokenType::FunctionExpression => Self::FunctionExpression,
            crate::lexer::token_type::TypeScriptTokenType::TemplateExpression => Self::TemplateExpression,
            crate::lexer::token_type::TypeScriptTokenType::TaggedTemplateExpression => Self::TaggedTemplateExpression,
            crate::lexer::token_type::TypeScriptTokenType::AsExpression => Self::AsExpression,
            crate::lexer::token_type::TypeScriptTokenType::TypeAssertionExpression => Self::TypeAssertionExpression,
            crate::lexer::token_type::TypeScriptTokenType::NonNullExpression => Self::NonNullExpression,
            crate::lexer::token_type::TypeScriptTokenType::UpdateExpression => Self::UpdateExpression,
            crate::lexer::token_type::TypeScriptTokenType::ExpressionStatement => Self::ExpressionStatement,
            crate::lexer::token_type::TypeScriptTokenType::BlockStatement => Self::BlockStatement,
            crate::lexer::token_type::TypeScriptTokenType::IfStatement => Self::IfStatement,
            crate::lexer::token_type::TypeScriptTokenType::WhileStatement => Self::WhileStatement,
            crate::lexer::token_type::TypeScriptTokenType::ForStatement => Self::ForStatement,
            crate::lexer::token_type::TypeScriptTokenType::ForInStatement => Self::ForInStatement,
            crate::lexer::token_type::TypeScriptTokenType::ForOfStatement => Self::ForOfStatement,
            crate::lexer::token_type::TypeScriptTokenType::DoWhileStatement => Self::DoWhileStatement,
            crate::lexer::token_type::TypeScriptTokenType::SwitchStatement => Self::SwitchStatement,
            crate::lexer::token_type::TypeScriptTokenType::CaseClause => Self::CaseClause,
            crate::lexer::token_type::TypeScriptTokenType::DefaultClause => Self::DefaultClause,
            crate::lexer::token_type::TypeScriptTokenType::TryStatement => Self::TryStatement,
            crate::lexer::token_type::TypeScriptTokenType::CatchClause => Self::CatchClause,
            crate::lexer::token_type::TypeScriptTokenType::FinallyClause => Self::FinallyClause,
            crate::lexer::token_type::TypeScriptTokenType::ThrowStatement => Self::ThrowStatement,
            crate::lexer::token_type::TypeScriptTokenType::ReturnStatement => Self::ReturnStatement,
            crate::lexer::token_type::TypeScriptTokenType::BreakStatement => Self::BreakStatement,
            crate::lexer::token_type::TypeScriptTokenType::ContinueStatement => Self::ContinueStatement,
            crate::lexer::token_type::TypeScriptTokenType::DebuggerStatement => Self::DebuggerStatement,
            crate::lexer::token_type::TypeScriptTokenType::WithStatement => Self::WithStatement,
            crate::lexer::token_type::TypeScriptTokenType::BindingPattern => Self::BindingPattern,
            crate::lexer::token_type::TypeScriptTokenType::ArrayBindingPattern => Self::ArrayBindingPattern,
            crate::lexer::token_type::TypeScriptTokenType::ObjectBindingPattern => Self::ObjectBindingPattern,
            crate::lexer::token_type::TypeScriptTokenType::BindingElement => Self::BindingElement,
            crate::lexer::token_type::TypeScriptTokenType::TypeReference => Self::TypeReference,
            crate::lexer::token_type::TypeScriptTokenType::TypeLiteral => Self::TypeLiteral,
            crate::lexer::token_type::TypeScriptTokenType::FunctionType => Self::FunctionType,
            crate::lexer::token_type::TypeScriptTokenType::ConstructorType => Self::ConstructorType,
            crate::lexer::token_type::TypeScriptTokenType::ArrayType => Self::ArrayType,
            crate::lexer::token_type::TypeScriptTokenType::TupleType => Self::TupleType,
            crate::lexer::token_type::TypeScriptTokenType::UnionType => Self::UnionType,
            crate::lexer::token_type::TypeScriptTokenType::IntersectionType => Self::IntersectionType,
            crate::lexer::token_type::TypeScriptTokenType::ConditionalType => Self::ConditionalType,
            crate::lexer::token_type::TypeScriptTokenType::MappedType => Self::MappedType,
            crate::lexer::token_type::TypeScriptTokenType::IndexedAccessType => Self::IndexedAccessType,
            crate::lexer::token_type::TypeScriptTokenType::PropertySignature => Self::PropertySignature,
            crate::lexer::token_type::TypeScriptTokenType::MethodSignature => Self::MethodSignature,
            crate::lexer::token_type::TypeScriptTokenType::LiteralType => Self::LiteralType,
            crate::lexer::token_type::TypeScriptTokenType::TypeQuery => Self::TypeQuery,
            crate::lexer::token_type::TypeScriptTokenType::TypePredicate => Self::TypePredicate,
            crate::lexer::token_type::TypeScriptTokenType::TypeAnnotation => Self::TypeAnnotation,
            crate::lexer::token_type::TypeScriptTokenType::TypeParameter => Self::TypeParameter,
            crate::lexer::token_type::TypeScriptTokenType::HeritageClause => Self::HeritageClause,
            crate::lexer::token_type::TypeScriptTokenType::EnumMember => Self::EnumMember,
            _ => Self::Error,
        }
    }
}
