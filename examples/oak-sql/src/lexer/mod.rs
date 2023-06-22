use crate::{kind::SqlSyntaxKind, language::SqlLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, SqlLanguage>;

static SQL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone, Debug)]
pub struct SqlLexer<'config> {
    _config: &'config SqlLanguage,
}

impl<'config> Lexer<SqlLanguage> for SqlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<SqlLanguage>) -> LexOutput<SqlLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SqlLexer<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '-' => {
                        if state.starts_with("--") {
                            self.skip_comment(state);
                        }
                        else {
                            self.lex_operators(state);
                        }
                    }
                    '/' => {
                        if state.starts_with("/*") {
                            self.skip_comment(state);
                        }
                        else {
                            self.lex_operators(state);
                        }
                    }
                    '\'' | '"' => {
                        self.lex_string_literal(state);
                    }
                    '0'..='9' => {
                        self.lex_number_literal(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier_or_keyword(state);
                    }
                    '<' | '>' | '!' | '=' | '+' | '*' | '%' => {
                        self.lex_operators(state);
                    }
                    '(' | ')' | ',' | ';' | '.' => {
                        self.lex_single_char_tokens(state);
                    }
                    _ => {
                        // 如果没有匹配任何模式，跳过当前字符并添加错误 token
                        state.advance(ch.len_utf8());
                        state.add_token(SqlSyntaxKind::Error, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SqlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SqlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SQL_WHITESPACE.scan(state, SqlSyntaxKind::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 行注释: -- ... 直到换行
        if state.starts_with("--") {
            state.advance(2);
            state.take_while(|ch| ch != '\n' && ch != '\r');
            state.add_token(SqlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if state.starts_with("/*") {
            state.advance(2);
            while state.not_at_end() {
                if state.starts_with("*/") {
                    state.advance(2);
                    break;
                }
                if let Some(ch) = state.current() {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(SqlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(quote) = state.current() {
            if quote != '\'' && quote != '"' {
                return false;
            }
            state.advance(1);
            let mut escaped = false;
            while state.not_at_end() {
                let ch = match state.peek() {
                    Some(c) => c,
                    None => break,
                };

                if ch == quote && !escaped {
                    state.advance(1); // 消费结束引号
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(SqlSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;
        state.advance(1);

        // 整数部分
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 小数部分
        if state.peek() == Some('.') {
            let next = state.peek_next_n(1);
            if next.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // 消费 '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // 指数部分
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let next = state.peek_next_n(1);
                if next == Some('+') || next == Some('-') || next.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_float = true;
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { SqlSyntaxKind::FloatLiteral } else { SqlSyntaxKind::NumberLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !ch.is_alphabetic() && ch != '_' {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_alphanumeric() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.source().get_text_in(oak_core::Range { start, end }).to_uppercase();
        let kind = match text.as_str() {
            "SELECT" => SqlSyntaxKind::Select,
            "FROM" => SqlSyntaxKind::From,
            "WHERE" => SqlSyntaxKind::Where,
            "INSERT" => SqlSyntaxKind::Insert,
            "UPDATE" => SqlSyntaxKind::Update,
            "DELETE" => SqlSyntaxKind::Delete,
            "CREATE" => SqlSyntaxKind::Create,
            "DROP" => SqlSyntaxKind::Drop,
            "ALTER" => SqlSyntaxKind::Alter,
            "TABLE" => SqlSyntaxKind::Table,
            "INDEX" => SqlSyntaxKind::Index,
            "INTO" => SqlSyntaxKind::Into,
            "VALUES" => SqlSyntaxKind::Values,
            "SET" => SqlSyntaxKind::Set,
            "JOIN" => SqlSyntaxKind::Join,
            "INNER" => SqlSyntaxKind::Inner,
            "LEFT" => SqlSyntaxKind::Left,
            "RIGHT" => SqlSyntaxKind::Right,
            "FULL" => SqlSyntaxKind::Full,
            "OUTER" => SqlSyntaxKind::Outer,
            "ON" => SqlSyntaxKind::On,
            "AND" => SqlSyntaxKind::And,
            "OR" => SqlSyntaxKind::Or,
            "NOT" => SqlSyntaxKind::Not,
            "NULL" => SqlSyntaxKind::Null,
            "TRUE" => SqlSyntaxKind::True,
            "FALSE" => SqlSyntaxKind::False,
            "AS" => SqlSyntaxKind::As,
            "BY" => SqlSyntaxKind::By,
            "ORDER" => SqlSyntaxKind::Order,
            "ASC" => SqlSyntaxKind::Asc,
            "DESC" => SqlSyntaxKind::Desc,
            "GROUP" => SqlSyntaxKind::Group,
            "HAVING" => SqlSyntaxKind::Having,
            "LIMIT" => SqlSyntaxKind::Limit,
            "OFFSET" => SqlSyntaxKind::Offset,
            "UNION" => SqlSyntaxKind::Union,
            "ALL" => SqlSyntaxKind::All,
            "DISTINCT" => SqlSyntaxKind::Distinct,
            "PRIMARY" => SqlSyntaxKind::Primary,
            "KEY" => SqlSyntaxKind::Key,
            "FOREIGN" => SqlSyntaxKind::Foreign,
            "REFERENCES" => SqlSyntaxKind::References,
            "DEFAULT" => SqlSyntaxKind::Default,
            "UNIQUE" => SqlSyntaxKind::Unique,
            "AUTO_INCREMENT" => SqlSyntaxKind::AutoIncrement,
            "INT" => SqlSyntaxKind::Int,
            "INTEGER" => SqlSyntaxKind::Integer,
            "VARCHAR" => SqlSyntaxKind::Varchar,
            "CHAR" => SqlSyntaxKind::Char,
            "TEXT" => SqlSyntaxKind::Text,
            "DATE" => SqlSyntaxKind::Date,
            "TIME" => SqlSyntaxKind::Time,
            "TIMESTAMP" => SqlSyntaxKind::Timestamp,
            "DECIMAL" => SqlSyntaxKind::Decimal,
            "FLOAT" => SqlSyntaxKind::Float,
            "DOUBLE" => SqlSyntaxKind::Double,
            "BOOLEAN" => SqlSyntaxKind::Boolean,
            _ => SqlSyntaxKind::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        let ops = [
            ("<=", SqlSyntaxKind::LessEqual),
            (">=", SqlSyntaxKind::GreaterEqual),
            ("<>", SqlSyntaxKind::NotEqual),
            ("!=", SqlSyntaxKind::NotEqual),
            ("=", SqlSyntaxKind::Equal),
            ("<", SqlSyntaxKind::Less),
            (">", SqlSyntaxKind::Greater),
            ("+", SqlSyntaxKind::Plus),
            ("-", SqlSyntaxKind::Minus),
            ("*", SqlSyntaxKind::Star),
            ("/", SqlSyntaxKind::Slash),
            ("%", SqlSyntaxKind::Percent),
        ];

        for (op, kind) in ops {
            if state.starts_with(op) {
                state.advance(op.len());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        let kind = match ch {
            '(' => SqlSyntaxKind::LeftParen,
            ')' => SqlSyntaxKind::RightParen,
            ',' => SqlSyntaxKind::Comma,
            ';' => SqlSyntaxKind::Semicolon,
            '.' => SqlSyntaxKind::Dot,
            _ => return false,
        };

        state.advance(ch.len_utf8());
        state.add_token(kind, start, state.get_position());
        true
    }
}
