extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, LitInt, LitStr, Result, Token, braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro]
pub fn test_lexer(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TestConfig);
    let name = &config.name;
    let language = &config.language;
    let lexer = &config.lexer;
    let extension = &config.extension;
    let path = &config.path;
    let timeout = config.timeout.unwrap_or(10);

    let expanded = quote! {
        #[test]
        fn #name() -> Result<(), oak_core::OakError> {
            use oak_core::helpers::LexerTester;
            use std::{path::Path, time::Duration};

            let here = Path::new(env!("CARGO_MANIFEST_DIR"));
            let language = Box::leak(Box::new(#language::default()));
            let lexer = #lexer::new(language);
            let test_runner = LexerTester::new(here.join(#path))
                .with_extension(#extension)
                .with_timeout(Duration::from_secs(#timeout));

            test_runner.run_tests::<#language, _>(lexer)
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn test_parser(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TestConfig);
    let name = &config.name;
    let language = &config.language;
    let parser = &config.lexer; // reusing the same field for parser
    let extension = &config.extension;
    let path = &config.path;
    let timeout = config.timeout.unwrap_or(10);

    let expanded = quote! {
        #[test]
        fn #name() -> Result<(), oak_core::OakError> {
            use oak_core::helpers::ParserTester;
            use std::{path::Path, time::Duration};

            let here = Path::new(env!("CARGO_MANIFEST_DIR"));
            let lang = Box::leak(Box::new(#language::default()));
            let parser = Box::leak(Box::new(#parser::new(lang)));
            let test_runner = ParserTester::new(here.join(#path))
                .with_extension(#extension)
                .with_timeout(Duration::from_secs(#timeout));

            test_runner.run_tests::<#language, _>(parser)
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn oak_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    let expanded = quote! {
        #(#attrs)*
        #[test]
        #vis #sig {
            use std::sync::mpsc;
            use std::time::Duration;
            use std::thread;

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let result = (move || #body)();
                let _ = tx.send(result);
            });

            match rx.recv_timeout(Duration::from_secs(10)) {
                Ok(result) => result,
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    panic!("Test timed out after 10 seconds. Possible infinite loop detected in parser.");
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("Test thread panicked or disconnected unexpectedly.");
                }
            }
        }
    };
    TokenStream::from(expanded)
}

struct TestConfig {
    name: Ident,
    language: Ident,
    lexer: Ident,
    extension: LitStr,
    path: LitStr,
    timeout: Option<u64>,
}

impl Parse for TestConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = None;
        let mut language = None;
        let mut lexer = None;
        let mut extension = None;
        let mut path = None;
        let mut timeout = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "name" => name = Some(input.parse()?),
                "language" => language = Some(input.parse()?),
                "lexer" | "parser" => lexer = Some(input.parse()?),
                "extension" => extension = Some(input.parse()?),
                "path" => path = Some(input.parse()?),
                "timeout" => {
                    let lit: LitInt = input.parse()?;
                    timeout = Some(lit.base10_parse()?);
                }
                _ => return Err(syn::Error::new(key.span(), format!("未知字段: {}", key))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(TestConfig {
            name: name.ok_or_else(|| input.error("缺少字段: name"))?,
            language: language.ok_or_else(|| input.error("缺少字段: language"))?,
            lexer: lexer.ok_or_else(|| input.error("缺少字段: lexer/parser"))?,
            extension: extension.ok_or_else(|| input.error("缺少字段: extension"))?,
            path: path.ok_or_else(|| input.error("缺少字段: path"))?,
            timeout,
        })
    }
}

#[proc_macro]
pub fn doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DocExpr);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}

enum DocExpr {
    Nil,
    Line,
    SoftLine,
    SoftLineSpace,
    HardLine,
    Indent(Box<DocExpr>),
    Group(Box<DocExpr>),
    Concat(Vec<DocExpr>),
    Text(LitStr),
    Expr(Expr),
}

impl Parse for DocExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse::<DocExpr>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(DocExpr::Concat(items))
        }
        else if input.peek(Ident) {
            let lookahead = input.fork();
            let ident = lookahead.parse::<Ident>()?;
            match ident.to_string().as_str() {
                "nil" | "line" | "soft_line" | "soft_line_space" | "hard_line" => {
                    input.parse::<Ident>()?; // consume
                    match ident.to_string().as_str() {
                        "nil" => Ok(DocExpr::Nil),
                        "line" => Ok(DocExpr::Line),
                        "soft_line" => Ok(DocExpr::SoftLine),
                        "soft_line_space" => Ok(DocExpr::SoftLineSpace),
                        "hard_line" => Ok(DocExpr::HardLine),
                        _ => unreachable!(),
                    }
                }
                "indent" => {
                    let ident = input.parse::<Ident>()?; // consume
                    if input.peek(syn::token::Brace) {
                        let content;
                        braced!(content in input);
                        let inner = content.parse::<DocExpr>().map_err(|mut e| {
                            let new_error = syn::Error::new(ident.span(), "indent 内部语法错误");
                            e.combine(new_error);
                            e
                        })?;
                        if !content.is_empty() {
                            return Err(content.error("indent 只接受一个表达式，多个表达式请使用 [] 包裹"));
                        }
                        Ok(DocExpr::Indent(Box::new(inner)))
                    }
                    else {
                        Ok(DocExpr::Indent(Box::new(input.parse()?)))
                    }
                }
                "group" => {
                    let ident = input.parse::<Ident>()?; // consume
                    if input.peek(syn::token::Brace) {
                        let content;
                        braced!(content in input);
                        let inner = content.parse::<DocExpr>().map_err(|mut e| {
                            let new_error = syn::Error::new(ident.span(), "group 内部语法错误");
                            e.combine(new_error);
                            e
                        })?;
                        if !content.is_empty() {
                            return Err(content.error("group 只接受一个表达式，多个表达式请使用 [] 包裹"));
                        }
                        Ok(DocExpr::Group(Box::new(inner)))
                    }
                    else {
                        Ok(DocExpr::Group(Box::new(input.parse()?)))
                    }
                }
                _ => Ok(DocExpr::Expr(input.parse()?)),
            }
        }
        else if input.peek(LitStr) {
            Ok(DocExpr::Text(input.parse()?))
        }
        else {
            Ok(DocExpr::Expr(input.parse()?))
        }
    }
}

impl quote::ToTokens for DocExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_path = quote! { ::oak_pretty_print };
        match self {
            DocExpr::Nil => tokens.extend(quote! { #crate_path::Doc::Nil }),
            DocExpr::Line => tokens.extend(quote! { #crate_path::Doc::Line }),
            DocExpr::SoftLine => tokens.extend(quote! { #crate_path::Doc::SoftLine }),
            DocExpr::SoftLineSpace => tokens.extend(quote! { #crate_path::Doc::SoftLineSpace }),
            DocExpr::HardLine => tokens.extend(quote! { #crate_path::Doc::HardLine }),
            DocExpr::Indent(inner) => tokens.extend(quote! { #crate_path::Doc::Indent(Box::new(#inner)) }),
            DocExpr::Group(inner) => tokens.extend(quote! { #crate_path::Doc::Group(Box::new(#inner)) }),
            DocExpr::Concat(items) => {
                tokens.extend(quote! {
                    #crate_path::Doc::Concat(vec![
                        #( #items ),*
                    ])
                });
            }
            DocExpr::Text(lit) => tokens.extend(quote! { #crate_path::Doc::Text(#lit.to_string()) }),
            DocExpr::Expr(expr) => tokens.extend(quote! { #expr }),
        }
    }
}
