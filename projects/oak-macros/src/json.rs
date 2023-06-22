use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Lit, LitBool, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

pub fn json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as JsonExpr);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}

enum JsonExpr {
    Null,
    Bool(bool),
    Number(Expr),
    String(LitStr),
    Array(Vec<JsonExpr>),
    Object(Vec<(String, JsonExpr)>),
    Expr(Expr),
}

impl Parse for JsonExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Ident) {
            let ident = input.fork().parse::<syn::Ident>()?;
            if ident == "null" {
                input.parse::<syn::Ident>()?;
                return Ok(JsonExpr::Null);
            }
        }

        if input.peek(LitBool) {
            let b = input.parse::<LitBool>()?;
            Ok(JsonExpr::Bool(b.value))
        }
        else if input.peek(LitStr) {
            Ok(JsonExpr::String(input.parse()?))
        }
        else if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let mut fields = Vec::new();
            while !content.is_empty() {
                let key = if content.peek(LitStr) {
                    content.parse::<LitStr>()?.value()
                }
                else if content.peek(syn::Ident) {
                    content.parse::<syn::Ident>()?.to_string()
                }
                else {
                    return Err(content.error("expected string or identifier as key"));
                };
                content.parse::<Token![:]>()?;
                let value = content.parse::<JsonExpr>()?;
                fields.push((key, value));
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(JsonExpr::Object(fields))
        }
        else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut elements = Vec::new();
            while !content.is_empty() {
                elements.push(content.parse::<JsonExpr>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(JsonExpr::Array(elements))
        }
        else {
            // Check if it's a number literal or a complex expression
            let fork = input.fork();
            if let Ok(Lit::Float(f)) = fork.parse::<Lit>() {
                input.parse::<Lit>()?;
                Ok(JsonExpr::Number(syn::parse_quote!(#f)))
            }
            else if let Ok(Lit::Int(i)) = fork.parse::<Lit>() {
                input.parse::<Lit>()?;
                Ok(JsonExpr::Number(syn::parse_quote!(#i)))
            }
            else {
                Ok(JsonExpr::Expr(input.parse()?))
            }
        }
    }
}

impl ToTokens for JsonExpr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            JsonExpr::Null => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::Null(::oak_json::ast::JsonNull { span: (0..0).into() })
                });
            }
            JsonExpr::Bool(b) => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::Boolean(::oak_json::ast::JsonBoolean { value: #b, span: (0..0).into() })
                });
            }
            JsonExpr::Number(expr) => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::from(#expr)
                });
            }
            JsonExpr::String(lit) => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::from(#lit.to_string())
                });
            }
            JsonExpr::Array(elements) => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::Array(::oak_json::ast::JsonArray {
                        elements: vec![#(#elements),*],
                        span: (0..0).into(),
                    })
                });
            }
            JsonExpr::Object(fields) => {
                let fields = fields.iter().map(|(key, value)| {
                    quote! {
                        ::oak_json::ast::JsonField {
                            name: ::oak_json::ast::JsonString { value: #key.to_string(), span: (0..0).into() },
                            value: #value,
                            span: (0..0).into(),
                        }
                    }
                });
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::Object(::oak_json::ast::JsonObject {
                        fields: vec![#(#fields),*],
                        span: (0..0).into(),
                    })
                });
            }
            JsonExpr::Expr(expr) => {
                tokens.extend(quote! {
                    ::oak_json::ast::JsonValue::from(#expr)
                });
            }
        }
    }
}
