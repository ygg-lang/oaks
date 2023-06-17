use crate::{kind::GraphQLSyntaxKind, language::GraphQLLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, GraphQLLanguage>;

pub struct GraphQLLexer<'config> {
    config: &'config GraphQLLanguage,
}

impl<'config> GraphQLLexer<'config> {
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(GraphQLSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(GraphQLSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(GraphQLSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            // GraphQL 只有单行注释，以 # 开            state.advance(1); // 跳过 "#"
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(GraphQLSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理名称（标识符和关键字
    fn lex_name(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
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
                    "true" | "false" => GraphQLSyntaxKind::BooleanLiteral,
                    "null" => GraphQLSyntaxKind::NullLiteral,
                    _ => GraphQLSyntaxKind::Name,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理数字字面
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '-' {
                // 处理负号
                if ch == '-' {
                    state.advance(1);
                }

                // 处理整数部分
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        self.scan_digits(state);
                    }
                    else {
                        // 如果负号后面不是数字，回退
                        if ch == '-' {
                            return false;
                        }
                    }
                }

                let mut is_float = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1);
                    self.scan_digits(state);
                    is_float = true;
                }

                // 检查科学记数法
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    self.scan_digits(state);
                    is_float = true;
                }

                let token_kind = if is_float { GraphQLSyntaxKind::FloatLiteral } else { GraphQLSyntaxKind::IntLiteral };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 扫描数字
    fn scan_digits(&self, state: &mut State) {
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始的引号

            // 检查是否是块字符串（三个引号）
            if let Some('"') = state.peek() {
                if let Some('"') = source.get_char_at(state.get_position() + 1) {
                    // 块字符串
                    state.advance(2); // 跳过另外两个引号
                    self.lex_block_string(state, source, start_pos);
                    return true;
                }
            }

            // 普通字符串
            let mut escaped = false;
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                }
                else if ch == '\\' {
                    escaped = true;
                }
                else if ch == '"' {
                    state.advance(1); // 跳过结束的引                    found_end = true;
                    break;
                }
                else if ch == '\n' {
                    break; // 普通字符串不能跨行
                }
                state.advance(ch.len_utf8());
            }

            if !found_end {
                state.add_error(source.syntax_error("Unterminated string literal", start_pos));
            }

            state.add_token(GraphQLSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理块字符串
    fn lex_block_string(&self, state: &mut State, source: &SourceText, start_pos: usize) {
        let mut found_end = false;

        while let Some(ch) = state.peek() {
            if ch == '"' {
                if let Some('"') = source.get_char_at(state.get_position() + 1) {
                    if let Some('"') = source.get_char_at(state.get_position() + 2) {
                        state.advance(3); // 跳过结束的三个引                        found_end = true;
                        break;
                    }
                }
            }
            state.advance(ch.len_utf8());
        }

        if !found_end {
            state.add_error(source.syntax_error("Unterminated block string literal", start_pos));
        }

        state.add_token(GraphQLSyntaxKind::StringLiteral, start_pos, state.get_position());
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let next_ch = source.get_char_at(state.get_position() + 1);
            let third_ch = source.get_char_at(state.get_position() + 2);

            let (token_kind, advance_count) = match (ch, next_ch, third_ch) {
                // 三字符操作符
                ('.', Some('.'), Some('.')) => (GraphQLSyntaxKind::Spread, 3),

                // 单字符操作符和分隔符
                ('(', _, _) => (GraphQLSyntaxKind::LeftParen, 1),
                (')', _, _) => (GraphQLSyntaxKind::RightParen, 1),
                ('[', _, _) => (GraphQLSyntaxKind::LeftBracket, 1),
                (']', _, _) => (GraphQLSyntaxKind::RightBracket, 1),
                ('{', _, _) => (GraphQLSyntaxKind::LeftBrace, 1),
                ('}', _, _) => (GraphQLSyntaxKind::RightBrace, 1),
                (',', _, _) => (GraphQLSyntaxKind::Comma, 1),
                (':', _, _) => (GraphQLSyntaxKind::Colon, 1),
                (';', _, _) => (GraphQLSyntaxKind::Semicolon, 1),
                ('|', _, _) => (GraphQLSyntaxKind::Pipe, 1),
                ('&', _, _) => (GraphQLSyntaxKind::Ampersand, 1),
                ('=', _, _) => (GraphQLSyntaxKind::Equals, 1),
                ('!', _, _) => (GraphQLSyntaxKind::Exclamation, 1),
                ('@', _, _) => (GraphQLSyntaxKind::At, 1),
                ('$', _, _) => (GraphQLSyntaxKind::Dollar, 1),

                _ => return false,
            };

            state.advance(advance_count);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<GraphQLLanguage> for GraphQLLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<GraphQLSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(&mut state) {
                continue;
            }

            // 处理换行
            if self.lex_newline(&mut state) {
                continue;
            }

            // 处理注释
            if self.lex_comment(&mut state, source) {
                continue;
            }

            // 处理名称（标识符和关键字
            if self.lex_name(&mut state, source) {
                continue;
            }

            // 处理数字字面
            if self.lex_number(&mut state, source) {
                continue;
            }

            // 处理字符串字面量
            if self.lex_string(&mut state, source) {
                continue;
            }

            // 处理操作符和分隔
            if self.lex_operator_or_delimiter(&mut state, source) {
                continue;
            }

            // 未知字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.add_error(source.unexpected_character(ch, state.get_position()));
                state.advance(ch.len_utf8());
                state.add_token(GraphQLSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let pos = state.get_position();
        state.add_token(GraphQLSyntaxKind::Eof, pos, pos);

        state.finish()
    }
}
