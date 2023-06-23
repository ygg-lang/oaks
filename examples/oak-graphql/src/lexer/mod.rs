#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::GraphQLLanguage, lexer::token_type::GraphQLTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, GraphQLLanguage>;

static GRAPHQL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GRAPHQL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });
static GRAPHQL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct GraphQLLexer<'config> {
    _config: &'config GraphQLLanguage,
}

impl<'config> Lexer<GraphQLLanguage> for GraphQLLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<GraphQLLanguage>) -> LexOutput<GraphQLLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> GraphQLLexer<'config> {
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GRAPHQL_WHITESPACE.scan(state, GraphQLTokenType::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GRAPHQL_COMMENT.scan(state, GraphQLTokenType::Comment, GraphQLTokenType::Comment)
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        // 普通字符串 "..."
        if GRAPHQL_STRING.scan(state, GraphQLTokenType::StringLiteral) {
            return true;
        }

        // 多行字符串 """..."""
        if state.starts_with("\"\"\"") {
            let start = state.get_position();
            state.advance(3); // 跳过开始的 """

            while state.not_at_end() {
                if state.starts_with("\"\"\"") {
                    state.advance(3); // 跳过结束的 """
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GraphQLTokenType::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理负号
        if state.starts_with("-") {
            state.advance(1);
        }

        // 处理整数部分
        if state.starts_with("0") {
            // 单独的 0
            state.advance(1);
            has_digits = true;
        }
        else {
            // 非零开头的数字
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                    has_digits = true;
                }
                else {
                    break;
                }
            }
        }

        // 处理小数部分
        if state.starts_with(".") && has_digits {
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch.is_ascii_digit() {
                    state.advance(1); // 跳过 .
                    is_float = true;

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        // 处理指数部分
        if (state.starts_with("e") || state.starts_with("E")) && has_digits {
            state.advance(1);
            is_float = true;

            // 处理指数符号
            if state.starts_with("+") || state.starts_with("-") {
                state.advance(1);
            }

            // 处理指数数字
            let mut exp_digits = false;
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                    exp_digits = true;
                }
                else {
                    break;
                }
            }
            if !exp_digits {
                return false;
            }
        }

        if !has_digits {
            return false;
        }

        let kind = if is_float { GraphQLTokenType::FloatLiteral } else { GraphQLTokenType::IntLiteral };
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 词法分析标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 标识符必须以字母或下划线开始
        if let Some(first_ch) = state.peek() {
            if !first_ch.is_alphabetic() && first_ch != '_' {
                return false;
            }

            state.advance(first_ch.len_utf8());

            // 后续字符可以是字母、数字或下划线
            while let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            let end = state.get_position();
            let text = state.get_text_in((start..end).into());
            let kind = self.keyword_or_identifier(&text);
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> GraphQLTokenType {
        match text {
            // 关键字
            "query" => GraphQLTokenType::QueryKeyword,
            "mutation" => GraphQLTokenType::MutationKeyword,
            "subscription" => GraphQLTokenType::SubscriptionKeyword,
            "fragment" => GraphQLTokenType::FragmentKeyword,
            "on" => GraphQLTokenType::OnKeyword,
            "type" => GraphQLTokenType::TypeKeyword,
            "interface" => GraphQLTokenType::InterfaceKeyword,
            "union" => GraphQLTokenType::UnionKeyword,
            "scalar" => GraphQLTokenType::ScalarKeyword,
            "enum" => GraphQLTokenType::EnumKeyword,
            "input" => GraphQLTokenType::InputKeyword,
            "extend" => GraphQLTokenType::ExtendKeyword,
            "schema" => GraphQLTokenType::SchemaKeyword,
            "directive" => GraphQLTokenType::DirectiveKeyword,
            "implements" => GraphQLTokenType::ImplementsKeyword,
            "repeats" => GraphQLTokenType::RepeatsKeyword,

            // 特殊字面量
            "true" | "false" => GraphQLTokenType::BooleanLiteral,
            "null" => GraphQLTokenType::NullLiteral,

            // 默认为名称
            _ => GraphQLTokenType::Name,
        }
    }

    /// 词法分析操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 三字符操作符
        if state.starts_with("...") {
            state.advance(3);
            state.add_token(GraphQLTokenType::Spread, start, state.get_position());
            return true;
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '(' => Some(GraphQLTokenType::LeftParen),
                ')' => Some(GraphQLTokenType::RightParen),
                '[' => Some(GraphQLTokenType::LeftBracket),
                ']' => Some(GraphQLTokenType::RightBracket),
                '{' => Some(GraphQLTokenType::LeftBrace),
                '}' => Some(GraphQLTokenType::RightBrace),
                ',' => Some(GraphQLTokenType::Comma),
                ':' => Some(GraphQLTokenType::Colon),
                ';' => Some(GraphQLTokenType::Semicolon),
                '|' => Some(GraphQLTokenType::Pipe),
                '&' => Some(GraphQLTokenType::Ampersand),
                '=' => Some(GraphQLTokenType::Equals),
                '!' => Some(GraphQLTokenType::Exclamation),
                '@' => Some(GraphQLTokenType::At),
                '$' => Some(GraphQLTokenType::Dollar),
                _ => None,
            };

            if let Some(token_kind) = kind {
                state.advance(ch.len_utf8());
                let end = state.get_position();
                state.add_token(token_kind, start, end);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }
}
