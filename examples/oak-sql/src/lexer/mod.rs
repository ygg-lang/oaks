use crate::{language::SqlLanguage, syntax::SqlSyntaxKind};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, SqlLanguage>;

pub struct SqlLexer<'config> {
    config: &'config SqlLanguage,
}

impl<'config> SqlLexer<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
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
            state.add_token(SqlSyntaxKind::Whitespace, start_pos, state.get_position());
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

    /// 处理行注
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SqlSyntaxKind::LineComment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理块注
    fn lex_block_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SqlSyntaxKind::BlockComment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十进制数
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理科学计数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(SqlSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\'' {
                        state.advance(1);
                        // 处理双单引号转义
                        if let Some('\'') = state.peek() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SqlSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理标识符或关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
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

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = self.keyword_or_identifier(text);
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

    /// 处理带引号的标识
    fn lex_quoted_identifier(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            let is_valid_quote = match quote {
                '"' => self.config.quoted_identifiers,
                '`' => self.config.backtick_identifiers,
                '[' => self.config.bracket_identifiers,
                _ => false,
            };

            if is_valid_quote {
                let end_quote = if quote == '[' { ']' } else { quote };
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == end_quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 标识符不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SqlSyntaxKind::Identifier_, start_pos, state.get_position());
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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> SqlSyntaxKind {
        let text = if self.config.case_sensitive {
            text
        }
        else {
            // 对于不区分大小写的情况，转换为小写进行比
            &text.to_lowercase()
        };

        match text {
            "select" => SqlSyntaxKind::Select,
            "from" => SqlSyntaxKind::From,
            "where" => SqlSyntaxKind::Where,
            "insert" => SqlSyntaxKind::Insert,
            "into" => SqlSyntaxKind::Into,
            "values" => SqlSyntaxKind::Values,
            "update" => SqlSyntaxKind::Update,
            "set" => SqlSyntaxKind::Set,
            "delete" => SqlSyntaxKind::Delete,
            "create" => SqlSyntaxKind::Create,
            "table" => SqlSyntaxKind::Table,
            "drop" => SqlSyntaxKind::Drop,
            "alter" => SqlSyntaxKind::Alter,
            "add" => SqlSyntaxKind::Add,
            "column" => SqlSyntaxKind::Column,
            "primary" => SqlSyntaxKind::Primary,
            "key" => SqlSyntaxKind::Key,
            "foreign" => SqlSyntaxKind::Foreign,
            "references" => SqlSyntaxKind::References,
            "index" => SqlSyntaxKind::Index,
            "unique" => SqlSyntaxKind::Unique,
            "not" => SqlSyntaxKind::Not,
            "null" => SqlSyntaxKind::Null,
            "default" => SqlSyntaxKind::Default,
            "auto_increment" => SqlSyntaxKind::Auto_Increment,
            "and" => SqlSyntaxKind::And,
            "or" => SqlSyntaxKind::Or,
            "in" => SqlSyntaxKind::In,
            "like" => SqlSyntaxKind::Like,
            "between" => SqlSyntaxKind::Between,
            "is" => SqlSyntaxKind::Is,
            "as" => SqlSyntaxKind::As,
            "join" => SqlSyntaxKind::Join,
            "inner" => SqlSyntaxKind::Inner,
            "left" => SqlSyntaxKind::Left,
            "right" => SqlSyntaxKind::Right,
            "full" => SqlSyntaxKind::Full,
            "outer" => SqlSyntaxKind::Outer,
            "on" => SqlSyntaxKind::On,
            "group" => SqlSyntaxKind::Group,
            "by" => SqlSyntaxKind::By,
            "having" => SqlSyntaxKind::Having,
            "order" => SqlSyntaxKind::Order,
            "asc" => SqlSyntaxKind::Asc,
            "desc" => SqlSyntaxKind::Desc,
            "limit" => SqlSyntaxKind::Limit,
            "offset" => SqlSyntaxKind::Offset,
            "union" => SqlSyntaxKind::Union,
            "all" => SqlSyntaxKind::All,
            "distinct" => SqlSyntaxKind::Distinct,
            "count" => SqlSyntaxKind::Count,
            "sum" => SqlSyntaxKind::Sum,
            "avg" => SqlSyntaxKind::Avg,
            "min" => SqlSyntaxKind::Min,
            "max" => SqlSyntaxKind::Max,
            "int" => SqlSyntaxKind::Int,
            "integer" => SqlSyntaxKind::Integer,
            "varchar" => SqlSyntaxKind::Varchar,
            "char" => SqlSyntaxKind::Char,
            "text" => SqlSyntaxKind::Text,
            "date" => SqlSyntaxKind::Date,
            "time" => SqlSyntaxKind::Time,
            "timestamp" => SqlSyntaxKind::Timestamp,
            "decimal" => SqlSyntaxKind::Decimal,
            "float" => SqlSyntaxKind::Float,
            "double" => SqlSyntaxKind::Double,
            "boolean" => SqlSyntaxKind::Boolean,
            "true" | "false" => SqlSyntaxKind::BooleanLiteral,
            _ => SqlSyntaxKind::Identifier_,
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    SqlSyntaxKind::Plus
                }
                '-' => {
                    // 这里不处理注释，因为已经在其他地方处理了
                    state.advance(1);
                    SqlSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    SqlSyntaxKind::Star
                }
                '/' => {
                    // 这里不处理注释，因为已经在其他地方处理了
                    state.advance(1);
                    SqlSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    SqlSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    SqlSyntaxKind::Equal
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SqlSyntaxKind::NotEqual
                    }
                    else {
                        return false; // 单独! 不是有效SQL 操作
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SqlSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        SqlSyntaxKind::NotEqual
                    }
                    else {
                        SqlSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SqlSyntaxKind::GreaterEqual
                    }
                    else {
                        SqlSyntaxKind::Greater
                    }
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SqlSyntaxKind::LeftParen,
                ')' => SqlSyntaxKind::RightParen,
                ',' => SqlSyntaxKind::Comma,
                ';' => SqlSyntaxKind::Semicolon,
                '.' => SqlSyntaxKind::Dot,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<SqlLanguage> for SqlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<SqlSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_quoted_identifier(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SqlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(SqlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
