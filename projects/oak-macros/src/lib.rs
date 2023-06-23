#![warn(missing_docs)]
//! Procedural macros for the Oak language framework.
//!
//! This crate provides various macros for testing, deriving traits,
//! and defining language rules.

extern crate proc_macro;

use proc_macro::TokenStream;

mod define_rules;
mod doc;
mod format_rule;
mod json;
mod test;
mod to_doc;

/// Generates a lexer test.
#[proc_macro]
pub fn test_lexer(input: TokenStream) -> TokenStream {
    test::test_lexer(input)
}

/// Generates a parser test.
#[proc_macro]
pub fn test_parser(input: TokenStream) -> TokenStream {
    test::test_parser(input)
}

/// Attribute macro for defining Oak tests.
#[proc_macro_attribute]
pub fn oak_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::oak_test(attr, item)
}

/// Derives the `AsDocument` trait.
#[proc_macro_derive(AsDocument, attributes(oak))]
pub fn derive_as_document(input: TokenStream) -> TokenStream {
    to_doc::derive_as_document(input)
}

/// Derives the `FormatRule` trait.
#[proc_macro_derive(FormatRule, attributes(oak))]
pub fn derive_format_rule(input: TokenStream) -> TokenStream {
    format_rule::derive_format_rule(input)
}

/// Macro for defining language rules.
#[proc_macro]
pub fn define_rules(input: TokenStream) -> TokenStream {
    define_rules::define_rules(input)
}

/// Macro for generating documentation from templates.
#[proc_macro]
pub fn doc(input: TokenStream) -> TokenStream {
    doc::doc(input)
}

/// Macro for defining JSON-like structures.
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    json::json(input)
}
