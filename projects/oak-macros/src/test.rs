use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitInt, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

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
            use oak_diagnostic::testing::lexing::LexerTester;
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
            use oak_diagnostic::testing::parsing::ParserTester;
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
