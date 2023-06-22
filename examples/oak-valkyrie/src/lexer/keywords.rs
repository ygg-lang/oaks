use serde::{Deserialize, Serialize};

/// Keywords or soft keywords
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ValkyrieKeywords {
    /// Declare a namespace in Valkyrie.
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
    /// Declare a trait.
    ///
    /// ```v
    /// trait Show { micro show(self); }
    /// ```
    Trait,
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
    /// Catch clause.
    Catch,
    /// While loop.
    ///
    /// ```v
    /// while true { ... }
    /// ```
    While,
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
    /// Raise an error.
    ///
    /// ```v
    /// raise error;
    /// ```
    Raise,
    /// Mutable variable.
    ///
    /// ```v
    /// let mut x = 1;
    /// ```
    Mut,
}
