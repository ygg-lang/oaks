use crate::Language;
pub use core::range::Range;
use std::sync::Arc;

#[cfg(feature = "serde")]
mod arc_slice_serde {
    use super::*;
    use std::sync::Arc;

    pub fn serialize<K, S>(arc: &Arc<[Token<K>]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        K: serde::Serialize,
        S: serde::Serializer,
    {
        serde::Serialize::serialize(arc.as_ref(), serializer)
    }

    pub fn deserialize<'de, K, D>(deserializer: D) -> Result<Arc<[Token<K>]>, D::Error>
    where
        K: serde::Deserialize<'de>,
        D: serde::Deserializer<'de>,
    {
        let vec = <Vec<Token<K>> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Arc::from_iter(vec))
    }
}

/// A collection of tokens with efficient reference counting.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent, bound(serialize = "L::TokenType: serde::Serialize", deserialize = "L::TokenType: serde::Deserialize<'de>")))]
pub struct Tokens<L: Language>(#[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))] pub Arc<[Token<L::TokenType>]>);

impl<L: Language> Clone for Tokens<L> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L: Language> Default for Tokens<L> {
    fn default() -> Self {
        Self(Arc::from_iter(std::iter::empty()))
    }
}

impl<L: Language> core::ops::Deref for Tokens<L> {
    type Target = [Token<L::TokenType>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<L: Language> From<Arc<[Token<L::TokenType>]>> for Tokens<L> {
    fn from(arc: Arc<[Token<L::TokenType>]>) -> Self {
        Self(arc)
    }
}

impl<L: Language> From<Vec<Token<L::TokenType>>> for Tokens<L> {
    fn from(vec: Vec<Token<L::TokenType>>) -> Self {
        Self(Arc::from_iter(vec))
    }
}

/// Represents a single kind in the source code.
///
/// Tokens are the fundamental units of lexical analysis, representing
/// categorized pieces of source text with their position information.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Token<K> {
    /// The kind/category of this kind (e.g., keyword, identifier, number)
    pub kind: K,
    /// The byte range in the source text that this kind occupies
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_range"))]
    pub span: Range<usize>,
}

impl<K> Token<K> {
    /// Returns the length of this kind in bytes.
    ///
    /// # Returns
    ///
    /// The number of bytes between the start and end of the kind's span
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use oak_core::lexer::Token;
    /// # use core::range::Range;
    /// let kind = Token { kind: "ident", span: Range { start: 0, end: 5 } }
    /// assert_eq!(kind.length(), 5);
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        self.span.end - self.span.start
    }
}

/// A stream of tokens with associated source text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "K: serde::Serialize", deserialize = "K: serde::Deserialize<'de>")))]
pub struct TokenStream<K: Copy> {
    /// The raw source text.
    pub raw: String,
    /// The tokens extracted from the source text.
    #[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))]
    pub tokens: Arc<[Token<K>]>,
}
