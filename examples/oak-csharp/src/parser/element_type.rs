use oak_core::{ElementType, UniversalElementRole};

/// Element types for the C# parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum CSharpElementType {
    // Tokens (copied from CSharpTokenType)
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// An identifier.
    Identifier,
    /// A numeric literal.
    Number,
    /// A string literal.
    String,
    /// A character literal.
    Character,
    /// A verbatim string literal (e.g., `@"..."`).
    VerbatimString,
    /// An interpolated string literal (e.g., `$"..."`).
    InterpolatedString,
    /// A number literal (generic).
    NumberLiteral,
    /// A string literal (generic).
    StringLiteral,
    /// A character literal (generic).
    CharLiteral,

    // Keywords
    /// The `abstract` keyword.
    Abstract,
    /// The `as` keyword.
    As,
    /// The `base` keyword.
    Base,
    /// The `bool` keyword.
    Bool,
    /// The `break` keyword.
    Break,
    /// The `byte` keyword.
    Byte,
    /// The `case` keyword.
    Case,
    /// The `catch` keyword.
    Catch,
    /// The `char` keyword.
    Char,
    /// The `checked` keyword.
    Checked,
    /// The `class` keyword.
    Class,
    /// The `const` keyword.
    Const,
    /// The `continue` keyword.
    Continue,
    /// The `decimal` keyword.
    Decimal,
    /// The `default` keyword.
    Default,
    /// The `delegate` keyword.
    Delegate,
    /// The `do` keyword.
    Do,
    /// The `double` keyword.
    Double,
    /// The `else` keyword.
    Else,
    /// The `enum` keyword.
    Enum,
    /// The `event` keyword.
    Event,
    /// The `explicit` keyword.
    Explicit,
    /// The `extern` keyword.
    Extern,
    /// The `false` keyword.
    False,
    /// The `finally` keyword.
    Finally,
    /// The `fixed` keyword.
    Fixed,
    /// The `float` keyword.
    Float,
    /// The `for` keyword.
    For,
    /// The `foreach` keyword.
    Foreach,
    /// The `goto` keyword.
    Goto,
    /// The `if` keyword.
    If,
    /// The `implicit` keyword.
    Implicit,
    /// The `in` keyword.
    In,
    /// The `int` keyword.
    Int,
    /// The `interface` keyword.
    Interface,
    /// The `internal` keyword.
    Internal,
    /// The `is` keyword.
    Is,
    /// The `lock` keyword.
    Lock,
    /// The `long` keyword.
    Long,
    /// The `namespace` keyword.
    Namespace,
    /// The `new` keyword.
    New,
    /// The `null` keyword.
    Null,
    /// The `object` keyword.
    Object,
    /// The `operator` keyword.
    Operator,
    /// The `out` keyword.
    Out,
    /// The `override` keyword.
    Override,
    /// The `params` keyword.
    Params,
    /// The `private` keyword.
    Private,
    /// The `protected` keyword.
    Protected,
    /// The `public` keyword.
    Public,
    /// The `readonly` keyword.
    Readonly,
    /// The `record` keyword.
    Record,
    /// The `ref` keyword.
    Ref,
    /// The `return` keyword.
    Return,
    /// The `sbyte` keyword.
    Sbyte,
    /// The `sealed` keyword.
    Sealed,
    /// The `short` keyword.
    Short,
    /// The `sizeof` keyword.
    Sizeof,
    /// The `stackalloc` keyword.
    Stackalloc,
    /// The `static` keyword.
    Static,
    /// The `struct` keyword.
    Struct,
    /// The `switch` keyword.
    Switch,
    /// The `this` keyword.
    This,
    /// The `throw` keyword.
    Throw,
    /// The `true` keyword.
    True,
    /// The `try` keyword.
    Try,
    /// The `typeof` keyword.
    Typeof,
    /// The `uint` keyword.
    Uint,
    /// The `ulong` keyword.
    Ulong,
    /// The `unchecked` keyword.
    Unchecked,
    /// The `unsafe` keyword.
    Unsafe,
    /// The `ushort` keyword.
    Ushort,
    /// The `using` keyword.
    Using,
    /// The `virtual` keyword.
    Virtual,
    /// The `void` keyword.
    Void,
    /// The `volatile` keyword.
    Volatile,
    /// The `while` keyword.
    While,

    // Long keyword variants
    /// Variant of `abstract` keyword.
    AbstractKeyword,
    /// Variant of `as` keyword.
    AsKeyword,
    /// Variant of `base` keyword.
    BaseKeyword,
    /// Variant of `bool` keyword.
    BoolKeyword,
    /// Variant of `break` keyword.
    BreakKeyword,
    /// Variant of `byte` keyword.
    ByteKeyword,
    /// Variant of `case` keyword.
    CaseKeyword,
    /// Variant of `catch` keyword.
    CatchKeyword,
    /// Variant of `char` keyword.
    CharKeyword,
    /// Variant of `checked` keyword.
    CheckedKeyword,
    /// Variant of `class` keyword.
    ClassKeyword,
    /// Variant of `const` keyword.
    ConstKeyword,
    /// Variant of `continue` keyword.
    ContinueKeyword,
    /// Variant of `decimal` keyword.
    DecimalKeyword,
    /// Variant of `default` keyword.
    DefaultKeyword,
    /// Variant of `delegate` keyword.
    DelegateKeyword,
    /// Variant of `do` keyword.
    DoKeyword,
    /// Variant of `double` keyword.
    DoubleKeyword,
    /// Variant of `else` keyword.
    ElseKeyword,
    /// Variant of `enum` keyword.
    EnumKeyword,
    /// Variant of `event` keyword.
    EventKeyword,
    /// Variant of `explicit` keyword.
    ExplicitKeyword,
    /// Variant of `extern` keyword.
    ExternKeyword,
    /// Variant of `false` keyword.
    FalseKeyword,
    /// Variant of `finally` keyword.
    FinallyKeyword,
    /// Variant of `fixed` keyword.
    FixedKeyword,
    /// Variant of `float` keyword.
    FloatKeyword,
    /// Variant of `for` keyword.
    ForKeyword,
    /// Variant of `foreach` keyword.
    ForeachKeyword,
    /// Variant of `goto` keyword.
    GotoKeyword,
    /// Variant of `if` keyword.
    IfKeyword,
    /// Variant of `implicit` keyword.
    ImplicitKeyword,
    /// Variant of `in` keyword.
    InKeyword,
    /// Variant of `int` keyword.
    IntKeyword,
    /// Variant of `interface` keyword.
    InterfaceKeyword,
    /// Variant of `internal` keyword.
    InternalKeyword,
    /// Variant of `is` keyword.
    IsKeyword,
    /// Variant of `lock` keyword.
    LockKeyword,
    /// Variant of `long` keyword.
    LongKeyword,
    /// Variant of `namespace` keyword.
    NamespaceKeyword,
    /// Variant of `new` keyword.
    NewKeyword,
    /// Variant of `null` keyword.
    NullKeyword,
    /// Variant of `object` keyword.
    ObjectKeyword,
    /// Variant of `operator` keyword.
    OperatorKeyword,
    /// Variant of `out` keyword.
    OutKeyword,
    /// Variant of `override` keyword.
    OverrideKeyword,
    /// Variant of `params` keyword.
    ParamsKeyword,
    /// Variant of `private` keyword.
    PrivateKeyword,
    /// Variant of `protected` keyword.
    ProtectedKeyword,
    /// Variant of `public` keyword.
    PublicKeyword,
    /// Variant of `readonly` keyword.
    ReadonlyKeyword,
    /// Variant of `ref` keyword.
    RefKeyword,
    /// Variant of `return` keyword.
    ReturnKeyword,
    /// Variant of `sbyte` keyword.
    SbyteKeyword,
    /// Variant of `sealed` keyword.
    SealedKeyword,
    /// Variant of `short` keyword.
    ShortKeyword,
    /// Variant of `sizeof` keyword.
    SizeofKeyword,
    /// Variant of `stackalloc` keyword.
    StackallocKeyword,
    /// Variant of `static` keyword.
    StaticKeyword,
    /// Variant of `string` keyword.
    StringKeyword,
    /// Variant of `struct` keyword.
    StructKeyword,
    /// Variant of `switch` keyword.
    SwitchKeyword,
    /// Variant of `this` keyword.
    ThisKeyword,
    /// Variant of `throw` keyword.
    ThrowKeyword,
    /// Variant of `true` keyword.
    TrueKeyword,
    /// Variant of `try` keyword.
    TryKeyword,
    /// Variant of `typeof` keyword.
    TypeofKeyword,
    /// Variant of `uint` keyword.
    UintKeyword,
    /// Variant of `ulong` keyword.
    UlongKeyword,
    /// Variant of `unchecked` keyword.
    UncheckedKeyword,
    /// Variant of `unsafe` keyword.
    UnsafeKeyword,
    /// Variant of `ushort` keyword.
    UshortKeyword,
    /// Variant of `using` keyword.
    UsingKeyword,
    /// Variant of `virtual` keyword.
    VirtualKeyword,
    /// Variant of `void` keyword.
    VoidKeyword,
    /// Variant of `volatile` keyword.
    VolatileKeyword,
    /// Variant of `while` keyword.
    WhileKeyword,

    // Contextual keywords
    /// The `add` keyword (contextual).
    AddKeyword,
    /// The `alias` keyword (contextual).
    AliasKeyword,
    /// The `ascending` keyword (contextual).
    AscendingKeyword,
    /// The `by` keyword (contextual).
    ByKeyword,
    /// The `descending` keyword (contextual).
    DescendingKeyword,
    /// The `from` keyword (contextual).
    FromKeyword,
    /// The `get` keyword (contextual).
    GetKeyword,
    /// The `global` keyword (contextual).
    GlobalKeyword,
    /// The `group` keyword (contextual).
    GroupKeyword,
    /// The `into` keyword (contextual).
    IntoKeyword,
    /// The `join` keyword (contextual).
    JoinKeyword,
    /// The `let` keyword (contextual).
    LetKeyword,
    /// The `orderby` keyword (contextual).
    OrderbyKeyword,
    /// The `partial` keyword (contextual).
    PartialKeyword,
    /// The `remove` keyword (contextual).
    RemoveKeyword,
    /// The `select` keyword (contextual).
    SelectKeyword,
    /// The `set` keyword (contextual).
    SetKeyword,
    /// The `value` keyword (contextual).
    ValueKeyword,
    /// The `var` keyword (contextual).
    VarKeyword,
    /// The `where` keyword (contextual).
    WhereKeyword,
    /// The `yield` keyword (contextual).
    YieldKeyword,

    // Operators
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `&` operator.
    Ampersand,
    /// The `|` operator.
    Pipe,
    /// The `^` operator.
    Caret,
    /// The `~` operator.
    Tilde,
    /// Bitwise AND operator.
    BitAnd,
    /// Bitwise OR operator.
    BitOr,
    /// Bitwise XOR operator.
    BitXor,
    /// Bitwise NOT operator.
    BitNot,
    /// Left shift operator (`<<`).
    LeftShift,
    /// Right shift operator (`>>`).
    RightShift,
    /// Equality operator (`==`).
    Equal,
    /// Inequality operator (`!=`).
    NotEqual,
    /// Less than operator (`<`).
    Less,
    /// Less than or equal operator (`<=`).
    LessEqual,
    /// Greater than operator (`>`).
    Greater,
    /// Greater than or equal operator (`>=`).
    GreaterEqual,
    /// Logical AND operator (`&&`).
    LogicalAnd,
    /// Logical OR operator (`||`).
    LogicalOr,
    /// Logical NOT operator (`!`).
    LogicalNot,
    /// Question mark (`?`).
    Question,
    /// Null-coalescing operator (`??`).
    QuestionQuestion,
    /// Increment operator (`++`).
    Increment,
    /// Decrement operator (`--`).
    Decrement,
    /// Arrow operator (`->`).
    Arrow,
    /// Lambda operator (`=>`).
    Lambda,

    // Assignment operators
    /// Simple assignment operator (`=`).
    Assign,
    /// Addition assignment operator (`+=`).
    PlusAssign,
    /// Subtraction assignment operator (`-=`).
    MinusAssign,
    /// Multiplication assignment operator (`*=`).
    StarAssign,
    /// Division assignment operator (`/=`).
    SlashAssign,
    /// Modulo assignment operator (`%=`).
    PercentAssign,
    /// Bitwise AND assignment operator (`&=`).
    AmpersandAssign,
    /// Bitwise OR assignment operator (`|=`).
    PipeAssign,
    /// Bitwise XOR assignment operator (`^=`).
    CaretAssign,
    /// Left shift assignment operator (`<<=`).
    LeftShiftAssign,
    /// Right shift assignment operator (`>>=`).
    RightShiftAssign,
    /// Null-coalescing assignment operator (`??=`).
    QuestionQuestionAssign,
    /// Logical AND assignment operator (`&&=`).
    AndAssign,
    /// Logical OR assignment operator (`||=`).
    OrAssign,
    /// Logical XOR assignment operator (`^=`).
    XorAssign,

    // Delimiters
    /// Opening parenthesis (`(`).
    LeftParen,
    /// Closing parenthesis (`)`).
    RightParen,
    /// Opening bracket (`[`).
    LeftBracket,
    /// Closing bracket (`]`).
    RightBracket,
    /// Opening brace (`{`).
    LeftBrace,
    /// Closing brace (`}`).
    RightBrace,
    /// Comma (`,`).
    Comma,
    /// Semicolon (`;`).
    Semicolon,
    /// Colon (`:`).
    Colon,
    /// Double colon (`::`).
    ColonColon,
    /// Dot (`.`).
    Dot,
    /// Null-conditional operator (`?.`).
    QuestionDot,
    /// At sign (`@`).
    At,
    /// Hash sign (`#`).
    Hash,
    /// Dollar sign (`$`).
    Dollar,

    /// End of file.
    Eof,
    /// An error token.
    Error,

    // Non-terminal elements
    /// An `if` statement.
    IfStatement,
    /// A `while` statement.
    WhileStatement,
    /// A `for` statement.
    ForStatement,
    /// A block of statements.
    Block,
    /// A `return` statement.
    ReturnStatement,
    /// A method declaration.
    MethodDeclaration,
    /// A property declaration.
    PropertyDeclaration,
    /// A field declaration.
    FieldDeclaration,
    /// An indexer declaration.
    IndexerDeclaration,
    /// An identifier name.
    IdentifierName,
    /// A literal expression.
    LiteralExpression,
    /// A binary expression.
    BinaryExpression,
    /// An invocation expression.
    InvocationExpression,
    /// An element access expression.
    ElementAccessExpression,
    /// A member access expression.
    MemberAccessExpression,
    /// An assignment expression.
    AssignmentExpression,
    /// A `break` statement.
    BreakStatement,
    /// A `continue` statement.
    ContinueStatement,
    /// An expression statement.
    ExpressionStatement,
    /// A `foreach` statement.
    ForeachStatement,
    /// A namespace declaration.
    NamespaceDeclaration,
    /// A `using` directive.
    UsingDirective,
    /// A class declaration.
    ClassDeclaration,
    /// An interface declaration.
    InterfaceDeclaration,
    /// A struct declaration.
    StructDeclaration,
    /// An enum declaration.
    EnumDeclaration,
    /// A record declaration.
    RecordDeclaration,
    /// A delegate declaration.
    DelegateDeclaration,
    /// An event declaration.
    EventDeclaration,
    /// An `await` expression.
    AwaitExpression,
    /// A parenthesized expression.
    ParenthesizedExpression,
    /// A `this` expression.
    ThisExpression,
    /// A `base` expression.
    BaseExpression,
    /// An object creation expression.
    ObjectCreationExpression,
    /// A lambda expression.
    LambdaExpression,
    /// A `switch` statement.
    SwitchStatement,
    /// A `try` statement.
    TryStatement,
    /// An attribute list.
    AttributeList,
    /// An attribute.
    Attribute,
    /// The root of the document.
    Root,
}

