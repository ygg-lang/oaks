use crate::{
    language::DejavuLanguage,
    lexer::{
        DejavuKeywords,
        token_type::DejavuSyntaxKind::{self, *},
    },
};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, DejavuLanguage, S>;

impl<'config> super::DejavuParser<'config> {
    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(t) = state.current() {
            match t.kind {
                Whitespace | Newline | LineComment | BlockComment => {
                    state.bump();
                }
                _ => break,
            }
        }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, DejavuLanguage> {
        let cp = state.checkpoint();
        
        while state.not_at_end() && !state.at(Eof) {
            let start_index = state.tokens.index();
            
            if state.at(StringPart) {
                state.bump();
            }
            else if state.at(TemplateControlStart) {
                self.parse_template_control(state).ok();
            }
            else if state.at(InterpolationStart) {
                self.parse_template_interpolation(state).ok();
            }
            else if self.parse_source_file(state).is_err() && state.tokens.index() == start_index {
                state.bump();
            }
        }

        state.finish_at(cp, DejavuRoot)
    }

    fn parse_template_control<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(TemplateControlStart)?;
        
        while state.not_at_end() && !state.at(TemplateControlEnd) {
            let start_index = state.tokens.index();
            if self.parse_source_file(state).is_err() && state.tokens.index() == start_index {
                state.bump();
            }
        }
        
