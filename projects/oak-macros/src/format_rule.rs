use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitInt, parse_macro_input};

pub fn derive_format_rule(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut priority = 0u8;
    for attr in &input.attrs {
        if attr.path().is_ident("oak") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("priority") {
                    let value = meta.value()?;
                    let lit: LitInt = value.parse()?;
                    priority = lit.base10_parse()?;
                }
                Ok(())
            });
        }
    }

    let name_str = name.to_string();

    let expanded = quote! {
        impl #impl_generics ::oak_pretty_print::FormatRule<L> for #name #ty_generics #where_clause {
            fn name(&self) -> &str { #name_str }
            fn priority(&self) -> u8 { #priority }
        }
    };

    TokenStream::from(expanded)
}
