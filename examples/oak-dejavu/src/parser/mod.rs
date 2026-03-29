use crate::{DejavuLanguage, DejavuLexer};
pub use element_type::DejavuElementType;
use oak_core::{Parser, TextEdit, parser::ParseCache};
// Import extension trait to bring parse methods into scope
use parse_raw::DejavuParserExt;

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

/// Checkpoint type alias for parser state positions.
pub type Checkpoint = (usize, usize);

/// Control flow type enumeration.
#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowType {
    /// For loop with pattern and iterable expression.
    For {
        /// Pattern for binding loop variable.
        pattern: String,
        /// Expression to iterate over.
        iterable: String,
        /// Optional filter condition.
        filter: Option<String>,
    },
    /// If statement with condition expression.
    If {
        /// Condition expression.
        condition: String,
    },
    /// While loop with condition expression.
    While {
        /// Condition expression.
        condition: String,
    },
    /// Loop statement with pattern and iterable expression.
    Loop {
        /// Pattern for binding loop variable.
        pattern: String,
        /// Expression to iterate over.
        iterable: String,
    },
}

/// Control flow frame for tracking nested control flow structures.
#[derive(Debug, Clone)]
pub struct ControlFlowFrame {
    /// Control flow type.
    pub flow_type: ControlFlowType,
    /// Starting checkpoint (before the control statement).
    pub start_checkpoint: Checkpoint,
    /// Whether we're in an else branch.
    pub in_else: bool,
}

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

    /// Parse control flow start statement (for, if, while, loop).
    fn parse_control_start<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        if let Some(token) = state.current() {
            match token.kind {
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::For) => {
                    self.parse_for_statement(state, stack, cp)?;
                }
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If) => {
                    self.parse_if_statement(state, stack, cp)?;
                }
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::While) => {
                    self.parse_while_statement(state, stack, cp)?;
                }
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Loop) => {
                    self.parse_loop_statement(state, stack, cp)?;
                }
                _ => {
                    while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                        state.bump();
                    }
                    if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                        state.bump();
                    }
                    state.finish_at(cp, DejavuElementType::TemplateControl);
                }
            }
        }
        Ok(())
    }

    /// Parse control flow end statement (end for, end if, end while, else).
    fn parse_control_end<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        if let Some(token) = state.current() {
            match token.kind {
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End) => {
                    self.parse_end_statement(state, stack, cp)?;
                }
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else) => {
                    self.parse_else_statement(state, stack, cp)?;
                }
                _ => {
                    while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                        state.bump();
                    }
                    if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                        state.bump();
                    }
                    state.finish_at(cp, DejavuElementType::TemplateControl);
                }
            }
        }
        Ok(())
    }

    /// Parse for loop statement.
    fn parse_for_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::For))?;
        self.skip_trivia(state);

        let pattern_cp = state.checkpoint();
        if state.at(crate::lexer::token_type::DejavuTokenType::Identifier) {
            state.bump();
            self.skip_trivia(state);

            if state.at(crate::lexer::token_type::DejavuTokenType::Comma) {
                state.bump();
                self.skip_trivia(state);
                if state.at(crate::lexer::token_type::DejavuTokenType::Identifier) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
        }
        state.finish_at(pattern_cp, DejavuElementType::Pattern);

        if state.at(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::In)) {
            state.bump();
            self.skip_trivia(state);
        }

        let expr_cp = state.checkpoint();
        let inner_cp = state.checkpoint();
        let mut has_tokens = false;
        while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            if let Some(token) = state.current() {
                if token.kind == crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If) {
                    break;
                }
            }
            has_tokens = true;
            state.bump();
        }
        if has_tokens {
            state.finish_at(inner_cp, DejavuElementType::PathExpression);
        }
        state.finish_at(expr_cp, DejavuElementType::Expression);

        if state.at(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If)) {
            state.bump();
            self.skip_trivia(state);
            let filter_cp = state.checkpoint();
            let filter_inner_cp = state.checkpoint();
            let mut has_filter_tokens = false;
            while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                has_filter_tokens = true;
                state.bump();
            }
            if has_filter_tokens {
                state.finish_at(filter_inner_cp, DejavuElementType::PathExpression);
            }
            state.finish_at(filter_cp, DejavuElementType::Expression);
        }

        if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        let frame = ControlFlowFrame { flow_type: ControlFlowType::For { pattern: String::new(), iterable: String::new(), filter: None }, start_checkpoint: cp, in_else: false };
        stack.push(frame);

        Ok(())
    }

    /// Parse if statement.
    fn parse_if_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If))?;
        self.skip_trivia(state);

        let expr_cp = state.checkpoint();
        let inner_cp = state.checkpoint();
        let mut has_tokens = false;
        while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            has_tokens = true;
            state.bump();
        }
        if has_tokens {
            state.finish_at(inner_cp, DejavuElementType::PathExpression);
        }
        state.finish_at(expr_cp, DejavuElementType::Expression);

        if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        let frame = ControlFlowFrame { flow_type: ControlFlowType::If { condition: String::new() }, start_checkpoint: cp, in_else: false };
        stack.push(frame);

        Ok(())
    }

    /// Parse while loop statement.
    fn parse_while_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::While))?;
        self.skip_trivia(state);

        let expr_cp = state.checkpoint();
        let inner_cp = state.checkpoint();
        let mut has_tokens = false;
        while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            has_tokens = true;
            state.bump();
        }
        if has_tokens {
            state.finish_at(inner_cp, DejavuElementType::PathExpression);
        }
        state.finish_at(expr_cp, DejavuElementType::Expression);

        if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        let frame = ControlFlowFrame { flow_type: ControlFlowType::While { condition: String::new() }, start_checkpoint: cp, in_else: false };
        stack.push(frame);

        Ok(())
    }

    /// Parse loop statement.
    fn parse_loop_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Loop))?;
        self.skip_trivia(state);

        let pattern_cp = state.checkpoint();
        if state.at(crate::lexer::token_type::DejavuTokenType::Identifier) {
            state.bump();
            self.skip_trivia(state);

            if state.at(crate::lexer::token_type::DejavuTokenType::Comma) {
                state.bump();
                self.skip_trivia(state);
                if state.at(crate::lexer::token_type::DejavuTokenType::Identifier) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
        }
        state.finish_at(pattern_cp, DejavuElementType::Pattern);

        if state.at(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::In)) {
            state.bump();
            self.skip_trivia(state);
        }

        let expr_cp = state.checkpoint();
        let inner_cp = state.checkpoint();
        let mut has_tokens = false;
        while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            has_tokens = true;
            state.bump();
        }
        if has_tokens {
            state.finish_at(inner_cp, DejavuElementType::NamePath);
        }
        state.finish_at(expr_cp, DejavuElementType::Expression);

        if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        let frame = ControlFlowFrame { flow_type: ControlFlowType::Loop { pattern: String::new(), iterable: String::new() }, start_checkpoint: cp, in_else: false };
        stack.push(frame);

        Ok(())
    }

    /// Parse end statement.
    fn parse_end_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, _cp: Checkpoint) -> Result<(), oak_core::OakError> {
        // 当前 token 是 End 关键字，消耗它让它成为节点的一部分
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End))?;
        self.skip_trivia(state);

        // 检查下一个 token 来确定控制流类型
        let expected_keyword = if let Some(token) = state.current() {
            match token.kind {
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::For) => Some("for"),
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If) => Some("if"),
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::While) => Some("while"),
                crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Loop) => Some("loop"),
                _ => None,
            }
        }
        else {
            None
        };

        let frame = if let Some(keyword) = expected_keyword {
            // 弹出栈帧并验证类型匹配
            if let Some(frame) = stack.pop() {
                let actual_keyword = match &frame.flow_type {
                    ControlFlowType::For { .. } => "for",
                    ControlFlowType::If { .. } => "if",
                    ControlFlowType::While { .. } => "while",
                    ControlFlowType::Loop { .. } => "loop",
                };

                if keyword != actual_keyword {
                    return Err(oak_core::OakError::custom_error(format!("Mismatched end marker: expected 'end {}' but found 'end {}'", actual_keyword, keyword)));
                }
                Some(frame)
            }
            else {
                return Err(oak_core::OakError::custom_error(format!("Unexpected 'end {}' without matching '{}' block", keyword, keyword)));
            }
        }
        else {
            if let Some(frame) = stack.pop() {
                let actual_keyword = match &frame.flow_type {
                    ControlFlowType::For { .. } => "for",
                    ControlFlowType::If { .. } => "if",
                    ControlFlowType::While { .. } => "while",
                    ControlFlowType::Loop { .. } => "loop",
                };
                return Err(oak_core::OakError::custom_error(format!("Missing keyword after 'end': expected 'end {}'", actual_keyword)));
            }
            else {
                return Err(oak_core::OakError::custom_error("Unexpected 'end' without matching control flow block"));
            }
        };

        // 消耗 for/if/while/loop 关键字和剩余 token，让它们成为节点的一部分
        while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
            state.bump();
        }

        // 现在调用 finish_at，它会包含从 for 开始到 end for 的所有 token
        if let Some(frame) = frame {
            let element_type = match &frame.flow_type {
                ControlFlowType::For { .. } => DejavuElementType::ForControl,
                ControlFlowType::If { .. } => DejavuElementType::IfControl,
                ControlFlowType::While { .. } => DejavuElementType::WhileControl,
                ControlFlowType::Loop { .. } => DejavuElementType::LoopControl,
            };
            state.finish_at(frame.start_checkpoint, element_type);
        }

        Ok(())
    }

    /// Parse else statement.
    fn parse_else_statement<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<ControlFlowFrame>, cp: Checkpoint) -> Result<(), oak_core::OakError> {
        state.expect(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else))?;
        self.skip_trivia(state);

        let is_else_if = state.at(crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If));

        if let Some(frame) = stack.last_mut() {
            match &frame.flow_type {
                ControlFlowType::If { .. } => {
                    if is_else_if {
                        state.finish_at(cp, DejavuElementType::ElseBranch);
                    }
                    else {
                        frame.in_else = true;
                        state.finish_at(cp, DejavuElementType::ElseBranch);
                    }
                }
                ControlFlowType::For { .. } => {
                    frame.in_else = true;
                    state.finish_at(cp, DejavuElementType::ElseBranch);
                }
                ControlFlowType::While { .. } => {
                    return Err(oak_core::OakError::custom_error("'else' cannot be used with 'while' block"));
                }
                ControlFlowType::Loop { .. } => {
                    return Err(oak_core::OakError::custom_error("'else' cannot be used with 'loop' block"));
                }
            }
        }
        else {
            return Err(oak_core::OakError::custom_error("Unexpected 'else' without matching control flow block"));
        }

        if is_else_if {
            while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                state.bump();
            }
        }
        else {
            while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                state.bump();
            }
        }

        Ok(())
    }
}

