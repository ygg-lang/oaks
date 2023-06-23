#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Keywords or soft keywords
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum DejavuKeywords {
    /// Declare a namespace in Dejavu.
    ///
    /// ```v
    /// namespace path { ... }
    /// ```
    Namespace,
    /// Import a declaration from another namespace.
    ///
    /// ```v
    /// using package::module::path;
    /// ```
    Using,
    /// Declare a class.
    ///
    /// ```v
    /// class Point { x: f64, y: f64 }
    /// ```
    Class,
    /// Declare a singleton.
    ///
    /// ```v
    /// singleton GlobalConfig { ... }
    /// ```
    Singleton,
    /// Declare a trait.
    ///
    /// ```v
    /// trait Show { micro show(self) }
    /// ```
    Trait,
    /// Declare flags.
    ///
    /// ```v
    /// flags Permissions { Read, Write, Execute }
    /// ```
    Flags,
    /// Declare enums.
    ///
    /// ```v
    /// enums Priority { High = 3, Normal = 2, Low = 1 }
    /// ```
    Enums,
    /// Declare a union.
    ///
    /// ```v
    /// union Option<T> { Some(T), None }
    /// ```
    Union,
    /// Declare a micro function.
    ///
    /// ```v
    /// micro add(x: i32, y: i32) -> i32 { x + y }
    /// ```
    Micro,
    /// Declare a mezzo function.
    ///
    /// ```v
    /// mezzo add(x: i32, y: i32) -> i32 { x + y }
    /// ```
    Mezzo,
    /// Declare a macro.
    ///
    /// ```v
    /// macro log(msg: string) { ... }
    /// ```
    Macro,
    /// Declare a widget.
    ///
    /// ```v
    /// widget Button { ... }
    /// ```
    Widget,
    /// Declare a variable.
    ///
    /// ```v
    /// let x = 42;
    /// ```
    Let,
    /// If expression.
    ///
    /// ```v
    /// if condition { ... } else { ... }
    /// ```
    If,
    /// Else clause.
    Else,
    /// Match expression.
    ///
    /// ```v
    /// match x { 1 => 2, _ => 0 }
    /// ```
    Match,
    /// Match case.
    Case,
    /// Match guard.
    ///
    /// ```v
    /// match x { y when y > 0 => y }
    /// ```
    When,
    /// Try expression.
    ///
    /// ```v
    /// try expression catch { pattern => expression }
    /// ```
    Try,
    /// Lambda expression.
    ///
    /// ```v
    /// lambda(x) { x + 1 }
    /// ```
    Lambda,
    /// Catch clause.
    Catch,
    /// While loop.
    ///
    /// ```v
    /// while true { ... }
    /// ```
    While,
    /// Loop.
    ///
    /// ```v
    /// loop { ... }
    /// loop i in list { ... }
    /// ```
    Loop,
    /// For loop.
    ///
    /// ```v
    /// for i in 0..10 { ... }
    /// ```
    For,
    /// Return from function.
    ///
    /// ```v
    /// return x;
    /// ```
    Return,
    /// Break from loop.
    ///
    /// ```v
    /// break @label value;
    /// ```
    Break,
    /// Continue to next iteration.
    ///
    /// ```v
    /// continue @label;
    /// ```
    Continue,
    /// Boolean true.
    True,
    /// Boolean false.
    False,
    /// Null value.
    Null,
    /// Yield value from generator.
    ///
    /// ```v
    /// yield x;
    /// ```
    Yield,
    /// In clause for loops.
    ///
    /// ```v
    /// for i in 0..10 { ... };
    /// ```
    In,
    /// Raise an error.
    ///
    /// ```v
    /// raise error;
    /// ```
    Raise,
    /// Effect declaration.
    ///
    /// ```v
    /// effect Console { micro log(msg: string) }
    /// ```
    Effect,
    /// Resume from an effect operation.
    ///
    /// ```v
    /// resume value;
    /// ```
    Resume,
    /// From keyword for yield from.
    ///
    /// ```v
    /// yield from list;
    /// ```
    From,
    /// Mutable variable.
    ///
    /// ```v
    /// let mut x = 1;
    /// ```
    Mut,
    /// Type check.
    ///
    /// ```v
    /// if x is T { ... }
    /// ```
    Is,
    /// Type pattern in match.
    ///
    /// ```v
    /// match x { type T => ... }
    /// ```
    ///
    /// ```v
    /// match x { type T => ... }
    /// ```
    Type,
    /// Type casting.
    ///
    /// ```v
    /// x as T
    /// ```
    As,
}
