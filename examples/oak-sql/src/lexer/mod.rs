use crate::{kind::SqlSyntaxKind, language::SqlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, SqlLanguage>;

static SQL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SQL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["--"] });
static SQL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[derive(Clone)]
pub struct SqlLexer<'config> {
    config: &'config SqlLanguage,
}

impl<'config> Lexer<SqlLanguage> for SqlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<SqlLanguage>,
    ) -> LexOutput<SqlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> SqlLexer<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
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

            // 如果没有匹配任何模式，跳过当前字符并添加错误 token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SqlSyntaxKind::Error, safe_point, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(SqlSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match SQL_WHITESPACE.scan(state.rest(), state.get_position(), SqlSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: -- ... 直到换行
        if rest.starts_with("--") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(SqlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(SqlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if ch == '\'' || ch == '"' {
            let quote = ch;
            state.advance(1);
            let mut escaped = false;

            while let Some(ch) = state.peek() {
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

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = self.keyword_kind(&text).unwrap_or(SqlSyntaxKind::Identifier);
        state.add_token(kind, start, end);
        true
    }

    fn keyword_kind(&self, text: &str) -> Option<SqlSyntaxKind> {
        match text.to_uppercase().as_str() {
            "SELECT" => Some(SqlSyntaxKind::Select),
            "FROM" => Some(SqlSyntaxKind::From),
            "WHERE" => Some(SqlSyntaxKind::Where),
            "INSERT" => Some(SqlSyntaxKind::Insert),
            "INTO" => Some(SqlSyntaxKind::Into),
            "VALUES" => Some(SqlSyntaxKind::Values),
            "UPDATE" => Some(SqlSyntaxKind::Update),
            "SET" => Some(SqlSyntaxKind::Set),
            "DELETE" => Some(SqlSyntaxKind::Delete),
            "CREATE" => Some(SqlSyntaxKind::Create),
            "DROP" => Some(SqlSyntaxKind::Drop),
            "ALTER" => Some(SqlSyntaxKind::Alter),
            "ADD" => Some(SqlSyntaxKind::Add),
            "COLUMN" => Some(SqlSyntaxKind::Column),
            "TABLE" => Some(SqlSyntaxKind::Table),
            "PRIMARY" => Some(SqlSyntaxKind::Primary),
            "KEY" => Some(SqlSyntaxKind::Key),
            "FOREIGN" => Some(SqlSyntaxKind::Foreign),
            "REFERENCES" => Some(SqlSyntaxKind::References),
            "INDEX" => Some(SqlSyntaxKind::Index),
            "UNIQUE" => Some(SqlSyntaxKind::Unique),
            "NOT" => Some(SqlSyntaxKind::Not),
            "NULL" => Some(SqlSyntaxKind::Null),
            "DEFAULT" => Some(SqlSyntaxKind::Default),
            "AUTO_INCREMENT" => Some(SqlSyntaxKind::AutoIncrement),
            "AND" => Some(SqlSyntaxKind::And),
            "OR" => Some(SqlSyntaxKind::Or),
            "IN" => Some(SqlSyntaxKind::In),
            "LIKE" => Some(SqlSyntaxKind::Like),
            "BETWEEN" => Some(SqlSyntaxKind::Between),
            "IS" => Some(SqlSyntaxKind::Is),
            "AS" => Some(SqlSyntaxKind::As),
            "JOIN" => Some(SqlSyntaxKind::Join),
            "INNER" => Some(SqlSyntaxKind::Inner),
            "LEFT" => Some(SqlSyntaxKind::Left),
            "RIGHT" => Some(SqlSyntaxKind::Right),
            "FULL" => Some(SqlSyntaxKind::Full),
            "OUTER" => Some(SqlSyntaxKind::Outer),
            "ON" => Some(SqlSyntaxKind::On),
            "GROUP" => Some(SqlSyntaxKind::Group),
            "BY" => Some(SqlSyntaxKind::By),
            "HAVING" => Some(SqlSyntaxKind::Having),
            "ORDER" => Some(SqlSyntaxKind::Order),
            "ASC" => Some(SqlSyntaxKind::Asc),
            "DESC" => Some(SqlSyntaxKind::Desc),
            "LIMIT" => Some(SqlSyntaxKind::Limit),
            "OFFSET" => Some(SqlSyntaxKind::Offset),
            "UNION" => Some(SqlSyntaxKind::Union),
            "ALL" => Some(SqlSyntaxKind::All),
            "DISTINCT" => Some(SqlSyntaxKind::Distinct),
            "COUNT" => Some(SqlSyntaxKind::Count),
            "SUM" => Some(SqlSyntaxKind::Sum),
            "AVG" => Some(SqlSyntaxKind::Avg),
            "MIN" => Some(SqlSyntaxKind::Min),
            "MAX" => Some(SqlSyntaxKind::Max),
            "VIEW" => Some(SqlSyntaxKind::View),
            "DATABASE" => Some(SqlSyntaxKind::Database),
            "SCHEMA" => Some(SqlSyntaxKind::Schema),
            "TRUE" => Some(SqlSyntaxKind::True),
            "FALSE" => Some(SqlSyntaxKind::False),
            "EXISTS" => Some(SqlSyntaxKind::Exists),
            "CASE" => Some(SqlSyntaxKind::Case),
            "WHEN" => Some(SqlSyntaxKind::When),
            "THEN" => Some(SqlSyntaxKind::Then),
            "ELSE" => Some(SqlSyntaxKind::Else),
            "END" => Some(SqlSyntaxKind::End),
            "IF" => Some(SqlSyntaxKind::If),
            "BEGIN" => Some(SqlSyntaxKind::Begin),
            "COMMIT" => Some(SqlSyntaxKind::Commit),
            "ROLLBACK" => Some(SqlSyntaxKind::Rollback),
            "TRANSACTION" => Some(SqlSyntaxKind::Transaction),
            // 数据类型
            "INT" => Some(SqlSyntaxKind::Int),
            "INTEGER" => Some(SqlSyntaxKind::Integer),
            "VARCHAR" => Some(SqlSyntaxKind::Varchar),
            "CHAR" => Some(SqlSyntaxKind::Char),
            "TEXT" => Some(SqlSyntaxKind::Text),
            "DATE" => Some(SqlSyntaxKind::Date),
            "TIME" => Some(SqlSyntaxKind::Time),
            "TIMESTAMP" => Some(SqlSyntaxKind::Timestamp),
            "DECIMAL" => Some(SqlSyntaxKind::Decimal),
            "FLOAT" => Some(SqlSyntaxKind::Float),
            "DOUBLE" => Some(SqlSyntaxKind::Double),
            "BOOLEAN" => Some(SqlSyntaxKind::Boolean),
            _ => None,
        }
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 优先匹配较长的操作符
        let patterns: &[(&str, SqlSyntaxKind)] = &[
            ("<=", SqlSyntaxKind::Le),
            (">=", SqlSyntaxKind::Ge),
            ("!=", SqlSyntaxKind::Ne),
            ("<>", SqlSyntaxKind::Ne),
            ("||", SqlSyntaxKind::Concat),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '=' => Some(SqlSyntaxKind::Equal),
                '<' => Some(SqlSyntaxKind::Lt),
                '>' => Some(SqlSyntaxKind::Gt),
                '+' => Some(SqlSyntaxKind::Plus),
                '-' => Some(SqlSyntaxKind::Minus),
                '*' => Some(SqlSyntaxKind::Star),
                '/' => Some(SqlSyntaxKind::Slash),
                '%' => Some(SqlSyntaxKind::Percent),
                '.' => Some(SqlSyntaxKind::Dot),
                _ => None,
            };
            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => SqlSyntaxKind::LeftParen,
                ')' => SqlSyntaxKind::RightParen,
                '{' => SqlSyntaxKind::LeftBrace,
                '}' => SqlSyntaxKind::RightBrace,
                '[' => SqlSyntaxKind::LeftBracket,
                ']' => SqlSyntaxKind::RightBracket,
                ',' => SqlSyntaxKind::Comma,
                ';' => SqlSyntaxKind::Semicolon,
                ':' => SqlSyntaxKind::Colon,
                '?' => SqlSyntaxKind::Question,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