impl CSharpElementType {
    /// Returns true if the element type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Base
                | Self::Bool
                | Self::Break
                | Self::Byte
                | Self::Case
                | Self::Catch
                | Self::Char
                | Self::Checked
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Decimal
                | Self::Default
                | Self::Delegate
                | Self::Do
                | Self::Double
                | Self::Else
                | Self::Enum
                | Self::Event
                | Self::Explicit
                | Self::Extern
                | Self::False
                | Self::Finally
                | Self::Fixed
                | Self::Float
                | Self::For
                | Self::Foreach
                | Self::Goto
                | Self::If
                | Self::Implicit
                | Self::In
                | Self::Int
                | Self::Interface
                | Self::Internal
                | Self::Is
                | Self::Lock
                | Self::Long
                | Self::Namespace
                | Self::New
                | Self::Null
                | Self::Object
                | Self::Operator
                | Self::Out
                | Self::Override
                | Self::Params
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Readonly
                | Self::Ref
                | Self::Return
                | Self::Sbyte
                | Self::Sealed
                | Self::Short
                | Self::Sizeof
                | Self::Stackalloc
                | Self::Static
                | Self::Struct
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typeof
                | Self::Uint
                | Self::Ulong
                | Self::Unchecked
                | Self::Unsafe
                | Self::Ushort
                | Self::Using
                | Self::Virtual
                | Self::Void
                | Self::Volatile
                | Self::While
                | Self::AbstractKeyword
                | Self::AsKeyword
                | Self::BaseKeyword
                | Self::BoolKeyword
                | Self::BreakKeyword
                | Self::ByteKeyword
                | Self::CaseKeyword
                | Self::CatchKeyword
                | Self::CharKeyword
                | Self::CheckedKeyword
                | Self::ClassKeyword
                | Self::ConstKeyword
                | Self::ContinueKeyword
                | Self::DecimalKeyword
                | Self::DefaultKeyword
                | Self::DelegateKeyword
                | Self::DoKeyword
                | Self::DoubleKeyword
                | Self::ElseKeyword
                | Self::EnumKeyword
                | Self::EventKeyword
                | Self::ExplicitKeyword
                | Self::ExternKeyword
                | Self::FalseKeyword
                | Self::FinallyKeyword
                | Self::FixedKeyword
                | Self::FloatKeyword
                | Self::ForKeyword
                | Self::ForeachKeyword
                | Self::GotoKeyword
                | Self::IfKeyword
                | Self::ImplicitKeyword
                | Self::InKeyword
                | Self::IntKeyword
                | Self::InterfaceKeyword
                | Self::InternalKeyword
                | Self::IsKeyword
                | Self::LockKeyword
                | Self::LongKeyword
                | Self::NamespaceKeyword
                | Self::NewKeyword
                | Self::NullKeyword
                | Self::ObjectKeyword
                | Self::OperatorKeyword
                | Self::OutKeyword
                | Self::OverrideKeyword
                | Self::ParamsKeyword
                | Self::PrivateKeyword
                | Self::ProtectedKeyword
                | Self::PublicKeyword
                | Self::ReadonlyKeyword
                | Self::RefKeyword
                | Self::ReturnKeyword
                | Self::SbyteKeyword
                | Self::SealedKeyword
                | Self::ShortKeyword
                | Self::SizeofKeyword
                | Self::StackallocKeyword
                | Self::StaticKeyword
                | Self::StringKeyword
                | Self::StructKeyword
                | Self::SwitchKeyword
                | Self::ThisKeyword
                | Self::ThrowKeyword
                | Self::TrueKeyword
                | Self::TryKeyword
                | Self::TypeofKeyword
                | Self::UintKeyword
                | Self::UlongKeyword
                | Self::UncheckedKeyword
                | Self::UnsafeKeyword
                | Self::UshortKeyword
                | Self::UsingKeyword
                | Self::VirtualKeyword
                | Self::VoidKeyword
                | Self::VolatileKeyword
                | Self::WhileKeyword
                | Self::AddKeyword
                | Self::AliasKeyword
                | Self::AscendingKeyword
                | Self::ByKeyword
                | Self::DescendingKeyword
                | Self::FromKeyword
                | Self::GetKeyword
                | Self::GlobalKeyword
                | Self::GroupKeyword
                | Self::IntoKeyword
                | Self::JoinKeyword
                | Self::LetKeyword
                | Self::OrderbyKeyword
                | Self::PartialKeyword
                | Self::RemoveKeyword
                | Self::SelectKeyword
                | Self::SetKeyword
                | Self::ValueKeyword
                | Self::VarKeyword
                | Self::WhereKeyword
                | Self::YieldKeyword
        )
    }
}

