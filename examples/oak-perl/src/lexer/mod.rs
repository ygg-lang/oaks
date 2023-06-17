use crate::{kind::PerlSyntaxKind, language::PerlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PerlLanguage>;

pub struct PerlLexer<'config> {
    config: &'config PerlLanguage,
}

impl<'config> PerlLexer<'config> {
    pub fn new(config: &'config PerlLanguage) -> Self {
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
            state.add_token(PerlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(PerlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PerlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(PerlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨行（除非转义
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    state.add_token(PerlSyntaxKind::StringLiteral, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = state.source.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1);

                            // 小数部分
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
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
                if let Some(e_char) = state.peek() {
                    if e_char == 'e' || e_char == 'E' {
                        let saved_pos = state.get_position();
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数部分
                        let exp_start = state.get_position();
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        if state.get_position() == exp_start {
                            // 没有有效的指数，回退
                            state.set_position(saved_pos);
                        }
                    }
                }

                state.add_token(PerlSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理变量前缀
    fn lex_variable_prefix(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '$' => PerlSyntaxKind::Scalar,
                '@' => PerlSyntaxKind::Array,
                '%' => PerlSyntaxKind::Hash,
                '&' => PerlSyntaxKind::Code,
                '*' => PerlSyntaxKind::Glob,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State) -> bool {
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

                let text = state.source.get_text(start_pos, state.get_position());
                let token_kind = match text {
                    "package" => PerlSyntaxKind::Package,
                    "use" => PerlSyntaxKind::Use,
                    "sub" => PerlSyntaxKind::Sub,
                    "my" => PerlSyntaxKind::My,
                    "our" => PerlSyntaxKind::Our,
                    "local" => PerlSyntaxKind::Local,
                    "if" => PerlSyntaxKind::If,
                    "elsif" => PerlSyntaxKind::Elsif,
                    "else" => PerlSyntaxKind::Else,
                    "unless" => PerlSyntaxKind::Unless,
                    "while" => PerlSyntaxKind::While,
                    "until" => PerlSyntaxKind::Until,
                    "for" => PerlSyntaxKind::For,
                    "foreach" => PerlSyntaxKind::Foreach,
                    "do" => PerlSyntaxKind::Do,
                    "last" => PerlSyntaxKind::Last,
                    "next" => PerlSyntaxKind::Next,
                    "redo" => PerlSyntaxKind::Redo,
                    "return" => PerlSyntaxKind::Return,
                    "die" => PerlSyntaxKind::Die,
                    "warn" => PerlSyntaxKind::Warn,
                    "print" => PerlSyntaxKind::Print,
                    "printf" => PerlSyntaxKind::Printf,
                    "chomp" => PerlSyntaxKind::Chomp,
                    "chop" => PerlSyntaxKind::Chop,
                    "length" => PerlSyntaxKind::Length,
                    "substr" => PerlSyntaxKind::Substr,
                    "index" => PerlSyntaxKind::Index,
                    "rindex" => PerlSyntaxKind::Rindex,
                    "split" => PerlSyntaxKind::Split,
                    "join" => PerlSyntaxKind::Join,
                    "push" => PerlSyntaxKind::Push,
                    "pop" => PerlSyntaxKind::Pop,
                    "shift" => PerlSyntaxKind::Shift,
                    "unshift" => PerlSyntaxKind::Unshift,
                    "sort" => PerlSyntaxKind::Sort,
                    "reverse" => PerlSyntaxKind::Reverse,
                    "keys" => PerlSyntaxKind::Keys,
                    "values" => PerlSyntaxKind::Values,
                    "each" => PerlSyntaxKind::Each,
                    "exists" => PerlSyntaxKind::Exists,
                    "delete" => PerlSyntaxKind::Delete,
                    "defined" => PerlSyntaxKind::Defined,
                    "undef" => PerlSyntaxKind::Undef,
                    "ref" => PerlSyntaxKind::Ref,
                    "bless" => PerlSyntaxKind::Bless,
                    "new" => PerlSyntaxKind::New,
                    "can" => PerlSyntaxKind::Can,
                    "isa" => PerlSyntaxKind::Isa,
                    _ => PerlSyntaxKind::Identifier,
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

    /// 处理运算
    fn lex_operators(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Increment
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::PlusAssign
                    }
                    else {
                        PerlSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Decrement
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Arrow
                    }
                    else {
                        PerlSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PerlSyntaxKind::PowerAssign
                        }
                        else {
                            PerlSyntaxKind::Power
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::MultiplyAssign
                    }
                    else {
                        PerlSyntaxKind::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::DivideAssign
                    }
                    else {
                        PerlSyntaxKind::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::ModuloAssign
                    }
                    else {
                        PerlSyntaxKind::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Equal
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::FatComma
                    }
                    else {
                        PerlSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::NotEqual
                    }
                    else {
                        PerlSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            PerlSyntaxKind::Spaceship
                        }
                        else {
                            PerlSyntaxKind::LessEqual
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PerlSyntaxKind::LeftShiftAssign
                        }
                        else {
                            PerlSyntaxKind::LeftShift
                        }
                    }
                    else {
                        PerlSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PerlSyntaxKind::RightShiftAssign
                        }
                        else {
                            PerlSyntaxKind::RightShift
                        }
                    }
                    else {
                        PerlSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::BitwiseAndAssign
                    }
                    else {
                        PerlSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::BitwiseOrAssign
                    }
                    else {
                        PerlSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::BitwiseXorAssign
                    }
                    else {
                        PerlSyntaxKind::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    PerlSyntaxKind::BitwiseNot
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Range
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::ConcatAssign
                    }
                    else {
                        PerlSyntaxKind::Concat
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
    fn lex_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => PerlSyntaxKind::LeftParen,
                ')' => PerlSyntaxKind::RightParen,
                '[' => PerlSyntaxKind::LeftBracket,
                ']' => PerlSyntaxKind::RightBracket,
                '{' => PerlSyntaxKind::LeftBrace,
                '}' => PerlSyntaxKind::RightBrace,
                ';' => PerlSyntaxKind::Semicolon,
                ',' => PerlSyntaxKind::Comma,
                '?' => PerlSyntaxKind::Question,
                ':' => PerlSyntaxKind::Colon,
                '\\' => PerlSyntaxKind::Backslash,
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

impl<'config> Lexer<PerlLanguage> for PerlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PerlSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_variable_prefix(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state, source) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PerlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(PerlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
