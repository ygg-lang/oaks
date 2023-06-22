use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Ident, LitStr, Result, Token, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

pub fn doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DocExpr);
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}

pub enum DocExpr {
    Nil,
    Line,
    SoftLine,
    SoftLineSpace,
    HardLine,
    Indent(Box<DocExpr>),
    Group(Box<DocExpr>),
    Concat(Vec<DocExpr>),
    Join { items: Expr, sep: Box<DocExpr> },
    Text(LitStr),
    Expr(Expr),
}

impl Parse for DocExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let first = Self::parse_single(input)?;
        if input.peek(Token![,]) {
            let mut items = vec![first];
            while input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                if input.is_empty() {
                    break;
                }
                items.push(Self::parse_single(input)?);
            }
            Ok(DocExpr::Concat(items))
        }
        else {
            Ok(first)
        }
    }
}

impl DocExpr {
    fn parse_single(input: ParseStream) -> Result<Self> {
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
                "nil" => {
                    input.parse::<Ident>()?;
                    Ok(DocExpr::Nil)
                }
                "line" => {
                    input.parse::<Ident>()?;
                    Ok(DocExpr::Line)
                }
                "soft_line" => {
                    input.parse::<Ident>()?;
                    Ok(DocExpr::SoftLine)
                }
                "soft_line_space" => {
                    input.parse::<Ident>()?;
                    Ok(DocExpr::SoftLineSpace)
                }
                "hard_line" => {
                    input.parse::<Ident>()?;
                    Ok(DocExpr::HardLine)
                }
                "indent" => {
                    input.parse::<Ident>()?;
                    let content;
                    syn::parenthesized!(content in input);
                    Ok(DocExpr::Indent(Box::new(content.parse()?)))
                }
                "group" => {
                    input.parse::<Ident>()?;
                    let content;
                    syn::parenthesized!(content in input);
                    Ok(DocExpr::Group(Box::new(content.parse()?)))
                }
                "join" => {
                    input.parse::<Ident>()?;
                    let content;
                    syn::parenthesized!(content in input);
                    let items = content.parse()?;
                    content.parse::<Token![,]>()?;
                    let sep = content.parse()?;
                    Ok(DocExpr::Join { items, sep: Box::new(sep) })
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

impl ToTokens for DocExpr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            DocExpr::Nil => tokens.extend(quote! { ::oak_pretty_print::Doc::Nil }),
            DocExpr::Line => tokens.extend(quote! { ::oak_pretty_print::Doc::Line }),
            DocExpr::SoftLine => tokens.extend(quote! { ::oak_pretty_print::Doc::SoftLine }),
            DocExpr::SoftLineSpace => tokens.extend(quote! { ::oak_pretty_print::Doc::SoftLineSpace }),
            DocExpr::HardLine => tokens.extend(quote! { ::oak_pretty_print::Doc::HardLine }),
            DocExpr::Indent(content) => tokens.extend(quote! { ::oak_pretty_print::Doc::Indent(Box::new(#content)) }),
            DocExpr::Group(content) => tokens.extend(quote! { ::oak_pretty_print::Doc::Group(Box::new(#content)) }),
            DocExpr::Concat(items) => {
                tokens.extend(quote! { ::oak_pretty_print::Doc::Concat(vec![#(#items),*]) });
            }
            DocExpr::Join { items, sep } => {
                tokens.extend(quote! {
                    ::oak_pretty_print::Doc::Concat({
                        #[allow(unused_imports)]
                        use ::oak_pretty_print::document::JoinDoc;
                        #items.into_iter()
                            .map(|i| {
                                #[allow(unused_imports)]
                                use ::oak_pretty_print::ToDocument;
                                i.to_document()
                            })
                            .collect::<Vec<_>>()
                            .join_doc(#sep.into())
                    })
                });
            }
            DocExpr::Text(lit) => tokens.extend(quote! { ::oak_pretty_print::Doc::Text(#lit.into()) }),
            DocExpr::Expr(expr) => {
                tokens.extend(quote! {
                    {
                        #[allow(unused_imports)]
                        use ::oak_pretty_print::ToDocument;
                        #expr.to_document()
                    }
                });
            }
        }
    }
}
