/// Element types for the IDL language.
pub mod element_type;

use crate::{language::IdlLanguage, lexer::IdlLexer, parser::element_type::IdlElementType};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::Source,
};

/// A parser for the IDL language.
pub struct IdlParser<'config> {
    pub(crate) config: &'config IdlLanguage,
}

impl<'config> IdlParser<'config> {
    /// Creates a new IDL parser with the given configuration.
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<IdlLanguage> for IdlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<IdlLanguage>) -> ParseOutput<'a, IdlLanguage> {
        let lexer = IdlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                if !parse_top_level_item(state) {
                    state.bump();
                }
            }

            Ok(state.finish_at(checkpoint, IdlElementType::SourceFile))
        })
    }
}

/// Parse a top-level item
fn parse_top_level_item<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if parse_interface(state) {
        true
    } else if parse_struct(state) {
        true
    } else if parse_enum(state) {
        true
    } else if parse_typedef(state) {
        true
    } else if parse_const(state) {
        true
    } else if parse_module(state) {
        true
    } else {
        state.restore(checkpoint);
        false
    }
}

/// Parse an interface
fn parse_interface<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Interface) {
        state.restore(checkpoint);
        return false;
    }

    state.bump();

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !parse_members(state) {
        state.restore(checkpoint);
        return false;
    }

    state.finish_at(checkpoint, IdlElementType::Interface);

    true
}

/// Parse struct (dictionary)
fn parse_struct<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Struct) {
        state.restore(checkpoint);
        return false;
    }

    state.bump();

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !parse_fields(state) {
        state.restore(checkpoint);
        return false;
    }

    state.finish_at(checkpoint, IdlElementType::Struct);

    true
}

/// Parse enum
fn parse_enum<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Enum) {
        state.restore(checkpoint);
        return false;
    }

    state.bump();

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !parse_variants(state) {
        state.restore(checkpoint);
        return false;
    }

    state.finish_at(checkpoint, IdlElementType::Enum);

    true
}

/// Parse typedef
fn parse_typedef<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Typedef) {
        state.restore(checkpoint);
        return false;
    }

    state.bump();

    if !parse_type(state) {
        state.restore(checkpoint);
        return false;
    }

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Semicolon) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    state.finish_at(checkpoint, IdlElementType::Typedef);

    true
}

/// Parse const
fn parse_const<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Const) {
        state.restore(checkpoint);
        return false;
    }

    state.bump();

    if !parse_type(state) {
        state.restore(checkpoint);
        return false;
    }

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Assign) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::StringLiteral) && !state.at(IdlTokenType::NumberLiteral) && !state.at(IdlTokenType::BooleanLiteral) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Semicolon) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    state.finish_at(checkpoint, IdlElementType::Const);

    true
}

/// Parse module
fn parse_module<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::Module) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !parse_module_items(state) {
        state.restore(checkpoint);
        return false;
    }

    state.finish_at(checkpoint, IdlElementType::Module);

    true
}

/// Parse members
fn parse_members<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::LeftBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    while !state.at(IdlTokenType::RightBrace) && state.not_at_end() {
        if !parse_member(state) {
            state.bump();
        }
    }

    if !state.at(IdlTokenType::RightBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    true
}

/// Parse a single member
fn parse_member<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if state.at(IdlTokenType::Attribute) || (state.at(IdlTokenType::Readonly) && state.peek_kind_at(1) == Some(IdlTokenType::Attribute)) {
        let readonly = state.at(IdlTokenType::Readonly);
        if readonly {
            state.bump();
        }
        state.bump();

        if !parse_type(state) {
            state.restore(checkpoint);
            return false;
        }

        if !state.at(IdlTokenType::Identifier) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        if !state.at(IdlTokenType::Semicolon) {
            state.restore(checkpoint);
            return false;
        }
        state.bump();

        let attr_checkpoint = state.checkpoint();
        state.finish_at(attr_checkpoint, IdlElementType::Attribute);
        return true;
    }

    if !parse_type(state) {
        state.restore(checkpoint);
        return false;
    }

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::LeftParen) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !parse_params(state) {
        state.restore(checkpoint);
        return false;
    }

    if !state.at(IdlTokenType::RightParen) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Semicolon) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    let op_checkpoint = state.checkpoint();
    state.finish_at(op_checkpoint, IdlElementType::Operation);

    true
}

