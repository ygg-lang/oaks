use crate::{language::AsciiDocLanguage, lexer::token_type::AsciiDocTokenType, parser::element_type::AsciiDocElementType as ET};
use oak_core::{
    OakError,
    parser::{Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// AsciiDoc element types and role definitions.
pub mod element_type;

pub use element_type::AsciiDocElementType;

/// State alias for the AsciiDoc parser.
pub(crate) type State<'a, S> = ParserState<'a, AsciiDocLanguage, S>;

/// Parser for the AsciiDoc language.
pub struct AsciiDocParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config AsciiDocLanguage,
}

impl<'config> AsciiDocParser<'config> {
    /// Creates a new `AsciiDocParser` instance.
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
        Self { config }
    }

    fn is_block_start(&self, kind: AsciiDocTokenType) -> bool {
        matches!(
            kind,
            AsciiDocTokenType::Header1
                | AsciiDocTokenType::Header2
                | AsciiDocTokenType::Header3
                | AsciiDocTokenType::Header4
                | AsciiDocTokenType::Header5
                | AsciiDocTokenType::Header6
                | AsciiDocTokenType::ListMarker
                | AsciiDocTokenType::CodeBlockMarker
                | AsciiDocTokenType::AttributeMarker
                | AsciiDocTokenType::AdmonitionMarker
                | AsciiDocTokenType::PageBreak
        )
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return Ok(()),
        };

        match kind {
            AsciiDocTokenType::Header1 | AsciiDocTokenType::Header2 | AsciiDocTokenType::Header3 | AsciiDocTokenType::Header4 | AsciiDocTokenType::Header5 | AsciiDocTokenType::Header6 => self.parse_header(state),
            AsciiDocTokenType::ListMarker => self.parse_list(state),
            AsciiDocTokenType::CodeBlockMarker => self.parse_code_block(state),
            AsciiDocTokenType::AttributeMarker => self.parse_attribute(state),
            AsciiDocTokenType::AdmonitionMarker => self.parse_admonition(state),
            AsciiDocTokenType::PageBreak => {
                let checkpoint = state.checkpoint();
                state.bump();
                state.finish_at(checkpoint, ET::PageBreak);
                Ok(())
            }
            AsciiDocTokenType::Newline => {
                state.bump();
                Ok(())
            }
            AsciiDocTokenType::Whitespace => {
                state.bump();
                Ok(())
            }
            _ => self.parse_paragraph(state),
        }
    }

    fn parse_header<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        let kind = state.peek_kind().unwrap();
        state.bump(); // bump marker

        // Parse title content until newline
        self.parse_inlines_until_newline(state);

        state.finish_at(checkpoint, ET::from(kind));
        Ok(())
    }

    fn parse_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        while state.at(AsciiDocTokenType::ListMarker) {
            let item_checkpoint = state.checkpoint();
            state.bump(); // marker

            // Parse item content
            while state.not_at_end() {
                if state.at(AsciiDocTokenType::Newline) {
                    let cp = state.checkpoint();
                    state.bump();
                    if state.at(AsciiDocTokenType::Newline) || self.is_block_start(state.peek_kind().unwrap_or(AsciiDocTokenType::Eof)) {
                        state.restore(cp);
                        break;
                    }
                }
                else if self.is_block_start(state.peek_kind().unwrap_or(AsciiDocTokenType::Eof)) {
                    break;
                }
                self.parse_inline(state);
            }

            state.finish_at(item_checkpoint, ET::ListItem);
        }

        state.finish_at(checkpoint, ET::List);
        Ok(())
    }

    fn parse_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // start marker

        while state.not_at_end() && !state.at(AsciiDocTokenType::CodeBlockMarker) {
            state.bump();
        }

        if state.at(AsciiDocTokenType::CodeBlockMarker) {
            state.bump(); // end marker
        }

        state.finish_at(checkpoint, ET::CodeBlock);
        Ok(())
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // marker

        self.parse_inlines_until_newline(state);

        state.finish_at(checkpoint, ET::Attribute);
        Ok(())
    }

    fn parse_admonition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // marker

        self.parse_paragraph(state)?;

        state.finish_at(checkpoint, ET::Admonition);
        Ok(())
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind == AsciiDocTokenType::Newline {
                    let cp = state.checkpoint();
                    state.bump();
                    if let Some(next) = state.peek_kind() {
                        if next == AsciiDocTokenType::Newline || self.is_block_start(next) {
                            state.restore(cp);
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
                else if self.is_block_start(kind) {
                    break;
                }
            }
            self.parse_inline(state);
        }

        state.finish_at(checkpoint, ET::Paragraph);
        Ok(())
    }

    fn parse_inlines_until_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() && !state.at(AsciiDocTokenType::Newline) {
            self.parse_inline(state);
        }
    }

    fn parse_inline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return,
        };

        match kind {
            AsciiDocTokenType::BoldMarker | AsciiDocTokenType::ItalicMarker | AsciiDocTokenType::MonospaceMarker => {
                let marker_kind = kind;
                state.bump(); // Start marker
                while state.not_at_end() && !state.at(marker_kind) && !state.at(AsciiDocTokenType::Newline) {
                    self.parse_inline(state);
                }
                if state.at(marker_kind) {
                    state.bump(); // End marker
                }
                state.finish_at(
                    checkpoint,
                    match marker_kind {
                        AsciiDocTokenType::BoldMarker => ET::Bold,
                        AsciiDocTokenType::ItalicMarker => ET::Italic,
                        AsciiDocTokenType::MonospaceMarker => ET::Monospace,
                        _ => ET::Text,
                    },
                );
            }
            AsciiDocTokenType::LinkMarker => {
                state.bump(); // link: or http:
                if state.at(AsciiDocTokenType::LeftBracket) {
                    state.bump();
                    while state.not_at_end() && !state.at(AsciiDocTokenType::RightBracket) && !state.at(AsciiDocTokenType::Newline) {
                        self.parse_inline(state);
                    }
                    if state.at(AsciiDocTokenType::RightBracket) {
                        state.bump();
                    }
                }
                state.finish_at(checkpoint, ET::Link);
            }
            _ => {
                state.bump();
            }
        }
    }
}

impl<'config> Parser<AsciiDocLanguage> for AsciiDocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<AsciiDocLanguage>) -> oak_core::ParseOutput<'a, AsciiDocLanguage> {
        let lexer = crate::lexer::AsciiDocLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_block(state)?;
            }

            Ok(state.finish_at(checkpoint, element_type::AsciiDocElementType::Root))
        })
    }
}
