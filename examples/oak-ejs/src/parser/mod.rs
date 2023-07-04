/// EJS Parser module
///
/// This module defines the parser for EJS templates, responsible for
/// constructing an abstract syntax tree from the token stream.
pub mod element_type;

use crate::{language::EjsLanguage, lexer::token_type::EjsTokenType, parser::element_type::EjsElementType};
use oak_core::{
    OakError, ParseCache, TextEdit,
    parser::{Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, EjsLanguage, S>;

/// EJS template parser
///
/// This parser processes EJS templates and constructs a syntax tree
/// representing the template structure including text content, code blocks,
/// and output expressions.
pub struct EjsParser<'config> {
    /// Configuration for the EJS language
    pub(crate) config: &'config EjsLanguage,
}

impl<'config> EjsParser<'config> {
    /// Creates a new EJS parser with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the EJS language configuration
    ///
    /// # Returns
    ///
    /// A new `EjsParser` instance
    pub fn new(config: &'config EjsLanguage) -> Self {
        Self { config }
    }

    /// Parses the template content
    ///
    /// This method handles the main parsing loop, processing text content
    /// and EJS tags until the end of the source is reached.
    fn parse_template<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            match state.peek_kind() {
                Some(EjsTokenType::Text) => {
                    self.parse_text(state);
                }
                Some(EjsTokenType::OpenTag) => {
                    self.parse_code_block(state)?;
                }
                Some(EjsTokenType::OpenTagOutputEscape) => {
                    self.parse_output_escape(state)?;
                }
                Some(EjsTokenType::OpenTagOutputRaw) => {
                    self.parse_output_raw(state)?;
                }
                Some(EjsTokenType::OpenTagComment) => {
                    self.parse_comment(state)?;
                }
                Some(EjsTokenType::EscapedOpenTag) => {
                    self.parse_escaped_tag(state)?;
                }
                _ => {
                    state.advance();
                }
            }
        }

        Ok(())
    }

    /// Parses plain text content
    ///
    /// Text content is any content outside of EJS tags.
    fn parse_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump();
        state.finish_at(cp, EjsElementType::Text);
    }

    /// Parses a code block `<% ... %>`
    ///
    /// Code blocks contain JavaScript code that is executed but not output.
    fn parse_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(EjsTokenType::CloseTag) | Some(EjsTokenType::CloseTagTrim) => {
                    state.bump();
                    break;
                }
                _ => {
                    state.advance();
                }
            }
        }

        state.finish_at(cp, EjsElementType::Code);
        Ok(())
    }

    /// Parses an escaped output expression `<%= ... %>`
    ///
    /// The expression is evaluated and the result is HTML-escaped before output.
    fn parse_output_escape<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(EjsTokenType::CloseTag) | Some(EjsTokenType::CloseTagTrim) => {
                    state.bump();
                    break;
                }
                _ => {
                    state.advance();
                }
            }
        }

        state.finish_at(cp, EjsElementType::OutputEscape);
        Ok(())
    }

    /// Parses a raw output expression `<%- ... %>`
    ///
    /// The expression is evaluated and the result is output without escaping.
    fn parse_output_raw<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(EjsTokenType::CloseTag) | Some(EjsTokenType::CloseTagTrim) => {
                    state.bump();
                    break;
                }
                _ => {
                    state.advance();
                }
            }
        }

        state.finish_at(cp, EjsElementType::OutputRaw);
        Ok(())
    }

    /// Parses a comment `<%# ... %>`
    ///
    /// Comments are not rendered in the output.
    fn parse_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(EjsTokenType::CloseTag) | Some(EjsTokenType::CloseTagTrim) => {
                    state.bump();
                    break;
                }
                _ => {
                    state.advance();
                }
            }
        }

        state.finish_at(cp, EjsElementType::Comment);
        Ok(())
    }

    /// Parses an escaped tag `<%%`
    ///
    /// Escaped tags are rendered as literal `<%` in the output.
    fn parse_escaped_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump();
        state.finish_at(cp, EjsElementType::EscapedTag);
        Ok(())
    }
}

impl<'config> Parser<EjsLanguage> for EjsParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<EjsLanguage>) -> oak_core::parser::ParseOutput<'a, EjsLanguage> {
        let lexer = crate::lexer::EjsLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            self.parse_template(state)?;
            Ok(state.finish_at(cp, EjsElementType::Root))
        })
    }
}
