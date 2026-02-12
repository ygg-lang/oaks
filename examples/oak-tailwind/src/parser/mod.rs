//! Parser implementation for Tailwind DSL.
/// Element types for the Tailwind language.
pub mod element_type;

use crate::{
    ast::{TailwindArbitraryValue, TailwindClass, TailwindClassKind, TailwindComment, TailwindDirective, TailwindModifier, TailwindNode, TailwindRoot, TailwindUtility},
    language::TailwindLanguage,
    lexer::{TailwindLexer, token_type::TailwindTokenType},
    parser::element_type::TailwindElementType,
};
use core::range::Range;
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, TailwindLanguage, S>;

/// Parser for the Tailwind language.
#[derive(Debug, Clone, Copy, Default)]
pub struct TailwindParser {
    /// Language configuration
    pub config: TailwindLanguage,
}

impl TailwindParser {
    /// Creates a new `TailwindParser` with the given configuration.
    pub fn new(config: TailwindLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<Option<TailwindNode>, OakError> {
        match state.peek_kind() {
            Some(TailwindTokenType::Directive) => {
                let directive = self.parse_directive(state)?;
                Ok(Some(TailwindNode::Directive(directive)))
            }
            Some(TailwindTokenType::Modifier) | Some(TailwindTokenType::Utility) | Some(TailwindTokenType::Important) | Some(TailwindTokenType::ArbitraryValue) => {
                let class = self.parse_class(state)?;
                Ok(Some(TailwindNode::Class(class)))
            }
            Some(TailwindTokenType::Comment) => {
                let token = state.current().unwrap();
                let start_pos = token.span.start;
                let content = state.source.get_text_in(token.span).to_string();

                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, TailwindElementType::Comment);

                let end_pos = state.current_offset();
                Ok(Some(TailwindNode::Comment(TailwindComment { span: Range { start: start_pos, end: end_pos }, content })))
            }
            _ => {
                state.advance();
                Ok(None)
            }
        }
    }

    /// Parses a Tailwind class (e.g., hover:bg-red-500, !p-4).
    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<TailwindClass, OakError> {
        let checkpoint = state.checkpoint();
        let start_pos = state.current_offset();
        let mut is_important = false;

        // Optional important flag at start
        if state.eat(TailwindTokenType::Important) {
            is_important = true;
        }

        let mut modifiers = Vec::new();
        // Zero or more modifiers
        while state.at(TailwindTokenType::Modifier) {
            let mod_token = state.current().unwrap();
            let mod_range = mod_token.span;
            let mod_text = state.source.get_text_in(mod_range).to_string();

            let mod_cp = state.checkpoint();
            state.bump();
            state.finish_at(mod_cp, TailwindElementType::Modifier);

            modifiers.push(TailwindModifier { span: mod_range, name: mod_text });
        }

        // Utility or arbitrary value
        let kind = if state.at(TailwindTokenType::Utility) {
            let util_token = state.current().unwrap();
            let util_range = util_token.span;
            let util_text = state.source.get_text_in(util_range).to_string();

            let util_cp = state.checkpoint();
            state.bump();
            state.finish_at(util_cp, TailwindElementType::Utility);

            TailwindClassKind::Utility(TailwindUtility { span: util_range, name: util_text })
        }
        else if state.at(TailwindTokenType::ArbitraryValue) {
            let arb_token = state.current().unwrap();
            let arb_range = arb_token.span;
            let arb_text = state.source.get_text_in(arb_range).to_string();

            let arb_cp = state.checkpoint();
            state.bump();
            state.finish_at(arb_cp, TailwindElementType::ArbitraryValue);

            TailwindClassKind::ArbitraryValue(TailwindArbitraryValue { span: arb_range, value: arb_text })
        }
        else {
            // Fallback for incomplete class
            TailwindClassKind::Utility(TailwindUtility { span: Range { start: state.current_offset(), end: state.current_offset() }, name: String::new() })
        };

        // Optional important flag at end
        if state.eat(TailwindTokenType::Important) {
            is_important = true;
        }

        state.finish_at(checkpoint, TailwindElementType::Class);
        let end_pos = state.current_offset();

        Ok(TailwindClass { span: Range { start: start_pos, end: end_pos }, is_important, modifiers, kind })
    }

    /// Parses a directive (e.g., @tailwind base).
    fn parse_directive<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<TailwindDirective, OakError> {
        let checkpoint = state.checkpoint();
        let start_pos = state.current_offset();

        let dir_token = state.current().unwrap();
        let name = state.source.get_text_in(dir_token.span).to_string();
        state.expect(TailwindTokenType::Directive)?;

        let mut body_parts = Vec::new();
        // Consume anything until semicolon or end of line/file
        while state.not_at_end() && !state.at(TailwindTokenType::Semicolon) {
            if let Some(token) = state.current() {
                body_parts.push(state.source.get_text_in(token.span).to_string());
            }
            state.bump();
        }

        let body = if body_parts.is_empty() { None } else { Some(body_parts.join("")) };

        state.eat(TailwindTokenType::Semicolon);
        state.finish_at(checkpoint, TailwindElementType::Directive);
        let end_pos = state.current_offset();

        Ok(TailwindDirective { span: Range { start: start_pos, end: end_pos }, name, body })
    }
}

impl Parser<TailwindLanguage> for TailwindParser {
    /// Parses the source text into a Tailwind syntax tree.
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TailwindLanguage>) -> ParseOutput<'a, TailwindLanguage> {
        let lexer = TailwindLexer::new(self.config);
        let mut ast_nodes = Vec::new();

        let output = parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            let start_pos = state.current_offset();

            while state.not_at_end() {
                if let Ok(Some(node)) = self.parse_node(state) {
                    ast_nodes.push(node);
                }
            }

            let root_range = Range { start: start_pos, end: state.current_offset() };
            let _root_ast = TailwindRoot::new(root_range, ast_nodes);
            // Note: In a real implementation, we might want to store the AST somewhere or return it.
            // For now, we follow the Oak pattern of focusing on the GreenNode.

            Ok(state.finish_at(checkpoint, TailwindElementType::Root))
        });

        output
    }
}
