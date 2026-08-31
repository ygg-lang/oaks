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
        unsafe { std::mem::transmute(token as u16) }
    }
}