impl<'config> Parser<DejavuLanguage> for DejavuParser<'config> {
    fn parse<'a, S: oak_core::Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DejavuLanguage>) -> oak_core::parser::ParseOutput<'a, DejavuLanguage> {
        oak_core::parser::parse_with_lexer(&DejavuLexer::new(&self.config), text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            let mut control_flow_stack: Vec<ControlFlowFrame> = Vec::new();

            while state.not_at_end() {
                self.skip_trivia(state);
                if let Some(token) = state.current() {
                    match token.kind {
                        crate::lexer::token_type::DejavuTokenType::CodeStart | crate::lexer::token_type::DejavuTokenType::TemplateControlStart => {
                            let cp = state.checkpoint();
                            state.bump();
                            self.skip_trivia(state);

                            let is_control_end = if let Some(next_token) = state.current() {
                                matches!(next_token.kind, crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::End) | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Else))
                            }
                            else {
                                false
                            };

                            let is_control_start = if let Some(next_token) = state.current() {
                                matches!(
                                    next_token.kind,
                                    crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::For)
                                        | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::If)
                                        | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::While)
                                        | crate::lexer::token_type::DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::Loop)
                                )
                            }
                            else {
                                false
                            };

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

                            if is_control_end {
                                self.parse_control_end(state, &mut control_flow_stack, cp)?;
                            }
                            else if is_control_start {
                                self.parse_control_start(state, &mut control_flow_stack, cp)?;
                            }
                            else if is_interpolation {
                                let _expr_cp = state.checkpoint();
                                self.parse_expression_internal(state, 0).ok();
                                while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                                    state.bump();
                                }
                                if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                                    state.bump();
                                }
                                state.finish_at(cp, DejavuElementType::Interpolation);
                            }
                            else {
                                while state.not_at_end() && !state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) && !state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                                    state.bump();
                                }
                                if state.at(crate::lexer::token_type::DejavuTokenType::TemplateControlEnd) || state.at(crate::lexer::token_type::DejavuTokenType::CodeEnd) {
                                    state.bump();
                                }
                                state.finish_at(cp, DejavuElementType::TemplateControl);
                            }
                        }
                        crate::lexer::token_type::DejavuTokenType::StringPart => {
                            let cp = state.checkpoint();
                            state.bump();
                            state.finish_at(cp, DejavuElementType::TemplateText);
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

            if !control_flow_stack.is_empty() {
                for frame in control_flow_stack {
                    let flow_type_name = match frame.flow_type {
                        ControlFlowType::For { .. } => "for",
                        ControlFlowType::If { .. } => "if",
                        ControlFlowType::While { .. } => "while",
                        ControlFlowType::Loop { .. } => "loop",
                    };
                    return Err(oak_core::OakError::custom_error(format!("Unclosed control flow: '{}' block not closed with 'end {}'", flow_type_name, flow_type_name)));
                }
            }

            Ok(state.finish_at(checkpoint, DejavuElementType::Root))
        })
    }
}
