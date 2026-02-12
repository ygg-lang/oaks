/// Javadoc parser module.
pub mod element_type;

use crate::{
    language::JavadocLanguage,
    lexer::{JavadocLexer, token_type::JavadocTokenType},
    parser::element_type::JavadocElementType,
};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, JavadocLanguage, S>;
type ET = JavadocElementType;
type TT = JavadocTokenType;

/// A parser for Javadoc comments.
pub struct JavadocParser<'config> {
    pub(crate) config: &'config JavadocLanguage,
}

impl<'config> JavadocParser<'config> {
    /// Creates a new `JavadocParser` with the given configuration.
    pub fn new(config: &'config JavadocLanguage) -> Self {
        Self { config }
    }

    /// Parses the main description part of a Javadoc comment.
    fn parse_description<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        let mut has_content = false;

        while state.not_at_end() {
            if self.is_at_block_tag(state) {
                break;
            }

            if state.at(TT::CommentEnd) {
                break;
            }

            if state.at(TT::LeftBrace) {
                self.parse_inline_tag(state);
                has_content = true;
            }
            else {
                state.bump();
                has_content = true;
            }
        }

        if has_content {
            state.finish_at(checkpoint, ET::Description);
        }
    }

    /// Checks if the parser is at a block tag.
    fn is_at_block_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        // A block tag starts with @ at the beginning of a line (ignoring whitespace and *)
        if let Some(k) = state.peek_kind() {
            if matches!(k, TT::ParamTag | TT::ReturnTag | TT::ThrowsTag | TT::ExceptionTag | TT::SeeTag | TT::SinceTag | TT::VersionTag | TT::AuthorTag | TT::DeprecatedTag | TT::Tag) {
                // Check if it's effectively at the start of a line
                // In a real Javadoc, it's preceded by Newline and optional Whitespace/Asterisk
                return true;
            }
        }
        false
    }

    /// Parses a block tag.
    fn parse_block_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // The tag itself

        // Parse the rest of the line as tag content
        while state.not_at_end() && !state.at(TT::Newline) && !state.at(TT::CommentEnd) {
            if state.at(TT::LeftBrace) {
                self.parse_inline_tag(state);
            }
            else {
                state.bump();
            }
        }

        state.finish_at(checkpoint, ET::BlockTag);
    }

    /// Parses an inline tag like {@link ...}.
    fn parse_inline_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // {

        while state.not_at_end() && !state.at(TT::RightBrace) && !state.at(TT::CommentEnd) {
            state.bump();
        }

        if state.at(TT::RightBrace) {
            state.bump();
        }

        state.finish_at(checkpoint, ET::InlineTag);
    }
}

impl<'config> Parser<JavadocLanguage> for JavadocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavadocLanguage>) -> ParseOutput<'a, JavadocLanguage> {
        let lexer = JavadocLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                if state.at(TT::CommentStart) {
                    let comment_checkpoint = state.checkpoint();
                    state.bump();

                    // Parse description
                    self.parse_description(state);

                    // Parse block tags
                    while state.not_at_end() && !state.at(TT::CommentEnd) {
                        if self.is_at_block_tag(state) {
                            self.parse_block_tag(state);
                        }
                        else {
                            state.bump();
                        }
                    }

                    if state.at(TT::CommentEnd) {
                        state.bump();
                    }

                    state.finish_at(comment_checkpoint, ET::Comment);
                }
                else {
                    state.advance();
                }
            }

            Ok(state.finish_at(checkpoint, ET::Root))
        })
    }
}
