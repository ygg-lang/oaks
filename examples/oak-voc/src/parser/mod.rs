use oak_core::{
    OakError, Source, TextEdit, TokenType,
    parser::{ParseCache, Parser, ParserState},
};

/// Element types for the VOC language.
pub mod element_type;

use crate::{
    language::VocLanguage,
    lexer::{VocLexer, token_type::VocTokenType},
    parser::element_type::VocElementType,
};

pub(crate) type State<'a, S> = ParserState<'a, VocLanguage, S>;

/// Parser for VOC (Visual eXtended) documents.
pub struct VocParser<'config> {
    /// Language configuration reference.
    pub(crate) config: &'config VocLanguage,
}

impl<'config> VocParser<'config> {
    /// Creates a new VOC parser with the given configuration.
    pub fn new(config: &'config VocLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(token) = state.current() {
            if token.kind.is_ignored() {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn section_name<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Option<std::borrow::Cow<'_, str>> {
        let token = state.current()?;
        let text = state.source.get_text_in(token.span);
        let s = text.as_ref();
        if s.starts_with('<') && s.ends_with('>') {
            let inner = &s[1..s.len() - 1];
            Some(std::borrow::Cow::Owned(inner.to_string()))
        }
        else {
            None
        }
    }

    fn parse_template_section<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VocElementType::TemplateSection, |state| {
            state.bump();
            self.skip_trivia(state);

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::SectionClose) {
                    break;
                }
                if state.at(VocTokenType::Eof) {
                    break;
                }
                self.parse_template_node(state)?;
            }

            self.skip_trivia(state);
            if state.at(VocTokenType::SectionClose) {
                state.bump();
            }

            Ok(())
        })
    }

    fn parse_template_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        if state.at(VocTokenType::TagOpen) {
            self.parse_template_element(state)
        }
        else if state.at(VocTokenType::SelfCloseTag) {
            state.incremental_node(VocElementType::TemplateElement, |state| {
                state.bump();
                Ok(())
            })
        }
        else if state.at(VocTokenType::Text) {
            state.bump();
            Ok(())
        }
        else {
            state.bump();
            Ok(())
        }
    }

    fn parse_template_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VocElementType::TemplateElement, |state| {
            state.bump();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::SectionClose) || state.at(VocTokenType::TagClose) || state.at(VocTokenType::Eof) {
                    break;
                }
                self.parse_template_node(state)?;
            }

            self.skip_trivia(state);
            if state.at(VocTokenType::TagClose) {
                state.bump();
            }

            Ok(())
        })
    }

    fn parse_script_section<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VocElementType::ScriptSection, |state| {
            state.bump();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::SectionClose) {
                    break;
                }
                if state.at(VocTokenType::Eof) {
                    break;
                }
                state.bump();
            }

            self.skip_trivia(state);
            if state.at(VocTokenType::SectionClose) {
                state.bump();
            }

            Ok(())
        })
    }

    fn parse_style_section<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VocElementType::StyleSection, |state| {
            state.bump();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::SectionClose) {
                    break;
                }
                if state.at(VocTokenType::Eof) {
                    break;
                }

                if state.at(VocTokenType::Selector) {
                    self.parse_style_rule(state)?;
                }
                else if state.at(VocTokenType::Variable) {
                    state.incremental_node(VocElementType::StyleRule, |state| {
                        state.bump();
                        Ok(())
                    })?;
                }
                else if state.at(VocTokenType::BlockClose) {
                    state.bump();
                }
                else {
                    state.bump();
                }
            }

            self.skip_trivia(state);
            if state.at(VocTokenType::SectionClose) {
                state.bump();
            }

            Ok(())
        })
    }

    fn parse_style_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VocElementType::StyleRule, |state| {
            state.bump();

            self.skip_trivia(state);
            if state.at(VocTokenType::BlockOpen) {
                state.bump();
            }

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::BlockClose) {
                    state.bump();
                    break;
                }
                if state.at(VocTokenType::SectionClose) || state.at(VocTokenType::Eof) {
                    break;
                }

                if state.at(VocTokenType::Property) {
                    state.incremental_node(VocElementType::StyleProperty, |state| {
                        state.bump();
                        Ok(())
                    })?;
                }
                else if state.at(VocTokenType::Variable) {
                    state.incremental_node(VocElementType::StyleProperty, |state| {
                        state.bump();
                        Ok(())
                    })?;
                }
                else {
                    state.bump();
                }
            }

            Ok(())
        })
    }
}

impl<'config> Parser<VocLanguage> for VocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VocLanguage>) -> oak_core::ParseOutput<'a, VocLanguage> {
        let lexer = VocLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.skip_trivia(state);

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VocTokenType::Eof) {
                    break;
                }

                if state.at(VocTokenType::SectionOpen) {
                    let name = self.section_name(state);
                    match name.as_deref() {
                        Some("template") => {
                            let _ = self.parse_template_section(state);
                        }
                        Some("script") => {
                            let _ = self.parse_script_section(state);
                        }
                        Some("style") => {
                            let _ = self.parse_style_section(state);
                        }
                        _ => {
                            state.bump();
                        }
                    }
                }
                else {
                    state.bump();
                }
            }

            while state.not_at_end() {
                if let Some(token) = state.current() {
                    if token.kind.is_ignored() {
                        state.bump();
                        continue;
                    }
                }
                break;
            }

            Ok(state.finish_at(checkpoint, VocElementType::Root))
        })
    }
}
