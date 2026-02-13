pub mod element_type;

use crate::{language::RbqLanguage, lexer::token_type::RbqTokenType, parser::element_type::RbqElementType};
use oak_core::{OakError, Parser, ParserState, Source, TextEdit, TokenType};

pub(crate) type State<'a, S> = ParserState<'a, RbqLanguage, S>;

/// Parser for the RBQ language.
pub struct RbqParser<'config> {
    pub(crate) config: &'config RbqLanguage,
}

impl<'config> RbqParser<'config> {
    /// Creates a new `RbqParser` with the given configuration.
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { config }
    }

    /// Parses the root node of an RBQ document.
    pub(crate) fn parse_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(RbqTokenType::Eof) {
            let checkpoint = state.checkpoint();
            self.parse_top_level(state)?;
            self.skip_trivia(state);

            if state.checkpoint() == checkpoint && state.not_at_end() && !state.at(RbqTokenType::Eof) {
                state.bump()
            }
        }

        Ok(())
    }

    /// Parses a top-level definition or expression.
    fn parse_top_level<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        // Handle annotations
        while state.at(RbqTokenType::At) {
            self.parse_annotation(state)?;
            self.skip_trivia(state)
        }

        if state.at(RbqTokenType::NamespaceKw) {
            self.parse_namespace(state)
        }
        else if state.at(RbqTokenType::UseKw) {
            self.parse_import(state)
        }
        else if state.at(RbqTokenType::ClassKw) {
            self.parse_class(state)
        }
        else if state.at(RbqTokenType::StructKw) {
            self.parse_struct(state)
        }
        else if state.at(RbqTokenType::UnionKw) {
            self.parse_union(state)
        }
        else if state.at(RbqTokenType::EnumKw) {
            self.parse_enum(state)
        }
        else if state.at(RbqTokenType::TraitKw) {
            self.parse_trait(state)
        }
        else if state.at(RbqTokenType::TypeKw) {
            self.parse_type_alias(state)
        }
        else if state.at(RbqTokenType::MicroKw) {
            self.parse_micro_function(state)
        }
        else if state.at(RbqTokenType::Semicolon) {
            state.bump();
            Ok(())
        }
        else {
            // Handle expressions or potential DSL pipelines
            let checkpoint = state.checkpoint();
            self.parse_query_pipeline(state)?;
            self.skip_trivia(state);
            state.eat(RbqTokenType::Semicolon);
            self.skip_trivia(state);
            if state.checkpoint() == checkpoint && state.not_at_end() {
                state.bump()
            }
            Ok(())
        }
    }

    /// Parses an annotation (e.g., `@name` or `@name(args)`).
    fn parse_annotation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::Annotation, |state| {
            state.eat(RbqTokenType::At);
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.record_expected("annotation name")
            }
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftParen) {
                self.skip_trivia(state);
                state.incremental_node(RbqElementType::AnnotationArgs, |state| {
                    while !state.at(RbqTokenType::RightParen) && state.not_at_end() {
                        let checkpoint = state.checkpoint();
                        self.parse_expression(state)?;
                        self.skip_trivia(state);
                        if !state.eat(RbqTokenType::Comma) {
                            if state.checkpoint() == checkpoint {
                                state.bump();
                                self.skip_trivia(state);
                            }
                            break;
                        }
                        self.skip_trivia(state);
                        if state.checkpoint() == checkpoint {
                            state.bump();
                            self.skip_trivia(state);
                        }
                    }
                    Ok(())
                })?;
                self.skip_trivia(state);
                state.eat(RbqTokenType::RightParen);
            }
            Ok(())
        })
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::NamespaceDef, |state| {
            state.bump(); // namespace
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_top_level(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            else {
                state.eat(RbqTokenType::Semicolon);
            }
            Ok(())
        })
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::ImportDef, |state| {
            state.bump(); // use
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.eat(RbqTokenType::Semicolon);
            Ok(())
        })
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::StructDef, |state| {
            state.bump(); // struct
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.record_expected("struct name")
            }
            self.skip_trivia(state);
            // Handle using Traits
            if state.at(RbqTokenType::UsingKw) {
                state.incremental_node(RbqElementType::UsingDef, |state| {
                    state.bump(); // using
                    self.skip_trivia(state);
                    self.parse_path(state)?;
                    self.skip_trivia(state);
                    while state.eat(RbqTokenType::Comma) {
                        self.skip_trivia(state);
                        self.parse_path(state)?;
                        self.skip_trivia(state);
                    }
                    Ok(())
                })?;
            }
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqTokenType::At) {
                        self.parse_annotation(state)?
                    }
                    else if state.at(RbqTokenType::UsingKw) {
                        self.parse_using(state)?
                    }
                    else {
                        self.parse_field(state)?
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::ClassDef, |state| {
            state.bump(); // class
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.record_expected("class name")
            }
            self.skip_trivia(state);
            // Handle using Traits
            if state.at(RbqTokenType::UsingKw) {
                state.incremental_node(RbqElementType::UsingDef, |state| {
                    state.bump(); // using
                    self.skip_trivia(state);
                    self.parse_path(state)?;
                    self.skip_trivia(state);
                    while state.eat(RbqTokenType::Comma) {
                        self.skip_trivia(state);
                        self.parse_path(state)?;
                        self.skip_trivia(state);
                    }
                    Ok(())
                })?;
            }
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqTokenType::At) {
                        self.parse_annotation(state)?
                    }
                    else if state.at(RbqTokenType::UsingKw) {
                        self.parse_using(state)?
                    }
                    else {
                        self.parse_field(state)?
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::FieldDef, |state| {
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.record_expected("field name")
            }
            self.skip_trivia(state);
            state.eat(RbqTokenType::Colon);
            self.skip_trivia(state);
            self.parse_type_ref(state)?;
            self.skip_trivia(state);
            if state.eat(RbqTokenType::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqTokenType::Semicolon);
            Ok(())
        })
    }

    fn parse_using<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::UsingDef, |state| {
            state.bump(); // using
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.eat(RbqTokenType::Semicolon);
            Ok(())
        })
    }

    fn parse_union<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::UnionDef, |state| {
            state.bump(); // union
            self.skip_trivia(state);
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_union_member(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_enum<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::EnumDef, |state| {
            state.bump(); // enum
            self.skip_trivia(state);
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_enum_member(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_enum_member<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::EnumMember, |state| {
            self.skip_trivia(state);
            if state.at(RbqTokenType::At) {
                self.parse_annotation(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            if state.eat(RbqTokenType::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            if !state.eat(RbqTokenType::Comma) {
                self.skip_trivia(state);
                state.eat(RbqTokenType::Semicolon);
            }
            Ok(())
        })
    }

    fn parse_union_member<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::UnionMember, |state| {
            self.skip_trivia(state);
            if state.at(RbqTokenType::At) {
                self.parse_annotation(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            if state.at(RbqTokenType::LeftBrace) {
                // ADT member with fields
                state.bump();
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_field(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            else if state.eat(RbqTokenType::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            self.skip_trivia(state);
            state.eat(RbqTokenType::Semicolon);
            Ok(())
        })
    }

    fn parse_trait<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::TraitDef, |state| {
            state.bump(); // trait
            self.skip_trivia(state);
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqTokenType::At) {
                        self.parse_annotation(state)?
                    }
                    else if state.at(RbqTokenType::UsingKw) {
                        self.parse_using(state)?
                    }
                    else if state.at(RbqTokenType::MicroKw) {
                        self.parse_micro_function(state)?
                    }
                    else {
                        self.parse_field(state)?
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_type_alias<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::TypeDef, |state| {
            state.bump(); // type
            self.skip_trivia(state);
            state.eat(RbqTokenType::Ident);
            self.skip_trivia(state);
            state.eat(RbqTokenType::Eq);
            self.skip_trivia(state);
            self.parse_type_ref(state)?;
            self.skip_trivia(state);
            state.eat(RbqTokenType::Semicolon);
            Ok(())
        })
    }

    fn parse_micro_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::MicroDef, |state| {
            state.bump(); // micro
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.record_expected("micro name")
            }
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftParen) {
                while state.not_at_end() && !state.at(RbqTokenType::RightParen) {
                    let checkpoint = state.checkpoint();
                    state.incremental_node(RbqElementType::FieldDef, |state| {
                        if !state.eat(RbqTokenType::Ident) { // arg name
                            state.record_expected("argument name");
                        }
                        self.skip_trivia(state);
                        if state.eat(RbqTokenType::Colon) {
                            self.skip_trivia(state);
                            self.parse_type_ref(state)?;
                        }
                        Ok(())
                    })?;
                    self.skip_trivia(state);
                    if !state.at(RbqTokenType::RightParen) && !state.eat(RbqTokenType::Comma) {
                        if state.checkpoint() == checkpoint && state.not_at_end() {
                            state.bump();
                            self.skip_trivia(state);
                        }
                        break;
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightParen);
            }
            self.skip_trivia(state);
            if state.eat(RbqTokenType::Arrow) {
                self.skip_trivia(state);
                self.parse_type_ref(state)?;
            }
            self.skip_trivia(state);
            if state.at(RbqTokenType::LeftBrace) {
                self.parse_block(state)?;
            }
            else {
                state.eat(RbqTokenType::Semicolon);
            }
            Ok(())
        })
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::Block, |state| {
            state.eat(RbqTokenType::LeftBrace);
            while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                self.skip_trivia(state);
                let checkpoint = state.checkpoint();
                self.parse_expression(state)?;
                state.eat(RbqTokenType::Semicolon);
                self.skip_trivia(state);
                if state.checkpoint() == checkpoint && state.not_at_end() {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.eat(RbqTokenType::RightBrace);
            Ok(())
        })
    }

    fn parse_type_ref<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::TypeRef, |state| {
            self.skip_trivia(state);
            if state.eat(RbqTokenType::LeftBrace) {
                // Inline struct type: { field: type; ... }
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_field(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqTokenType::RightBrace);
            }
            else {
                state.eat(RbqTokenType::Ampersand); // Physical foreign key &T
                self.skip_trivia(state);
                self.parse_path(state)?;
                self.skip_trivia(state);
                if state.eat(RbqTokenType::Lt) {
                    self.skip_trivia(state);
                    state.incremental_node(RbqElementType::GenericArgs, |state| {
                        while state.not_at_end() && !state.at(RbqTokenType::Gt) {
                            let checkpoint = state.checkpoint();
                            // Support literals in generic args (e.g. vector<f32, 768>)
                            if state.at(RbqTokenType::NumberLiteral) || state.at(RbqTokenType::StringLiteral) {
                                state.incremental_node(RbqElementType::Literal, |state| {
                                    state.bump();
                                    Ok(())
                                })?;
                            }
                            else {
                                self.parse_type_ref(state)?;
                            }
                            self.skip_trivia(state);
                            if !state.at(RbqTokenType::Gt) && !state.eat(RbqTokenType::Comma) {
                                if state.checkpoint() == checkpoint && state.not_at_end() {
                                    state.bump();
                                    self.skip_trivia(state);
                                }
                                break;
                            }
                            self.skip_trivia(state);
                            if state.checkpoint() == checkpoint && state.not_at_end() {
                                state.bump();
                                self.skip_trivia(state);
                            }
                        }
                        Ok(())
                    })?;
                    self.skip_trivia(state);
                    state.eat(RbqTokenType::Gt);
                }
            }
            self.skip_trivia(state);
            state.eat(RbqTokenType::Question); // Optional type T?
            Ok(())
        })
    }

    fn parse_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if !state.eat(RbqTokenType::Ident) {
            state.eat(RbqTokenType::Utf8Kw);
        }
        self.skip_trivia(state);
        while state.eat(RbqTokenType::Dot) {
            self.skip_trivia(state);
            if !state.eat(RbqTokenType::Ident) {
                state.eat(RbqTokenType::Utf8Kw);
            }
            self.skip_trivia(state);
        }
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_binary_expr(state, 0)
    }

    fn parse_binary_expr<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        self.parse_unary_expr(state)?;
        self.skip_trivia(state);

        while let Some(token) = state.current() {
            let precedence = token.kind.precedence();
            if precedence == 0 || precedence < min_precedence {
                break;
            }

            state.bump(); // operator
            self.skip_trivia(state);
            self.parse_binary_expr(state, precedence + 1)?;
            self.skip_trivia(state);
            state.finish_at(checkpoint, RbqElementType::BinaryExpr);
        }

        Ok(())
    }

    // Removed get_precedence as it's now in RbqTokenType

    fn parse_unary_expr<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        if TokenType::role(&state.current().map(|t| t.kind).unwrap_or(RbqTokenType::Error)) == oak_core::UniversalTokenRole::Operator {
            state.bump();
            self.skip_trivia(state);
            self.parse_unary_expr(state)?;
            state.finish_at(checkpoint, RbqElementType::UnaryExpr);
            Ok(())
        } else {
            self.parse_primary_expr(state)
        }
    }

    fn parse_primary_expr<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        if state.at(RbqTokenType::Dollar) {
            state.incremental_node(RbqElementType::MagicVar, |state| {
                state.bump();
                self.skip_trivia(state);
                state.eat(RbqTokenType::Ident);
                Ok(())
            })?
        }
        else if state.at(RbqTokenType::StringLiteral) || state.at(RbqTokenType::NumberLiteral) || state.at(RbqTokenType::TrueKw) || state.at(RbqTokenType::FalseKw) {
            state.incremental_node(RbqElementType::Literal, |state| {
                state.bump();
                Ok(())
            })?
        }
        else if state.at(RbqTokenType::Ident) || state.at(RbqTokenType::Utf8Kw) {
            state.incremental_node(RbqElementType::Ident, |state| {
                state.bump();
                Ok(())
            })?
        }
        else if state.at(RbqTokenType::LeftParen) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression(state)?;
            self.skip_trivia(state);
            state.eat(RbqTokenType::RightParen);
        }
        else if state.at(RbqTokenType::LeftBrace) {
            self.parse_closure_or_pipeline(state)?
        }
        else if state.at(RbqTokenType::MicroKw) {
            self.parse_micro_function(state)?
        }
        else {
            state.record_expected("expression");
            if state.not_at_end() {
                state.bump()
            }
        }

        self.skip_trivia(state);
        // Handle member access, calls, and indexing
        loop {
            let checkpoint = state.checkpoint();
            if state.at(RbqTokenType::Dot) {
                state.incremental_node(RbqElementType::MemberExpr, |state| {
                    state.bump(); // .
                    self.skip_trivia(state);
                    state.eat(RbqTokenType::Ident);
                    self.skip_trivia(state);
                    if state.at(RbqTokenType::LeftParen) {
                        state.bump();
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(RbqTokenType::RightParen) {
                            let arg_checkpoint = state.checkpoint();
                            self.parse_expression(state)?;
                            self.skip_trivia(state);
                            if !state.eat(RbqTokenType::Comma) {
                                break;
                            }
                            self.skip_trivia(state);
                            if state.checkpoint() == arg_checkpoint {
                                break;
                            }
                        }
                        state.eat(RbqTokenType::RightParen);
                    }
                    Ok(())
                })?
            }
            else if state.at(RbqTokenType::LeftParen) {
                state.incremental_node(RbqElementType::CallExpr, |state| {
                    state.bump(); // (
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RbqTokenType::RightParen) {
                        let arg_checkpoint = state.checkpoint();
                        self.parse_expression(state)?;
                        self.skip_trivia(state);
                        if !state.eat(RbqTokenType::Comma) {
                            break;
                        }
                        self.skip_trivia(state);
                        if state.checkpoint() == arg_checkpoint {
                            break;
                        }
                    }
                    state.eat(RbqTokenType::RightParen);
                    Ok(())
                })?
            }
            else if state.at(RbqTokenType::LeftBracket) {
                state.incremental_node(RbqElementType::MemberExpr, |state| {
                    // Reusing MemberExpr for indexer
                    state.bump(); // [
                    self.skip_trivia(state);
                    self.parse_expression(state)?;
                    self.skip_trivia(state);
                    state.eat(RbqTokenType::RightBracket);
                    Ok(())
                })?
            }
            else {
                break;
            }

            self.skip_trivia(state);
            if state.checkpoint() == checkpoint {
                break;
            }
        }

        Ok(())
    }

    fn parse_closure_or_pipeline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.bump(); // {
        self.skip_trivia(state);

        // Check if it's a closure with args: { |a, b| ... }
        if state.at(RbqTokenType::Pipe) {
            state.restore(checkpoint);
            return self.parse_closure(state);
        }

        // It could be a regular closure { expr; expr; } or a pipeline { base | step | step }
        // Let's parse the first expression
        self.parse_expression(state)?;
        self.skip_trivia(state);

        if state.at(RbqTokenType::Pipe) {
            // It's a pipeline: { base | step | step }
            while state.eat(RbqTokenType::Pipe) {
                self.skip_trivia(state);
                state.incremental_node(RbqElementType::PipelineStep, |state| {
                    if !state.eat(RbqTokenType::Ident) {
                        state.record_expected("pipeline step name");
                    }
                    self.skip_trivia(state);
                    if state.eat(RbqTokenType::LeftParen) {
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(RbqTokenType::RightParen) {
                            let arg_checkpoint = state.checkpoint();
                            self.parse_expression(state)?;
                            self.skip_trivia(state);
                            if !state.eat(RbqTokenType::Comma) {
                                break;
                            }
                            self.skip_trivia(state);
                            if state.checkpoint() == arg_checkpoint {
                                break;
                            }
                        }
                        state.eat(RbqTokenType::RightParen);
                    }
                    Ok(())
                })?;
                self.skip_trivia(state);
            }
            state.eat(RbqTokenType::RightBrace);
            state.finish_at(checkpoint, RbqElementType::QueryPipeline);
            Ok(())
        }
        else {
            // It's a regular closure: { expr; expr; }
            if state.eat(RbqTokenType::Semicolon) {
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                    let step_checkpoint = state.checkpoint();
                    self.parse_expression(state)?;
                    self.skip_trivia(state);
                    state.eat(RbqTokenType::Semicolon);
                    self.skip_trivia(state);
                    if state.checkpoint() == step_checkpoint {
                        state.bump();
                    }
                }
            }
            state.eat(RbqTokenType::RightBrace);
            state.finish_at(checkpoint, RbqElementType::Closure);
            Ok(())
        }
    }

    fn parse_closure<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::Closure, |state| {
            state.bump(); // {
            self.skip_trivia(state);
            if state.eat(RbqTokenType::Pipe) {
                state.incremental_node(RbqElementType::ClosureArgs, |state| {
                    while state.not_at_end() && !state.at(RbqTokenType::Pipe) {
                        state.eat(RbqTokenType::Ident);
                        self.skip_trivia(state);
                        if !state.eat(RbqTokenType::Comma) {
                            break;
                        }
                        self.skip_trivia(state);
                    }
                    state.eat(RbqTokenType::Pipe);
                    Ok(())
                })?;
            }
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RbqTokenType::RightBrace) {
                let checkpoint = state.checkpoint();
                self.parse_expression(state)?;
                self.skip_trivia(state);
                state.eat(RbqTokenType::Semicolon);
                self.skip_trivia(state);
                if state.checkpoint() == checkpoint && state.not_at_end() {
                    state.bump()
                }
            }
            state.eat(RbqTokenType::RightBrace);
            Ok(())
        })
    }

    fn parse_query_pipeline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqElementType::QueryPipeline, |state| {
            self.parse_expression(state)?;
            self.skip_trivia(state);
            while state.eat(RbqTokenType::Pipe) {
                self.skip_trivia(state);
                state.incremental_node(RbqElementType::PipelineStep, |state| {
                    if !state.eat(RbqTokenType::Ident) {
                        state.record_expected("pipeline step name")
                    }
                    self.skip_trivia(state);
                    if state.eat(RbqTokenType::LeftParen) {
                        while state.not_at_end() && !state.at(RbqTokenType::RightParen) {
                            let checkpoint = state.checkpoint();
                            self.parse_expression(state)?;
                            self.skip_trivia(state);
                            if !state.at(RbqTokenType::RightParen) && !state.eat(RbqTokenType::Comma) {
                                if state.checkpoint() == checkpoint && state.not_at_end() {
                                    state.bump();
                                    self.skip_trivia(state);
                                }
                                break;
                            }
                            self.skip_trivia(state);
                            if state.checkpoint() == checkpoint && state.not_at_end() {
                                state.bump();
                                self.skip_trivia(state);
                            }
                        }
                        state.eat(RbqTokenType::RightParen);
                    }
                    Ok(())
                })?;
                self.skip_trivia(state);
            }
            Ok(())
        })
    }

    /// Skips whitespace and comments.
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(token) = state.current() {
            if token.kind.is_ignored() { state.bump() } else { break }
        }
    }
}

impl<'config> Parser<RbqLanguage> for RbqParser<'config> {
    /// Parses the source text into an RBQ syntax tree.
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<RbqLanguage>) -> oak_core::ParseOutput<'a, RbqLanguage> {
        let lexer = crate::lexer::RbqLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_root(state)?;
            Ok(state.finish_at(checkpoint, crate::parser::element_type::RbqElementType::Root))
        })
    }
}
