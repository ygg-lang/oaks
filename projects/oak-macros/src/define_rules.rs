use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, Result, Token, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

pub fn define_rules(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RulesInput);
    let mut expanded = quote! {
        let mut rules: Vec<Box<dyn FormatRule<L>>> = Vec::new();
    };

    for rule in input.rules {
        let name = &rule.name;
        let priority = &rule.priority;

        let applies_to_node = if let Some(node) = &rule.node {
            let arg = &node.node_arg;
            let cond = &node.cond;
            quote! {
                fn applies_to_node(&self, _node: &oak_core::tree::RedNode<L>) -> bool {
                    let #arg = _node;
                    #cond
                }
            }
        }
        else {
            quote! {
                fn applies_to_node(&self, _node: &oak_core::tree::RedNode<L>) -> bool {
                    false
                }
            }
        };

        let apply_node = if let Some(node) = &rule.node {
            let node_arg = &node.node_arg;
            let ctx_arg = &node.ctx_arg;
            let source_arg = &node.source_arg;
            let children_arg = &node.children_arg;
            let body = &node.body;
            quote! {
                fn apply_node<'a>(&self, _node: &oak_core::tree::RedNode<L>, _context: &FormatContext<L>, _source: &'a str, _format_children: &dyn Fn(&oak_core::tree::RedNode<L>) -> FormatResult<Doc<'a>>) -> FormatResult<Option<Doc<'a>>> {
                    let #node_arg = _node;
                    let #ctx_arg = _context;
                    let #source_arg = _source;
                    let #children_arg = _format_children;
                    #body
                }
            }
        }
        else {
            quote! {
                fn apply_node<'a>(&self, _node: &oak_core::tree::RedNode<L>, _context: &FormatContext<L>, _source: &'a str, _format_children: &dyn Fn(&oak_core::tree::RedNode<L>) -> FormatResult<Doc<'a>>) -> FormatResult<Option<Doc<'a>>> {
                    Ok(None)
                }
            }
        };

        let applies_to_token = if let Some(token) = &rule.token {
            let arg = &token.token_arg;
            let cond = &token.cond;
            quote! {
                fn applies_to_token(&self, _token: &oak_core::tree::RedLeaf<L>) -> bool {
                    let #arg = _token;
                    #cond
                }
            }
        }
        else {
            quote! {
                fn applies_to_token(&self, _token: &oak_core::tree::RedLeaf<L>) -> bool {
                    false
                }
            }
        };

        let apply_token = if let Some(token) = &rule.token {
            let token_arg = &token.token_arg;
            let ctx_arg = &token.ctx_arg;
            let source_arg = &token.source_arg;
            let body = &token.body;
            quote! {
                fn apply_token<'a>(&self, _token: &oak_core::tree::RedLeaf<L>, _context: &FormatContext<L>, _source: &'a str) -> FormatResult<Option<Doc<'a>>> {
                    let #token_arg = _token;
                    let #ctx_arg = _context;
                    let #source_arg = _source;
                    #body
                }
            }
        }
        else {
            quote! {
                fn apply_token<'a>(&self, _token: &oak_core::tree::RedLeaf<L>, _context: &FormatContext<L>, _source: &'a str) -> FormatResult<Option<Doc<'a>>> {
                    Ok(None)
                }
            }
        };

        expanded.extend(quote! {
            {
                #[allow(non_camel_case_types)]
                struct #name<L: oak_core::language::Language>(core::marker::PhantomData<L>);
                impl<L: oak_core::language::Language> FormatRule<L> for #name<L> {
                    fn name(&self) -> &str { stringify!(#name) }
                    fn priority(&self) -> u8 { #priority }
                    #applies_to_node
                    #apply_node
                    #applies_to_token
                    #apply_token
                }
                rules.push(Box::new(#name(core::marker::PhantomData)));
            }
        });
    }

    let result = quote! {
        {
            #expanded
            rules
        }
    };
    TokenStream::from(result)
}

pub struct RulesInput {
    pub rules: Vec<RuleDefinition>,
}

pub struct RuleDefinition {
    pub name: Ident,
    pub priority: Expr,
    pub node: Option<NodeHandler>,
    pub token: Option<TokenHandler>,
}

pub struct NodeHandler {
    pub node_arg: Ident,
    pub ctx_arg: Ident,
    pub source_arg: Ident,
    pub children_arg: Ident,
    pub cond: Expr,
    pub body: Expr,
}

pub struct TokenHandler {
    pub token_arg: Ident,
    pub ctx_arg: Ident,
    pub source_arg: Ident,
    pub cond: Expr,
    pub body: Expr,
}

impl Parse for RulesInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut rules = Vec::new();
        while !input.is_empty() {
            rules.push(input.parse()?);
        }
        Ok(RulesInput { rules })
    }
}

impl Parse for RuleDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut priority = None;
        let mut node = None;
        let mut token = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            match key.to_string().as_str() {
                "priority" => {
                    content.parse::<Token![:]>()?;
                    priority = Some(content.parse::<Expr>()?);
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                "node" => {
                    let args_content;
                    syn::parenthesized!(args_content in content);
                    let node_arg: Ident = args_content.parse()?;
                    args_content.parse::<Token![,]>()?;
                    let ctx_arg: Ident = args_content.parse()?;
                    args_content.parse::<Token![,]>()?;
                    let source_arg: Ident = args_content.parse()?;
                    args_content.parse::<Token![,]>()?;
                    let children_arg: Ident = args_content.parse()?;

                    content.parse::<Token![if]>()?;
                    let cond: Expr = content.parse()?;
                    content.parse::<Token![=>]>()?;
                    let body: Expr = content.parse()?;
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }

                    node = Some(NodeHandler { node_arg, ctx_arg, source_arg, children_arg, cond, body });
                }
                "token" => {
                    let args_content;
                    syn::parenthesized!(args_content in content);
                    let token_arg: Ident = args_content.parse()?;
                    args_content.parse::<Token![,]>()?;
                    let ctx_arg: Ident = args_content.parse()?;
                    args_content.parse::<Token![,]>()?;
                    let source_arg: Ident = args_content.parse()?;

                    content.parse::<Token![if]>()?;
                    let cond: Expr = content.parse()?;
                    content.parse::<Token![=>]>()?;
                    let body: Expr = content.parse()?;
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }

                    token = Some(TokenHandler { token_arg, ctx_arg, source_arg, cond, body });
                }
                _ => return Err(syn::Error::new(key.span(), "未知规则字段")),
            }
        }

        Ok(RuleDefinition { name, priority: priority.ok_or_else(|| content.error("缺少 priority 字段"))?, node, token })
    }
}
