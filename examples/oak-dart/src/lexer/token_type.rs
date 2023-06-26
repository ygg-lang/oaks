use oak_core::{Token, TokenType, UniversalTokenRole};

/// A Dart token with its associated type information.
pub type DartToken = Token<DartTokenType>;

/// Represents the different types of tokens that can be produced by the Dart lexer.
///
/// This enum encompasses all token types found in Dart source code, including
/// structural tokens, literals, keywords, operators, and delimiters.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DartTokenType {
    /// The root node of a Dart abstract syntax tree.
    Root,

    /// A class declaration token.
    ClassDeclaration,

    /// A function declaration token.
    FunctionDeclaration,

    /// A variable declaration token.
    VariableDeclaration,

    /// Whitespace characters (spaces, tabs, etc.).
    Whitespace,

    /// A newline character.
    Newline,

    /// An identifier (variable name, function name, etc.).
    Identifier,

    /// An integer literal (e.g., `42`, `0xFF`).
    IntegerLiteral,

    /// A double-precision floating-point literal (e.g., `3.14`, `1.0e-10`).
    DoubleLiteral,

    /// A string literal (e.g., `"hello"`, `'world'`, `'''multi-line'''`).
    StringLiteral,

    /// A boolean literal (`true` or `false`).
    BooleanLiteral,

    /// A null literal (`null`).
    NullLiteral,

    /// The `abstract` keyword.
    Abstract,

    /// The `as` keyword for type casting.
    As,

    /// The `assert` keyword for assertions.
    Assert,

    /// The `async` keyword for asynchronous functions.
    Async,

    /// The `await` keyword for awaiting futures.
    Await,

    /// The `break` keyword for loop control.
    Break,

    /// The `case` keyword in switch statements.
    Case,

    /// The `catch` keyword in try-catch blocks.
    Catch,

    /// The `class` keyword for class declarations.
    Class,

    /// The `const` keyword for compile-time constants.
    Const,

    /// The `continue` keyword for loop control.
    Continue,

    /// The `covariant` keyword for type parameters.
    Covariant,

    /// The `default` keyword in switch statements.
    Default,

    /// The `deferred` keyword for lazy imports.
    Deferred,

    /// The `do` keyword for do-while loops.
    Do,

    /// The `dynamic` keyword for dynamic typing.
    Dynamic,

    /// The `else` keyword in if-else statements.
    Else,

    /// The `enum` keyword for enumeration declarations.
    Enum,

    /// The `export` keyword for library exports.
    Export,

    /// The `extends` keyword for class inheritance.
    Extends,

    /// The `extension` keyword for extension methods.
    Extension,

    /// The `external` keyword for external functions.
    External,

    /// The `factory` keyword for factory constructors.
    Factory,

    /// The `false` boolean literal.
    False,

    /// The `final` keyword for immutable variables.
    Final,

    /// The `finally` keyword in try-finally blocks.
    Finally,

    /// The `for` keyword for for-loops.
    For,

    /// The `function` keyword (used in typedefs).
    Function,

    /// The `get` keyword for getter declarations.
    Get,

    /// The `hide` keyword in import directives.
    Hide,

    /// The `if` keyword for conditional statements.
    If,

    /// The `implements` keyword for interface implementation.
    Implements,

    /// The `import` keyword for library imports.
    Import,

    /// The `in` keyword for for-in loops.
    In,

    /// The `interface` keyword for interface declarations.
    Interface,

    /// The `int` built-in type.
    Int,

    /// The `is` keyword for type checking.
    Is,

    /// The `late` keyword for late initialization.
    Late,

    /// The `library` keyword for library declarations.
    Library,

    /// The `mixin` keyword for mixin declarations.
    Mixin,

    /// The `new` keyword for object instantiation.
    New,

    /// The `null` literal.
    Null,

    /// The `on` keyword for exception handling in catch clauses.
    On,

    /// The `operator` keyword for operator overloading.
    Operator,

    /// The `part` keyword for library parts.
    Part,

    /// The `required` keyword for required named parameters.
    Required,

    /// The `rethrow` keyword for re-throwing exceptions.
    Rethrow,

    /// The `return` keyword for function returns.
    Return,

    /// The `set` keyword for setter declarations.
    Set,

    /// The `show` keyword in import directives.
    Show,

    /// The `static` keyword for static members.
    Static,

    /// The `super` keyword for superclass access.
    Super,

    /// The `switch` keyword for switch statements.
    Switch,

    /// The `sync` keyword for synchronous generators.
    Sync,

    /// The `this` keyword for current instance reference.
    This,

    /// The `throw` keyword for throwing exceptions.
    Throw,

    /// The `true` boolean literal.
    True,

    /// The `try` keyword for exception handling.
    Try,

    /// The `typedef` keyword for type aliases.
    Typedef,

    /// The `var` keyword for variable declarations.
    Var,

    /// The `void` type for functions returning no value.
    Void,

    /// The `while` keyword for while-loops.
    While,

    /// The `with` keyword for mixin applications.
    With,

    /// The `yield` keyword for generator functions.
    Yield,

    /// The addition operator (`+`).
    Plus,

    /// The subtraction operator (`-`).
    Minus,

    /// The multiplication operator (`*`).
    Star,

    /// The division operator (`/`).
    Slash,

    /// The modulo operator (`%`).
    Percent,

    /// The integer division operator (`~/`).
    TildeSlash,

    /// The assignment operator (`=`).
    Equal,

    /// The equality operator (`==`).
    EqualEqual,

    /// The inequality operator (`!=`).
    BangEqual,

    /// The less-than operator (`<`).
    Less,

    /// The greater-than operator (`>`).
    Greater,

    /// The less-than-or-equal operator (`<=`).
    LessEqual,

    /// The greater-than-or-equal operator (`>=`).
    GreaterEqual,

    /// The left shift operator (`<<`).
    LeftShift,

    /// The right shift operator (`>>`).
    RightShift,

    /// The bitwise AND operator (`&`).
    Ampersand,

    /// The bitwise OR operator (`|`).
    Pipe,

    /// The bitwise XOR operator (`^`).
    Caret,

    /// The bitwise NOT operator (`~`).
    Tilde,

    /// The logical NOT operator (`!`).
    Bang,

    /// The logical AND operator (`&&`).
    AmpersandAmpersand,

    /// The logical OR operator (`||`).
    PipePipe,

    /// The conditional (ternary) operator start (`?`).
    Question,

    /// The null-aware coalescing operator (`??`).
    QuestionQuestion,

    /// The increment operator (`++`).
    PlusPlus,

    /// The decrement operator (`--`).
    MinusMinus,

    /// The addition assignment operator (`+=`).
    PlusEqual,

    /// The subtraction assignment operator (`-=`).
    MinusEqual,

    /// The multiplication assignment operator (`*=`).
    StarEqual,

    /// The division assignment operator (`/=`).
    SlashEqual,

    /// The modulo assignment operator (`%=`).
    PercentEqual,

    /// The integer division assignment operator (`~/=`).
    TildeSlashEqual,

    /// The left shift assignment operator (`<<=`).
    LeftShiftEqual,

    /// The right shift assignment operator (`>>=`).
    RightShiftEqual,

    /// The bitwise AND assignment operator (`&=`).
    AmpersandEqual,

    /// The bitwise OR assignment operator (`|=`).
    PipeEqual,

    /// The bitwise XOR assignment operator (`^=`).
    CaretEqual,

    /// The null-aware coalescing assignment operator (`??=`).
    QuestionQuestionEqual,

    /// The arrow operator (`=>`) for expression functions.
    Arrow,

    /// The member access operator (`.`).
    Dot,

    /// The cascade operator (`..`).
    DotDot,

    /// The spread operator (`...`).
    DotDotDot,

    /// The null-aware member access operator (`?.`).
    QuestionDot,

    /// The left parenthesis (`(`).
    LeftParen,

    /// The right parenthesis (`)`).
    RightParen,

    /// The left bracket (`[`).
    LeftBracket,

    /// The right bracket (`]`).
    RightBracket,

    /// The left brace (`{`).
    LeftBrace,

    /// The right brace (`}`).
    RightBrace,

    /// The semicolon (`;`).
    Semicolon,

    /// The comma (`,`).
    Comma,

    /// The colon (`:`).
    Colon,

    /// The at symbol (`@`) for annotations.
    At,

    /// The hash symbol (`#`) for library directives.
    Hash,

    /// A single-line comment (`// ...`).
    LineComment,

    /// A multi-line block comment (`/* ... */`).
    BlockComment,

    /// A documentation comment (`/// ...` or `/** ... */`).
    DocComment,

    /// An error token representing a lexical error.
    Error,

    /// The end-of-file marker.
    Eof,
}

impl TokenType for DartTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::DoubleLiteral | Self::StringLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}

impl DartTokenType {
    /// Returns `true` if this token type represents a Dart keyword.
    ///
    /// Keywords are reserved words in Dart that have special meaning and
    /// cannot be used as identifiers.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Assert
                | Self::Async
                | Self::Await
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Covariant
                | Self::Default
                | Self::Deferred
                | Self::Do
                | Self::Dynamic
                | Self::Else
                | Self::Enum
                | Self::Export
                | Self::Extends
                | Self::Extension
                | Self::External
                | Self::Factory
                | Self::False
                | Self::Final
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::Get
                | Self::Hide
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Interface
                | Self::Int
                | Self::Is
                | Self::Late
                | Self::Library
                | Self::Mixin
                | Self::New
                | Self::Null
                | Self::On
                | Self::Operator
                | Self::Part
                | Self::Required
                | Self::Rethrow
                | Self::Return
                | Self::Set
                | Self::Show
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::Sync
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typedef
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Yield
        )
    }
}

/// An alias for `DartTokenType`, representing the kind of a Dart token.
pub type DartTokenKind = DartTokenType;