/// Parse fields
fn parse_fields<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::LeftBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    while !state.at(IdlTokenType::RightBrace) && state.not_at_end() {
        if !parse_field(state) {
            state.bump();
        }
    }

    if !state.at(IdlTokenType::RightBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    true
}

/// Parse a single field
fn parse_field<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !parse_type(state) {
        state.restore(checkpoint);
        return false;
    }

    if !state.at(IdlTokenType::Identifier) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    if !state.at(IdlTokenType::Semicolon) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    state.finish_at(checkpoint, IdlElementType::Field);

    true
}

/// Parse variants
fn parse_variants<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::LeftBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    while !state.at(IdlTokenType::RightBrace) && state.not_at_end() {
        if state.at(IdlTokenType::StringLiteral) {
            state.bump();

            if state.at(IdlTokenType::Comma) {
                state.bump();
            }
        } else {
            state.bump();
        }
    }

    if !state.at(IdlTokenType::RightBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    true
}

/// Parse module items
fn parse_module_items<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    if !state.at(IdlTokenType::LeftBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    while !state.at(IdlTokenType::RightBrace) && state.not_at_end() {
        if !parse_top_level_item(state) {
            state.bump();
        }
    }

    if !state.at(IdlTokenType::RightBrace) {
        state.restore(checkpoint);
        return false;
    }
    state.bump();

    true
}

/// Parse parameters
fn parse_params<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    while !state.at(IdlTokenType::RightParen) && state.not_at_end() {
        if state.at(IdlTokenType::In) || state.at(IdlTokenType::Out) || state.at(IdlTokenType::Inout) {
            state.bump();
        }

        if !parse_type(state) {
            return false;
        }

        if !state.at(IdlTokenType::Identifier) {
            return false;
        }
        state.bump();

        if state.at(IdlTokenType::Comma) {
            state.bump();
        }
    }

    true
}

/// Parse a type
fn parse_type<'a, S: Source + ?Sized>(state: &mut oak_core::parser::ParserState<'a, IdlLanguage, S>) -> bool {
    use crate::lexer::token_type::IdlTokenType;

    let checkpoint = state.checkpoint();

    let basic_types = [
        IdlTokenType::Void,
        IdlTokenType::Boolean,
        IdlTokenType::Byte,
        IdlTokenType::Octet,
        IdlTokenType::Short,
        IdlTokenType::UnsignedShort,
        IdlTokenType::Long,
        IdlTokenType::Float,
        IdlTokenType::Double,
        IdlTokenType::Char,
        IdlTokenType::WChar,
        IdlTokenType::String,
        IdlTokenType::WString,
        IdlTokenType::Any,
        IdlTokenType::Object,
        IdlTokenType::ValueBase,
    ];

    for token_type in &basic_types {
        if state.at(*token_type) {
            state.bump();
            return true;
        }
    }

    if state.at(IdlTokenType::Identifier) {
        state.bump();
        return true;
    }

    if state.at(IdlTokenType::Sequence) {
        state.bump();
        if state.at(IdlTokenType::LeftBracket) {
            state.bump();
            if parse_type(state) {
                if state.at(IdlTokenType::RightBracket) {
                    state.bump();
                    return true;
                }
            }
        }
    }

    if state.at(IdlTokenType::Union) {
        state.bump();
        if state.at(IdlTokenType::LeftBracket) {
            state.bump();
            while !state.at(IdlTokenType::RightBracket) && state.not_at_end() {
                if !parse_type(state) {
                    state.bump();
                } else {
                    if state.at(IdlTokenType::Or) {
                        state.bump();
                    }
                }
            }
            if state.at(IdlTokenType::RightBracket) {
                state.bump();
                return true;
            }
        }
    }

    state.restore(checkpoint);
    false
}
