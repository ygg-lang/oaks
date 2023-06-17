use crate::{kind::DartSyntaxKind, language::DartLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

pub struct DartLexer<'config> {
    config: &'config DartLanguage,
}

impl<'config> DartLexer<'config> {
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut LexerState<DartLanguage>) -> bool {
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
            state.add_token(DartSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(DartSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(DartSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut LexerState<DartLanguage>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in((start_pos..end_pos).into()).unwrap_or("");
                let token_kind = match text {
                    "abstract" => DartSyntaxKind::Abstract,
                    "as" => DartSyntaxKind::As,
                    "assert" => DartSyntaxKind::Assert,
                    "async" => DartSyntaxKind::Async,
                    "await" => DartSyntaxKind::Await,
                    "break" => DartSyntaxKind::Break,
                    "case" => DartSyntaxKind::Case,
                    "catch" => DartSyntaxKind::Catch,
                    "class" => DartSyntaxKind::Class,
                    "const" => DartSyntaxKind::Const,
                    "continue" => DartSyntaxKind::Continue,
                    "covariant" => DartSyntaxKind::Covariant,
                    "default" => DartSyntaxKind::Default,
                    "deferred" => DartSyntaxKind::Deferred,
                    "do" => DartSyntaxKind::Do,
                    "dynamic" => DartSyntaxKind::Dynamic,
                    "else" => DartSyntaxKind::Else,
                    "enum" => DartSyntaxKind::Enum,
                    "export" => DartSyntaxKind::Export,
                    "extends" => DartSyntaxKind::Extends,
                    "extension" => DartSyntaxKind::Extension,
                    "external" => DartSyntaxKind::External,
                    "factory" => DartSyntaxKind::Factory,
                    "false" => DartSyntaxKind::False,
                    "final" => DartSyntaxKind::Final,
                    "finally" => DartSyntaxKind::Finally,
                    "for" => DartSyntaxKind::For,
                    "function" => DartSyntaxKind::Function,
                    "get" => DartSyntaxKind::Get,
                    "hide" => DartSyntaxKind::Hide,
                    "if" => DartSyntaxKind::If,
                    "implements" => DartSyntaxKind::Implements,
                    "import" => DartSyntaxKind::Import,
                    "in" => DartSyntaxKind::In,
                    "interface" => DartSyntaxKind::Interface,
                    "is" => DartSyntaxKind::Is,
                    "late" => DartSyntaxKind::Late,
                    "library" => DartSyntaxKind::Library,
                    "mixin" => DartSyntaxKind::Mixin,
                    "new" => DartSyntaxKind::New,
                    "null" => DartSyntaxKind::Null,
                    "on" => DartSyntaxKind::On,
                    "operator" => DartSyntaxKind::Operator,
                    "part" => DartSyntaxKind::Part,
                    "required" => DartSyntaxKind::Required,
                    "rethrow" => DartSyntaxKind::Rethrow,
                    "return" => DartSyntaxKind::Return,
                    "set" => DartSyntaxKind::Set,
                    "show" => DartSyntaxKind::Show,
                    "static" => DartSyntaxKind::Static,
                    "super" => DartSyntaxKind::Super,
                    "switch" => DartSyntaxKind::Switch,
                    "sync" => DartSyntaxKind::Sync,
                    "this" => DartSyntaxKind::This,
                    "throw" => DartSyntaxKind::Throw,
                    "true" => DartSyntaxKind::True,
                    "try" => DartSyntaxKind::Try,
                    "typedef" => DartSyntaxKind::Typedef,
                    "var" => DartSyntaxKind::Var,
                    "void" => DartSyntaxKind::Void,
                    "while" => DartSyntaxKind::While,
                    "with" => DartSyntaxKind::With,
                    "yield" => DartSyntaxKind::Yield,
                    _ => DartSyntaxKind::Identifier,
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
    fn lex_number(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_double = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    // 确保不是范围操作..
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过小数                            is_double = true;

                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        is_double = true;

                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
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
                }

                let token_kind = if is_double { DartSyntaxKind::DoubleLiteral } else { DartSyntaxKind::IntegerLiteral };

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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(DartSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符
                state.add_token(DartSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理行注
    fn lex_line_comment(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(DartSyntaxKind::LineComment, start_pos, state.get_position());
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

    /// 处理块注
    fn lex_block_comment(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = state.peek_next_n(1) {
                            state.advance(2);
                            state.add_token(DartSyntaxKind::BlockComment, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                // 未闭合的块注                state.add_token(DartSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理操作
    fn lex_operator(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    match state.peek() {
                        Some('+') => {
                            state.advance(1);
                            DartSyntaxKind::PlusPlus
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::PlusEqual
                        }
                        _ => DartSyntaxKind::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            DartSyntaxKind::MinusMinus
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::MinusEqual
                        }
                        _ => DartSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DartSyntaxKind::StarEqual
                    }
                    else {
                        DartSyntaxKind::Star
                    }
                }
                '/' => {
                    // 已经在注释处理中处理// /*
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DartSyntaxKind::SlashEqual
                    }
                    else {
                        DartSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DartSyntaxKind::PercentEqual
                    }
                    else {
                        DartSyntaxKind::Percent
                    }
                }
                '~' => {
                    state.advance(1);
                    match state.peek() {
                        Some('/') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                DartSyntaxKind::TildeSlashEqual
                            }
                            else {
                                DartSyntaxKind::TildeSlash
                            }
                        }
                        _ => DartSyntaxKind::Tilde,
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::EqualEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            DartSyntaxKind::Arrow
                        }
                        _ => DartSyntaxKind::Equal,
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DartSyntaxKind::BangEqual
                    }
                    else {
                        DartSyntaxKind::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::LessEqual
                        }
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                DartSyntaxKind::LeftShiftEqual
                            }
                            else {
                                DartSyntaxKind::LeftShift
                            }
                        }
                        _ => DartSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::GreaterEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                DartSyntaxKind::RightShiftEqual
                            }
                            else {
                                DartSyntaxKind::RightShift
                            }
                        }
                        _ => DartSyntaxKind::Greater,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            DartSyntaxKind::AmpersandAmpersand
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::AmpersandEqual
                        }
                        _ => DartSyntaxKind::Ampersand,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            DartSyntaxKind::PipePipe
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::PipeEqual
                        }
                        _ => DartSyntaxKind::Pipe,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DartSyntaxKind::CaretEqual
                    }
                    else {
                        DartSyntaxKind::Caret
                    }
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                DartSyntaxKind::QuestionQuestionEqual
                            }
                            else {
                                DartSyntaxKind::QuestionQuestion
                            }
                        }
                        Some('.') => {
                            state.advance(1);
                            DartSyntaxKind::QuestionDot
                        }
                        _ => DartSyntaxKind::Question,
                    }
                }
                '.' => {
                    state.advance(1);
                    match state.peek() {
                        Some('.') => {
                            state.advance(1);
                            if let Some('.') = state.peek() {
                                state.advance(1);
                                DartSyntaxKind::DotDotDot
                            }
                            else {
                                DartSyntaxKind::DotDot
                            }
                        }
                        _ => DartSyntaxKind::Dot,
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
    fn lex_delimiter(&self, state: &mut LexerState<DartLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => DartSyntaxKind::LeftParen,
                ')' => DartSyntaxKind::RightParen,
                '[' => DartSyntaxKind::LeftBracket,
                ']' => DartSyntaxKind::RightBracket,
                '{' => DartSyntaxKind::LeftBrace,
                '}' => DartSyntaxKind::RightBrace,
                ';' => DartSyntaxKind::Semicolon,
                ',' => DartSyntaxKind::Comma,
                ':' => DartSyntaxKind::Colon,
                '@' => DartSyntaxKind::At,
                '#' => DartSyntaxKind::Hash,
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

impl<'config> Lexer<DartLanguage> for DartLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<DartSyntaxKind> {
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

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
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
                state.add_token(DartSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DartSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
