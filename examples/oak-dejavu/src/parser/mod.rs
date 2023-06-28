use crate::{DejavuLanguage, DejavuLexer};
use oak_core::{Parser, TextEdit, parser::ParseCache};

pub use element_type::DejavuElementType;

/// Dejavu parser.
pub struct DejavuParser<'config> {
    /// Language configuration.
    config: &'config DejavuLanguage,
}

pub(crate) type State<'a, S> = oak_core::parser::ParserState<'a, DejavuLanguage, S>;

impl<'config> DejavuParser<'config> {
    /// Create a new Dejavu parser.
    pub fn new(config: &'config DejavuLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn skip_trivia<'a, S: oak_core::Source + ?Sized>(&self, state: &mut oak_core::parser::ParserState<'a, DejavuLanguage, S>) {
        state.skip_trivia();
    }
}

impl<'config> Parser<DejavuLanguage> for DejavuParser<'config> {
    fn parse<'a, S: oak_core::Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DejavuLanguage>) -> oak_core::parser::ParseOutput<'a, DejavuLanguage> {
        oak_core::parser::parse_with_lexer(&DejavuLexer::new(&self.config), text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.skip_trivia(state);
                if let Some(token) = state.current() {
                    match token.kind {
                        crate::lexer::token_type::DejavuTokenType::CodeStart | crate::lexer::token_type::DejavuTokenType::TemplateControlStart => {
                            // Check if this looks like an interpolation (not a control statement)
                            let cp = state.checkpoint();

                            // 先消耗开始标记
                            state.bump();
                            self.skip_trivia(state);

                            let is_interpolation = if let Some(next_token) = state.current() {
                                matches!(
                                    next_token.kind,
                                    crate::lexer::token_type::DejavuTokenType::Identifier
                                        | crate::lexer::token_type::DejavuTokenType::StringLiteral
                                        | crate::lexer::token_type::DejavuTokenType::IntegerLiteral
                                        | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::True)
                                        | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::False)
                                )
                            }
                            else {
                                false
                            };

                            state.restore(cp);

                            if is_interpolation {
                                // It's an interpolation, use parse_template_code
                                self.parse_template_code(state).ok();
                            }
                            else {
                                // It's a control statement
                                self.parse_template_control(state).ok();
                            }
                        }
                        crate::lexer::token_type::DejavuTokenType::StringPart => {
                            // Add template text to AST
                            state.bump();
                        }
                        _ => {
                            state.advance();
                        }
                    }
                }
                else {
                    state.advance();
                }
            }
            Ok(state.finish_at(checkpoint, DejavuElementType::Root))
        })
    }
}

/// Element type definitions.
pub mod element_type;
/// Control flow parsing utilities.
pub mod parse_control_flow;
/// Expression parsing utilities.
pub mod parse_expr;
/// Raw block parsing utilities.
pub mod parse_raw;
/// Type parsing utilities.
pub mod parse_types;

// Import extension trait to bring parse methods into scope
use parse_raw::DejavuParserExt;
