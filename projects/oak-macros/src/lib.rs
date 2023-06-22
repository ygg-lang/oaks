extern crate proc_macro;

use proc_macro::TokenStream;

mod define_rules;
mod doc;
mod format_rule;
mod json;
mod test;
mod to_doc;

#[proc_macro]
pub fn test_lexer(input: TokenStream) -> TokenStream {
    test::test_lexer(input)
}

#[proc_macro]
pub fn test_parser(input: TokenStream) -> TokenStream {
    test::test_parser(input)
}

#[proc_macro_attribute]
pub fn oak_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    test::oak_test(attr, item)
}

#[proc_macro_derive(AsDocument, attributes(oak))]
pub fn derive_as_document(input: TokenStream) -> TokenStream {
    to_doc::derive_as_document(input)
}

#[proc_macro_derive(FormatRule, attributes(oak))]
pub fn derive_format_rule(input: TokenStream) -> TokenStream {
    format_rule::derive_format_rule(input)
}

#[proc_macro]
pub fn define_rules(input: TokenStream) -> TokenStream {
    define_rules::define_rules(input)
}

#[proc_macro]
pub fn doc(input: TokenStream) -> TokenStream {
    doc::doc(input)
}

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    json::json(input)
}
