use crate::{kind::PascalSyntaxKind, language::PascalLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, PascalLanguage>;

static PASCAL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static PASCAL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "{", block_end: "}", nested_blocks: false });

#[derive(Clone, Debug)]
pub struct PascalLexer<'config> {
    _config: &'config PascalLanguage,
}

impl<'config> PascalLexer<'config> {
    pub fn new(config: &'config PascalLanguage) -> Self {
        Self { _config: config }
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        PASCAL_WHITESPACE.scan(state, PascalSyntaxKind::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // Line comment starting with //
        if state.rest().starts_with("//") {
            return PASCAL_COMMENT.scan(state, PascalSyntaxKind::Comment, PascalSyntaxKind::Comment);
        }

        // Block comment: { ... }
        if state.current() == Some('{') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '}' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        // Block comment: (* ... *)
        if state.rest().starts_with("(*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // Pascal 字符串字面量：'...'
        if state.current() == Some('\'') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    // 检查是否是转义的单引号 ''
                    if state.peek_next_n(1) == Some('\'') {
                        state.advance(2); // 跳过 ''
                        continue;
                    }
                    else {
                        state.advance(1); // 结束引号
                        break;
                    }
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.to_lowercase().as_str() {
                    "program" => PascalSyntaxKind::Program,
                    "var" => PascalSyntaxKind::Var,
                    "const" => PascalSyntaxKind::Const,
                    "type" => PascalSyntaxKind::Type,
                    "procedure" => PascalSyntaxKind::Procedure,
                    "function" => PascalSyntaxKind::Function,
                    "begin" => PascalSyntaxKind::Begin,
                    "end" => PascalSyntaxKind::End,
                    "if" => PascalSyntaxKind::If,
                    "then" => PascalSyntaxKind::Then,
                    "else" => PascalSyntaxKind::Else,
                    "while" => PascalSyntaxKind::While,
                    "do" => PascalSyntaxKind::Do,
                    "for" => PascalSyntaxKind::For,
                    "to" => PascalSyntaxKind::To,
                    "downto" => PascalSyntaxKind::Downto,
                    "repeat" => PascalSyntaxKind::Repeat,
                    "until" => PascalSyntaxKind::Until,
                    "case" => PascalSyntaxKind::Case,
                    "of" => PascalSyntaxKind::Of,
                    "with" => PascalSyntaxKind::With,
                    "record" => PascalSyntaxKind::Record,
                    "array" => PascalSyntaxKind::Array,
                    "set" => PascalSyntaxKind::Set,
                    "file" => PascalSyntaxKind::File,
                    "packed" => PascalSyntaxKind::Packed,
                    "nil" => PascalSyntaxKind::Nil,
                    "true" => PascalSyntaxKind::True,
                    "false" => PascalSyntaxKind::False,
                    "and" => PascalSyntaxKind::And,
                    "or" => PascalSyntaxKind::Or,
                    "not" => PascalSyntaxKind::Not,
                    "div" => PascalSyntaxKind::Div,
                    "mod" => PascalSyntaxKind::Mod,
                    "in" => PascalSyntaxKind::In,

                    _ => PascalSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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

    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dot = false;

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dot { PascalSyntaxKind::RealLiteral } else { PascalSyntaxKind::IntegerLiteral };

                state.add_token(kind, start_pos, state.get_position());
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

    fn lex_operators_and_punctuation<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    PascalSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    PascalSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    PascalSyntaxKind::Multiply
                }
                '/' => {
                    state.advance(1);
                    PascalSyntaxKind::Divide
                }
                '=' => {
                    state.advance(1);
                    PascalSyntaxKind::Equal
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::NotEqual
                    }
                    else {
                        PascalSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::GreaterEqual
                    }
                    else {
                        PascalSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::Assign
                    }
                    else {
                        PascalSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PascalSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PascalSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::Range
                    }
                    else {
                        PascalSyntaxKind::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PascalSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PascalSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    PascalSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PascalSyntaxKind::RightBracket
                }
                '^' => {
                    state.advance(1);
                    PascalSyntaxKind::Caret
                }
                '\n' => {
                    state.advance(1);
                    PascalSyntaxKind::Newline
                }
                _ => {
                    state.advance(ch.len_utf8());
                    PascalSyntaxKind::Error
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl Lexer<PascalLanguage> for PascalLexer<'_> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<PascalLanguage>) -> LexOutput<PascalLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl PascalLexer<'_> {
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let safe_point = state.get_position();
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串
            if self.lex_string(state) {
                continue;
            }

            // 处理标识符和关键字
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // 处理数字
            if self.lex_number(state) {
                continue;
            }

            // 处理操作符和标点符号
            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何模式，创建错误 token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PascalSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        Ok(())
    }
}
