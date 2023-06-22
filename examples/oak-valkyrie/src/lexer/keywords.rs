use serde::{Deserialize, Serialize};

/// Keywords or soft keywords
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ValkyrieKeywords {
    /// Declare a namespace in Valkyrie.
    ///
    /// ```v
    /// namespace package::module::path
    /// ```
    Namespace,
    /// Import a declaration from another namespace.
    ///
    /// ```v
    /// using package::module::path;
    /// ```
    Using,
    ///
    Class,
    Trait,
    Union,
    Micro,
    Mezzo,
    Macro,
    Fn,
    Let,
    If,
    Else,
    Match,
    Case,
    When,
    Try,
    Catch,
    While,
    For,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
}
