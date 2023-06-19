use crate::{kind::CoqSyntaxKind, language::CoqLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

pub struct CoqLexer<'config> {
    config: &'config CoqLanguage,
}

impl<'config> CoqLexer<'config> {
    pub fn new(config: &'config CoqLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
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
            state.add_token(CoqSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CoqSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CoqSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('(') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                // Coq 多行注释 (* ... *)
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

                state.add_token(CoqSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    // 处理转义字符
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(CoqSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 继续读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过小数点
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                state.add_token(CoqSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                // 继续读取标识符字符
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否为关键字
                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text {
                    "Theorem" => CoqSyntaxKind::Theorem,
                    "Lemma" => CoqSyntaxKind::Lemma,
                    "Definition" => CoqSyntaxKind::Definition,
                    "Fixpoint" => CoqSyntaxKind::Fixpoint,
                    "Inductive" => CoqSyntaxKind::Inductive,
                    "Record" => CoqSyntaxKind::Record,
                    "Module" => CoqSyntaxKind::Module,
                    "Class" => CoqSyntaxKind::Class,
                    "Instance" => CoqSyntaxKind::Instance,
                    "Proof" => CoqSyntaxKind::Proof,
                    "Qed" => CoqSyntaxKind::Qed,
                    "End" => CoqSyntaxKind::End,
                    "match" => CoqSyntaxKind::Match,
                    "with" => CoqSyntaxKind::With,
                    "Type" => CoqSyntaxKind::Type,
                    "Set" => CoqSyntaxKind::Set,
                    "Prop" => CoqSyntaxKind::Prop,
                    "forall" => CoqSyntaxKind::Forall,
                    "fun" => CoqSyntaxKind::Fun,
                    "let" => CoqSyntaxKind::Let,
                    "in" => CoqSyntaxKind::In,
                    "if" => CoqSyntaxKind::If,
                    "then" => CoqSyntaxKind::Then,
                    "else" => CoqSyntaxKind::Else,
                    "intros" => CoqSyntaxKind::Intros,
                    "simpl" => CoqSyntaxKind::Simpl,
                    "reflexivity" => CoqSyntaxKind::Reflexivity,
                    "rewrite" => CoqSyntaxKind::Rewrite,
                    "apply" => CoqSyntaxKind::Apply,
                    "exact" => CoqSyntaxKind::Exact,
                    "assumption" => CoqSyntaxKind::Assumption,
                    "auto" => CoqSyntaxKind::Auto,
                    "trivial" => CoqSyntaxKind::Trivial,
                    "discriminate" => CoqSyntaxKind::Discriminate,
                    "injection" => CoqSyntaxKind::Injection,
                    "inversion" => CoqSyntaxKind::Inversion,
                    "destruct" => CoqSyntaxKind::Destruct,
                    "induction" => CoqSyntaxKind::Induction,
                    "generalize" => CoqSyntaxKind::Generalize,
                    "clear" => CoqSyntaxKind::Clear,
                    "unfold" => CoqSyntaxKind::Unfold,
                    "fold" => CoqSyntaxKind::Fold,
                    "compute" => CoqSyntaxKind::Compute,
                    "eval" => CoqSyntaxKind::Eval,
                    "Check" => CoqSyntaxKind::Check,
                    "Print" => CoqSyntaxKind::Print,
                    "Search" => CoqSyntaxKind::Search,
                    "Locate" => CoqSyntaxKind::Locate,
                    "About" => CoqSyntaxKind::About,
                    "Show" => CoqSyntaxKind::Show,
                    "Goal" => CoqSyntaxKind::Goal,
                    "Goals" => CoqSyntaxKind::Goals,
                    "Undo" => CoqSyntaxKind::Undo,
                    "Restart" => CoqSyntaxKind::Restart,
                    "Abort" => CoqSyntaxKind::Abort,
                    "Admit" => CoqSyntaxKind::Admit,
                    "Admitted" => CoqSyntaxKind::Admitted,
                    "Parameter" => CoqSyntaxKind::Parameter,
                    "Axiom" => CoqSyntaxKind::Axiom,
                    "Variable" => CoqSyntaxKind::Variable,
                    "Hypothesis" => CoqSyntaxKind::Hypothesis,
                    "Section" => CoqSyntaxKind::Section,
                    "Chapter" => CoqSyntaxKind::Chapter,
                    "Require" => CoqSyntaxKind::Require,
                    "Import" => CoqSyntaxKind::Import,
                    "Export" => CoqSyntaxKind::Export,
                    "Open" => CoqSyntaxKind::Open,
                    "Close" => CoqSyntaxKind::Close,
                    "Scope" => CoqSyntaxKind::Scope,
                    "Notation" => CoqSyntaxKind::Notation,
                    "Infix" => CoqSyntaxKind::Infix,
                    "Reserved" => CoqSyntaxKind::Reserved,
                    "Bind" => CoqSyntaxKind::Bind,
                    "Delimit" => CoqSyntaxKind::Delimit,
                    "Arguments" => CoqSyntaxKind::Arguments,
                    "Implicit" => CoqSyntaxKind::Implicit,
                    "Coercion" => CoqSyntaxKind::Coercion,
                    "Identity" => CoqSyntaxKind::Identity,
                    "Canonical" => CoqSyntaxKind::Canonical,
                    "Structure" => CoqSyntaxKind::Structure,
                    _ => CoqSyntaxKind::Identifier,
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

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter(&self, state: &mut LexerState<impl Source, CoqLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => {
                    state.advance(1);
                    CoqSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    CoqSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    CoqSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    CoqSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    CoqSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    CoqSyntaxKind::RightBrace
                }
                ':' => {
                    state.advance(1);
                    CoqSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    CoqSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    CoqSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    CoqSyntaxKind::Dot
                }
                '|' => {
                    state.advance(1);
                    CoqSyntaxKind::Pipe
                }
                '_' => {
                    state.advance(1);
                    CoqSyntaxKind::Underscore
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        CoqSyntaxKind::DoubleArrow
                    }
                    else {
                        CoqSyntaxKind::Equal
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        CoqSyntaxKind::Arrow
                    }
                    else {
                        CoqSyntaxKind::Minus
                    }
                }
                '+' => {
                    state.advance(1);
                    CoqSyntaxKind::Plus
                }
                '*' => {
                    state.advance(1);
                    CoqSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    CoqSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    CoqSyntaxKind::Percent
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CoqSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        CoqSyntaxKind::NotEqual
                    }
                    else {
                        CoqSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CoqSyntaxKind::GreaterEqual
                    }
                    else {
                        CoqSyntaxKind::Greater
                    }
                }
                '~' => {
                    state.advance(1);
                    CoqSyntaxKind::Tilde
                }
                '@' => {
                    state.advance(1);
                    CoqSyntaxKind::At
                }
                '?' => {
                    state.advance(1);
                    CoqSyntaxKind::Question
                }
                '!' => {
                    state.advance(1);
                    CoqSyntaxKind::Exclamation
                }
                '&' => {
                    state.advance(1);
                    CoqSyntaxKind::Ampersand
                }
                '#' => {
                    state.advance(1);
                    CoqSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    CoqSyntaxKind::Dollar
                }
                '\\' => {
                    state.advance(1);
                    CoqSyntaxKind::Backslash
                }
                '^' => {
                    state.advance(1);
                    CoqSyntaxKind::Caret
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CoqLanguage> for CoqLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CoqLanguage>,
    ) -> LexOutput<CoqLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
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
            if self.lex_comment(&mut state) {
                continue;
            }

            // 处理字符串字面量
            if self.lex_string(&mut state) {
                continue;
            }

            // 处理数字字面量
            if self.lex_number(&mut state) {
                continue;
            }

            // 处理标识符或关键字
            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            // 处理操作符和分隔符
            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，检查是否到达文件末尾
            if let Some(ch) = state.peek() {
                // 跳过当前字符并标记为错误
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(CoqSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 到达文件末尾，退出循环
                break;
            }
        }

        // 添加 EOF token
        let pos = state.get_position();
        state.add_token(CoqSyntaxKind::Eof, pos, pos);
        state.finish(Ok(()))
    }
}
