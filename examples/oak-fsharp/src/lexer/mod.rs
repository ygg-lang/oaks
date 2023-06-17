use crate::{kind::FSharpSyntaxKind, language::FSharpLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, FSharpLanguage>;

pub struct FSharpLexer<'config> {
    config: &'config FSharpLanguage,
}

impl<'config> FSharpLexer<'config> {
    pub fn new(config: &'config FSharpLanguage) -> Self {
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
            state.add_token(FSharpSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(FSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(FSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理单行注释 //
    fn lex_line_comment(&self, state: &mut State) -> bool {
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                let start_pos = state.get_position();
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(FSharpSyntaxKind::LineComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理块注(* *)
    fn lex_block_comment(&self, state: &mut State) -> bool {
        if let Some('(') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                let start_pos = state.get_position();
                state.advance(2);

                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('(') = state.peek() {
                        if let Some('*') = state.peek_next_n(1) {
                            state.advance(2);
                            depth += 1;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else if let Some('*') = state.peek() {
                        if let Some(')') = state.peek_next_n(1) {
                            state.advance(2);
                            depth -= 1;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(FSharpSyntaxKind::BlockComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let range = start_pos..state.get_position();
                let text = source.get_text_in(range.into()).unwrap_or("");
                let kind = match text {
                    "let" => FSharpSyntaxKind::Let,
                    "rec" => FSharpSyntaxKind::Rec,
                    "and" => FSharpSyntaxKind::And,
                    "in" => FSharpSyntaxKind::In,
                    "if" => FSharpSyntaxKind::If,
                    "then" => FSharpSyntaxKind::Then,
                    "else" => FSharpSyntaxKind::Else,
                    "elif" => FSharpSyntaxKind::Elif,
                    "match" => FSharpSyntaxKind::Match,
                    "with" => FSharpSyntaxKind::With,
                    "when" => FSharpSyntaxKind::When,
                    "function" => FSharpSyntaxKind::Function,
                    "fun" => FSharpSyntaxKind::Fun,
                    "type" => FSharpSyntaxKind::Type,
                    "val" => FSharpSyntaxKind::Val,
                    "mutable" => FSharpSyntaxKind::Mutable,
                    "of" => FSharpSyntaxKind::Of,
                    "as" => FSharpSyntaxKind::As,
                    // "module" => FSharpSyntaxKind::Module,
                    "namespace" => FSharpSyntaxKind::Namespace,
                    "open" => FSharpSyntaxKind::Open,
                    "try" => FSharpSyntaxKind::Try,
                    "finally" => FSharpSyntaxKind::Finally,
                    "exception" => FSharpSyntaxKind::Exception,
                    "raise" => FSharpSyntaxKind::Raise,
                    "failwith" => FSharpSyntaxKind::Failwith,
                    "for" => FSharpSyntaxKind::For,
                    "to" => FSharpSyntaxKind::To,
                    "downto" => FSharpSyntaxKind::Downto,
                    "do" => FSharpSyntaxKind::Do,
                    "done" => FSharpSyntaxKind::Done,
                    "while" => FSharpSyntaxKind::While,
                    "yield" => FSharpSyntaxKind::Yield,
                    "return" => FSharpSyntaxKind::Return,
                    "class" => FSharpSyntaxKind::Class,
                    "interface" => FSharpSyntaxKind::Interface,
                    "inherit" => FSharpSyntaxKind::Inherit,
                    "abstract" => FSharpSyntaxKind::Abstract,
                    "override" => FSharpSyntaxKind::Override,
                    "default" => FSharpSyntaxKind::Default,
                    "member" => FSharpSyntaxKind::Member,
                    "static" => FSharpSyntaxKind::Static,
                    "new" => FSharpSyntaxKind::New,
                    "lazy" => FSharpSyntaxKind::Lazy,
                    "async" => FSharpSyntaxKind::Async,
                    "seq" => FSharpSyntaxKind::Seq,
                    "use" => FSharpSyntaxKind::Use,
                    "begin" => FSharpSyntaxKind::Begin,
                    "end" => FSharpSyntaxKind::End,
                    "struct" => FSharpSyntaxKind::Struct,
                    "sig" => FSharpSyntaxKind::Sig,
                    "not" => FSharpSyntaxKind::Not,
                    "true" | "false" => FSharpSyntaxKind::BooleanLiteral,
                    _ => FSharpSyntaxKind::Identifier,
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

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 读取数字部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = state.peek_next_n(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            is_float = true;

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

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        is_float = true;

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

                // 检查后缀
                if let Some(ch) = state.peek() {
                    if ch == 'f' || ch == 'F' || ch == 'd' || ch == 'D' || ch == 'm' || ch == 'M' {
                        state.advance(1);
                        is_float = true;
                    }
                    else if ch == 'L' || ch == 'l' || ch == 'u' || ch == 'U' || ch == 'y' || ch == 's' {
                        state.advance(1);
                    }
                }

                let kind = if is_float { FSharpSyntaxKind::FloatLiteral } else { FSharpSyntaxKind::IntegerLiteral };

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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨                } else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符            state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::CharLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 无效的字符字面量
            state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理运算
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::Arrow, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Minus, start_pos, state.get_position());
                    }
                    true
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::StarStar, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Star, start_pos, state.get_position());
                    }
                    true
                }
                '/' => {
                    // 检查是否是注释
                    if let Some('/') = state.peek_next_n(1) {
                        false // 让注释处理函数处                    
                    }
                    else {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::Slash, start_pos, state.get_position());
                        true
                    }
                }
                '%' => {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::Percent, start_pos, state.get_position());
                    true
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::DoubleArrow, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::NotEqual, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('<') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::LeftShift, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::ComposeBack, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::LessThan, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::RightShift, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::Compose, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::GreaterThan, start_pos, state.get_position());
                    }
                    true
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        if let Some('&') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::BitwiseAnd, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::AndAnd, start_pos, state.get_position());
                        }
                    }
                    else {
                        // 单个 & F# 中通常用于引用
                        state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
                    }
                    true
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        if let Some('|') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::BitwiseOr, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::OrOr, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Pipe, start_pos, state.get_position());
                    }
                    true
                }
                '^' => {
                    state.advance(1);
                    if let Some('^') = state.peek() {
                        state.advance(1);
                        if let Some('^') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::BitwiseXor, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
                    }
                    true
                }
                '~' => {
                    state.advance(1);
                    if let Some('~') = state.peek() {
                        state.advance(1);
                        if let Some('~') = state.peek() {
                            state.advance(1);
                            state.add_token(FSharpSyntaxKind::BitwiseNot, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
                        }
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
                    }
                    true
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        state.add_token(FSharpSyntaxKind::Cons, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(FSharpSyntaxKind::Colon, start_pos, state.get_position());
                    }
                    true
                }
                '@' => {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::At, start_pos, state.get_position());
                    true
                }
                '$' => {
                    state.advance(1);
                    state.add_token(FSharpSyntaxKind::Dollar, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => {
                    // 检查是否是块注释开                    if let Some('*') = state.peek_next_n(1) {
                    return false; // 让注释处理函数处                    }
                    FSharpSyntaxKind::LeftParen
                }
                ')' => FSharpSyntaxKind::RightParen,
                '[' => FSharpSyntaxKind::LeftBracket,
                ']' => FSharpSyntaxKind::RightBracket,
                '{' => FSharpSyntaxKind::LeftBrace,
                '}' => FSharpSyntaxKind::RightBrace,
                ',' => FSharpSyntaxKind::Comma,
                ';' => FSharpSyntaxKind::Semicolon,
                '.' => FSharpSyntaxKind::Dot,
                '?' => FSharpSyntaxKind::Question,
                '_' => FSharpSyntaxKind::Underscore,
                '`' => FSharpSyntaxKind::Backtick,
                '#' => FSharpSyntaxKind::Hash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理单位字面()
    fn lex_unit(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('(') = state.peek() {
            if let Some(')') = state.peek_next_n(1) {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::UnitLiteral, start_pos, state.get_position());
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

impl<'config> Lexer<FSharpLanguage> for FSharpLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<FSharpSyntaxKind> {
        let mut state = State::new(source);

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

            if self.lex_unit(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
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
                state.add_token(FSharpSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(FSharpSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
