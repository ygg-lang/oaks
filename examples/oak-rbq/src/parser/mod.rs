use crate::{kind::RbqSyntaxKind, language::RbqLanguage};
use oak_core::{OakError, Parser, ParserState, Source, TextEdit, TokenType};

pub(crate) type State<'a, S> = ParserState<'a, RbqLanguage, S>;

pub struct RbqParser<'config> {
    pub(crate) config: &'config RbqLanguage,
}

impl<'config> RbqParser<'config> {
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        while state.not_at_end() {
            let checkpoint = state.checkpoint();
            self.parse_top_level(state)?;
            self.skip_trivia(state);

            if state.checkpoint() == checkpoint && state.not_at_end() {
                state.bump();
            }
        }

        Ok(())
    }

    fn parse_top_level<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        // Handle annotations
        while state.at(RbqSyntaxKind::At) {
            self.parse_annotation(state)?;
            self.skip_trivia(state);
        }

        if state.at(RbqSyntaxKind::NamespaceKw) {
            self.parse_namespace(state)
        }
        else if state.at(RbqSyntaxKind::UseKw) {
            self.parse_import(state)
        }
        else if state.at(RbqSyntaxKind::ClassKw) {
            self.parse_class(state)
        }
        else if state.at(RbqSyntaxKind::StructKw) {
            self.parse_struct(state)
        }
        else if state.at(RbqSyntaxKind::UnionKw) {
            self.parse_union(state)
        }
        else if state.at(RbqSyntaxKind::EnumKw) {
            self.parse_enum(state)
        }
        else if state.at(RbqSyntaxKind::TraitKw) {
            self.parse_trait(state)
        }
        else if state.at(RbqSyntaxKind::TypeKw) {
            self.parse_type_alias(state)
        }
        else if state.at(RbqSyntaxKind::MicroKw) {
            self.parse_micro_function(state)
        }
        else {
            // Handle expressions or potential DSL pipelines
            let checkpoint = state.checkpoint();
            self.parse_expression(state)?;
            self.skip_trivia(state);
            if state.checkpoint() == checkpoint && state.not_at_end() {
                state.bump();
            }
            Ok(())
        }
    }

    fn parse_annotation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::Annotation, |state| {
            state.eat(RbqSyntaxKind::At);
            self.skip_trivia(state);
            if !state.eat(RbqSyntaxKind::Ident) {
                state.record_expected("annotation name");
            }
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftParen) {
                self.skip_trivia(state);
                state.incremental_node(RbqSyntaxKind::AnnotationArgs, |state| {
                    while !state.at(RbqSyntaxKind::RightParen) && state.not_at_end() {
                        let checkpoint = state.checkpoint();
                        self.parse_expression(state)?;
                        self.skip_trivia(state);
                        if !state.eat(RbqSyntaxKind::Comma) {
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
                state.eat(RbqSyntaxKind::RightParen);
            }
            Ok(())
        })
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::NamespaceDef, |state| {
            state.bump(); // namespace
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_top_level(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            else {
                state.eat(RbqSyntaxKind::Semicolon);
            }
            Ok(())
        })
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::ImportDef, |state| {
            state.bump(); // use
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::StructDef, |state| {
            state.bump(); // struct
            self.skip_trivia(state);
            if !state.eat(RbqSyntaxKind::Ident) {
                state.record_expected("struct name");
            }
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqSyntaxKind::At) {
                        self.parse_annotation(state)?;
                    }
                    else if state.at(RbqSyntaxKind::UsingKw) {
                        self.parse_using(state)?;
                    }
                    else {
                        self.parse_field(state)?;
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::ClassDef, |state| {
            state.bump(); // class
            self.skip_trivia(state);
            if !state.eat(RbqSyntaxKind::Ident) {
                state.record_expected("class name");
            }
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqSyntaxKind::At) {
                        self.parse_annotation(state)?;
                    }
                    else if state.at(RbqSyntaxKind::UsingKw) {
                        self.parse_using(state)?;
                    }
                    else {
                        self.parse_field(state)?;
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_field<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::FieldDef, |state| {
            self.skip_trivia(state);
            if !state.eat(RbqSyntaxKind::Ident) {
                state.record_expected("field name");
            }
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Colon);
            self.skip_trivia(state);
            self.parse_type_ref(state)?;
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_using<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::UsingDef, |state| {
            state.bump(); // using
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_union<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::UnionDef, |state| {
            state.bump(); // union
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_union_member(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_enum<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::EnumDef, |state| {
            state.bump(); // enums
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_enum_member(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_enum_member<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::EnumMember, |state| {
            self.skip_trivia(state);
            if state.at(RbqSyntaxKind::At) {
                self.parse_annotation(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            if !state.eat(RbqSyntaxKind::Comma) {
                self.skip_trivia(state);
                state.eat(RbqSyntaxKind::Semicolon);
            }
            Ok(())
        })
    }

    fn parse_union_member<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::UnionMember, |state| {
            self.skip_trivia(state);
            if state.at(RbqSyntaxKind::At) {
                self.parse_annotation(state)?;
                self.skip_trivia(state);
            }
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            if state.at(RbqSyntaxKind::LeftBrace) {
                // ADT member with fields
                state.bump();
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    self.parse_field(state)?;
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            else if state.eat(RbqSyntaxKind::Eq) {
                self.skip_trivia(state);
                self.parse_expression(state)?;
                self.skip_trivia(state);
            }
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_trait<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::TraitDef, |state| {
            state.bump(); // trait
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftBrace) {
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    if state.at(RbqSyntaxKind::UsingKw) {
                        self.parse_using(state)?;
                    }
                    else {
                        self.parse_field(state)?;
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightBrace);
            }
            Ok(())
        })
    }

    fn parse_type_alias<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::TypeDef, |state| {
            state.bump(); // type
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ident);
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Eq);
            self.skip_trivia(state);
            self.parse_type_ref(state)?;
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_micro_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::MicroDef, |state| {
            state.bump(); // micro
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ident); // Optional identifier for lambda form
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::LeftParen) {
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RbqSyntaxKind::RightParen) {
                    self.skip_trivia(state);
                    let checkpoint = state.checkpoint();
                    state.eat(RbqSyntaxKind::Ident);
                    self.skip_trivia(state);
                    state.eat(RbqSyntaxKind::Colon);
                    self.skip_trivia(state);
                    self.parse_type_ref(state)?;
                    self.skip_trivia(state);
                    if !state.eat(RbqSyntaxKind::Comma) {
                        break;
                    }
                    self.skip_trivia(state);
                    if state.checkpoint() == checkpoint && state.not_at_end() {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.eat(RbqSyntaxKind::RightParen);
            }
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::Arrow) {
                self.skip_trivia(state);
                self.parse_type_ref(state)?;
                self.skip_trivia(state);
            }
            if state.at(RbqSyntaxKind::LeftBrace) {
                self.parse_closure(state)?;
            }
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Semicolon);
            Ok(())
        })
    }

    fn parse_type_ref<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::TypeRef, |state| {
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Ampersand); // Physical foreign key &T
            self.skip_trivia(state);
            self.parse_path(state)?;
            self.skip_trivia(state);
            if state.eat(RbqSyntaxKind::Lt) {
                self.skip_trivia(state);
                state.incremental_node(RbqSyntaxKind::GenericArgs, |state| {
                    while state.not_at_end() && !state.at(RbqSyntaxKind::Gt) {
                        let checkpoint = state.checkpoint();
                        self.parse_type_ref(state)?;
                        self.skip_trivia(state);
                        if !state.at(RbqSyntaxKind::Gt) && !state.eat(RbqSyntaxKind::Comma) {
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
                state.eat(RbqSyntaxKind::Gt);
            }
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::Question); // Optional type T?
            Ok(())
        })
    }

    fn parse_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if !state.eat(RbqSyntaxKind::Ident) {
            state.eat(RbqSyntaxKind::Utf8Kw);
        }
        self.skip_trivia(state);
        while state.eat(RbqSyntaxKind::Dot) {
            self.skip_trivia(state);
            if !state.eat(RbqSyntaxKind::Ident) {
                state.eat(RbqSyntaxKind::Utf8Kw);
            }
            self.skip_trivia(state);
        }
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_binary_expr(state, 0)
    }

    fn parse_binary_expr<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> Result<(), OakError> {
        self.parse_primary_expr(state)?;
        self.skip_trivia(state);

        while let Some(token) = state.current() {
            let precedence = token.kind.precedence();
            if precedence == 0 || precedence < min_precedence {
                break;
            }

            state.incremental_node(RbqSyntaxKind::BinaryExpr, |state| {
                state.bump(); // operator
                self.skip_trivia(state);
                self.parse_binary_expr(state, precedence + 1)?;
                self.skip_trivia(state);
                Ok(())
            })?;
        }

        Ok(())
    }

    // Removed get_precedence as it's now in RbqSyntaxKind

    fn parse_primary_expr<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);

        if state.at(RbqSyntaxKind::Dollar) {
            state.incremental_node(RbqSyntaxKind::MagicVar, |state| {
                state.bump();
                self.skip_trivia(state);
                state.eat(RbqSyntaxKind::Ident);
                Ok(())
            })?;
        }
        else if state.at(RbqSyntaxKind::StringLiteral) || state.at(RbqSyntaxKind::NumberLiteral) || state.at(RbqSyntaxKind::TrueKw) || state.at(RbqSyntaxKind::FalseKw) {
            state.incremental_node(RbqSyntaxKind::Literal, |state| {
                state.bump();
                Ok(())
            })?;
        }
        else if state.at(RbqSyntaxKind::Ident) || state.at(RbqSyntaxKind::Utf8Kw) {
            state.bump();
        }
        else if state.at(RbqSyntaxKind::LeftParen) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression(state)?;
            self.skip_trivia(state);
            state.eat(RbqSyntaxKind::RightParen);
        }
        else if state.at(RbqSyntaxKind::LeftBrace) {
            self.parse_closure(state)?;
        }
        else if state.at(RbqSyntaxKind::MicroKw) {
            self.parse_micro_function(state)?;
        }
        else {
            state.record_expected("expression");
            if state.not_at_end() {
                state.bump();
            }
        }

        self.skip_trivia(state);
        // Handle member access, calls, and indexing
        loop {
            let checkpoint = state.checkpoint();
            if state.at(RbqSyntaxKind::Dot) {
                state.incremental_node(RbqSyntaxKind::MemberExpr, |state| {
                    state.bump(); // .
                    self.skip_trivia(state);
                    state.eat(RbqSyntaxKind::Ident);
                    self.skip_trivia(state);
                    if state.at(RbqSyntaxKind::LeftParen) {
                        state.bump();
                        self.skip_trivia(state);
                        while state.not_at_end() && !state.at(RbqSyntaxKind::RightParen) {
                            let arg_checkpoint = state.checkpoint();
                            self.parse_expression(state)?;
                            self.skip_trivia(state);
                            if !state.eat(RbqSyntaxKind::Comma) {
                                break;
                            }
                            self.skip_trivia(state);
                            if state.checkpoint() == arg_checkpoint {
                                break;
                            }
                        }
                        state.eat(RbqSyntaxKind::RightParen);
                    }
                    Ok(())
                })?;
            }
            else if state.at(RbqSyntaxKind::LeftParen) {
                state.incremental_node(RbqSyntaxKind::CallExpr, |state| {
                    state.bump(); // (
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RbqSyntaxKind::RightParen) {
                        let arg_checkpoint = state.checkpoint();
                        self.parse_expression(state)?;
                        self.skip_trivia(state);
                        if !state.eat(RbqSyntaxKind::Comma) {
                            break;
                        }
                        self.skip_trivia(state);
                        if state.checkpoint() == arg_checkpoint {
                            break;
                        }
                    }
                    state.eat(RbqSyntaxKind::RightParen);
                    Ok(())
                })?;
            }
            else if state.at(RbqSyntaxKind::LeftBracket) {
                state.incremental_node(RbqSyntaxKind::MemberExpr, |state| {
                    // Reusing MemberExpr for indexer
                    state.bump(); // [
                    self.skip_trivia(state);
                    self.parse_expression(state)?;
                    self.skip_trivia(state);
                    state.eat(RbqSyntaxKind::RightBracket);
                    Ok(())
                })?;
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

    fn parse_closure<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(RbqSyntaxKind::Closure, |state| {
            state.bump(); // {
            while state.not_at_end() && !state.at(RbqSyntaxKind::RightBrace) {
                self.skip_trivia(state);
                let checkpoint = state.checkpoint();
                self.parse_expression(state)?;
                self.skip_trivia(state);
                state.eat(RbqSyntaxKind::Semicolon);
                self.skip_trivia(state);
                if state.checkpoint() == checkpoint && state.not_at_end() {
                    state.bump();
                }
            }
            state.eat(RbqSyntaxKind::RightBrace);
            Ok(())
        })
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
}

impl<'config> Parser<RbqLanguage> for RbqParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<RbqLanguage>) -> oak_core::ParseOutput<'a, RbqLanguage> {
        let lexer = crate::lexer::RbqLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_root(state)?;
            Ok(state.finish_at(checkpoint, RbqSyntaxKind::Root))
        })
    }
}
