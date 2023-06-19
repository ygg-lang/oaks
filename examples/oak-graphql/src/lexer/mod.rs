use crate::{kind::GraphQLSyntaxKind, language::GraphQLLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, GraphQLLanguage>;

static GRAPHQL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GRAPHQL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static GRAPHQL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct GraphQLLexer<'config> {
    config: &'config GraphQLLanguage,
}

impl<'config> Lexer<GraphQLLanguage> for GraphQLLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<GraphQLLanguage>,
    ) -> LexOutput<GraphQLLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> GraphQLLexer<'config> {
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(GraphQLSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match GRAPHQL_WHITESPACE.scan(state.rest(), state.get_position(), GraphQLSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match GRAPHQL_COMMENT.scan(state.rest(), state.get_position(), GraphQLSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        // 普通字符串 "..."
        if let Some(token) = GRAPHQL_STRING.scan(state.rest(), state.get_position(), GraphQLSyntaxKind::StringLiteral) {
            state.advance_with(token);
            return true;
        }

        // 多行字符串 """..."""
        if state.rest().starts_with("\"\"\"") {
            let start = state.get_position();
            state.advance(3); // 跳过开始的 """

            while state.not_at_end() {
                if state.rest().starts_with("\"\"\"") {
                    state.advance(3); // 跳过结束的 """
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GraphQLSyntaxKind::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理负号
        if state.rest().starts_with('-') {
            state.advance(1);
        }

        // 处理整数部分
        if state.rest().starts_with('0') {
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
        if state.rest().starts_with('.') && has_digits {
            if let Some(next_ch) = state.rest().chars().nth(1) {
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
        if (state.rest().starts_with('e') || state.rest().starts_with('E')) && has_digits {
            state.advance(1);
            is_float = true;

            // 处理指数符号
            if state.rest().starts_with('+') || state.rest().starts_with('-') {
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
                // 指数部分必须有数字
                return false;
            }
        }

        if has_digits {
            let end = state.get_position();
            let kind = if is_float { GraphQLSyntaxKind::FloatLiteral } else { GraphQLSyntaxKind::IntLiteral };
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 词法分析标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn keyword_or_identifier(&self, text: &str) -> GraphQLSyntaxKind {
        match text {
            // 关键字
            "query" => GraphQLSyntaxKind::QueryKeyword,
            "mutation" => GraphQLSyntaxKind::MutationKeyword,
            "subscription" => GraphQLSyntaxKind::SubscriptionKeyword,
            "fragment" => GraphQLSyntaxKind::FragmentKeyword,
            "on" => GraphQLSyntaxKind::OnKeyword,
            "type" => GraphQLSyntaxKind::TypeKeyword,
            "interface" => GraphQLSyntaxKind::InterfaceKeyword,
            "union" => GraphQLSyntaxKind::UnionKeyword,
            "scalar" => GraphQLSyntaxKind::ScalarKeyword,
            "enum" => GraphQLSyntaxKind::EnumKeyword,
            "input" => GraphQLSyntaxKind::InputKeyword,
            "extend" => GraphQLSyntaxKind::ExtendKeyword,
            "schema" => GraphQLSyntaxKind::SchemaKeyword,
            "directive" => GraphQLSyntaxKind::DirectiveKeyword,
            "implements" => GraphQLSyntaxKind::ImplementsKeyword,
            "repeats" => GraphQLSyntaxKind::RepeatsKeyword,

            // 特殊字面量
            "true" | "false" => GraphQLSyntaxKind::BooleanLiteral,
            "null" => GraphQLSyntaxKind::NullLiteral,

            // 默认为名称
            _ => GraphQLSyntaxKind::Name,
        }
    }

    /// 词法分析操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 三字符操作符
        if rest.starts_with("...") {
            state.advance(3);
            state.add_token(GraphQLSyntaxKind::Spread, start, state.get_position());
            return true;
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '(' => Some(GraphQLSyntaxKind::LeftParen),
                ')' => Some(GraphQLSyntaxKind::RightParen),
                '[' => Some(GraphQLSyntaxKind::LeftBracket),
                ']' => Some(GraphQLSyntaxKind::RightBracket),
                '{' => Some(GraphQLSyntaxKind::LeftBrace),
                '}' => Some(GraphQLSyntaxKind::RightBrace),
                ',' => Some(GraphQLSyntaxKind::Comma),
                ':' => Some(GraphQLSyntaxKind::Colon),
                ';' => Some(GraphQLSyntaxKind::Semicolon),
                '|' => Some(GraphQLSyntaxKind::Pipe),
                '&' => Some(GraphQLSyntaxKind::Ampersand),
                '=' => Some(GraphQLSyntaxKind::Equals),
                '!' => Some(GraphQLSyntaxKind::Exclamation),
                '@' => Some(GraphQLSyntaxKind::At),
                '$' => Some(GraphQLSyntaxKind::Dollar),
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
