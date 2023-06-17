use crate::{kind::OCamlSyntaxKind, language::OCamlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, OCamlLanguage>;

pub struct OCamlLexer<'config> {
    config: &'config OCamlLanguage,
}

impl<'config> OCamlLexer<'config> {
    pub fn new(config: &'config OCamlLanguage) -> Self {
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
            state.add_token(OCamlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(OCamlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(OCamlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('(') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('(') = state.peek() {
                        if let Some('*') = state.peek_next_n(1) {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }

                    if let Some('*') = state.peek() {
                        if let Some(')') = state.peek_next_n(1) {
                            state.advance(2);
                            depth -= 1;
                            continue;
                        }
                    }

                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(OCamlSyntaxKind::Comment, start_pos, state.get_position());
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

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, source: &SourceText, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");

                let token_kind = match text {
                    "and" => OCamlSyntaxKind::And,
                    "as" => OCamlSyntaxKind::As,
                    "assert" => OCamlSyntaxKind::Assert,
                    "begin" => OCamlSyntaxKind::Begin,
                    "class" => OCamlSyntaxKind::Class,
                    "constraint" => OCamlSyntaxKind::Constraint,
                    "do" => OCamlSyntaxKind::Do,
                    "done" => OCamlSyntaxKind::Done,
                    "downto" => OCamlSyntaxKind::Downto,
                    "else" => OCamlSyntaxKind::Else,
                    "end" => OCamlSyntaxKind::End,
                    "exception" => OCamlSyntaxKind::Exception,
                    "external" => OCamlSyntaxKind::External,
                    "false" => OCamlSyntaxKind::False,
                    "for" => OCamlSyntaxKind::For,
                    "fun" => OCamlSyntaxKind::Fun,
                    "function" => OCamlSyntaxKind::Function,
                    "functor" => OCamlSyntaxKind::Functor,
                    "if" => OCamlSyntaxKind::If,
                    "in" => OCamlSyntaxKind::In,
                    "include" => OCamlSyntaxKind::Include,
                    "inherit" => OCamlSyntaxKind::Inherit,
                    "initializer" => OCamlSyntaxKind::Initializer,
                    "lazy" => OCamlSyntaxKind::Lazy,
                    "let" => OCamlSyntaxKind::Let,
                    "match" => OCamlSyntaxKind::Match,
                    "method" => OCamlSyntaxKind::Method,
                    "module" => OCamlSyntaxKind::Module,
                    "mutable" => OCamlSyntaxKind::Mutable,
                    "new" => OCamlSyntaxKind::New,
                    "object" => OCamlSyntaxKind::Object,
                    "of" => OCamlSyntaxKind::Of,
                    "open" => OCamlSyntaxKind::Open,
                    "or" => OCamlSyntaxKind::Or,
                    "private" => OCamlSyntaxKind::Private,
                    "rec" => OCamlSyntaxKind::Rec,
                    "sig" => OCamlSyntaxKind::Sig,
                    "struct" => OCamlSyntaxKind::Struct,
                    "then" => OCamlSyntaxKind::Then,
                    "to" => OCamlSyntaxKind::To,
                    "true" => OCamlSyntaxKind::True,
                    "try" => OCamlSyntaxKind::Try,
                    "type" => OCamlSyntaxKind::Type,
                    "val" => OCamlSyntaxKind::Val,
                    "virtual" => OCamlSyntaxKind::Virtual,
                    "when" => OCamlSyntaxKind::When,
                    "while" => OCamlSyntaxKind::While,
                    "with" => OCamlSyntaxKind::With,
                    _ => OCamlSyntaxKind::Identifier,
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

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点                let mut is_float = false;
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过 '.'

                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 处理指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);

                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
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

                let token_kind = if is_float { OCamlSyntaxKind::Float } else { OCamlSyntaxKind::Integer };

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

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(OCamlSyntaxKind::String, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符            state.add_token(OCamlSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符
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
                else {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(OCamlSyntaxKind::Char, start_pos, state.get_position());
                    return true;
                }
            }

            // 未闭合的字符
            state.add_token(OCamlSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operators(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    OCamlSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::Arrow
                    }
                    else {
                        OCamlSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    OCamlSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    OCamlSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    OCamlSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::DoubleArrow
                    }
                    else {
                        OCamlSyntaxKind::Equal
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::NotEqual
                    }
                    else {
                        OCamlSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::GreaterEqual
                    }
                    else {
                        OCamlSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::Assign
                    }
                    else {
                        OCamlSyntaxKind::Colon
                    }
                }
                '|' => {
                    state.advance(1);
                    OCamlSyntaxKind::Pipe
                }
                '&' => {
                    state.advance(1);
                    OCamlSyntaxKind::Ampersand
                }
                '!' => {
                    state.advance(1);
                    OCamlSyntaxKind::Exclamation
                }
                '?' => {
                    state.advance(1);
                    OCamlSyntaxKind::Question
                }
                ';' => {
                    state.advance(1);
                    OCamlSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    OCamlSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        OCamlSyntaxKind::DotDot
                    }
                    else {
                        OCamlSyntaxKind::Dot
                    }
                }
                '_' => {
                    state.advance(1);
                    OCamlSyntaxKind::Underscore
                }
                '`' => {
                    state.advance(1);
                    OCamlSyntaxKind::Backquote
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
                '(' => OCamlSyntaxKind::LeftParen,
                ')' => OCamlSyntaxKind::RightParen,
                '[' => OCamlSyntaxKind::LeftBracket,
                ']' => OCamlSyntaxKind::RightBracket,
                '{' => OCamlSyntaxKind::LeftBrace,
                '}' => OCamlSyntaxKind::RightBrace,
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

impl<'config> Lexer<OCamlLanguage> for OCamlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<OCamlSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(source, &mut state) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(OCamlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(OCamlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
