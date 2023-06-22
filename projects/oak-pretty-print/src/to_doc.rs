use crate::Document;
use alloc::{boxed::Box, string::String, vec::Vec};

/// A trait for types that can be converted to a document for pretty printing.
pub trait AsDocument {
    /// Converts this type to a document for pretty printing.
    fn as_document(&self) -> Document<'_>;
}

/// A trait for types that can be converted to a document value, potentially consuming the input.
pub trait ToDocument<'a> {
    /// Converts this type to a document value.
    fn to_document(self) -> Document<'a>;
}

impl AsDocument for String {
    fn as_document(&self) -> Document<'_> {
        Document::Text(self.as_str().into())
    }
}

impl AsDocument for str {
    fn as_document(&self) -> Document<'_> {
        Document::Text(self.into())
    }
}

impl<'a> AsDocument for Document<'a> {
    fn as_document(&self) -> Document<'_> {
        self.clone()
    }
}

impl<T: AsDocument> AsDocument for Vec<T> {
    fn as_document(&self) -> Document<'_> {
        Document::Concat(self.iter().map(|t| t.as_document()).collect())
    }
}

impl<T: AsDocument> AsDocument for Option<T> {
    fn as_document(&self) -> Document<'_> {
        match self {
            Some(t) => t.as_document(),
            None => Document::Nil,
        }
    }
}

impl<T: AsDocument + ?Sized> AsDocument for &T {
    fn as_document(&self) -> Document<'_> {
        (**self).as_document()
    }
}

impl<T: AsDocument + ?Sized> AsDocument for Box<T> {
    fn as_document(&self) -> Document<'_> {
        self.as_ref().as_document()
    }
}

impl<'a> ToDocument<'a> for Document<'a> {
    fn to_document(self) -> Document<'a> {
        self
    }
}

impl<'a, T: AsDocument + ?Sized> ToDocument<'a> for &'a T {
    fn to_document(self) -> Document<'a> {
        self.as_document()
    }
}

impl<'a> ToDocument<'a> for String {
    fn to_document(self) -> Document<'a> {
        Document::Text(self.into())
    }
}