impl ElementType for CSharpElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::CSharpTokenType> for CSharpElementType {
    fn from(token: crate::lexer::token_type::CSharpTokenType) -> Self {
                match token {
            crate::lexer::token_type::CSharpTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::CSharpTokenType::Newline => Self::Newline,
            crate::lexer::token_type::CSharpTokenType::Comment => Self::Comment,
            crate::lexer::token_type::CSharpTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::CSharpTokenType::Number => Self::Number,
            crate::lexer::token_type::CSharpTokenType::String => Self::String,
            crate::lexer::token_type::CSharpTokenType::Character => Self::Character,
            crate::lexer::token_type::CSharpTokenType::VerbatimString => Self::VerbatimString,
            crate::lexer::token_type::CSharpTokenType::InterpolatedString => Self::InterpolatedString,
            crate::lexer::token_type::CSharpTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::CSharpTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::CSharpTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::CSharpTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::CSharpTokenType::As => Self::As,
            crate::lexer::token_type::CSharpTokenType::Async => Self::Error,
            crate::lexer::token_type::CSharpTokenType::Await => Self::Error,
            crate::lexer::token_type::CSharpTokenType::Base => Self::Base,
            crate::lexer::token_type::CSharpTokenType::Bool => Self::Bool,
            crate::lexer::token_type::CSharpTokenType::Break => Self::Break,
            crate::lexer::token_type::CSharpTokenType::Byte => Self::Byte,
            crate::lexer::token_type::CSharpTokenType::Case => Self::Case,
            crate::lexer::token_type::CSharpTokenType::Catch => Self::Catch,
            crate::lexer::token_type::CSharpTokenType::Char => Self::Char,
            crate::lexer::token_type::CSharpTokenType::Checked => Self::Checked,
            crate::lexer::token_type::CSharpTokenType::Class => Self::Class,
            crate::lexer::token_type::CSharpTokenType::Const => Self::Const,
            crate::lexer::token_type::CSharpTokenType::Continue => Self::Continue,
            crate::lexer::token_type::CSharpTokenType::Decimal => Self::Decimal,
            crate::lexer::token_type::CSharpTokenType::Default => Self::Default,
            crate::lexer::token_type::CSharpTokenType::Delegate => Self::Delegate,
            crate::lexer::token_type::CSharpTokenType::Do => Self::Do,
            crate::lexer::token_type::CSharpTokenType::Double => Self::Double,
            crate::lexer::token_type::CSharpTokenType::Else => Self::Else,
            crate::lexer::token_type::CSharpTokenType::Enum => Self::Enum,
            crate::lexer::token_type::CSharpTokenType::Event => Self::Event,
            crate::lexer::token_type::CSharpTokenType::Explicit => Self::Explicit,
            crate::lexer::token_type::CSharpTokenType::Extern => Self::Extern,
            crate::lexer::token_type::CSharpTokenType::False => Self::False,
            crate::lexer::token_type::CSharpTokenType::Finally => Self::Finally,
            crate::lexer::token_type::CSharpTokenType::Fixed => Self::Fixed,
            crate::lexer::token_type::CSharpTokenType::Float => Self::Float,
            crate::lexer::token_type::CSharpTokenType::For => Self::For,
            crate::lexer::token_type::CSharpTokenType::Foreach => Self::Foreach,
            crate::lexer::token_type::CSharpTokenType::Goto => Self::Goto,
            crate::lexer::token_type::CSharpTokenType::If => Self::If,
            crate::lexer::token_type::CSharpTokenType::Implicit => Self::Implicit,
            crate::lexer::token_type::CSharpTokenType::In => Self::In,
            crate::lexer::token_type::CSharpTokenType::Int => Self::Int,
            crate::lexer::token_type::CSharpTokenType::Interface => Self::Interface,
            crate::lexer::token_type::CSharpTokenType::Internal => Self::Internal,
            crate::lexer::token_type::CSharpTokenType::Is => Self::Is,
            crate::lexer::token_type::CSharpTokenType::Lock => Self::Lock,
            crate::lexer::token_type::CSharpTokenType::Long => Self::Long,
            crate::lexer::token_type::CSharpTokenType::Namespace => Self::Namespace,
            crate::lexer::token_type::CSharpTokenType::New => Self::New,
            crate::lexer::token_type::CSharpTokenType::Null => Self::Null,
            crate::lexer::token_type::CSharpTokenType::Object => Self::Object,
            crate::lexer::token_type::CSharpTokenType::Operator => Self::Operator,
            crate::lexer::token_type::CSharpTokenType::Out => Self::Out,
            crate::lexer::token_type::CSharpTokenType::Override => Self::Override,
            crate::lexer::token_type::CSharpTokenType::Params => Self::Params,
            crate::lexer::token_type::CSharpTokenType::Private => Self::Private,
            crate::lexer::token_type::CSharpTokenType::Protected => Self::Protected,
            crate::lexer::token_type::CSharpTokenType::Public => Self::Public,
            crate::lexer::token_type::CSharpTokenType::Readonly => Self::Readonly,
            crate::lexer::token_type::CSharpTokenType::Record => Self::Record,
            crate::lexer::token_type::CSharpTokenType::Ref => Self::Ref,
            crate::lexer::token_type::CSharpTokenType::Return => Self::Return,
            crate::lexer::token_type::CSharpTokenType::Sbyte => Self::Sbyte,
            crate::lexer::token_type::CSharpTokenType::Sealed => Self::Sealed,
            crate::lexer::token_type::CSharpTokenType::Short => Self::Short,
            crate::lexer::token_type::CSharpTokenType::Sizeof => Self::Sizeof,
            crate::lexer::token_type::CSharpTokenType::Stackalloc => Self::Stackalloc,
            crate::lexer::token_type::CSharpTokenType::Static => Self::Static,
            crate::lexer::token_type::CSharpTokenType::Struct => Self::Struct,
            crate::lexer::token_type::CSharpTokenType::Switch => Self::Switch,
            crate::lexer::token_type::CSharpTokenType::This => Self::This,
            crate::lexer::token_type::CSharpTokenType::Throw => Self::Throw,
            crate::lexer::token_type::CSharpTokenType::True => Self::True,
            crate::lexer::token_type::CSharpTokenType::Try => Self::Try,
            crate::lexer::token_type::CSharpTokenType::Typeof => Self::Typeof,
            crate::lexer::token_type::CSharpTokenType::Uint => Self::Uint,
            crate::lexer::token_type::CSharpTokenType::Ulong => Self::Ulong,
            crate::lexer::token_type::CSharpTokenType::Unchecked => Self::Unchecked,
            crate::lexer::token_type::CSharpTokenType::Unsafe => Self::Unsafe,
            crate::lexer::token_type::CSharpTokenType::Ushort => Self::Ushort,
            crate::lexer::token_type::CSharpTokenType::Using => Self::Using,
            crate::lexer::token_type::CSharpTokenType::Virtual => Self::Virtual,
            crate::lexer::token_type::CSharpTokenType::Void => Self::Void,
            crate::lexer::token_type::CSharpTokenType::Volatile => Self::Volatile,
            crate::lexer::token_type::CSharpTokenType::While => Self::While,
            crate::lexer::token_type::CSharpTokenType::AbstractKeyword => Self::AbstractKeyword,
            crate::lexer::token_type::CSharpTokenType::AsKeyword => Self::AsKeyword,
            crate::lexer::token_type::CSharpTokenType::AsyncKeyword => Self::Error,
            crate::lexer::token_type::CSharpTokenType::AwaitKeyword => Self::Error,
            crate::lexer::token_type::CSharpTokenType::BaseKeyword => Self::BaseKeyword,
            crate::lexer::token_type::CSharpTokenType::BoolKeyword => Self::BoolKeyword,
            crate::lexer::token_type::CSharpTokenType::BreakKeyword => Self::BreakKeyword,
            crate::lexer::token_type::CSharpTokenType::ByteKeyword => Self::ByteKeyword,
            crate::lexer::token_type::CSharpTokenType::CaseKeyword => Self::CaseKeyword,
            crate::lexer::token_type::CSharpTokenType::CatchKeyword => Self::CatchKeyword,
            crate::lexer::token_type::CSharpTokenType::CharKeyword => Self::CharKeyword,
            crate::lexer::token_type::CSharpTokenType::CheckedKeyword => Self::CheckedKeyword,
            crate::lexer::token_type::CSharpTokenType::ClassKeyword => Self::ClassKeyword,
            crate::lexer::token_type::CSharpTokenType::ConstKeyword => Self::ConstKeyword,
            crate::lexer::token_type::CSharpTokenType::ContinueKeyword => Self::ContinueKeyword,
            crate::lexer::token_type::CSharpTokenType::DecimalKeyword => Self::DecimalKeyword,
            crate::lexer::token_type::CSharpTokenType::DefaultKeyword => Self::DefaultKeyword,
            crate::lexer::token_type::CSharpTokenType::DelegateKeyword => Self::DelegateKeyword,
            crate::lexer::token_type::CSharpTokenType::DoKeyword => Self::DoKeyword,
            crate::lexer::token_type::CSharpTokenType::DoubleKeyword => Self::DoubleKeyword,
            crate::lexer::token_type::CSharpTokenType::ElseKeyword => Self::ElseKeyword,
            crate::lexer::token_type::CSharpTokenType::EnumKeyword => Self::EnumKeyword,
            crate::lexer::token_type::CSharpTokenType::EventKeyword => Self::EventKeyword,
            crate::lexer::token_type::CSharpTokenType::ExplicitKeyword => Self::ExplicitKeyword,
            crate::lexer::token_type::CSharpTokenType::ExternKeyword => Self::ExternKeyword,
            crate::lexer::token_type::CSharpTokenType::FalseKeyword => Self::FalseKeyword,
            crate::lexer::token_type::CSharpTokenType::FinallyKeyword => Self::FinallyKeyword,
            crate::lexer::token_type::CSharpTokenType::FixedKeyword => Self::FixedKeyword,
            crate::lexer::token_type::CSharpTokenType::FloatKeyword => Self::FloatKeyword,
            crate::lexer::token_type::CSharpTokenType::ForKeyword => Self::ForKeyword,
            crate::lexer::token_type::CSharpTokenType::ForeachKeyword => Self::ForeachKeyword,
            crate::lexer::token_type::CSharpTokenType::GotoKeyword => Self::GotoKeyword,
            crate::lexer::token_type::CSharpTokenType::IfKeyword => Self::IfKeyword,
            crate::lexer::token_type::CSharpTokenType::ImplicitKeyword => Self::ImplicitKeyword,
            crate::lexer::token_type::CSharpTokenType::InKeyword => Self::InKeyword,
            crate::lexer::token_type::CSharpTokenType::IntKeyword => Self::IntKeyword,
            crate::lexer::token_type::CSharpTokenType::InterfaceKeyword => Self::InterfaceKeyword,
            crate::lexer::token_type::CSharpTokenType::InternalKeyword => Self::InternalKeyword,
            crate::lexer::token_type::CSharpTokenType::IsKeyword => Self::IsKeyword,
            crate::lexer::token_type::CSharpTokenType::LockKeyword => Self::LockKeyword,
            crate::lexer::token_type::CSharpTokenType::LongKeyword => Self::LongKeyword,
            crate::lexer::token_type::CSharpTokenType::NamespaceKeyword => Self::NamespaceKeyword,
            crate::lexer::token_type::CSharpTokenType::NewKeyword => Self::NewKeyword,
            crate::lexer::token_type::CSharpTokenType::NullKeyword => Self::NullKeyword,
            crate::lexer::token_type::CSharpTokenType::ObjectKeyword => Self::ObjectKeyword,
            crate::lexer::token_type::CSharpTokenType::OperatorKeyword => Self::OperatorKeyword,
            crate::lexer::token_type::CSharpTokenType::OutKeyword => Self::OutKeyword,
            crate::lexer::token_type::CSharpTokenType::OverrideKeyword => Self::OverrideKeyword,
            crate::lexer::token_type::CSharpTokenType::ParamsKeyword => Self::ParamsKeyword,
            crate::lexer::token_type::CSharpTokenType::PrivateKeyword => Self::PrivateKeyword,
            crate::lexer::token_type::CSharpTokenType::ProtectedKeyword => Self::ProtectedKeyword,
            crate::lexer::token_type::CSharpTokenType::PublicKeyword => Self::PublicKeyword,
            crate::lexer::token_type::CSharpTokenType::ReadonlyKeyword => Self::ReadonlyKeyword,
            crate::lexer::token_type::CSharpTokenType::RefKeyword => Self::RefKeyword,
            crate::lexer::token_type::CSharpTokenType::ReturnKeyword => Self::ReturnKeyword,
            crate::lexer::token_type::CSharpTokenType::SbyteKeyword => Self::SbyteKeyword,
            crate::lexer::token_type::CSharpTokenType::SealedKeyword => Self::SealedKeyword,
            crate::lexer::token_type::CSharpTokenType::ShortKeyword => Self::ShortKeyword,
            crate::lexer::token_type::CSharpTokenType::SizeofKeyword => Self::SizeofKeyword,
            crate::lexer::token_type::CSharpTokenType::StackallocKeyword => Self::StackallocKeyword,
            crate::lexer::token_type::CSharpTokenType::StaticKeyword => Self::StaticKeyword,
            crate::lexer::token_type::CSharpTokenType::StringKeyword => Self::StringKeyword,
            crate::lexer::token_type::CSharpTokenType::StructKeyword => Self::StructKeyword,
            crate::lexer::token_type::CSharpTokenType::SwitchKeyword => Self::SwitchKeyword,
            crate::lexer::token_type::CSharpTokenType::ThisKeyword => Self::ThisKeyword,
            crate::lexer::token_type::CSharpTokenType::ThrowKeyword => Self::ThrowKeyword,
            crate::lexer::token_type::CSharpTokenType::TrueKeyword => Self::TrueKeyword,
            crate::lexer::token_type::CSharpTokenType::TryKeyword => Self::TryKeyword,
            crate::lexer::token_type::CSharpTokenType::TypeofKeyword => Self::TypeofKeyword,
            crate::lexer::token_type::CSharpTokenType::UintKeyword => Self::UintKeyword,
            crate::lexer::token_type::CSharpTokenType::UlongKeyword => Self::UlongKeyword,
            crate::lexer::token_type::CSharpTokenType::UncheckedKeyword => Self::UncheckedKeyword,
            crate::lexer::token_type::CSharpTokenType::UnsafeKeyword => Self::UnsafeKeyword,
            crate::lexer::token_type::CSharpTokenType::UshortKeyword => Self::UshortKeyword,
            crate::lexer::token_type::CSharpTokenType::UsingKeyword => Self::UsingKeyword,
            crate::lexer::token_type::CSharpTokenType::VirtualKeyword => Self::VirtualKeyword,
            crate::lexer::token_type::CSharpTokenType::VoidKeyword => Self::VoidKeyword,
            crate::lexer::token_type::CSharpTokenType::VolatileKeyword => Self::VolatileKeyword,
            crate::lexer::token_type::CSharpTokenType::WhileKeyword => Self::WhileKeyword,
            crate::lexer::token_type::CSharpTokenType::AddKeyword => Self::AddKeyword,
            crate::lexer::token_type::CSharpTokenType::AliasKeyword => Self::AliasKeyword,
            crate::lexer::token_type::CSharpTokenType::AscendingKeyword => Self::AscendingKeyword,
            crate::lexer::token_type::CSharpTokenType::ByKeyword => Self::ByKeyword,
            crate::lexer::token_type::CSharpTokenType::DescendingKeyword => Self::DescendingKeyword,
            crate::lexer::token_type::CSharpTokenType::FromKeyword => Self::FromKeyword,
            crate::lexer::token_type::CSharpTokenType::GetKeyword => Self::GetKeyword,
            crate::lexer::token_type::CSharpTokenType::GlobalKeyword => Self::GlobalKeyword,
            crate::lexer::token_type::CSharpTokenType::GroupKeyword => Self::GroupKeyword,
            crate::lexer::token_type::CSharpTokenType::IntoKeyword => Self::IntoKeyword,
            crate::lexer::token_type::CSharpTokenType::JoinKeyword => Self::JoinKeyword,
            crate::lexer::token_type::CSharpTokenType::LetKeyword => Self::LetKeyword,
            crate::lexer::token_type::CSharpTokenType::OrderbyKeyword => Self::OrderbyKeyword,
            crate::lexer::token_type::CSharpTokenType::PartialKeyword => Self::PartialKeyword,
            crate::lexer::token_type::CSharpTokenType::RemoveKeyword => Self::RemoveKeyword,
            crate::lexer::token_type::CSharpTokenType::SelectKeyword => Self::SelectKeyword,
            crate::lexer::token_type::CSharpTokenType::SetKeyword => Self::SetKeyword,
            crate::lexer::token_type::CSharpTokenType::ValueKeyword => Self::ValueKeyword,
            crate::lexer::token_type::CSharpTokenType::VarKeyword => Self::VarKeyword,
            crate::lexer::token_type::CSharpTokenType::WhereKeyword => Self::WhereKeyword,
            crate::lexer::token_type::CSharpTokenType::YieldKeyword => Self::YieldKeyword,
            crate::lexer::token_type::CSharpTokenType::Plus => Self::Plus,
            crate::lexer::token_type::CSharpTokenType::Minus => Self::Minus,
            crate::lexer::token_type::CSharpTokenType::Star => Self::Star,
            crate::lexer::token_type::CSharpTokenType::Slash => Self::Slash,
            crate::lexer::token_type::CSharpTokenType::Percent => Self::Percent,
            crate::lexer::token_type::CSharpTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::CSharpTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::CSharpTokenType::Caret => Self::Caret,
            crate::lexer::token_type::CSharpTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::CSharpTokenType::BitAnd => Self::BitAnd,
            crate::lexer::token_type::CSharpTokenType::BitOr => Self::BitOr,
            crate::lexer::token_type::CSharpTokenType::BitXor => Self::BitXor,
            crate::lexer::token_type::CSharpTokenType::BitNot => Self::BitNot,
            crate::lexer::token_type::CSharpTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::CSharpTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::CSharpTokenType::Equal => Self::Equal,
            crate::lexer::token_type::CSharpTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::CSharpTokenType::Less => Self::Less,
            crate::lexer::token_type::CSharpTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::CSharpTokenType::Greater => Self::Greater,
            crate::lexer::token_type::CSharpTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::CSharpTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::CSharpTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::CSharpTokenType::LogicalNot => Self::LogicalNot,
            crate::lexer::token_type::CSharpTokenType::Question => Self::Question,
            crate::lexer::token_type::CSharpTokenType::QuestionQuestion => Self::QuestionQuestion,
            crate::lexer::token_type::CSharpTokenType::Increment => Self::Increment,
            crate::lexer::token_type::CSharpTokenType::Decrement => Self::Decrement,
            crate::lexer::token_type::CSharpTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::CSharpTokenType::Lambda => Self::Lambda,
            crate::lexer::token_type::CSharpTokenType::Assign => Self::Assign,
            crate::lexer::token_type::CSharpTokenType::PlusAssign => Self::PlusAssign,
            crate::lexer::token_type::CSharpTokenType::MinusAssign => Self::MinusAssign,
            crate::lexer::token_type::CSharpTokenType::StarAssign => Self::StarAssign,
            crate::lexer::token_type::CSharpTokenType::SlashAssign => Self::SlashAssign,
            crate::lexer::token_type::CSharpTokenType::PercentAssign => Self::PercentAssign,
            crate::lexer::token_type::CSharpTokenType::AmpersandAssign => Self::AmpersandAssign,
            crate::lexer::token_type::CSharpTokenType::PipeAssign => Self::PipeAssign,
            crate::lexer::token_type::CSharpTokenType::CaretAssign => Self::CaretAssign,
            crate::lexer::token_type::CSharpTokenType::LeftShiftAssign => Self::LeftShiftAssign,
            crate::lexer::token_type::CSharpTokenType::RightShiftAssign => Self::RightShiftAssign,
            crate::lexer::token_type::CSharpTokenType::QuestionQuestionAssign => Self::QuestionQuestionAssign,
            crate::lexer::token_type::CSharpTokenType::AndAssign => Self::AndAssign,
            crate::lexer::token_type::CSharpTokenType::OrAssign => Self::OrAssign,
            crate::lexer::token_type::CSharpTokenType::XorAssign => Self::XorAssign,
            crate::lexer::token_type::CSharpTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::CSharpTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::CSharpTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::CSharpTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::CSharpTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::CSharpTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::CSharpTokenType::Comma => Self::Comma,
            crate::lexer::token_type::CSharpTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::CSharpTokenType::Colon => Self::Colon,
            crate::lexer::token_type::CSharpTokenType::ColonColon => Self::ColonColon,
            crate::lexer::token_type::CSharpTokenType::Dot => Self::Dot,
            crate::lexer::token_type::CSharpTokenType::QuestionDot => Self::QuestionDot,
            crate::lexer::token_type::CSharpTokenType::At => Self::At,
            crate::lexer::token_type::CSharpTokenType::Hash => Self::Hash,
            crate::lexer::token_type::CSharpTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::CSharpTokenType::Eof => Self::Eof,
            crate::lexer::token_type::CSharpTokenType::Error => Self::Error,
            _ => Self::Error,
        }
    }
}