        state.expect(TemplateControlEnd)?;
        Ok(state.finish_at(cp, DejavuSyntaxKind::ApplyBlock)) // Use ApplyBlock or similar for control
    }

    fn parse_template_interpolation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(InterpolationStart)?;
        
        self.parse_expression_internal(state, 0);
        
        state.expect(InterpolationEnd)?;
        Ok(state.finish_at(cp, DejavuSyntaxKind::Interpolation))
    }

    fn parse_source_file<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        self.skip_trivia(state);
        if let Some(t) = state.current() {
            match t.kind {
                Eof => Err(OakError::custom_error("Unexpected end of file")),
                At | Bolt => {
                    let next_significant = self.peek_after_attributes(state);
                    match next_significant {
                        Some(Keyword(DejavuKeywords::Micro)) => self.parse_micro(state),
                        Some(Keyword(DejavuKeywords::Widget)) => self.parse_widget(state),
                        Some(Keyword(DejavuKeywords::Class)) => self.parse_class(state),
                        Some(Keyword(DejavuKeywords::Flags)) => self.parse_flags(state),
                        Some(Keyword(DejavuKeywords::Enums)) => self.parse_enums(state),
                        Some(Keyword(DejavuKeywords::Trait)) => self.parse_trait(state),
                        Some(Keyword(DejavuKeywords::Effect)) => self.parse_effect(state),
                        Some(Keyword(DejavuKeywords::Mezzo)) => self.parse_mezzo(state),
                        Some(Keyword(DejavuKeywords::Singleton)) => self.parse_singleton(state),
                        Some(Keyword(DejavuKeywords::Namespace)) => self.parse_namespace(state),
                        Some(Keyword(DejavuKeywords::Let)) => self.parse_let_statement(state),
                        _ => self.parse_expression_statement(state),
                    }
                }
                Keyword(DejavuKeywords::Namespace) => self.parse_namespace(state),
                Keyword(DejavuKeywords::Using) => self.parse_using_statement(state),
                Keyword(DejavuKeywords::Micro) => self.parse_micro(state),
                Keyword(DejavuKeywords::Widget) => self.parse_widget(state),
                Keyword(DejavuKeywords::Class) => self.parse_class(state),
                Keyword(DejavuKeywords::Flags) => self.parse_flags(state),
                Keyword(DejavuKeywords::Enums) => self.parse_enums(state),
                Keyword(DejavuKeywords::Trait) => self.parse_trait(state),
                Keyword(DejavuKeywords::Effect) => self.parse_effect(state),
                Keyword(DejavuKeywords::Mezzo) => self.parse_mezzo(state),
                Keyword(DejavuKeywords::Singleton) => self.parse_singleton(state),
                Keyword(DejavuKeywords::Let) => self.parse_let_statement(state),
                _ => self.parse_expression_statement(state),
            }
        }
        else {
            Err(OakError::custom_error("Unexpected end of file"))
        }
    }

    fn peek_after_attributes<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> Option<crate::lexer::token_type::DejavuSyntaxKind> {
        let mut offset = 0;
        while let Some(kind) = state.peek_kind_at(offset) {
            match kind {
                At | Bolt => {
                    offset += 1;
                    while let Some(k2) = state.peek_kind_at(offset) {
                        if matches!(k2, Whitespace | Newline | LineComment | BlockComment) { offset += 1 } else { break }
                    }
                    if let Some(k2) = state.peek_kind_at(offset) {
                        if k2 == Identifier {
                            offset += 1;
                        }
                        else if k2 == LeftBracket {
                            let mut depth = 1;
                            offset += 1;
                            while depth > 0 {
                                if let Some(k3) = state.peek_kind_at(offset) {
                                    if k3 == LeftBracket {
                                        depth += 1;
                                    }
                                    else if k3 == RightBracket {
                                        depth -= 1;
                                    }
                                    offset += 1;
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                    while let Some(k2) = state.peek_kind_at(offset) {
                        if matches!(k2, Whitespace | Newline | LineComment | BlockComment) {
                            offset += 1;
                        }
                        else {
                            break;
                        }
                    }
                    if let Some(k2) = state.peek_kind_at(offset) {
                        if k2 == LeftParen {
                            let mut depth = 1;
                            offset += 1;
                            while depth > 0 {
                                if let Some(k3) = state.peek_kind_at(offset) {
                                    if k3 == LeftParen {
                                        depth += 1;
                                    }
                                    else if k3 == RightParen {
                                        depth -= 1;
                                    }
                                    offset += 1;
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }
                Whitespace | Newline | LineComment | BlockComment => offset += 1,
                _ => return Some(kind),
            }
        }
        None
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Namespace))?;
        self.skip_trivia(state);

        if state.at(Bang) {
            state.bump();
            self.skip_trivia(state);
        }

        self.parse_name_path(state)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }
        else if state.at(Semicolon) {
            state.bump();
        }
        Ok(state.finish_at(cp, Namespace))
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(At) || state.at(Bolt) {
            state.bump();
        }
        else {
            return Err(OakError::custom_error(format!("Expected @ or ↯, but found {:?}", state.current().map(|t| t.kind))));
        }
        self.skip_trivia(state);
        if state.at(LeftBracket) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightBracket) {
                self.parse_name_path(state)?;
                self.skip_trivia(state);
                if state.at(LeftParen) {
                    state.bump();
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        self.parse_expression_internal(state, 0);
                        self.skip_trivia(state);
                        if state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                        }
                    }
                    state.expect(RightParen)?;
                    self.skip_trivia(state);
                }
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightBracket)?
        }
        else {
            self.parse_name_path(state)?;
            self.skip_trivia(state);
            if state.at(LeftParen) {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightParen) {
                    self.parse_expression_internal(state, 0);
                    self.skip_trivia(state);
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightParen)?;
            }
        }
        Ok(state.finish_at(cp, Attribute))
    }

    fn parse_micro<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Micro))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(Arrow) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
    }

    fn parse_micro_lambda<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(Keyword(DejavuKeywords::Micro)) {
            state.bump();
        }
        else {
            state.expect(Keyword(DejavuKeywords::Lambda))?;
        }
        self.skip_trivia(state);

        // Name is optional for lambda
        if state.at(Identifier) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }
        self.skip_trivia(state);

        if state.at(Arrow) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
    }

    pub(crate) fn parse_mezzo<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Mezzo))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Mezzo))
    }

    fn parse_widget<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Widget))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Widget))
    }

    fn parse_singleton<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Singleton))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Class))
    }

    fn parse_trait<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Trait))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(Eof) {
                let cp_type = state.checkpoint();
                self.parse_name_path(state)?;
                state.finish_at(cp_type, Type);
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Trait))
    }

    fn parse_effect<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Effect))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, EffectDefinition))
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Class))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_parameter_list(state)?;
            self.skip_trivia(state);
        }

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(Eof) {
                let cp_type = state.checkpoint();
                self.parse_name_path(state)?;
                state.finish_at(cp_type, Type);
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Class))
    }

    fn parse_flags<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state)
        }

        state.expect(Keyword(DejavuKeywords::Flags))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_variant_block(state)?;
        }
        Ok(state.finish_at(cp, Flags))
    }

    fn parse_enums<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Enums))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_variant_block(state)?;
        }

        Ok(state.finish_at(cp, Enums))
    }

    fn parse_variant_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace || t.kind == Eof {
                break;
            }

            if t.kind == At || t.kind == Bolt || t.kind == Identifier {
                self.parse_variant(state)?;
            }
            else {
                // Ignore or skip other tokens within the block to maintain focus on variants
                state.bump();
            }

            self.skip_trivia(state);

            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, BlockExpression))
    }

    fn parse_variant<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(Eq) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression_internal(state, 0);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Variant))
    }

    fn parse_generic_parameter_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LessThan)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(GreaterThan) {
            state.expect(Identifier)?;
            self.skip_trivia(state);
            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(GreaterThan)?;
        Ok(state.finish_at(cp, GenericParameterList))
    }

    fn parse_generic_argument_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LessThan)?;
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(GreaterThan) {
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(GreaterThan)?;
        Ok(state.finish_at(cp, GenericArgumentList))
    }

    fn parse_parameter_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(LeftParen)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightParen || t.kind == Eof {
                break;
            }

            if t.kind == Identifier || t.kind == At || t.kind == Bolt {
                self.parse_parameter(state)?;
                self.skip_trivia(state);

                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            else {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightParen)?;

        Ok(state.finish_at(cp, ParameterList))
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Identifier)?;
        self.skip_trivia(state);
        if state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }
        Ok(state.finish_at(cp, Parameter))
    }

    fn parse_let_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        state.expect(Keyword(DejavuKeywords::Let))?;
        self.skip_trivia(state);

        if state.at(Keyword(DejavuKeywords::Mut)) {
            state.bump();
            self.skip_trivia(state);
        }

        self.parse_pattern(state)?;
        self.skip_trivia(state);

        if state.at(Eq) {
            state.bump();
            self.skip_trivia(state);

            self.parse_expression_internal(state, 0);
            self.skip_trivia(state);
        }

        if state.at(Semicolon) {
            state.bump();
        }

        Ok(state.finish_at(cp, LetStatement))
    }

    fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.at(At) || state.at(Bolt) {
            self.parse_attribute(state)?;
            self.skip_trivia(state);
        }

        self.parse_expression_internal(state, 0);

        if state.at(Semicolon) {
            state.bump();
        }

        Ok(state.finish_at(cp, ExpressionStatement))
    }

    pub(crate) fn parse_expression_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, DejavuLanguage> {
        self.skip_trivia(state);
        oak_core::parser::PrattParser::parse(state, min_precedence, self)
    }

    fn parse_name_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        if state.at(Identifier) || matches!(state.current().map(|t| t.kind), Some(Keyword(_))) {
            state.bump();
        }
        else {
            state.expect(Identifier)?;
        }
        self.skip_trivia(state);

        if state.at(LessThan) {
            self.parse_generic_argument_list(state)?;
            self.skip_trivia(state);
        }

        while state.at(ColonColon) || state.at(Dot) {
            state.bump();
            self.skip_trivia(state);
            if state.at(Identifier) || matches!(state.current().map(|t| t.kind), Some(Keyword(_))) {
                state.bump();
            }
            else {
                state.expect(Identifier)?;
            }
            self.skip_trivia(state);

            if state.at(LessThan) {
                self.parse_generic_argument_list(state)?;
                self.skip_trivia(state);
            }
        }

        let node = state.finish_at(cp, NamePath);
        Ok(node)
    }

    fn parse_using_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(DejavuKeywords::Using))?;
        self.skip_trivia(state);

        self.parse_name_path(state)?;
        self.skip_trivia(state);

        state.expect(Semicolon)?;

        Ok(state.finish_at(cp, UsingStatement))
    }

    pub(crate) fn parse_primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let t = state.current().ok_or_else(|| OakError::custom_error("Unexpected end of file"))?;

        match t.kind {
            Identifier => {
                let cp_name = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                if state.at(ColonColon) {
                    let cp_path = cp_name;
                    while state.at(ColonColon) {
                        state.bump();
                        self.skip_trivia(state);
                        state.expect(Identifier)?;
                        self.skip_trivia(state);
                    }
                    state.finish_at(cp_path, NamePath);
                    Ok(state.finish_at(cp, PathExpression))
                }
                else {
                    Ok(state.finish_at(cp, IdentifierExpression))
                }
            }
            IntegerLiteral | FloatLiteral | BoolLiteral | Keyword(DejavuKeywords::True) | Keyword(DejavuKeywords::False) => {
                state.bump();
                Ok(state.finish_at(cp, LiteralExpression))
            }
            StringLiteral | CharLiteral => self.parse_string_literal(state),
            LeftParen => {
                state.bump();
                self.parse_expression_internal(state, 0);
                state.expect(RightParen)?;
                Ok(state.finish_at(cp, ParenthesizedExpression))
            }
            LeftBracket => {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBracket) {
                    self.parse_expression_internal(state, 0);
                    self.skip_trivia(state);
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightBracket)?;
                Ok(state.finish_at(cp, LiteralExpression))
            }
            LeftBrace => self.parse_block_expr_node(state),
            Dollar => {
                state.bump();
                if state.at(IntegerLiteral) {
                    state.bump();
                }
                else if state.at(Keyword(DejavuKeywords::True)) || state.at(Keyword(DejavuKeywords::False)) {
                    state.bump();
                }
                else if state.at(Identifier) {
                    state.bump();
                }
                Ok(state.finish_at(cp, IdentifierExpression))
            }
            Keyword(DejavuKeywords::Class) => self.parse_anonymous_class(state),
            Keyword(DejavuKeywords::Micro) => self.parse_micro_lambda(state),
            Keyword(DejavuKeywords::Lambda) => self.parse_micro_lambda(state),
            Keyword(DejavuKeywords::If) => self.parse_if(state),
            Keyword(DejavuKeywords::Match) => self.parse_match(state),
            Keyword(DejavuKeywords::While) | Keyword(DejavuKeywords::For) | Keyword(DejavuKeywords::Loop) => self.parse_loop(state),
            Keyword(DejavuKeywords::Return) => self.parse_return(state),
            Keyword(DejavuKeywords::Break) => self.parse_break(state),
            Keyword(DejavuKeywords::Continue) => self.parse_continue(state),
            Keyword(DejavuKeywords::Yield) => self.parse_yield(state),
            Keyword(DejavuKeywords::Raise) => self.parse_raise(state),
            Keyword(DejavuKeywords::Resume) => self.parse_resume(state),
            Keyword(DejavuKeywords::Try) => self.parse_catch(state),
            At | Bolt => {
                state.bump();
                state.expect(Identifier)?;
                self.skip_trivia(state);
                // After @assert or ↯assert, we expect a parameter list which makes it a CallExpression
                if state.at(LeftParen) {
                    let cp_call = cp;
                    state.finish_at(cp, IdentifierExpression);

                    state.expect(LeftParen)?;
                    self.skip_trivia(state);
                    while state.not_at_end() && !state.at(RightParen) {
                        self.parse_expression_internal(state, 0);
                        self.skip_trivia(state);
                        if state.at(Comma) {
                            state.bump();
                            self.skip_trivia(state);
                        }
                    }
                    state.expect(RightParen)?;
                    self.skip_trivia(state);
                    if state.at(LeftBrace) {
                        self.parse_block_expr_node(state).ok();
                        Ok(state.finish_at(cp_call, ObjectExpression))
                    }
                    else {
                        Ok(state.finish_at(cp_call, CallExpression))
                    }
                }
                else {
                    Ok(state.finish_at(cp, IdentifierExpression))
                }
            }
            _ => Err(OakError::custom_error(format!("Unexpected token: {:?}", t.kind))),
        }
    }

    fn parse_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.bump(); // consume StringLiteral or CharLiteral

        // Now handle interleaved StringPart, InterpolationStart, InterpolationEnd
        // They appear after StringLiteral because of lexer's add_token order.
        while let Some(t) = state.current() {
            match t.kind {
                StringPart => {
                    state.bump();
                }
                InterpolationStart => {
                    let icp = state.checkpoint();
                    state.bump();
                    self.skip_trivia(state);
                    self.parse_expression_internal(state, 0);
                    self.skip_trivia(state);
                    state.expect(InterpolationEnd)?;
                    state.finish_at(icp, Interpolation);
                }
                TemplateControlStart => {
                    let icp = state.checkpoint();
                    state.bump();
                    self.skip_trivia(state);
                    // Inside <% %>, we can have any statement
                    self.parse_source_file(state)?;
                    self.skip_trivia(state);
                    state.expect(TemplateControlEnd)?;
                    state.finish_at(icp, TemplateControlStart); // Or a new kind if needed
                }
                _ => break,
            }
        }

        Ok(state.finish_at(cp, LiteralExpression))
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::If))?;
        self.skip_trivia(state);

        if state.at(Keyword(DejavuKeywords::Let)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Eq)?;
            self.skip_trivia(state);
            self.parse_expression_internal(state, 8);
        }
        else {
            self.parse_expression_internal(state, 8);
            self.skip_trivia(state);
            if state.at(Keyword(DejavuKeywords::Is)) {
                let cp_is = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                self.parse_name_path(state)?;
                state.finish_at(cp_is, BinaryExpression);
            }
        }

        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        self.skip_trivia(state);
        if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
            self.skip_trivia(state);
            if state.at(Keyword(DejavuKeywords::If)) {
                self.parse_if(state)?;
            }
            else {
                self.parse_block_expr_node(state)?;
            }
        }
        Ok(state.finish_at(cp, IfExpression))
    }

    fn parse_match<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Match))?;
        self.skip_trivia(state);
        self.parse_expression_internal(state, 8);
        self.skip_trivia(state);

        state.expect(LeftBrace)?;
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(RightBrace) && !state.at(Eof) {
            self.parse_match_arm(state)?;
            self.skip_trivia(state);
        }
        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, MatchExpression))
    }

    fn parse_match_arm<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        if state.at(Keyword(DejavuKeywords::Case)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_pattern(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::Type)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_name_path(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::When)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression_internal(state, 10);
        }
        else if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
        }
        else {
            self.parse_pattern(state)?;
        }
        self.skip_trivia(state);

        if state.at(Keyword(DejavuKeywords::When)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression_internal(state, 10);
            self.skip_trivia(state);
        }

        if state.at(Arrow) || state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            self.parse_expression_internal(state, 0);
        }
        else if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }
        else {
            return Err(OakError::custom_error(format!("Expected ->, : or {{, but found {:?}", state.current().map(|t| t.kind))));
        }
        self.skip_trivia(state);
        if state.at(Comma) || state.at(Semicolon) {
            state.bump();
        }
        Ok(state.finish_at(cp, MatchArm))
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(Keyword(DejavuKeywords::Is)) {
            state.bump();
            self.skip_trivia(state);
            self.parse_name_path(state)?;
        }
        else if state.at(Keyword(DejavuKeywords::Else)) {
            state.bump();
        }
        else if state.at(Identifier) {
            self.parse_name_path(state)?;
            self.skip_trivia(state);
            if state.at(LeftBrace) {
                state.bump();
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBrace) {
                    state.expect(Identifier)?;
                    self.skip_trivia(state);
                    if state.at(Eq) {
                        state.bump();
                        self.skip_trivia(state);
                        self.parse_pattern(state)?;
                        self.skip_trivia(state);
                    }
                    if state.at(Comma) {
                        state.bump();
                        self.skip_trivia(state);
                    }
                }
                state.expect(RightBrace)?;
            }
        }
        else if state.at(IntegerLiteral) || state.at(StringLiteral) || state.at(Keyword(DejavuKeywords::True)) || state.at(Keyword(DejavuKeywords::False)) {
            state.bump();
        }
        else if state.at(Underscore) {
            state.bump();
        }
        else {
            return Err(OakError::custom_error(format!("Expected pattern, found {:?}", state.current().map(|t| t.kind))));
        }
        Ok(state.finish_at(cp, Pattern))
    }

    fn parse_loop<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        let is_while = state.at(Keyword(DejavuKeywords::While));
        let is_for = state.at(Keyword(DejavuKeywords::For));
        let is_loop = state.at(Keyword(DejavuKeywords::Loop));
        state.bump(); // while, for or loop
        self.skip_trivia(state);

        if is_while && state.at(Keyword(DejavuKeywords::Let)) {
            state.bump(); // let
            self.skip_trivia(state);
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Eq)?;
            self.skip_trivia(state);
            self.parse_expression_internal(state, 8);
        }
        else if is_for {
            self.parse_pattern(state)?;
            self.skip_trivia(state);
            state.expect(Keyword(DejavuKeywords::In))?;
            self.skip_trivia(state);
            self.parse_expression_internal(state, 8);
        }
        else if is_loop {
            // Check if it's "loop pattern in expression"
            let pattern_cp = state.checkpoint();
            if self.parse_pattern(state).is_ok() {
                self.skip_trivia(state);
                if state.at(Keyword(DejavuKeywords::In)) {
                    state.bump(); // in
                    self.skip_trivia(state);
                    self.parse_expression_internal(state, 8);
                }
                else {
                    // Not a for-style loop, backtrack pattern if possible
                    state.restore(pattern_cp);
                }
            }
            else {
                state.restore(pattern_cp);
            }
        }
        else {
            self.parse_expression_internal(state, 8);
        }

        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        Ok(state.finish_at(cp, LoopExpression))
    }

    fn parse_return<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Return))?;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, ReturnExpression))
    }

    fn parse_break<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Break))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
            self.skip_trivia(state);
        }
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            self.parse_expression_internal(state, 0);
        }

        Ok(state.finish_at(cp, BreakExpression))
    }

    fn parse_continue<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Continue))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
        }

        Ok(state.finish_at(cp, ContinueExpression))
    }

    fn parse_yield<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Yield))?;
        self.skip_trivia(state);
        if state.at(Star) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, YieldExpression))
    }

    fn parse_raise<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Raise))?;
        self.skip_trivia(state);
        self.parse_expression_internal(state, 0);
        Ok(state.finish_at(cp, RaiseExpression))
    }

    fn parse_resume<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Resume))?;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            self.parse_expression_internal(state, 0);
        }
        Ok(state.finish_at(cp, ResumeExpression))
    }

    fn parse_catch<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Try))?;
        self.skip_trivia(state);

        // try Result<T, E> { ... }
        // check if we have a type inline expression before the block
        if state.at(Identifier) {
            let cp_type = state.checkpoint();
            self.parse_name_path(state)?;
            if state.at(LessThan) {
                self.parse_generic_argument_list(state)?;
            }
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        // try must be followed by a block
        self.parse_block_expr_node(state)?;
        self.skip_trivia(state);

        while state.at(Dot) {
            let cp_dot = state.checkpoint();
            state.bump();
            self.skip_trivia(state);

            if state.at(Keyword(DejavuKeywords::Catch)) {
                state.bump();
                self.skip_trivia(state);
                state.expect(LeftBrace)?;
                self.skip_trivia(state);
                while state.not_at_end() && !state.at(RightBrace) && !state.at(Eof) {
                    self.parse_match_arm(state)?;
                    self.skip_trivia(state);
                }
                state.expect(RightBrace)?;
                self.skip_trivia(state);
            }
            else {
                state.restore(cp_dot);
                break;
            }
        }
        Ok(state.finish_at(cp, CatchExpression))
    }

    fn parse_anonymous_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(DejavuKeywords::Class))?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) && !state.at(Eof) {
                state.expect(Identifier)?;
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        self.parse_block_expr_node(state)?;

        Ok(state.finish_at(cp, AnonymousClass))
    }

    pub(crate) fn parse_block_expr_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        if !state.at(LeftBrace) {
            return Err(OakError::custom_error(format!("expected token 'LeftBrace' at {:?}", state.current().map(|t| t.span.start))));
        }
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace || t.kind == Eof {
                break;
            }
            let start_index = state.tokens.index();
            match self.parse_statement(state) {
                Ok(_) => {}
                Err(_) => {
                    if state.tokens.index() == start_index {
                        state.bump();
                    }
                }
            }
            self.skip_trivia(state);

            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }
        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, BlockExpression))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DejavuLanguage>, OakError> {
        self.skip_trivia(state);
        if let Some(t) = state.current() {
            match t.kind {
                At | Bolt => {
                    let next = self.peek_after_attributes(state);
                    match next {
                        Some(Keyword(DejavuKeywords::Micro)) => self.parse_micro(state),
                        Some(Keyword(DejavuKeywords::Class)) => self.parse_class(state),
                        Some(Keyword(DejavuKeywords::Enums)) => self.parse_enums(state),
                        Some(Keyword(DejavuKeywords::Flags)) => self.parse_flags(state),
                        Some(Keyword(DejavuKeywords::Trait)) => self.parse_trait(state),
                        Some(Keyword(DejavuKeywords::Effect)) => self.parse_effect(state),
                        Some(Keyword(DejavuKeywords::Mezzo)) => self.parse_mezzo(state),
                        Some(Keyword(DejavuKeywords::Widget)) => self.parse_widget(state),
                        Some(Keyword(DejavuKeywords::Namespace)) => self.parse_namespace(state),
                        Some(Keyword(DejavuKeywords::Let)) => self.parse_let_statement(state),
                        _ => self.parse_expression_statement(state),
                    }
                }
                Keyword(DejavuKeywords::Let) => self.parse_let_statement(state),
                Keyword(DejavuKeywords::Micro) => self.parse_micro(state),
                Keyword(DejavuKeywords::Class) => self.parse_class(state),
                Keyword(DejavuKeywords::Enums) => self.parse_enums(state),
                Keyword(DejavuKeywords::Flags) => self.parse_flags(state),
                Keyword(DejavuKeywords::Trait) => self.parse_trait(state),
                Keyword(DejavuKeywords::Effect) => self.parse_effect(state),
                Keyword(DejavuKeywords::Mezzo) => self.parse_mezzo(state),
                Keyword(DejavuKeywords::Widget) => self.parse_widget(state),
                Keyword(DejavuKeywords::Namespace) => self.parse_namespace(state),
                Keyword(DejavuKeywords::Using) => self.parse_using_statement(state),
                _ => self.parse_expression_statement(state),
            }
        }
        else {
            Err(OakError::custom_error("Unexpected end of file"))
        }
    }
}
