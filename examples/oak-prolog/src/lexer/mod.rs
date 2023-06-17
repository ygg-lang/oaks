use crate::{kind::PrologSyntaxKind, language::PrologLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PrologLanguage>;

pub struct PrologLexer<'config> {
    config: &'config PrologLanguage,
}

impl<'config> PrologLexer<'config> {
    pub fn new(config: &'config PrologLanguage) -> Self {
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
            state.add_token(PrologSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(PrologSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PrologSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);

            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(PrologSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            // 多行注释 /* ... */
            state.advance(2);

            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(PrologSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        // 转义字符
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PrologSyntaxKind::String, start_pos, state.get_position());
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

                // 整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 小数部分
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
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

                // 科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
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

                state.add_token(PrologSyntaxKind::Integer, start_pos, state.get_position());
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

    /// 处理变量（以大写字母或下划线开头）
    fn lex_variable(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_uppercase() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(PrologSyntaxKind::Variable, start_pos, state.get_position());
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

    /// 处理原子（以小写字母开头的标识符）
    fn lex_atom(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_lowercase() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_range(start_pos, state.get_position());
                let kind = match text {
                    "is" | "mod" | "rem" | "div" | "abs" | "sign" | "float" | "floor" | "ceiling" | "round" | "truncate"
                    | "max" | "min" | "succ" | "plus" | "true" | "false" | "fail" | "cut" | "not" | "once" | "repeat"
                    | "halt" | "abort" | "trace" | "notrace" | "spy" | "nospy" | "debug" | "nodebug" | "write" | "read"
                    | "get" | "put" | "nl" | "tab" | "see" | "tell" | "seen" | "told" | "close" | "open" | "current_input"
                    | "current_output" | "set_input" | "set_output" | "flush_output" | "at_end_of_stream"
                    | "stream_property" | "assert" | "asserta" | "assertz" | "retract" | "retractall" | "abolish"
                    | "clause" | "current_predicate" | "predicate_property" | "functor" | "arg" | "univ" | "copy_term"
                    | "numbervars" | "term_variables" | "subsumes_term" | "compare" | "sort" | "keysort" | "length"
                    | "member" | "append" | "reverse" | "last" | "nth0" | "nth1" | "select" | "permutation" | "sublist"
                    | "prefix" | "suffix" | "findall" | "bagof" | "setof" | "forall" | "aggregate" | "aggregate_all" => {
                        PrologSyntaxKind::Keyword
                    }
                    _ => PrologSyntaxKind::Atom,
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

    /// 处理操作符
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 多字符操作符
        let multi_char_ops = [
            ":-", "?-", "-->", "=..", "\\=", "\\==", "=:=", "=\\=", "==", "\\=@=", "@<", "@=<", "@>", "@>=", "=<", ">=", "<<",
            ">>", "**", "//", "mod", "rem", "xor", "\\/", "/\\", "is", "=", "\\+", "->", ";", "|",
        ];

        for op in &multi_char_ops {
            if state.peek_string(op.len()) == Some(op.to_string()) {
                state.advance(op.len());
                state.add_token(PrologSyntaxKind::Operator, start_pos, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.peek() {
            match ch {
                '+' | '-' | '*' | '/' | '^' | '<' | '>' | '=' | '\\' | '~' | '@' => {
                    state.advance(1);
                    state.add_token(PrologSyntaxKind::Operator, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '(' | ')' | '[' | ']' | '{' | '}' | ',' | '.' | ';' | '|' | '!' => {
                    state.advance(1);
                    state.add_token(PrologSyntaxKind::Delimiter, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理引用原子（用单引号包围的原子）
    fn lex_quoted_atom(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    // 检查是否是转义的单引号
                    if state.peek_next_n(1) == Some('\'') {
                        state.advance(2); // 跳过 ''
                    }
                    else {
                        state.advance(1);
                        break;
                    }
                }
                else if ch == '\\' {
                    // 转义字符
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(PrologSyntaxKind::Atom, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PrologLanguage> for PrologLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PrologSyntaxKind> {
        let mut state = State::new(source);

        while !state.is_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_quoted_atom(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_variable(&mut state) {
                continue;
            }

            if self.lex_atom(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
            }
        }

        // 添加 EOF kind
        let pos = state.get_position();
        state.add_token(PrologSyntaxKind::Eof, pos, pos);

        state.finish()
    }
}
